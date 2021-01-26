use crate::application::AppError;
use tokio::task;

pub async fn spawn_blocking<F, R>(f: F) -> Result<R, AppError>
where
    F: FnOnce() -> Result<R, AppError> + Send + 'static,
    R: Send + 'static,
{
    task::spawn_blocking(f)
        .await
        .map_err(|_| AppError::new("failed to spawn thread for blocking operation".to_owned()))?
}
