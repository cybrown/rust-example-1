use crate::application::AppError;
use crate::application::AppResult;
use tokio::task;

pub async fn spawn_blocking<F, R>(f: F) -> AppResult<R>
where
    F: FnOnce() -> AppResult<R> + Send + 'static,
    R: Send + 'static,
{
    task::spawn_blocking(f)
        .await
        .map_err(|_| AppError::new("failed to spawn thread for blocking operation".to_owned()))?
}
