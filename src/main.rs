use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rand::seq::SliceRandom;
use std::fs;
use std::path::PathBuf;

async fn random_webp_image() -> impl IntoResponse {
    const IMAGE_DIR: &str = "/app/images/webp";
    let image_dir = IMAGE_DIR;
    println!("Attempting to read directory: {}", image_dir);

    let paths: Vec<PathBuf> = match fs::read_dir(image_dir) {
        Ok(entries) => entries
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "webp") {
                    println!("Found WebP file: {:?}", path);
                    Some(path)
                } else {
                    println!("Skipping non-WebP file: {:?}", path);
                    None
                }
            })
            .collect(),
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if paths.is_empty() {
        eprintln!("No WebP images found in the directory");
        return Err(StatusCode::NOT_FOUND);
    }

    println!("Total WebP images found: {}", paths.len());

    let random_image = match paths.choose(&mut rand::thread_rng()) {
        Some(path) => {
            println!("Selected random image: {:?}", path);
            path
        },
        None => {
            eprintln!("Failed to choose a random image");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let image_data = match fs::read(random_image) {
        Ok(data) => {
            println!("Successfully read image data, size: {} bytes", data.len());
            data
        },
        Err(e) => {
            eprintln!("Error reading image file: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut response = Response::builder()
        .body(axum::body::Body::from(image_data))
        .unwrap();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/webp"),
    );

    println!("Sending response with image");
    Ok(response)
}

#[tokio::main]
async fn main() {
    println!("Starting PixelPulse server");
    let app = Router::new().route("/random-image", get(random_webp_image));

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8544").await {
        Ok(l) => {
            println!("Successfully bound to 0.0.0.0:8544");
            l
        },
        Err(e) => {
            eprintln!("Failed to bind to 0.0.0.0:8544: {}", e);
            return;
        }
    };

    println!("PixelPulse server running on http://0.0.0.0:8544");
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
    }
}
