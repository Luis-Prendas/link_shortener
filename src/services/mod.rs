use sha1::{Digest, Sha1};

// Función para generar un hash
pub fn generate_hash(id: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(id);
    let result = hasher.finalize();
    format!("{:x}", result)[..4].to_string() // Tomamos los primeros 4 caracteres
}
