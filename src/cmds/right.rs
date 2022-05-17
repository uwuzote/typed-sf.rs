use core::marker::PhantomData;

use crate::bit::{Bit, False};
use crate::cmds::eof::EOF;
use crate::core::{Command, Run, Runner, State};
use crate::list::{Cons, List, Nil};

/// Go right (`> <Next>` in SF) command.
/// If right [`List`] is [`Nil`], current bit will be
/// `Filler` argument value (default -- [`False`])
#[derive(Debug)]
pub struct Right<Next = EOF, Filler = False>(PhantomData<(Next, Filler)>)
where
    Next: Command,
    Filler: Bit;

impl<Next, Filler> Command for Right<Next, Filler>
where
    Next: Command,
    Filler: Bit,
{
}

impl<LeftPrev, ValuePrev, ValueNew, RightNew, Next, Filler>
    Runner<LeftPrev, ValuePrev, Cons<ValueNew, RightNew>> for Right<Next, Filler>
where
    LeftPrev: List,
    ValuePrev: Bit,
    ValueNew: Bit,
    RightNew: List,
    Next: Runner<Cons<ValuePrev, LeftPrev>, ValueNew, RightNew>,
    Filler: Bit,
{
    type Run = Run<Next, State<Cons<ValuePrev, LeftPrev>, ValueNew, RightNew>>;
}

impl<Left, Value, Next, Filler> Runner<Left, Value, Nil> for Right<Next, Filler>
where
    Left: List,
    Value: Bit,
    Next: Runner<Cons<Value, Left>, Filler, Nil>,
    Filler: Bit,
{
    type Run = Run<Next, State<Cons<Value, Left>, Filler, Nil>>;
}
