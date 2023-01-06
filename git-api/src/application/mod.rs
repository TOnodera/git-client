use crate::{
    domain::{types::Result, CommandTrait},
    presentation::{InputData, OutputData},
};

pub mod git_branch_usecase;

pub trait UsecaseTrait
where
    Self::OutputData: OutputData,
    Self::Command: CommandTrait,
{
    type Command;
    type OutputData;
    fn accept_command(&self, command: Self::Command) -> Result<Self::OutputData>;
}

pub trait CommandFactory
where
    Self::CommandTrait: CommandTrait,
    Self::InputData: InputData,
{
    type CommandTrait;
    type InputData;
    fn new() -> Self;
    fn create(&self, input: Option<Self::InputData>) -> Result<Self::CommandTrait>;
}
