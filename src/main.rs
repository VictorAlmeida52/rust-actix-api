mod services;
mod schema;
mod model;

use std::collections::HashMap;
use actix_web::{
    web,
    App,
    HttpServer,
    middleware::Logger
};
use dotenv::dotenv;
use sqlx::{Postgres, Pool, postgres::PgPoolOptions};
use crate::model::TaskModel;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started on port: {}", 8080);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info")
    }

    dotenv().ok();
    env_logger::init();
    
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = match PgPoolOptions::new().max_connections(10).connect(&*db_url).await {
        Ok(pool) => {
            println!("Connection established.");
            pool
        }
        Err(e) => {
            println!("Connection couldn't be established: {:?}", e);
            std::process::exit(1);
        }
    };
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {db: pool.clone()}))
            .configure(services::config)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;
    Ok(())
}

async fn call() {

    // Tests with
    let mut map = HashMap::new();
    map.insert("title", "Título");
    map.insert("content", "Conteúdo");

    let client = reqwest::Client::new();
    let res = client.post("https://webhook.site/2e513f78-2e3d-4c8b-b585-a334d9f23693")
        .json(&map)
        .send()
        .await
        .expect("Erro na chamada HTTP");

    let parsed = res.json::<TaskModel>().await.expect("Erro tentando desserializar resposta");

    println!("{:?}", parsed);
}






















