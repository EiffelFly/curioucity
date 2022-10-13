use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct CurioucityError(anyhow::Error);

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for CurioucityError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
// Tell axum how to convert `AppError` into a response.
impl IntoResponse for CurioucityError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
