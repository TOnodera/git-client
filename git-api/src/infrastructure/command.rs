use crate::{
    domain::{error::AppError, types::Result, value::Env, CommandTrait},
    presentation::GitBranchCommandOutput,
};

use super::domain_service::git_branch_command_domain_service::GitBranchCommandDomainService;

pub struct GitBranchCommand {
    env: Env,
    service: GitBranchCommandDomainService,
}
impl CommandTrait for GitBranchCommand {
    type Output = GitBranchCommandOutput;
    type Env = Env;
    type DomainService = GitBranchCommandDomainService;
    fn new(env: Self::Env, service: Self::DomainService) -> Self {
        Self { env, service }
    }
    fn execute(&self) -> Result<Self::Output> {
        // git branch実行
        let result = std::process::Command::new("git")
            .arg(format!("--git-dir={}", self.env.git_dir))
            .arg("branch")
            .output()?;
        // エラーの場合は早期リターン
        if !result.status.success() {
            return Err(Box::new(AppError::CommandError));
        }
        // パース
        Ok(self.service.output(result.stdout))
    }
}
