mod post_controller;
mod server;

use domain::DomainError;
pub use post_controller::*;
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
