use dotenv::dotenv;

pub fn load_config() {
    dotenv().ok();
    // Puedes agregar más configuraciones aquí si es necesario
}
