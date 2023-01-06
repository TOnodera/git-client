use crate::presentation::OutputData;

use super::{types::Result, CommandTrait};

pub trait Handler<T: CommandTrait, O: OutputData> {
    fn handle(command: T) -> Result<O>;
}
