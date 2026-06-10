/*!
spool, a Rust written CDN uploader
Copyright (C) 2026, Tuxzilla T. Penguin
*/

/*
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
use http::{HeaderValue, Method, header};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::fs;
use tower_http::cors::CorsLayer;

mod auth;
mod entities;
mod etc;
mod upload;
use crate::{
    entities::users::{self},
    etc::random_ass_string,
};

#[tokio::main]
async fn main() {
    println!("{}", random_ass_string());
    println!("starting spool...");
    if let Err(e) = start_spool().await {
        eprintln!("spool crashed with an error! {}", e);
        eprintln!("Please report this issue on our Github repo at https://github.com/tuxza/spool"); // yeah do that
        std::process::exit(1);
    }
}

async fn start_spool() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("./uploads/temp")?;

    let db = etc::db_connection().await?;

    // the whole reason for this check is to 1. help init spool on first startup
    // 2. to avoid creating duplicate admin users
    // 3. prevent db tampering cause i said uid 1 special
    // of course you can (when i make it) create more users with admin perms

    let admin_user = users::Entity::find()
        .filter(users::Column::Uid.eq(1))
        .one(&db)
        .await?;

    match admin_user {
        None => {
            etc::setup_spool(&db).await?;
        }
        Some(user) => {
            if user.username != "admin" {
                return Err(
                    "database tampering detected ! : UID 1 Username column must be 'admin'".into(),
                );
            }
        }
    }

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let app = Router::new()
        .route("/", get(|| async { "spool server is running!" }))
        .route("/upload", post(upload::upload))
        .route("/auth/login", post(auth::login))
        .with_state(db)
        .layer(cors);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
