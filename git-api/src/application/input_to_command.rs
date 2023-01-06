use crate::domain::command::GitBranchCommand;
use crate::domain::types::Result;
use crate::domain::CommandTrait;
use crate::presentation::{InputData, OutputData};

use super::CommandFactory;

pub struct ConvertToGitBranchCommand;
impl<T: CommandTrait<O>, I: InputData, O: OutputData> CommandFactory<T, I, O>
    for ConvertToGitBranchCommand
{
    fn new() -> Self {
        Self
    }

    fn create(&self, input: Option<I>) -> Result<T> {
        Ok(GitBranchCommand::new())
    }
}
