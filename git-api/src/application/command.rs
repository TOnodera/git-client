use crate::domain::types::Result;

pub trait Command {
    fn execute<T>() -> Result<T>;
}

pub struct GitCommand {
    pub name: String,
    pub args: Vec<String>,
}
