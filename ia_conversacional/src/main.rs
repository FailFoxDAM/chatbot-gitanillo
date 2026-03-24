use axum::{routing::post, Json, Router};
use google_generative_ai_rs::v1::api::Client;
use google_generative_ai_rs::v1::gemini::{request::Request, Content, Model, Part, Role};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize)]
struct ChatResponse {
    reply: String,
}

async fn chat_handler(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").unwrap_or_else(|_| "TU_API_KEY_AQUI".to_string());
    
    // Usamos el modelo compatible con la v0.3.4
    let client = Client::new_from_model(Model::Gemini1_0Pro, api_key);

    let txt_request = Request {
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(payload.message),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        }],
        safety_settings: vec![],
        generation_config: None,
        tools: vec![],
    };

    let mut reply = "Perdona, lacho, no he podido contactar con el destino...".to_string();

    // Manejo de la respuesta de Gemini
    if let Ok(response) = client.post(30, &txt_request).await {
        if let Some(res) = response.rest() {
            if let Some(candidate) = res.candidates.first() {
                if let Some(part) = candidate.content.parts.first() {
                    if let Some(text) = &part.text {
                        reply = text.clone();
                    }
                }
            }
        }
    }

    Json(ChatResponse { reply })
}

#[tokio::main]
async fn main() {
    // Configuración de CORS para permitir que tu HTML (frontend) conecte con Rust
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/chat", post(chat_handler))
        .layer(cors);

    println!("🚀 Servidor 'Orgullo Romaní' activo en http://127.0.0.1:3000");

    // Configuración de escucha para Axum 0.8
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("No se pudo enlazar al puerto 3000");
        
    axum::serve(listener, app)
        .await
        .expect("Error al iniciar el servidor");
}