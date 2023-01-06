use crate::presentation::OutputData;

use super::{types::Result, CommandTrait};

pub trait Handler
where
    Self::Command: CommandTrait,
    Self::OutputData: OutputData,
{
    type Command;
    type OutputData;
    fn new() -> Self;
    fn handle(&self, command: Self::Command) -> Result<Self::OutputData>;
}
