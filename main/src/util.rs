use domain::DomainError;
use domain::DomainResult;
use tokio::task;

pub async fn spawn_blocking<F, R>(f: F) -> DomainResult<R>
where
    F: FnOnce() -> DomainResult<R> + Send + 'static,
    R: Send + 'static,
{
    task::spawn_blocking(f)
        .await
        .map_err(|_| DomainError::new("failed to spawn thread for blocking operation".to_owned()))?
}
