use crate::application::CommandFactory;
use crate::domain::types::Result;
use crate::domain::value::Env;
use crate::domain::CommandTrait;
use crate::presentation::GitBranchCommandInput;

use super::command::GitBranchCommand;
use super::domain_service::git_branch_command_domain_service::GitBranchCommandDomainService;

pub struct GitBranchCommandFactory {
    env: Env,
    service: GitBranchCommandDomainService,
}
impl CommandFactory for GitBranchCommandFactory {
    type CommandTrait = GitBranchCommand;
    type InputData = GitBranchCommandInput;
    type Env = Env;
    type DomainService = GitBranchCommandDomainService;
    fn new(env: Self::Env, service: Self::DomainService) -> Self {
        Self { env, service }
    }
    fn create(&self, _: Option<Self::InputData>) -> Result<Self::CommandTrait> {
        let command = GitBranchCommand::new(self.env.clone(), self.service.clone());
        Ok(command)
    }
}
