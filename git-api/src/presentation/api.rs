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
