/*!
spool, a Rust written CDN uploader
Copyright (C) 2026, Tuxzilla T. Penguin

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

Contact Tuxzilla at tuxzilla@tuxzilla.com
*/

use axum::{
    Router,
    routing::{get, post},
};
use http::{HeaderValue, Method};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use std::fs;
use tower_http::cors::CorsLayer;

mod entities;
mod etc;
mod upload;
use crate::entities::users;

#[tokio::main]
async fn main() {
    if let Err(e) = start_spool().await {
        eprintln!("spool crashed with an error! {}", e);
        eprintln!("Please report this issue on our Github repo at https://github.com/tuxza/spool"); // yeah do that
        std::process::exit(1);
    }
}

async fn start_spool() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("./uploads/temp")?;

    let db = etc::db_connection().await?;

    let admin_user = users::Entity::find()
        .filter(users::Column::Uid.eq(1))
        .one(&db)
        .await?;
    if admin_user.is_none() {
        etc::setup_spool(&db).await?;
    }

    let cors = CorsLayer::new()
        .allow_origin("https://localhost:3000/".parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST]);

    let app = Router::new()
        .route("/", get(|| async { "spool server is running!" }))
        .route("/upload", post(upload::upload))
        .with_state(db)
        .layer(cors);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("running on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
