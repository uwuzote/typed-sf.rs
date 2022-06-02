use crate::bit::Bit;
use crate::core::{Command, Runner, State};
use crate::list::List;

/// End-of-file command. Returns current state.
pub struct EOF;

impl Command for EOF {}

impl<Left, Value, Right, Default> Runner<Left, Value, Right, Default> for EOF
where
    Left: List,
    Value: Bit,
    Right: List,
    Default: Bit,
{
    type Run = State<Left, Value, Right, Default>;
}
