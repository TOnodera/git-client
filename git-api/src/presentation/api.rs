use crate::{
    application::{git_branch_usecase::GitBranchUsecase, UsecaseTrait},
    domain::{types::Result, value::{Env, CommitInfo}, Branch},
};

// apiインタフェース
pub trait SendGitCommandApiTrait {

    type GetBranchesResponse;
    type GetCommitsResponse;
    fn new(env: Env) -> Self;
    fn get_branches(&self) -> Self::GetBranchesResponse;
    fn get_commits(&self, head: &str, num: Option<u32>) -> Self::GetCommitsResponse;
}


// インタフェースの実装
pub struct SendGitCommandApi {
    env: Env,
}
impl SendGitCommandApiTrait for SendGitCommandApi {
    type GetBranchesResponse = Result<Vec<Branch>>;
    type GetCommitsResponse = Result<Vec<CommitInfo>>;
    fn new(env: Env) -> Self {
        Self { env }
    }
    fn get_branches(&self) -> Self::GetBranchesResponse {
        let result = GitBranchUsecase::new(self.env.clone()).run(None)?;
        Ok(result)
    }
    fn get_commits(&self, head: &str, num: Option<u32>) -> Self::GetCommitsResponse {
        todo!()
    }
}

