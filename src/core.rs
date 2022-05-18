use crate::bit::{Bit, False};
use crate::list::{List, Nil};

#[cfg(not(feature = "no_std"))]
/// Real value of any [`State`].
pub type RuntimeState = (Vec<bool>, bool, Vec<bool>);

/// Single-type trait for [`State`] type, used to get Left, Value
/// and Rigth types.
pub trait StateTrait {
    /// Left type ([`List`]).
    type Left: List;
    /// Value type ([`Bit`]).
    type Value: Bit;
    /// Right type ([`List`]).
    type Right: List;

    /// `Real` value of type.
    fn new() -> Self;

    #[cfg(not(feature = "no_std"))]
    /// `Real`, represented with rust's objects value of type.
    fn val() -> RuntimeState;
}

/// Zipper-list, representing current state of program.
/// Consists of left-hand [`List`], Value -- [`Bit`] and right-hand [`List`].
#[derive(Debug)]
pub struct State<Left, Value, Right>(Left, Value, Right)
where
    Left: List,
    Value: Bit,
    Right: List;

impl<LeftG, ValueG, RightG> StateTrait for State<LeftG, ValueG, RightG>
where
    LeftG: List,
    ValueG: Bit,
    RightG: List,
{
    type Left = LeftG;
    type Value = ValueG;
    type Right = RightG;

    #[cfg(not(feature = "no_std"))]
    #[inline(always)]
    fn val() -> RuntimeState {
        (LeftG::val(), ValueG::val(), RightG::val())
    }

    #[inline(always)]
    fn new() -> Self {
        State(LeftG::new(), ValueG::new(), RightG::new())
    }
}

/// Type-level enum of commands. Supertrait of [`Runner`]
pub trait Command {}

/// Represents result of running some [`Command`] on [`State`] via generic arguments.
pub trait Runner<Left, Value, Right>: Command
where
    Left: List,
    Value: Bit,
    Right: List,
{
    /// Result of running on [`State<Left, Value, Right>`].
    type Run: StateTrait;
}

/// Alias for running `Program` on `StateT`
pub type Run<Program, StateT = DefaultState> = <Program as Runner<
    <StateT as StateTrait>::Left,
    <StateT as StateTrait>::Value,
    <StateT as StateTrait>::Right,
>>::Run;

/// Default state, alias for [`State`]`<`[`Nil`]`, Value, `[`Nil`]`>`.
type DefaultState<Value = False> = State<Nil, Value, Nil>;