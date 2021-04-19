mod server;

use domain::DomainError;
pub use server::*;
use warp::reject::Reject;

#[derive(Debug)]
struct ApiError(DomainError);

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        Self(err)
    }
}

impl Reject for ApiError {}
