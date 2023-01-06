use crate::{
    domain::{types::Result, CommandTrait},
    presentation::{InputData, OutputData},
};

pub mod git_branch_usecase;

pub trait UsecaseTrait
where
    Self::InputData: InputData,
    Self::OutputData: OutputData,
{
    type InputData;
    type OutputData;
    fn new() -> Self;
    fn run(&self, input: Self::InputData) -> Result<Self::OutputData>;
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
