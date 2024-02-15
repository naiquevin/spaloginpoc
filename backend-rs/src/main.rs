use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use minijinja::{context, path_loader, Environment};
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub tmpl_env: Environment<'static>,
}

async fn login(State(state): State<AppState>) -> Html<String> {
    let tmpl = state.tmpl_env.get_template("login.html").unwrap();
    let ctx = context!();
    Html(tmpl.render(ctx).unwrap())
}

#[tokio::main]
async fn main() {
    let mut tmpl_env = Environment::new();
    tmpl_env.set_loader(path_loader("templates"));

    let state = AppState { tmpl_env };

    let app = Router::new()
        .route("/login", get(login))
        .with_state(state);

    let addr = String::from("0.0.0.0:5001");
    println!("Starting HTTP server listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
}
