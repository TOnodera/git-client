use crate::{
    domain::CommandTrait, infrastructure::command::GitBranchCommand, presentation::GitBranchOutput,
};

use super::UsecaseTrait;

pub struct GitBranchUsecase;

impl UsecaseTrait for GitBranchUsecase {
    type Command = GitBranchCommand;
    type OutputData = GitBranchOutput;

    fn accept_command(
        &self,
        command: Self::Command,
    ) -> crate::domain::types::Result<Self::OutputData> {
        todo!()
    }
}
