use axum::body::Body;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::{Form, Router, Json};
use cookie::Cookie;
use minijinja::{context, path_loader, Environment};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::net::TcpListener;

#[allow(dead_code)]
#[derive(Clone)]
struct AppState {
    tmpl_env: Environment<'static>,
    // A mapping of usernames and their passwords
    user_store: HashMap<&'static str, &'static str>,
}

#[derive(Debug)]
enum AppError {
    Templating(minijinja::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::Templating(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        };
        (status, err_msg.to_string()).into_response()
    }
}

/* Handler for login page */

async fn login_page(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let tmpl = state.tmpl_env.get_template("login.html")
        .map_err(AppError::Templating)?;
    let ctx = context!();
    let markup = tmpl.render(ctx).map_err(AppError::Templating)?;
    Ok(Html(markup))
}

/* Handler for login action */

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login_action(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>
) -> Response {
    match state.user_store.get(form.username.as_str()) {
        Some(stored_password) => {
            if &form.password == stored_password {
                let cookie = Cookie::build(("session_id", form.username))
                    .http_only(true);
                Response::builder()
                    .status(StatusCode::TEMPORARY_REDIRECT)
                    .header(header::SET_COOKIE, cookie.to_string())
                    .header(header::LOCATION, "/")
                    .body(Body::empty())
                    .unwrap()
            } else {
                (StatusCode::UNAUTHORIZED, "Authentication failed").into_response()
            }
        },
        None => (StatusCode::UNAUTHORIZED, "Authentication failed").into_response()
    }
}

/* XHR handler for info */

fn find_session_cookie<'a>(headers: &'a header::HeaderMap) -> Option<Cookie<'a>> {
    // This code is based on a similar fn found in the axum-extra
    // crate -
    // https://github.com/tokio-rs/axum/blob/main/axum-extra/src/extract/cookie/mod.rs#L106
    //
    // As per RFC-7540 multiple "Cookie" headers are possible so we
    // need to parse all of them (Ref:
    // https://www.rfc-editor.org/rfc/rfc7540#section-8.1.2.5)
    headers
        .get_all(header::COOKIE)
        .into_iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| Cookie::split_parse(value))
        .find(|c| c.as_ref().map_or(false, |v| v.name() == "session_id"))
        .and_then(|c| c.ok())
}

#[derive(Serialize)]
struct InfoResponse {
    user: String,
}

async fn info(headers: header::HeaderMap) -> Response {
    match find_session_cookie(&headers) {
        Some(cookie) => {
            let resp = InfoResponse { user: cookie.value().to_owned() };
            Json(resp).into_response()
        },
        None => {
            (StatusCode::UNAUTHORIZED, "Authentication failed").into_response()
        }
    }
}

/* Logout handler */

async fn logout(headers: header::HeaderMap) -> Response {
    let mut maybe_cookie = find_session_cookie(&headers);
    match maybe_cookie.as_mut() {
        Some(cookie) => {
            cookie.make_removal();
            Response::builder()
                .status(StatusCode::TEMPORARY_REDIRECT)
                .header(header::SET_COOKIE, cookie.to_string())
                .header(header::LOCATION, "/login")
                .body(Body::empty())
                .unwrap()
        },
        None => {
            Redirect::temporary("/login").into_response()
        }
    }
}

#[tokio::main]
async fn main() {
    let mut tmpl_env = Environment::new();
    tmpl_env.set_loader(path_loader("templates"));

    let mut user_store: HashMap<&'static str, &'static str> = HashMap::new();
    user_store.insert("vineet", "s3cret");

    let state = AppState { tmpl_env, user_store };

    let app = Router::new()
        .route("/login", get(login_page))
        .route("/login", post(login_action))
        .route("/logout", get(logout))
        .route("/info", get(info))
        .with_state(state);

    let addr = String::from("0.0.0.0:5001");
    println!("Starting HTTP server listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
}
