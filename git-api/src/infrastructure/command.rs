use std::vec;

use crate::{
    domain::{types::Result, Branch, CommandTrait},
    presentation::GitBranchCommandOutput,
};

pub struct GitBranchCommand;
impl CommandTrait for GitBranchCommand {
    type Output = GitBranchCommandOutput;
    fn new() -> Self {
        Self
    }
    fn execute(&self) -> Result<Self::Output> {
        Ok(vec![Branch::new("test", "abcdefg")])
    }
}
