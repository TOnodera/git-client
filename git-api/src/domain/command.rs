use crate::presentation::GitBranchOutput;

use super::{types::Result, CommandTrait};

pub struct GitBranchCommand;
impl CommandTrait for GitBranchCommand {
    type Output = GitBranchOutput;
    fn new() -> Self {
        Self
    }
    fn execute() -> Result<Self::Output> {
        Ok(GitBranchOutput {
            name: "test".to_string(),
        })
    }
}
