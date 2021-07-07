pub use emailer::*;
pub use repo::*;
pub use secure::*;

pub mod emailer;
pub mod repo;
pub mod secure;

pub trait Repository:
    ClientRepo + SessionRepo + RequestsRepo + UserRepo + AccessTokenRepo + AuthCodeRepo + Send + Sync
{
}

impl<T> Repository for T where
    T: ClientRepo
        + SessionRepo
        + RequestsRepo
        + UserRepo
        + AccessTokenRepo
        + AuthCodeRepo
        + Send
        + Sync
{
}
