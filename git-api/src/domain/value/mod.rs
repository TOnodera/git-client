use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::EnvTrait;
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

#[derive(Clone)]
pub struct Env {
    pub git_dir: String,
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
