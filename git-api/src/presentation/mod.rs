use serde::{Deserialize, Serialize};

use crate::domain::{types::Result, Branch, Env, EnvTrait};

pub trait InputData {
    // Empty
}

pub trait OutputData {
    // Empty
}

pub struct SendGitCommand {
    env: Env,
}
impl SendGitCommand {
    pub fn new(env: Env) -> Self {
        Self { env }
    }
    pub fn get_branches() -> Result<Vec<Branch>> {
        todo!()
    }
}

pub struct GitBranchInput;
impl InputData for GitBranchInput {}
#[derive(Debug, Clone, Serialize)]
pub struct GitBranchOutput {
    pub name: String,
}
impl OutputData for GitBranchOutput {}
