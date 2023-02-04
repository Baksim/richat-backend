mod routes;

use routes::create_router;

use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct AppState {
    pool: Pool<MySql>,
}

pub async fn start_server(pool: Pool<MySql>) {
    let state = AppState { pool };

    let app = create_router(state);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
