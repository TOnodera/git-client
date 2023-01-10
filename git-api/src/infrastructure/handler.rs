use crate::{
    domain::{handler::Handler, types::Result, CommandTrait},
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
    fn handle(&self, command: Self::Command) -> Result<Self::OutputData> {
        Ok(command.execute()?)
    }
}
