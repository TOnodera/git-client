use self::value::CommitHash;
use crate::domain::types::Result;
use crate::presentation::OutputData;

pub mod command;
pub mod error;
pub mod handler;
pub mod types;
pub mod value;

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
    Self::Env: EnvTrait,
    Self::DomainService: DomainService,
{
    type Output;
    type Env;
    type DomainService;
    fn new(env: Self::Env, service: Self::DomainService) -> Self;
    fn execute(&self) -> Result<Self::Output>;
}

// Envトレイト
pub trait EnvTrait {
    fn new(project_root: &str) -> Self;
    fn is_test(&self) -> bool;
}

// ドメインサービストレイト
pub trait DomainService {
    // Empty
}
