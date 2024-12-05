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
pub enum ItemsGetResponse {
    /// Ok
    Status200_Ok(Vec<models::ItemResponse>),
    /// Bad Request
    Status400_BadRequest(models::Problem),
    /// Internal Server Error
    Status500_InternalServerError(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ItemsPostResponse {
    /// Ok
    Status200_Ok(models::ItemResponse),
    /// Bad Request
    Status400_BadRequest(models::Problem),
    /// Unauthorized
    Status401_Unauthorized,
    /// Not Found
    Status404_NotFound(models::Problem),
    /// Internal Server Error
    Status500_InternalServerError(models::Problem),
}

/// Items
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Items {
    type Claims;

    /// ItemsGet - GET /items
    async fn items_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::ItemsGetQueryParams,
    ) -> Result<ItemsGetResponse, ()>;

    /// ItemsPost - POST /items
    async fn items_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        claims: Self::Claims,
        body: models::ItemRequest,
    ) -> Result<ItemsPostResponse, ()>;
}
