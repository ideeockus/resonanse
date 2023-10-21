// use axum::{routing, Router};
//
// mod services;
// use resonanse_common::models;
//
// const fn get_routing() -> Router {
//     // todo set up timeouts
//     Router::new()
//         .route("/", routing::get(root))
//         .route("/user/signin", routing::get(user_signin).post(user_signin))
//         .route("/user/signup", routing::get(user_signup))
//         .route("/user/me", routing::get(user_me))
//         .route("/user/get", routing::get(get_user))
// }
//
// fn root() {}
// fn user_signin() {}
// fn user_signup() {}
// fn user_me() {}
// fn get_user() {}
//
// #[tokio::main]
// async fn main() {
//     // build our application with a single route
//     let app = get_routing();
//
//     // run it with hyper on localhost:3000
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

fn main() {
    println!("test backend");
}
