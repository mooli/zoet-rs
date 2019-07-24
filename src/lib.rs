/*!
Adds `#[zoet]` macro to reduce boilerplate when implementing common traits.

If you are sick of writing `impl Deref for Bar` etc (and it not working because you confused it
with `AsRef`) and you would rather just implement these common traits as regular methods in your
`impl Bar` like in lesser languages, this crate is for you!

Unfortunately, it uses nightly features, so if you need to use the stable compiler, you are going
to have to wait in anticipation for the features to stablilise before you can use it.

It is superficially similar to the various derive macros such as [`derive_more`], except that
rather than generating traits based on the contents of a struct, it generates them based on
individual functions/methods. An example works better than a textual description ever would:

```
use zoet::zoet;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Length(usize);
#[zoet]
impl Length {
    #[zoet(Default)]              // generates `impl Default for Length`
    pub fn new() -> Self { Self(0) }

    #[zoet(From)]                 // generates `From<usize> for Length`
    fn from_usize(value: usize) -> Self { Self(value) }

    #[zoet(From)]                 // generates `From<Length> for usize`
    fn to_usize(self) -> usize { self.0 }

    #[zoet(AsRef, Borrow, Deref)] // generates all of those
    fn as_usize(&self) -> &usize { &self.0 }

    #[zoet(Add, AddAssign)]       // generates `impl Add for Length` and `impl AddAssign for Length`
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
}

let mut v = Length::default();
v += Length(1);
assert_eq!(v + Length(2), Length(3));
v += Length(4);
assert_eq!(v, Length(5));
assert_eq!(Length::from(v), Length(5));
```

Due to limitations in macro processing, you must add `#[zoet]` to your struct's impl block so that
the self type of its methods can be determined. This is obviously not necessary (or possible) for
free functions as they don't have a self type.

Transformations for most traits in the standard library are provided. Omitted are those which are
just marker traits (there's no code to generate), those which require multiple functions, and some
which don't quite seem worth it. The current list is as follows:

* `core::borrow`: `Borrow`, `BorrowMut`.
* `std::borrow`: `ToOwned`.
* `core::clone`: `Clone`.
* `core::convert`: `AsMut`, `AsRef`, `From`, `Into`, `TryFrom`, `TryInto`.
* `core::default`: `Default`.
* `core::fmt`: `Debug`, `Display`, `Write` (only the `write_str` method).
* `core::iterator`: `FromIterator`, `Iterator` (only the `next` method).
* `core::ops`: `Deref`, `DerefMut`, `Drop`, `Index`, `IndexMut`, plus all arithmetic and bit ops and
  assignment variants.
* `core::str`: `FromStr`.
* `std::string`: `ToString`.

If an arithmetic op sees an assignment-variant signature, it will generate a trivial implementation
which mutates its `mut self` and returns it. Oterwise, these traits simply generate the trait
boilerplate and forward the arguments as-is.

You are reminded that [`cargo-expand`] exists, and can be used to inspect the expanded text.

[`cargo-expand`]: https://crates.io/crates/cargo-expand
[`derive_more`]: https://crates.io/crates/derive_more

*/

// -- start of boilerplate that's generally pasted into the top of new projects -- //
//
// Turn the "allow" lints currently listed by `rustc -W help` (as of 2019-06-01) into warn lints,
// unless they're not useful:
//
#![cfg_attr(feature="clippy-insane", warn(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    bare_trait_objects,
    // box_pointers, // obsolete
    deprecated_in_future,
    // elided_lifetimes_in_paths, // suggests adding dubious <'_> noise everywhere
    ellipsis_inclusive_range_patterns, // `syn` sometimes generates these
    // - TODO: this became `warn` in June 2019, so perhaps we want to explicitly allow() it?
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    missing_copy_implementations, // too noisy; enable and inspect before release
    missing_debug_implementations, // too noisy; enable and inspect before release
    missing_doc_code_examples, // too noisy; enable and inspect before release
    // missing_docs, // too noisy; enable and inspect before release
    private_doc_tests, // broken; still complains if "private" item is pub-used
    //single_use_lifetimes, // gets confused too easily by macros
    trivial_casts,
    trivial_numeric_casts,
    // unreachable_pub, // too noisy; enable and inspect before release
    unsafe_code,
    // unstable_features, // silly; explicit use of #![feature] already indicates opt-in
    unused_extern_crates,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences,
))]
// Ditto for clippy lint categories (see https://github.com/rust-lang/rust-clippy):
#![cfg_attr(
    feature = "clippy-insane",
    warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo, clippy::restriction)
)]
// turn off individual noisy/buggy clippy lints:
#![cfg_attr(feature="clippy-insane", allow(
    clippy::missing_const_for_fn,
    // from clippy::restriction:
    clippy::implicit_return,    // bad style
    clippy::integer_arithmetic, // uh-huh
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items, // just moans about all public items
    clippy::shadow_reuse,                   // e.g. `let foo = bar(foo)`
    clippy::wildcard_enum_match_arm,
))]
// -- end of boilerplate that's generally pasted into the top of new projects -- //

// needed for phf_macros
#![feature(proc_macro_hygiene)]
// since we've ended up with a dependency on nightly anyway, we may as well fill our boots to tidy
// the code...
#![feature(box_patterns)]

extern crate proc_macro;

pub(crate) mod error;
pub(crate) mod function_args;
pub(crate) mod self_replacer;
pub(crate) mod traits;
pub(crate) mod with_tokens;
pub(crate) mod zoet;

pub(crate) mod preamble {
    pub(crate) use crate::{
        error::{Error, Result},
        function_args::{FunctionArgs, FunctionMeta},
        self_replacer::SelfReplacer,
        with_tokens::WithTokens,
    };
}

use quote::ToTokens;

/// The `#[zoet]` macro.
#[proc_macro_attribute]
pub fn zoet(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream
{
    match crate::zoet::zoet(&attr.into(), item.into()) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_token_stream().into(),
    }
}