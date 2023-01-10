use crate::{
    domain::{handler::Handler, types::Result, value::Env},
    infrastructure::{
        command_factory::GitBranchCommandFactory,
        domain_service::git_branch_command_domain_service::GitBranchCommandDomainService,
        handler::GitBranchComandHandler,
    },
    presentation::{GitBranchCommandInput, GitBranchCommandOutput},
};

use super::{CommandFactory, UsecaseTrait};

pub struct GitBranchUsecase {
    env: Env,
}

impl UsecaseTrait for GitBranchUsecase {
    type Env = Env;
    type InputData = GitBranchCommandInput;
    type OutputData = GitBranchCommandOutput;

    fn new(env: Self::Env) -> Self {
        Self { env }
    }
    fn run(&self, _: Self::InputData) -> Result<Self::OutputData> {
        let factory = GitBranchCommandFactory::new(self.env.clone());
        let command = factory.create(None)?;
        let service = GitBranchCommandDomainService::new();
        let handler = GitBranchComandHandler::new(Some(service))?;
        Ok(handler.handle(command)?)
    }
}
