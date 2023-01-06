use crate::{
    domain::{types::Result, CommandTrait},
    presentation::{InputData, OutputData},
};

pub mod input_to_command;

pub trait CommandFactory<T: CommandTrait<O>, I: InputData, O: OutputData> {
    fn new() -> Self;
    fn create(&self, input: Option<I>) -> Result<T>;
}
