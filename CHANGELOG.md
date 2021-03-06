# CHANGELOG

## 2020-09-14: Release 0.1.5

Ripped out the ad-hoc error and `compile_error!` injection and replaced it with the
`proc_macro_error` crate. This gave the opportunity to include notes in the compiler output and
generally improve the quality of error messages. A terrifying amount of refactoring was necessary to
get the right data in the right place for this.

`#[cfg]` and `#[doc_cfg]` attributes are now copied from the function to the derived impls. This
fixes the obvious (in retrospect) problem where the trait impl would always be created, and then
call a function which may not exist.

The documentation previously incorrectly claimed it supported `FromIterator`; this was a typo of
`IntoIterator`. `FromIterator` has a rather complex signature and since there is no simple and
obvious way to write the `#[zoet(FromIterator)]`-adorned function, it's not likely to be supported
any time soon.

## 2020-09-14: Release 0.1.5

Ripped out the ad-hoc error and `compile_error!` injection and replaced it with the
`proc_macro_error` crate. This gave the opportunity to include notes in the compiler output and
generally improve the quality of error messages. A terrifying amount of refactoring was necessary to
get the right data in the right place for this.

`#[cfg]` and `#[doc_cfg]` attributes are now copied from the function to the derived impls. This
fixes the obvious (in retrospect) problem where the trait impl would always be created, and then
call a function which may not exist.

The documentation previously incorrectly claimed it supported `FromIterator`; this was a typo for
`IntoIterator`. `FromIterator` has a rather complex signature and since there is no simple and
obvious way to write the `#[zoet(FromIterator)]`-adorned function, it's not likely to be supported
any time soon.

## 2019-11-06: Release 0.1.4

Added `#[allow(clippy::pedantic)]` to all generated impls. This allows you to turn on All The Lints
and find issues in your own code without having Clippy complain about macro-generated code that you
can't really do anything about.

Split into separate `zoet` and `zoet-macro` crates. This is a cascading change caused by wanting to
support `no_std` code by re-exporting names from the `alloc` crate, and thus needed the macro
implementation in a separate crate so the main crate could have `pub` items.

Implemented support for `Hash`.

## 2019-09-01: Release 0.1.3

Refactored to use `syn` 0.1.

Removed dependency on unstable features, so this macro can now be used in stable code. This also
means that `phf` is no longer used, which should improve compile times a little (but it's still lost
in the noise compared to `syn`.)

## 2019-08-26: Release 0.1.2

The example usage in the documentation didn't actually compile. This has been fixed.

The result-type parser now also accepts `Fallible`, as provided by the `failure` crate.

## 2019-08-23: Release 0.1.1

Expanded documentation somewhat.

The 0.1.0 release could generate implementations of `Ord`, `PartialEq` and `PartialOrd`, but these
were not documented. They now are.

`PartialOrd` now checks whether the function returns an `Option`, and if not, wraps the result in
`Some`. This allows one to apply `#zoet(Ord, PartialOrd)` to a single function to implement both
traits.

## 2019-07-19: Release 0.1.0

Initial release.
