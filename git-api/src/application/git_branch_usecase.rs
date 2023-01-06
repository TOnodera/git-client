use crate::{
    domain::{handler::Handler, CommandTrait},
    infrastructure::command::GitBranchCommand,
    infrastructure::handler::GitBranchComandHandler,
    presentation::GitBranchCommandOutput,
};

use super::UsecaseTrait;

pub struct GitBranchUsecase;

impl UsecaseTrait for GitBranchUsecase {
    type Command = GitBranchCommand;
    type OutputData = GitBranchCommandOutput;

    fn new() -> Self {
        Self
    }
    fn accept_command(
        &self,
        command: Self::Command,
    ) -> crate::domain::types::Result<Self::OutputData> {
        let handler = GitBranchComandHandler::new();
        Ok(handler.handle(command)?)
    }
}
