use core::marker::PhantomData;

use crate::bit::{False, True};
use crate::cmds::eof::EOF;
use crate::core::{Command, Run, Runner, State, StateTrait};
use crate::list::List;

/// Cycling (`[ <Body> ] <Next>` in SF) command.
/// If current [`Bit`][crate::Bit] is [`False`] -- executes `Next` [`Command`],
/// otherwise -- executes `Body` and itself again.
#[derive(Debug)]
pub struct Cycle<Body, Next = EOF>(PhantomData<(Body, Next)>)
where
    Body: Command,
    Next: Command;

impl<Body, Next> Command for Cycle<Body, Next>
where
    Body: Command,
    Next: Command,
{
}

impl<Left, Right, Body, Next> Runner<Left, False, Right> for Cycle<Body, Next>
where
    Left: List,
    Right: List,
    Body: Command,
    Next: Runner<Left, False, Right>,
{
    type Run = Run<Next, State<Left, False, Right>>;
}

impl<Left, Right, Body, Next> Runner<Left, True, Right> for Cycle<Body, Next>
where
    Left: List,
    Right: List,
    Body: Runner<Left, True, Right>,
    Next: Command,
    Cycle<Body, Next>: Runner<
        <<Body as Runner<Left, True, Right>>::Run as StateTrait>::Left,
        <<Body as Runner<Left, True, Right>>::Run as StateTrait>::Value,
        <<Body as Runner<Left, True, Right>>::Run as StateTrait>::Right,
    >,
{
    type Run = Run<Cycle<Body, Next>, Run<Body, State<Left, True, Right>>>;
}
