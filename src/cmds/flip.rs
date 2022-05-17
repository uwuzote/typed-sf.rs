use core::marker::PhantomData;

use crate::bit::Bit;
use crate::cmds::eof::EOF;
use crate::core::{Command, Run, Runner, State};
use crate::list::List;

/// Flip current [`Bit`] (`* <Next>` in SF) command.
/// Actually, just uses [`Bit::Neg`] type.
#[derive(Debug)]
pub struct Flip<Next = EOF>(PhantomData<Next>)
where
    Next: Command;

impl<Next> Command for Flip<Next> where Next: Command {}

impl<Left, Value, Right, Next> Runner<Left, Value, Right> for Flip<Next>
where
    Left: List,
    Value: Bit,
    Right: List,
    Next: Runner<Left, Value::Neg, Right>,
{
    type Run = Run<Next, State<Left, Value::Neg, Right>>;
}
