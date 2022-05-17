use crate::bit::Bit;
use crate::core::{Command, Runner, State};
use crate::list::List;

/// End-of-file command. Returns current state.
pub struct EOF;

impl Command for EOF {}

impl<Left, Value, Right> Runner<Left, Value, Right> for EOF
where
    Left: List,
    Value: Bit,
    Right: List,
{
    type Run = State<Left, Value, Right>;
}
