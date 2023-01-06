use crate::{
    domain::{handler::Handler, CommandTrait},
    presentation::GitBranchCommandOutput,
};

use super::command::GitBranchCommand;

pub struct GitBranchComandHandler;
impl Handler for GitBranchComandHandler {
    type Command = GitBranchCommand;
    type OutputData = GitBranchCommandOutput;

    fn new() -> Self {
        Self
    }
    fn handle(&self, command: Self::Command) -> crate::domain::types::Result<Self::OutputData> {
        Ok(command.execute()?)
    }
}
