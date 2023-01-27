mod routes;

use routes::create_router;

pub async fn start_server() {
    let app = create_router();
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

