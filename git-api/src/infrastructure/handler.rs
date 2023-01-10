use crate::{
    domain::{handler::Handler, types::Result, value::Env, CommandTrait},
    presentation::GitBranchCommandOutput,
};

use super::{
    command::GitBranchCommand,
    domain_service::git_branch_command_domain_service::GitBranchCommandDomainService,
};

pub struct GitBranchComandHandler {
    service: GitBranchCommandDomainService,
    env: Env,
}
impl Handler for GitBranchComandHandler {
    type Env = Env;
    type DomainService = GitBranchCommandDomainService;
    type Command = GitBranchCommand;
    type OutputData = GitBranchCommandOutput;

    fn new(env: Self::Env, service: Option<Self::DomainService>) -> Result<Self> {
        match service {
            Some(service) => Ok(Self { service, env }),
            None => Err(()),
        }
    }
    fn handle(&self, command: Self::Command) -> Result<Self::OutputData> {
        Ok(command.execute()?)
    }
}
