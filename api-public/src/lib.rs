pub async fn blocking<F, T>(or: T, f: F) -> Result<T, T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    actix_web::web::block(move || {
        let res = f();
        Ok(res) as Result<T, ()>
    })
    .await
    .map_err(|err| match err {
        actix_web::error::BlockingError::Canceled | actix_web::error::BlockingError::Error(_) => or,
    })
}

pub trait EachResult<T> {
    fn get(self) -> T;
}

impl<T> EachResult<T> for Result<T, T> {
    fn get(self) -> T {
        match self {
            Ok(value) => value,
            Err(error) => error,
        }
    }
}
