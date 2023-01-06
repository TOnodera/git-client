use crate::domain::types::Result;
use crate::presentation::OutputData;
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod handler;
pub mod types;

// コミットハッシュオブジェクト
#[derive(Debug, Deserialize, Serialize)]
pub struct CommitHash {
    hash: String,
}
impl CommitHash {
    pub fn new(hash: &str) -> Self {
        Self {
            hash: hash.to_string(),
        }
    }
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
impl Branch {
    pub fn new(name: &str, head: &str) -> Self {
        Self {
            name: name.to_string(),
            head: CommitHash::new(head),
        }
    }
}

// コマンドトレイト
pub trait CommandTrait
where
    Self::Output: OutputData,
{
    type Output;
    fn new() -> Self;
    fn execute(&self) -> Result<Self::Output>;
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
    git_dir: String,
    is_test: bool,
}

impl EnvTrait for Env {
    fn new(git_dir: &str) -> Self {
        Self {
            git_dir: git_dir.to_string(),
            is_test: false,
        }
    }

    fn is_test(&self) -> bool {
        self.is_test
    }
}
