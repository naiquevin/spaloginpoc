use axum::body::Body;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Form, Router};
use cookie::Cookie;
use minijinja::{context, path_loader, Environment};
use serde::Deserialize;
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
        .with_state(state);

    let addr = String::from("0.0.0.0:5001");
    println!("Starting HTTP server listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
}
