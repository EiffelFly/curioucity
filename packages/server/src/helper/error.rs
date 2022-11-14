use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tonic::Status;

#[derive(Debug)]
pub struct CurioucityAxumError(anyhow::Error);

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, CurioucityAxumError>`. That way you don't need to do that manually.
impl<E> From<E> for CurioucityAxumError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
// Tell axum how to convert `CurioucityAxumError` into a response.
impl IntoResponse for CurioucityAxumError {
    fn into_response(self) -> Response {
        println!("{:#?}", self);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

#[derive(Debug)]
pub struct CurioucityTonicError(anyhow::Error);

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, CurioucityTonicError>`. That way you don't need to do that manually.
impl<E> From<E> for CurioucityTonicError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl From<CurioucityTonicError> for Status {
    fn from(err: CurioucityTonicError) -> Self {
        let error_message = format!("Curioucity internal error: {:?}", err);
        Self::internal(error_message)
    }
}
