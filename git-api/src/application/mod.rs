use crate::{
    domain::{types::Result, CommandTrait},
    presentation::InputData,
};

pub mod input_to_command;

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
