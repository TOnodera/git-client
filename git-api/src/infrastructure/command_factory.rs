use crate::application::CommandFactory;
use crate::domain::types::Result;
use crate::presentation::GitBranchInput;

use super::command::GitBranchCommand;

pub struct GitBranchCommandFactory;
impl CommandFactory for GitBranchCommandFactory {
    type CommandTrait = GitBranchCommand;
    type InputData = GitBranchInput;
    fn new() -> Self {
        Self
    }
    fn create(&self, _: Option<Self::InputData>) -> Result<Self::CommandTrait> {
        todo!()
    }
}
