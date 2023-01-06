use crate::domain::command::GitBranchCommand;
use crate::domain::types::Result;
use crate::presentation::GitBranchInput;

use super::CommandFactory;

pub struct ConvertToGitBranchCommand;
impl CommandFactory for ConvertToGitBranchCommand {
    type CommandTrait = GitBranchCommand;
    type InputData = GitBranchInput;
    fn new() -> Self {
        Self
    }
    fn create(&self, _: Option<Self::InputData>) -> Result<Self::CommandTrait> {
        todo!()
    }
}
