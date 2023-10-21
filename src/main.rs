use axum::Router;

mod models;
mod services;

const fn get_routing() -> Router {
    // todo set up timeouts
    Router::new()
        .route("/", get(root))
        .route("/user/signin", get(user_signin).post(user_signin))
        .route("/user/signup", get(user_signup))
        .route("/user/me", get(user_me))
        .route("/user/get", get(get_user))
}

fn root() {

}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = get_routing();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}