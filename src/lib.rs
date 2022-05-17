#![cfg_attr(feature = "no_std", no_std)]
#![cfg_attr(feature = "require_docs", forbid(missing_docs))]
#![deny(warnings, clippy::all, clippy::pedantic)]

/*!
    Type-level implementation of [SF](https://esolangs.org/wiki/Smallfuck).
    Prooving that type-system turing-complete. Higtly inspired
    by [this article](https://sdleffler.github.io/RustTypeSystemTuringComplete/).

    # Features
    `typed-sf` supports "no_std" feature with limitations. Runtime
    representation of [`List`]'s and [`State`][StateTrait]'s unavaliable
    (they are using [`Vec`] to represent theyself).

    Full list of unsupported features:
    * [`List::val()`] (That includes [`Nil`] and [`Cons`]).
    * [`RuntimeState`].
    * [`StateTrait::val()`] (That includes [`State`] and [`DefaultState`]).

    Also where is "require_docs" feature which forbids undocumented `pub` items.
    If You want to contrubute, feel free to turn off this default feature.

    # How it works
    First, where's some traits what i'm calling type-level enums ([`Bit`], [`List`]).
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
        # fn add(self, y: Y) -> XaddY {unimplemented!()}
    }
    impl Sub<Y> for X {
        type Output = XsubY;
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
    But commands such as [`Left`] takes an generic argument, so it runs next command on
    modified state which does same and so on...

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
    pub(crate) mod eof;
    pub(crate) mod flip;
    pub(crate) mod left;
    pub(crate) mod right;
    pub(crate) mod utils;
}

pub use crate::bit::{Bit, False, True};
pub use crate::cmds::utils::{SetFalse, SetTrue};
pub use crate::cmds::{cycle::Cycle, eof::EOF, flip::Flip, left::Left, right::Right};
pub use crate::core::{Command, DefaultState, Run, Runner, State, StateTrait};
pub use crate::list::{Cons, List, Nil};

#[cfg(not(feature = "no_std"))]
pub use crate::core::RuntimeState;
