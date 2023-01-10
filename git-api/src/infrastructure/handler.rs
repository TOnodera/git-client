use crate::{
    domain::{error::AppError, handler::Handler, types::Result, value::Env, CommandTrait},
    presentation::GitBranchCommandOutput,
};

use super::{
    command::GitBranchCommand,
    domain_service::git_branch_command_domain_service::GitBranchCommandDomainService,
};

pub struct GitBranchComandHandler {
    service: GitBranchCommandDomainService,
}
impl Handler for GitBranchComandHandler {
    type DomainService = GitBranchCommandDomainService;
    type Command = GitBranchCommand;
    type OutputData = GitBranchCommandOutput;

    fn new(service: Option<Self::DomainService>) -> Result<Self> {
        match service {
            Some(service) => Ok(Self { service }),
            None => Err(Box::new(AppError::CommandError)),
        }
    }
    fn handle(&self, command: Self::Command) -> Result<Self::OutputData> {
        Ok(command.execute()?)
    }
}

#[cfg(test)]
mod GitBranchComandHandlerTest {}
