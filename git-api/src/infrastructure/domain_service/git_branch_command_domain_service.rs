use regex::Regex;

use crate::domain::{error::AppError, types::Result, Branch, DomainService};

#[derive(Clone)]
pub struct GitBranchCommandDomainService;
impl GitBranchCommandDomainService {
    pub fn new() -> Self {
        Self
    }
    pub fn parse(&self, command_result: Vec<u8>) -> Result<Vec<Branch>> {
        let output = std::str::from_utf8(&command_result)?;
        let re = Regex::new(r"^\*?\s+(.+?)\s+(\w+?)\s+(.+)$")?;
        let mut result = Vec::<Branch>::new();
        for line in output.lines() {
            if let Some(captures) = re.captures(line) {
                let name = &captures[1];
                let head = &captures[2];
                let is_current = Regex::new(r"^\*")?.is_match(line);
                result.push(Branch::new(name, head, is_current));
            }
        }
        Ok(result)
    }
}
impl DomainService for GitBranchCommandDomainService {}

#[cfg(test)]
mod GitBranchCommandDomainServiceTest {
    use crate::domain::types::Result;

    use super::GitBranchCommandDomainService;

    #[test]
    fn git_branchコマンドを配列に変換できる() -> Result<()> {
        let output = r#"
  main            0d65e2eb24259c418682d723cd547b39fcb6e0d1 comment
  feature/branch1 0d65e2eb24259c418682d723cd547b39fcb6e0d2 comment 
  feature/branch2 0d65e2eb24259c418682d723cd547b39fcb6e0d3 comment
* develop         0d65e2eb24259c418682d723cd547b39fcb6e0d4 comment
  staging         0d65e2eb24259c418682d723cd547b39fcb6e0d5 comment
  "#
        .as_bytes()
        .to_vec();

        // gitコマンドの出力をサービスオブジェクトで変換する
        let service = GitBranchCommandDomainService::new();
        let result = service.parse(output)?;

        let main = result.get(0).unwrap();
        assert_eq!(main.is_current, false);
        assert_eq!(main.name, "main");
        assert_eq!(main.head.hash, "0d65e2eb24259c418682d723cd547b39fcb6e0d1");

        let feature_branch1 = result.get(1).unwrap();
        assert_eq!(feature_branch1.is_current, false);
        assert_eq!(feature_branch1.name, "feature/branch1");
        assert_eq!(
            feature_branch1.head.hash,
            "0d65e2eb24259c418682d723cd547b39fcb6e0d2"
        );

        let feature_branch2 = result.get(2).unwrap();
        assert_eq!(feature_branch2.is_current, false);
        assert_eq!(feature_branch2.name, "feature/branch2");
        assert_eq!(
            feature_branch2.head.hash,
            "0d65e2eb24259c418682d723cd547b39fcb6e0d3"
        );

        let develop = result.get(3).unwrap();
        assert_eq!(develop.is_current, true); // カレントブランチ
        assert_eq!(develop.name, "develop");
        assert_eq!(
            develop.head.hash,
            "0d65e2eb24259c418682d723cd547b39fcb6e0d4"
        );

        let staging = result.get(4).unwrap();
        assert_eq!(staging.is_current, false);
        assert_eq!(staging.name, "staging");
        assert_eq!(
            staging.head.hash,
            "0d65e2eb24259c418682d723cd547b39fcb6e0d5"
        );

        Ok(())
    }
    #[test]
    fn ブランチが表示されていない場合はから配列を返す() -> Result<()> {
        let output = "".as_bytes().to_vec();

        // gitコマンドの出力をサービスオブジェクトで変換する
        let service = GitBranchCommandDomainService::new();
        let result = service.parse(output)?;

        assert_eq!(result.len(), 0);
        Ok(())
    }
}
