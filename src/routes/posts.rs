use crate::AppState;
use axum::extract::State;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::time;

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    ID: i32,
    title: String,
    body: String,
    author_id: i32,
    post_date: String,
}

pub async fn posts(State(state): State<AppState>) -> Json<Value> {
    let pool = state.pool;
    let mut posts = sqlx::query!("SELECT * FROM posts")
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut new_posts = Vec::new();
    for i in posts {
        new_posts.push(Post {
            ID: i.ID,
            title: i.title,
            body: i.body.unwrap(),
            author_id: i.author_id,
            post_date: i.post_date.to_string(),
        })
    }
    let posts = new_posts;
    println!("{:?}", posts);

    let posts = serde_json::to_string(&posts).unwrap();
    Json(serde_json::from_str(&*posts).unwrap())
}
