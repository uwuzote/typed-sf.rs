#![cfg_attr(feature = "no_std", no_std)]
#![cfg_attr(feature = "require_docs", deny(missing_docs))]
#![deny(warnings, clippy::all, clippy::pedantic)]
#![doc(test(attr(deny(warnings))))]

/*!
    Type-level implementation of [SF](https://esolangs.org/wiki/Smallfuck).
    Proving that type-system turing-complete. Highly inspired
    by [this article](https://sdleffler.github.io/RustTypeSystemTuringComplete/).

    # Features
    `typed-sf` supports "no_std" feature with limitations. Runtime
    representation of [`List`]'s and [`State`][StateTrait]'s unavailable
    (they are using [`Vec`] to represent themselves).

    Full list of unsupported features:
    * [`List::val()`] (That includes [`Nil`] and [`Cons`]).
    * [`RuntimeState`].
    * [`StateTrait::val()`] (That includes [`State`] and [`DefaultState`]).

    Also where is "require_docs" feature which forbids undocumented `pub` items.
    If You want to contrubute, feel free to turn off this default feature.

    # How it works
    First, there's some traits that i'm calling type-level enums ([`Bit`], [`List`]).
    Have a look at it:
    ```
    # #[derive(PartialEq, Debug)]
    enum Bit {
        True,
        False
    }

    impl Bit {
        fn neg(self) -> Bit {
            match self {
                Bit::True => Bit::False,
                Bit::False => Bit::True
            }
        }
    }

    assert_eq!(Bit::True, Bit::False.neg());
    assert_eq!(Bit::False, Bit::True.neg());
    ```
    ```
    trait Bit {
        type Neg: Bit;
        fn real() -> bool;
    }
    struct True;
    struct False;

    impl Bit for True {
        type Neg = False;
        fn real() -> bool { true }
    }

    impl Bit for False {
        type Neg = True;
        fn real() -> bool { false }
    }

    assert_eq!(True::real(), <False as Bit>::Neg::real());
    assert_eq!(False::real(), <True as Bit>::Neg::real());
    ```
    Of course, what's more bolierplate, but also instead of
    matching in runtime, compiler solves type-equasions in
    compile-time for us.

    To type-encode functions i'm using something like this:
    ```
    # use std::ops::{Add, Sub};
    #
    // A and B are arguments to 'function'.
    trait Call<A, B> { type Return; }

    struct Sum;
    struct Diff;

    impl<A, B> Call<A, B> for Sum
    where
        A: Add<B>
    {
        type Return = <A as Add<B>>::Output;
    }

    impl<A, B> Call<A, B> for Diff
    where
        A: Sub<B>
    {
        type Return = <A as Sub<B>>::Output;
    }

    # #[allow(dead_code)]
    type Apply<A, Fn, B> = <Fn as Call<A, B>>::Return;
    ```
    What we can use later as:
    ```
    # use std::ops::{Add, Sub};
    # trait Call<A, B> { type Return; }
    # struct Sum;
    # struct Diff;
    # impl<A, B> Call<A, B> for Sum where A: Add<B> { type Return = <A as Add<B>>::Output; }
    # impl<A, B> Call<A, B> for Diff where A: Sub<B> { type Return = <A as Sub<B>>::Output; }
    # type Apply<A, Fn, B> = <Fn as Call<A, B>>::Return;
    #
    struct X;
    struct Y;
    # #[derive(Debug, PartialEq)]
    struct XaddY;
    # #[derive(Debug, PartialEq)]
    struct XsubY;

    impl Add<Y> for X {
        type Output = XaddY;
        # #[allow(unused)]
        # fn add(self, y: Y) -> XaddY {unimplemented!()}
    }
    impl Sub<Y> for X {
        type Output = XsubY;
        # #[allow(unused)]
        # fn sub(self, y: Y) -> XsubY {unimplemented!()}
    }

    /* Add `::val() -> Self` function to all types... */
    # impl XaddY { fn val() -> Self {XaddY} }
    # impl XsubY { fn val() -> Self {XsubY} }

    assert_eq!(
        <Apply<X, Sum, Y>>::val(),
        XaddY::val()
    );

    assert_eq!(
        <Apply<X, Diff, Y>>::val(),
        XsubY::val()
    );
    ```
    That's basically how [`Runner`]`<`[`Left`][List]`, `[`Value`][Bit]`, `[`Right`][List]`>` works.
    But commands such as [`Left`] take a generic argument, so it runs next command on
    modified state which does the same and so on...

    # Examples
    Basic usage
    ```
    # use typed_sf::*;
    type SetTrue<Next = EOF> = Cycle<Flip, Flip<Next>>;
    // [*<<[*]*>>>] // Move any-sized chunk of True's 2 cells left
    # #[allow(non_camel_case_types)]
    type prog = Cycle<Flip<Left<Left<SetTrue<Right<Right<Right>>>>>>>;
    # #[allow(non_camel_case_types)]
    type result = Run<
        prog,
        State<Nil, True, Cons<True, Nil>>
    >;

    assert_eq!(
        <result as StateTrait>::val(),
        (vec![true, true, false, false], false, Vec::new())
    ); //                 ^^^^^^^^^^^^
       // Some waste, honestly don't why it is here...
    ```
*/

#[cfg(test)]
#[cfg(not(feature = "no_std"))]
mod tests;
#[cfg(test)]
#[cfg(feature = "no_std")]
#[path = "tests_no_std.rs"]
mod tests;

pub(crate) mod bit;
pub(crate) mod core;
pub(crate) mod list;
pub(crate) mod cmds {
    pub(crate) mod cycle;
    pub use self::cycle::Cycle;

    pub(crate) mod eof;
    pub use self::eof::EOF;

    pub(crate) mod flip;
    pub use self::flip::Flip;

    pub(crate) mod left;
    pub use left::Left;

    pub(crate) mod right;
    pub use right::Right;

    pub(crate) mod utils;
    pub use utils::*;
}

pub use crate::bit::*;
pub use crate::cmds::utils::*;
pub use crate::cmds::*;
pub use crate::core::*;
pub use crate::list::*;

#[cfg(not(feature = "no_std"))]
pub use crate::core::RuntimeState;
