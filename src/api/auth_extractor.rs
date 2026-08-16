use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::db::models::Claims;

/// Custom extractor for authenticated user claims
pub struct AuthClaims(pub Claims);

impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(AuthClaims)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
