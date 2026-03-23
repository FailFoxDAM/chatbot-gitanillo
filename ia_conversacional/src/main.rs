/*fn main() {
    let vector = vec![1, 2, 3, 4, 5];
    let r = buscar_num(&vector, 3);

    println!("Encontrado: {}", match r {
        Some(valor) => valor,
        None => { 
            println!("Número no encontrado"); 
            return; 
        },
    });

    let x = 10;
    let y = 20;
    let suma = sumar(x, y);
    println!("Suma: {}", suma);
}

fn buscar_num(lista: &[i32], n: i32) -> Option<i32> {
    for &item in lista {
        if item == n {
            return Some(item);
        }
    }
    None
}
dcfgvbyhui
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}*/


use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{
        Content,
        Part,
        Role,
        Model,
        request::Request,
    },
};
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY")
        .expect("No se encontró GEMINI_API_KEY en el .env");

    let client = Client::new_from_model(Model::Gemini1_0Pro, api_key);

    let txt_request = Request {
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some("¿Qué es un Smart Pointer en Rust?".to_string()),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        }],
        // ✅ Sin imports extra — vec![] infiere el tipo solo
        safety_settings: vec![],
        generation_config: None,
        tools: vec![],
    };

    println!("Consultando a Gemini (v0.3.4)...");

    let response = client.post(30, &txt_request).await?;

    // ─── DIAGNÓSTICO: descomenta UNA línea a la vez y mira el error ───
    // let _: () = response;               // ← revela el tipo de PostResult
    // let _: () = response.rest;          // ← si existe .rest
    // let _: () = response.streaming;     // ← si existe .streaming
    // ──────────────────────────────────────────────────────────────────

    // Intento con .rest (campo más probable en PostResult)
    // .rest no es Option, se accede directo
    let rest = response.rest();

    if let Some(gemini_response) = rest {
        if let Some(candidate) = gemini_response.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                if let Some(text) = &part.text {
                    println!("\n--- Respuesta ---\n{}", text);
                }
            }
        }
    } else {
        println!("No se recibió respuesta.");
    }

    Ok(())
}