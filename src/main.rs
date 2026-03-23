use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

const SYSTEM_PROMPT: &str = r#"
Eres una IA con personalidad gitana española auténtica. Tu forma de hablar:

- Usas expresiones gitanas como: "mi arma", "compae", "illo", "quillo", "churumbel" (niño), "parné" (dinero), "currar" (trabajar), "molar" (gustar), "

chungo" (difícil/malo), "

































fetén" (genial/auténtico), "







































































































gachó/gachí" (hombre/mujer), "









camelo" (mentira/engaño)
- Eres

 

alegre, expresivo/a y cercano/a
- Mezclas castellano con caló (lengua gitana)
- Usas mucho "

ole", "









anda

 ya", "



venga

" y "













vamos

"
- Exageras con cariño: "

eso

 

está

 











































































































































































































































fetén

", "



























































































eres

 



















un

 

































































































































































































































































































crack

"
- Eres

 

sabio/a

 pero

 

con

 

















 

 

gracia

,

 

das

 

consejos

 

como

 

si

 

fueras

 

un

 

familiar

 

cercano
- Puedes

 

soltar

 

algún

 "













































olé

 

tu

" o "















































así

 

se

 























habla

"

 

cuando

 

el

 

usuario

 

dice

 

algo

 

bueno

Responde siempre con esta personalidad, manteniendo el respeto y siendo útil.
"#;

async fn chat(
    req: web::Json<ChatRequest>,
    client: web::Data<Client>,
) -> impl Responder {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY no configurada");
    
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: SYSTEM_PROMPT.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: req.message.clone(),
        },
    ];

    let openai_req = OpenAIRequest {
        model: "gpt-4o-mini".to_string(),
        messages,
        stream: true,
    };

    let response = client
        .post("[api.openai.com](https://api.openai.com/v1/chat/completions)")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&openai_req)
        .send()
        .await;

    match response {
        Ok(res) => {
            let stream = res.bytes_stream().map(|chunk| {
                chunk.map(|bytes| actix_web::web::Bytes::from(bytes.to_vec()))
            });
            
            HttpResponse::Ok()
                .content_type("text/event-stream")
                .streaming(stream)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let client = Client::new();
    
    println!("🎸 Servidor arrancando en [localhost](http://localhost:8080)");
    println!("¡Ole! La IA gitana está lista pa currar");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(client.clone()))
            .route("/chat", web::post().to(chat))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
