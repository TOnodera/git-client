use std::vec;

use crate::{
    domain::{types::Result, value::Env, Branch, CommandTrait},
    presentation::GitBranchCommandOutput,
};

pub struct GitBranchCommand {
    env: Env,
}
impl CommandTrait for GitBranchCommand {
    type Output = GitBranchCommandOutput;
    type Env = Env;
    fn new(env: Self::Env) -> Self {
        Self { env }
    }
    fn execute(&self) -> Result<Self::Output> {
        todo!()
    }
}
