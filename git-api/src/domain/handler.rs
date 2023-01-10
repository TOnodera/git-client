use crate::presentation::OutputData;

use super::{types::Result, CommandTrait, DomainService, EnvTrait};

pub trait Handler
where
    Self::Command: CommandTrait,
    Self::OutputData: OutputData,
    Self::DomainService: DomainService,
    Self::Env: EnvTrait,
{
    type DomainService;
    type Command;
    type OutputData;
    type Env;
    fn new(env: Self::Env, service: Option<Self::DomainService>) -> Result<Self>
    where
        Self: Sized;
    fn handle(&self, command: Self::Command) -> Result<Self::OutputData>;
}
