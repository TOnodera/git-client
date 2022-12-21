use crate::types::{
    errors::{Error, ErrorDetail},
    types::Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommitHash {
    pub hash: String,
}

impl CommitHash {
    pub fn new(hash: String) -> Result<Self> {
        if hash.len() != 40 {
            return Err(Box::new(Error::InvalidValueError(ErrorDetail::new(
                "ハッシュ値が不正です",
                "ハッシュ値の値が40文字ではありません",
            ))));
        }
        Ok(Self { hash })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub head_hash: CommitHash,
}

impl Branch {
    pub fn new(name: String, head_hash: String) -> Result<Self> {
        let head_hash = CommitHash::new(head_hash)?;
        Ok(Self { name, head_hash })
    }
}

/// とりあえず最初はこれでやってみる
#[derive(Serialize, Deserialize)]
pub struct CommitInfo {
    pub commit_hash: CommitHash,
    pub parent_hash: CommitHash,
    pub author: String,
    pub committer: String,
    pub author_email: String,
    pub committer_email: String,
}
