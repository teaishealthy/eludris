use argon2::Argon2;
use rocket::{http::Status, response::status::Custom, serde::json::Json, State};
use rocket_db_pools::Connection;
use todel::{
    http::{Cache, ClientIP, TokenAuth, DB},
    models::{Emailer, PasswordDeleteCredentials, User},
    Conf,
};

use crate::rate_limit::{RateLimitedRouteResponse, RateLimiter};

/// Delete your user.
///
/// -----
///
/// ### Example
///
/// ```sh
/// curl \
///   -X DELETE \
///   -H "Authorization: <token>" \
///   --json '{"password": "wowsuchpassword"}'
///   https://api.eludris.gay/users
/// ```
#[autodoc("/users", category = "Users")]
#[delete("/", data = "<delete>")]
pub async fn delete_user(
    delete: Json<PasswordDeleteCredentials>,
    conf: &State<Conf>,
    verifier: &State<Argon2<'static>>,
    mailer: &State<Emailer>,
    mut db: Connection<DB>,
    mut cache: Connection<Cache>,
    session: TokenAuth,
    ip: ClientIP,
) -> RateLimitedRouteResponse<Custom<()>> {
    let mut rate_limiter = RateLimiter::new("delete_user", ip, conf);
    rate_limiter.process_rate_limit(&mut cache).await?;

    rate_limiter.wrap_response(Custom(
        Status::NoContent,
        User::delete(
            session.0.user_id,
            delete.into_inner(),
            verifier.inner(),
            mailer,
            conf,
            &mut db,
        )
        .await
        .map_err(|err| rate_limiter.add_headers(err))?,
    ))
}
