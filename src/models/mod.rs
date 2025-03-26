use serde::{Deserialize, Serialize};

// Estructura para representar un enlace acortado
#[derive(Serialize, Deserialize, Clone)]
pub struct ShortenedLink {
    pub id: String,
    pub url: String,
    pub shortened: String,
}
