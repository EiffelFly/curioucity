use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct CurioucityError(anyhow::Error);

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, CurioucityError>`. That way you don't need to do that manually.
impl<E> From<E> for CurioucityError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
// Tell axum how to convert `CurioucityError` into a response.
impl IntoResponse for CurioucityError {
    fn into_response(self) -> Response {
        println!("{:#?}", self);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
