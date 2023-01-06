use crate::domain::types::Result;
use crate::presentation::OutputData;
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod command;
pub mod handler;
pub mod types;

// コミットハッシュオブジェクト
#[derive(Debug, Deserialize, Serialize)]
pub struct CommitHash {
    hash: String,
}

// コミット情報オブジェクト
#[derive(Debug, Deserialize, Serialize)]
pub struct CommitInfo {
    commit_hash: Option<String>,
    tree_hash: Option<String>,
    parent_hash: Option<String>,
    author_name: Option<String>,
    author_email: Option<String>,
    #[serde(deserialize_with = "from_ts")]
    author_date: DateTime<Utc>,
    committer_name: Option<String>,
    committer_email: Option<String>,
    #[serde(deserialize_with = "from_ts")]
    commit_date: DateTime<Utc>,
    comment: Option<String>,
}

// ブランチオブジェクト
pub struct Branch {
    pub name: String,
    pub head: CommitHash,
}

// コマンドトレイト
pub trait CommandTrait
where
    Self::Output: OutputData,
{
    type Output;
    fn new() -> Self;
    fn execute() -> Result<Self::Output>;
}

// Gitコマンド
pub struct GitCommand {
    pub name: String,
    pub args: Vec<String>,
}

pub trait EnvTrait {
    fn new(project_root: &str) -> Self;
    fn is_test(&self) -> bool;
}

pub struct Env {
    project_root: String,
    is_test: bool,
}

impl EnvTrait for Env {
    fn new(project_root: &str) -> Self {
        Self {
            project_root: project_root.to_string(),
            is_test: false,
        }
    }

    fn is_test(&self) -> bool {
        self.is_test
    }
}
