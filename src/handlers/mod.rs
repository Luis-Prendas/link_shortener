use crate::{models::ShortenedLink, services, state::AppState};
use actix_web::{HttpResponse, Responder, web};

// Controlador para acortar un enlace
pub async fn shorten_link(data: web::Data<AppState>, link: web::Json<String>) -> impl Responder {
    let mut links = data.links.lock().unwrap();
    let link_url = link.into_inner();

    println!("Recibido enlace para acortar: {}", link_url);

    if let Some(existing_link) = links.values().find(|l| l.url == link_url) {
        println!("El enlace ya ha sido acortado: {}", existing_link.shortened);
        return HttpResponse::Ok().json(existing_link.clone());
    }

    let new_id = services::generate_hash(&link_url); // Usamos la función del servicio
    let shortened_url = format!("http://localhost:3000/{}", new_id);

    let new_link = ShortenedLink {
        id: new_id.clone(),
        url: link_url.clone(),
        shortened: shortened_url.clone(),
    };

    links.insert(new_id.clone(), new_link.clone());

    if let Err(e) = data.save(&links) {
        println!("Error al guardar el archivo: {}", e);
    }

    println!("Enlace acortado: {} -> {}", link_url, shortened_url);

    HttpResponse::Created().json(new_link)
}

// Controlador para redirigir a un enlace acortado
pub async fn get_link(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let links = data.links.lock().unwrap();
    let id_str = id.into_inner();

    println!("Solicitando enlace para ID: {}", id_str);

    if let Some(link) = links.get(&id_str) {
        println!("Redirigiendo al enlace original: {}", link.url);
        HttpResponse::Found()
            .append_header((actix_web::http::header::LOCATION, link.url.clone()))
            .finish()
    } else {
        println!("Enlace no encontrado para ID: {}", id_str);
        HttpResponse::NotFound().body("Link not found")
    }
}
