use crate::domain::Branch;

pub mod api;

pub trait InputData {
    // Empty
}

pub trait OutputData {
    // Empty
}

pub struct GitBranchInput;
impl InputData for GitBranchInput {}

pub type GitBranchCommandOutput = Vec<Branch>;
impl OutputData for GitBranchCommandOutput {}
