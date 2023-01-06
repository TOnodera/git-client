use crate::domain::Branch;

pub mod api;

pub trait InputData {
    // Empty
}

pub trait OutputData {
    // Empty
}

pub type GitBranchCommandInput = Option<()>;
impl InputData for GitBranchCommandInput {}

pub type GitBranchCommandOutput = Vec<Branch>;
impl OutputData for GitBranchCommandOutput {}
