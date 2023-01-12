use crate::{
    application::{git_branch_usecase::GitBranchUsecase, UsecaseTrait},
    domain::{types::Result, value::Env, Branch},
};

// apiインタフェース
pub trait SendGitCommandApiTrait {
    type Response;
    fn new(env: Env) -> Self;
    fn get_branches(&self) -> Self::Response;
}

// インタフェースの実装
pub struct SendGitCommandApi {
    env: Env,
}
impl SendGitCommandApiTrait for SendGitCommandApi {
    type Response = Result<Vec<Branch>>;
    fn new(env: Env) -> Self {
        Self { env }
    }
    fn get_branches(&self) -> Self::Response {
        let result = GitBranchUsecase::new(self.env.clone()).run(None)?;
        Ok(result)
    }
}
