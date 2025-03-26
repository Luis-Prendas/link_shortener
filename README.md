# Link Shortener API

Una API RESTful simple en Rust para acortar enlaces. Esta aplicación permite a los usuarios enviar un enlace largo y recibir un enlace corto, que redirige al enlace original.

## Tecnologías

- **Rust**: Lenguaje de programación utilizado para crear la API.
- **Actix-web**: Framework web para construir aplicaciones rápidas y eficientes en Rust.
- **SHA-1**: Algoritmo de hash utilizado para generar un identificador único para cada enlace.
- **Serde**: Para serialización y deserialización de datos en formato JSON.
- **dotenv**: Para gestionar las variables de entorno de manera segura.

## Instalación

### Prerrequisitos

Asegúrate de tener [Rust](https://www.rust-lang.org/) instalado en tu máquina. Si no lo tienes, puedes seguir las instrucciones en el sitio oficial para instalarlo.

### Clonar el repositorio

```bash
git clone https://github.com/tu-usuario/link_shortener.git
cd link_shortener
```

### Instalar las dependencias

```bash
cargo build
```

### Crear un archivo `.env`

Crea un archivo `.env` en la raíz del proyecto con las siguientes variables de entorno:

```env
SERVER_ADDR=127.0.0.1
SERVER_PORT=3000
LINKS_FILE_PATH=links.json
```

### Ejecutar el servidor

Para ejecutar el servidor, usa el siguiente comando:

```bash
cargo run
```

El servidor estará disponible en `http://127.0.0.1:3000`.

## Endpoints

### `POST /shorten`

Este endpoint recibe un enlace largo y devuelve el enlace acortado.

#### Request

```json
"http://www.google.com"
```

#### Response

```json
{
  "id": "4f8e",
  "url": "http://www.ejemplo.com",
  "shortened": "http://localhost:3000/4f8e"
}
```

### `GET /{id}`

Este endpoint redirige al enlace original usando el ID proporcionado.

#### Request

```bash
GET /4f8e
```

#### Response

Redirige a `http://www.ejemplo.com`.

### Manejo de errores

Si el enlace no se encuentra, se devuelve un código de estado `404 Not Found`.

```json
{
  "message": "Link not found"
}
```

## Contribuciones

Si deseas contribuir al proyecto, siéntete libre de hacer un fork y enviar un pull request.

## Licencia

Este proyecto está bajo la licencia MIT. Para más detalles, revisa el archivo [LICENSE](LICENSE).
