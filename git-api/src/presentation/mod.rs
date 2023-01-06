use serde::{Deserialize, Serialize};

use crate::{
    application::input_to_command,
    domain::{types::Result, Branch, Env, EnvTrait},
};

pub trait InputData {
    // Empty
}

pub trait OutputData {
    // Empty
}

pub struct SendGitCommand;
impl SendGitCommand {
    pub fn new() -> Self {
        Self
    }
    pub fn get_branches(env: Env) -> Result<Vec<Branch>> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GitBranchOutput {
    pub name: String,
}
impl OutputData for GitBranchOutput {}
