use crate::bit::{Bit, False};
use crate::list::{List, Nil};
use core::marker::PhantomData;

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
    /// 'Default' cell value ([`Bit`])
    type Default: Bit;

    /// `Real` value of type.
    fn new() -> Self;

    #[cfg(not(feature = "no_std"))]
    /// `Real`, represented with rust's objects value of type.
    fn val() -> RuntimeState;
}

/// Zipper-list, representing current state of program.
/// Consists of left-hand [`List`], Value -- [`Bit`] and right-hand [`List`].
#[derive(Debug)]
pub struct State<Left, Value, Right, Default>(Left, Value, Right, PhantomData<Default>)
where
    Left: List,
    Value: Bit,
    Right: List,
    Default: Bit;

impl<LeftG, ValueG, RightG, DefaultG> StateTrait for State<LeftG, ValueG, RightG, DefaultG>
where
    LeftG: List,
    ValueG: Bit,
    RightG: List,
    DefaultG: Bit,
{
    type Left = LeftG;
    type Value = ValueG;
    type Right = RightG;
    type Default = DefaultG;

    #[cfg(not(feature = "no_std"))]
    #[inline]
    fn val() -> RuntimeState {
        (LeftG::val(), ValueG::val(), RightG::val())
    }

    #[inline]
    fn new() -> Self {
        State(LeftG::new(), ValueG::new(), RightG::new(), PhantomData)
    }
}

/// Type-level enum of commands. Supertrait of [`Runner`]
pub trait Command {}

/// Represents result of running some [`Command`] on [`State`] via generic arguments.
pub trait Runner<Left, Value, Right, Default>: Command
where
    Left: List,
    Value: Bit,
    Right: List,
    Default: Bit,
{
    /// Result of running on [`State<Left, Value, Right>`].
    type Run: StateTrait;
}

/// Alias for running `Program` on `StateT`
pub type Run<Program, StateT = DefaultState> = <Program as Runner<
    <StateT as StateTrait>::Left,
    <StateT as StateTrait>::Value,
    <StateT as StateTrait>::Right,
    <StateT as StateTrait>::Default,
>>::Run;

/// Default state, alias for [`State`]`<`[`Nil`]`, Default, `[`Nil`]`, Default>`.
pub type DefaultState<Default = False> = State<Nil, Default, Nil, Default>;
