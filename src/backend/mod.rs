mod fixed_window;
mod memory;

pub use fixed_window::{FixedWindowBackend, FixedWindowInput, FixedWindowOutput};
pub use memory::{FixedWindowInMemory, FixedWindowInMemoryBuilder};

use actix_web::Result;
use async_trait::async_trait;

/// To implement your own rate limiting backend it is recommended to use
/// [async_trait](https://github.com/dtolnay/async-trait), and add the `#[async_trait(?Send)]`
/// attribute onto your trait implementation.
#[async_trait(?Send)]
pub trait Backend<I: 'static>: Clone {
    type Output;

    /// Process an incoming request.
    ///
    /// The input could include such things as a rate limit key, and the rate limit policy to be
    /// applied.
    ///
    /// Returns a boolean of whether to allow the request, and also can also return arbitrary output
    /// that can be used to transform the response, or rollback this operation.
    async fn request(&self, input: I) -> Result<(bool, Self::Output)>;

    /// Under certain conditions we may not want to rollback the request operation.
    ///
    /// E.g. We may want to exclude 5xx errors from counting against a user's rate limit,
    /// we can only exclude them after having already allowed the request through the rate limiter
    /// in the first place, so we must therefore deduct from the rate limit counter afterwards.
    ///
    /// Note that if this function fails there is not much the [RateLimiter](crate::RateLimiter)
    /// can do about it, given that the request has already been allowed.
    ///
    /// # Arguments
    ///
    /// * `previous`: The output of the [request()](Backend::request()).
    async fn rollback(&self, previous: Self::Output) -> Result<()>;
}
