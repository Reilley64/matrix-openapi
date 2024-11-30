use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};

/// Setup API Server.
pub fn new<I, A, C>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::parts::Parts
        + apis::users::Users<Claims = C>
        + apis::ApiKeyAuthHeader<Claims = C>
        + 'static,
    C: Send + Sync + 'static,
{
    // build our application with a route
    Router::new()
        .route("/parts", get(parts_get::<I, A>))
        .route("/users", post(users_post::<I, A, C>))
        .route("/users/:user_id", get(users_user_id_get::<I, A, C>))
        .with_state(api_impl)
}

#[tracing::instrument(skip_all)]
fn parts_get_validation(
    query_params: models::PartsGetQueryParams,
) -> std::result::Result<(models::PartsGetQueryParams,), ValidationErrors> {
    query_params.validate()?;

    Ok((query_params,))
}
/// PartsGet - GET /parts
#[tracing::instrument(skip_all)]
async fn parts_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Query(query_params): Query<models::PartsGetQueryParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::parts::Parts,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || parts_get_validation(query_params))
        .await
        .unwrap();

    let Ok((query_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .parts_get(method, host, cookies, query_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::parts::PartsGetResponse::Status200_Ok(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::parts::PartsGetResponse::Status400_BadRequest(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::parts::PartsGetResponse::Status500_InternalServerError(body) => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct UsersPostBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::UserRequest,
}

#[tracing::instrument(skip_all)]
fn users_post_validation(
    body: models::UserRequest,
) -> std::result::Result<(models::UserRequest,), ValidationErrors> {
    let b = UsersPostBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// UsersPost - POST /users
#[tracing::instrument(skip_all)]
async fn users_post<I, A, C>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    headers: HeaderMap,
    State(api_impl): State<I>,
    Json(body): Json<models::UserRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::users::Users<Claims = C> + apis::ApiKeyAuthHeader<Claims = C>,
{
    // Authentication
    let claims_in_header = api_impl
        .as_ref()
        .extract_claims_from_header(&headers, "Authorization")
        .await;
    let claims = None.or(claims_in_header);
    let Some(claims) = claims else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    };

    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || users_post_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .users_post(method, host, cookies, claims, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::users::UsersPostResponse::Status200_Ok(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::users::UsersPostResponse::Status400_BadRequest(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::users::UsersPostResponse::Status409_Conflict(body) => {
                let mut response = response.status(409);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::users::UsersPostResponse::Status500_InternalServerError(body) => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn users_user_id_get_validation(
    path_params: models::UsersUserIdGetPathParams,
) -> std::result::Result<(models::UsersUserIdGetPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// UsersUserIdGet - GET /users/{userId}
#[tracing::instrument(skip_all)]
async fn users_user_id_get<I, A, C>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::UsersUserIdGetPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::users::Users<Claims = C>,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || users_user_id_get_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .users_user_id_get(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::users::UsersUserIdGetResponse::Status200_Ok(body) => {
                let mut response = response.status(200);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::users::UsersUserIdGetResponse::Status400_BadRequest(body) => {
                let mut response = response.status(400);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::users::UsersUserIdGetResponse::Status404_NotFound(body) => {
                let mut response = response.status(404);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
            apis::users::UsersUserIdGetResponse::Status500_InternalServerError(body) => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/problem+json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
