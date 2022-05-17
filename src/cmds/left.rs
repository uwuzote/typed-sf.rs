use core::marker::PhantomData;

use crate::bit::{Bit, False};
use crate::cmds::eof::EOF;
use crate::core::{Command, Run, Runner, State};
use crate::list::{Cons, List, Nil};

/// Go left (`< <Next>` in SF) command.
/// If left [`List`] is [`Nil`], current bit will be
/// `Filler` argument value (default -- [`False`])
#[derive(Debug)]
pub struct Left<Next = EOF, Filler = False>(PhantomData<(Next, Filler)>)
where
    Next: Command,
    Filler: Bit;

impl<Next, Filler> Command for Left<Next, Filler>
where
    Next: Command,
    Filler: Bit,
{
}

impl<LeftNew, ValueNew, ValuePrev, RightPrev, Filler, Next>
    Runner<Cons<ValueNew, LeftNew>, ValuePrev, RightPrev> for Left<Next, Filler>
where
    LeftNew: List,
    ValueNew: Bit,
    ValuePrev: Bit,
    RightPrev: List,
    Next: Runner<LeftNew, ValueNew, Cons<ValuePrev, RightPrev>>,
    Filler: Bit,
{
    type Run = Run<Next, State<LeftNew, ValueNew, Cons<ValuePrev, RightPrev>>>;
}

impl<Value, Right, Next, Filler> Runner<Nil, Value, Right> for Left<Next, Filler>
where
    Value: Bit,
    Right: List,
    Filler: Bit,
    Next: Runner<Nil, Filler, Cons<Value, Right>>,
{
    type Run = Run<Next, State<Nil, Filler, Cons<Value, Right>>>;
}
