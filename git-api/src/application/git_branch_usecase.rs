use crate::{
    domain::{handler::Handler, types::Result},
    infrastructure::{
        command_factory::GitBranchCommandFactory,
        domain_service::git_branch_command_domain_service::GitBranchCommandDomainService,
        handler::GitBranchComandHandler,
    },
    presentation::{GitBranchCommandInput, GitBranchCommandOutput},
};

use super::{CommandFactory, UsecaseTrait};

pub struct GitBranchUsecase;

impl UsecaseTrait for GitBranchUsecase {
    type InputData = GitBranchCommandInput;
    type OutputData = GitBranchCommandOutput;

    fn new() -> Self {
        Self
    }
    fn run(&self, _: Self::InputData) -> Result<Self::OutputData> {
        let factory = GitBranchCommandFactory::new();
        let command = factory.create(None)?;
        let service = GitBranchCommandDomainService::new();
        let handler = GitBranchComandHandler::new(Some(service))?;
        Ok(handler.handle(command)?)
    }
}
