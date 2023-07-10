use actix_web::{
    body::BoxBody,
    http::StatusCode,
    HttpResponse,
    HttpResponseBuilder,
    ResponseError,
};
use std::fmt::Display;

#[derive(Debug)]
pub struct MyError {
    inner: anyhow::Error,
}

pub type MyResult<T> = Result<T, MyError>;

// impl std::error::Error for Error {} FIXME: i hate that implementing this breaks the from impl below

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<E: Into<anyhow::Error>> From<E> for MyError {
    fn from(value: E) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).body(format!("{}", self.inner))
    }
}

pub trait Context<T, E> {
    // Required methods
    fn context<C>(self, context: C) -> MyResult<T>
    where
        C: Display + Send + Sync + 'static;
}

impl<K, T, E> Context<T, E> for K
where
    K: anyhow::Context<T, E>,
{
    fn context<C>(self, context: C) -> MyResult<T>
    where
        C: Display + Send + Sync + 'static,
    {
        Ok(self.context(context)?)
    }
}
