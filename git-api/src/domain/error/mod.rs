use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("コマンドエラーが発生しました.")]
    CommandError,

    #[error("バリデーションエラー.")]
    ValidationError,
}
