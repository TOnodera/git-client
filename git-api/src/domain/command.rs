use crate::presentation::{GitBranchOutput, OutputData};

use super::{types::Result, CommandTrait};

pub struct GitBranchCommand;
impl<O: OutputData> CommandTrait<O> for GitBranchCommand {
    fn new() -> Self {
        Self
    }
    fn execute() -> Result<O> {
        Ok(GitBranchOutput {
            name: "test".to_string(),
        })
    }
}
