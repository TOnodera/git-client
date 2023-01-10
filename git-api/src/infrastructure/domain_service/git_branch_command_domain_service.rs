use crate::domain::{Branch, DomainService};

#[derive(Clone)]
pub struct GitBranchCommandDomainService;
impl GitBranchCommandDomainService {
    pub fn new() -> Self {
        Self
    }
    pub fn output(&self, command_result: Vec<u8>) -> Vec<Branch> {
        todo!()
    }
}
impl DomainService for GitBranchCommandDomainService {}
