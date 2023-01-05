use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
