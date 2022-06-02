use core::marker::PhantomData;

use crate::bit::Bit;
use crate::cmds::eof::EOF;
use crate::core::{Command, Run, Runner, State};
use crate::list::{Cons, List, Nil};

/// Go left (`< <Next>` in SF) command.
/// If left [`List`] is [`Nil`], current bit will be from
/// `Default` attribute from state.
#[derive(Debug)]
pub struct Left<Next = EOF>(PhantomData<Next>)
where
    Next: Command;

impl<Next> Command for Left<Next> where Next: Command {}

impl<LeftNew, ValueNew, ValuePrev, RightPrev, Default, Next>
    Runner<Cons<ValueNew, LeftNew>, ValuePrev, RightPrev, Default> for Left<Next>
where
    LeftNew: List,
    ValueNew: Bit,
    ValuePrev: Bit,
    RightPrev: List,
    Next: Runner<LeftNew, ValueNew, Cons<ValuePrev, RightPrev>, Default>,
    Default: Bit,
{
    type Run = Run<Next, State<LeftNew, ValueNew, Cons<ValuePrev, RightPrev>, Default>>;
}

impl<Value, Right, Next, Default> Runner<Nil, Value, Right, Default> for Left<Next>
where
    Value: Bit,
    Right: List,
    Default: Bit,
    Next: Runner<Nil, Default, Cons<Value, Right>, Default>,
{
    type Run = Run<Next, State<Nil, Default, Cons<Value, Right>, Default>>;
}
