use core::marker::PhantomData;

use crate::bit::Bit;
use crate::cmds::eof::EOF;
use crate::core::{Command, Run, Runner, State};
use crate::list::{Cons, List, Nil};

/// Go right (`> <Next>` in SF) command.
/// If right [`List`] is [`Nil`], current bit will be from
/// `Default` argument of state.
#[derive(Debug)]
pub struct Right<Next = EOF>(PhantomData<Next>)
where
    Next: Command;

impl<Next> Command for Right<Next> where Next: Command {}

impl<LeftPrev, ValuePrev, ValueNew, RightNew, Next, Default>
    Runner<LeftPrev, ValuePrev, Cons<ValueNew, RightNew>, Default> for Right<Next>
where
    LeftPrev: List,
    ValuePrev: Bit,
    ValueNew: Bit,
    RightNew: List,
    Next: Runner<Cons<ValuePrev, LeftPrev>, ValueNew, RightNew, Default>,
    Default: Bit,
{
    type Run = Run<Next, State<Cons<ValuePrev, LeftPrev>, ValueNew, RightNew, Default>>;
}

impl<Left, Value, Next, Default> Runner<Left, Value, Nil, Default> for Right<Next>
where
    Left: List,
    Value: Bit,
    Next: Runner<Cons<Value, Left>, Default, Nil, Default>,
    Default: Bit,
{
    type Run = Run<Next, State<Cons<Value, Left>, Default, Nil, Default>>;
}
