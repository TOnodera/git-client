use crate::presentation::OutputData;

use super::{types::Result, CommandTrait, DomainService};

pub trait Handler
where
    Self::Command: CommandTrait,
    Self::OutputData: OutputData,
    Self::DomainService: DomainService,
{
    type DomainService;
    type Command;
    type OutputData;
    fn new(service: Option<Self::DomainService>) -> Result<Self>
    where
        Self: Sized;
    fn handle(&self, command: Self::Command) -> Result<Self::OutputData>;
}
