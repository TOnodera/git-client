use crate::{
    domain::{types::Result, CommandTrait},
    presentation::GitBranchOutput,
};

pub struct GitBranchCommand;
impl CommandTrait for GitBranchCommand {
    type Output = GitBranchOutput;
    fn new() -> Self {
        Self
    }
    fn execute(&self) -> Result<Self::Output> {
        Ok(GitBranchOutput {
            name: "test".to_string(),
        })
    }
}
