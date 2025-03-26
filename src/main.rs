use std::collections::HashMap;
use std::sync::Mutex;

use crate::config::load_config;
use crate::handlers::{get_link, shorten_link};
use crate::state::AppState;
use actix_web::web::{self, Data};
use actix_web::{App, HttpServer};

mod config;
mod handlers;
mod models;
mod services;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_config(); // Cargar la configuración desde el archivo .env

    let server_addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let file_path = std::env::var("LINKS_FILE_PATH").unwrap_or_else(|_| "links.json".to_string());

    println!(
        "Servidor corriendo en http://{}:{}",
        server_addr, server_port
    );

    let app_state = AppState {
        links: Mutex::new(HashMap::new()),
        file_path,
    };

    let data = Data::new(app_state);

    // Load the links from the file
    let loaded_links = data.load().unwrap_or_default();
    *data.links.lock().unwrap() = loaded_links;

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/shorten", web::post().to(shorten_link))
            .route("/{id}", web::get().to(get_link))
    })
    .bind(format!("{}:{}", server_addr, server_port))?
    .run()
    .await
}
