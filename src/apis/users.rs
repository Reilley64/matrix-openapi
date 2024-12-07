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
pub enum UsersMeGetResponse {
    /// Ok
    Status200_Ok(models::UserResponse),
    /// Bad Request
    Status400_BadRequest(models::Problem),
    /// Unauthorized
    Status401_Unauthorized,
    /// Not Found
    Status404_NotFound(models::Problem),
    /// Internal Server Error
    Status500_InternalServerError(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersPostResponse {
    /// Ok
    Status200_Ok(models::UserResponse),
    /// Bad Request
    Status400_BadRequest(models::Problem),
    /// Unauthorized
    Status401_Unauthorized,
    /// Conflict
    Status409_Conflict(models::Problem),
    /// Internal Server Error
    Status500_InternalServerError(models::Problem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UsersUserIdGetResponse {
    /// Ok
    Status200_Ok(models::UserResponse),
    /// Bad Request
    Status400_BadRequest(models::Problem),
    /// Not Found
    Status404_NotFound(models::Problem),
    /// Internal Server Error
    Status500_InternalServerError(models::Problem),
}

/// Users
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Users {
    type Claims;

    /// UsersMeGet - GET /users/me
    async fn users_me_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        claims: Self::Claims,
    ) -> Result<UsersMeGetResponse, ()>;

    /// UsersPost - POST /users
    async fn users_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        claims: Self::Claims,
        body: models::UserRequest,
    ) -> Result<UsersPostResponse, ()>;

    /// UsersUserIdGet - GET /users/{userId}
    async fn users_user_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UsersUserIdGetPathParams,
    ) -> Result<UsersUserIdGetResponse, ()>;
}
