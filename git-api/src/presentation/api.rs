use crate::{
    application::{git_branch_usecase::GitBranchUsecase, CommandFactory, UsecaseTrait},
    domain::{types::Result, Branch, Env},
    infrastructure::command_factory::GitBranchCommandFactory,
};

pub struct SendGitCommandApi {
    env: Env,
}
impl SendGitCommandApi {
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

#[cfg(test)]
mod SendGitCommandApiTest {
    use crate::domain::{types::Result, Env, EnvTrait};

    use super::SendGitCommandApi;

    #[test]
    fn get_branches() -> Result<()> {
        let api = SendGitCommandApi::new(Env::new("!!!"));
        api.get_branches();
        Ok(())
    }
}
