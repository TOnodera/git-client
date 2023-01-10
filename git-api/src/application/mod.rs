use crate::{
    domain::{types::Result, CommandTrait, EnvTrait},
    presentation::{InputData, OutputData},
};

pub mod git_branch_usecase;

pub trait UsecaseTrait
where
    Self::InputData: InputData,
    Self::OutputData: OutputData,
    Self::Env: EnvTrait,
{
    type Env;
    type InputData;
    type OutputData;
    fn new(env: Self::Env) -> Self;
    fn run(&self, input: Self::InputData) -> Result<Self::OutputData>;
}

pub trait CommandFactory
where
    Self::CommandTrait: CommandTrait,
    Self::InputData: InputData,
    Self::Env: EnvTrait,
{
    type CommandTrait;
    type InputData;
    type Env;
    fn new(env: Self::Env) -> Self;
    fn create(&self, input: Option<Self::InputData>) -> Result<Self::CommandTrait>;
}
