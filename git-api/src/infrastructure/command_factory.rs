use crate::application::CommandFactory;
use crate::domain::types::Result;
use crate::domain::value::Env;
use crate::domain::CommandTrait;
use crate::presentation::GitBranchCommandInput;

use super::command::GitBranchCommand;

pub struct GitBranchCommandFactory {
    env: Env,
}
impl CommandFactory for GitBranchCommandFactory {
    type CommandTrait = GitBranchCommand;
    type InputData = GitBranchCommandInput;
    type Env = Env;
    fn new(env: Self::Env) -> Self {
        Self { env }
    }
    fn create(&self, _: Option<Self::InputData>) -> Result<Self::CommandTrait> {
        let command = GitBranchCommand::new(self.env.clone());
        Ok(command)
    }
}
