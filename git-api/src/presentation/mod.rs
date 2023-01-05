use crate::domain::{types::Result, Branch, CommitInfo};

pub struct SendCommand;
impl SendCommand {
    pub fn get_branches() -> Result<Vec<Branch>> {
        // application.get_branch()
        todo!()
    }
    pub fn get_commits(hash: &str) -> Result<Vec<CommitInfo>> {
        // application.get_commits(hash)
        todo!()
    }
}
