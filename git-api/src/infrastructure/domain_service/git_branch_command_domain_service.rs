use crate::domain::{Branch, DomainService};

pub struct GitBranchCommandDomainService;
impl GitBranchCommandDomainService {
    pub fn new() -> Self {
        Self
    }
    pub fn output(&self, command_result: &str) -> Vec<Branch> {
        todo!()
    }
}
impl DomainService for GitBranchCommandDomainService {}
