pub use emailer::*;
pub use repo::*;
pub use secure::*;

pub mod emailer;
pub mod repo;
pub mod secure;

pub trait Repository:
    AccessTokenRepo
    + AuthCodeRepo
    + ApplicationRepo
    + RequestsRepo
    + SessionRepo
    + UserRegistrationsRepo
    + UserRepo
    + Send
    + Sync
{
}

impl<T> Repository for T where
    T: AccessTokenRepo
        + AuthCodeRepo
        + ApplicationRepo
        + RequestsRepo
        + SessionRepo
        + UserRegistrationsRepo
        + UserRepo
        + Send
        + Sync
{
}
