use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use minijinja::{context, path_loader, Environment};
use std::collections::HashMap;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub tmpl_env: Environment<'static>,
    // A mapping of usernames and their passwords
    pub user_store: HashMap<&'static str, &'static str>
}

#[derive(Debug)]
pub enum AppError {
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

#[tokio::main]
async fn main() {
    let mut tmpl_env = Environment::new();
    tmpl_env.set_loader(path_loader("templates"));

    let mut user_store: HashMap<&'static str, &'static str> = HashMap::new();
    user_store.insert("vineet", "s3cret");

    let state = AppState { tmpl_env, user_store };

    let app = Router::new()
        .route("/login", get(login_page))
        .with_state(state);

    let addr = String::from("0.0.0.0:5001");
    println!("Starting HTTP server listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
}
