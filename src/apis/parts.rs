use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PartsGetResponse {
    /// Ok
    Status200_Ok(Vec<models::PartResponse>),
    /// Bad Request
    Status400_BadRequest(models::Problem),
    /// Internal Server Error
    Status500_InternalServerError(models::Problem),
}

/// Parts
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Parts {
    /// PartsGet - GET /parts
    async fn parts_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::PartsGetQueryParams,
    ) -> Result<PartsGetResponse, ()>;
}
