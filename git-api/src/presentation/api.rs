use crate::{
    application::{git_branch_usecase::GitBranchUsecase, UsecaseTrait},
    domain::{types::Result, value::Env, Branch},
};

pub struct SendGitCommandApi {
    env: Env,
}
impl SendGitCommandApi {
    pub fn new(env: Env) -> Self {
        Self { env }
    }
    pub fn get_branches(&self) -> Result<Vec<Branch>> {
        let result = GitBranchUsecase::new().run(None)?;
        Ok(result)
    }
}

#[cfg(test)]
mod SendGitCommandApiTest {
    use crate::domain::{types::Result, value::Env, EnvTrait};

    use super::SendGitCommandApi;

    #[test]
    fn get_branches() -> Result<()> {
        let api = SendGitCommandApi::new(Env::new("!!!"));
        api.get_branches();
        Ok(())
    }
}
