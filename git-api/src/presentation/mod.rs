use serde::Serialize;

use crate::{
    application::{git_branch_usecase::GitBranchUsecase, CommandFactory, UsecaseTrait},
    domain::{types::Result, Branch, Env},
    infrastructure::command_factory::GitBranchCommandFactory,
};

pub trait InputData {
    // Empty
}

pub trait OutputData {
    // Empty
}

pub struct SendGitCommand {
    env: Env,
}
impl SendGitCommand {
    pub fn new(env: Env) -> Self {
        Self { env }
    }
    pub fn get_branches(&self) -> Result<Vec<Branch>> {
        let factory = GitBranchCommandFactory::new();
        let command = factory.create(None)?;
        let result = GitBranchUsecase::new().accept_command(command)?;
        Ok(result)
    }
}

pub struct GitBranchInput;
impl InputData for GitBranchInput {}
// #[derive(Debug, Clone, Serialize)]
// pub struct GitBranchOutput {
//     pub name: String,
// }
// impl OutputData for GitBranchOutput {}

pub type GitBranchCommandOutput = Vec<Branch>;
impl OutputData for GitBranchCommandOutput {}
#[cfg(test)]
mod SendGitCommandTest {
    use crate::domain::{types::Result, Env, EnvTrait};

    use super::SendGitCommand;

    #[test]
    fn get_branches() -> Result<()> {
        let api = SendGitCommand::new(Env::new("!!!"));
        api.get_branches();
        Ok(())
    }
}
