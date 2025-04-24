use askama::Template;
use axum::{
    Router,
    response::Html,
    routing::{get, post},
};

#[derive(Template)]
#[template(path = "main.html")]
struct MainTemplate {
    doggo_src: String,
}

#[derive(Template)]
#[template(path = "doggo_image.html")]
struct DoggoImageTemplate {
    doggo_src: String,
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_root))
        .route("/skip", get(get_skip));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_skip() -> Html<String> {
    println!("GET /skip");
    let doggo_src = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let dog: DogApi = serde_json::from_str(&doggo_src).unwrap();
    let template = DoggoImageTemplate {
        doggo_src: dog.message,
    };
    Html(template.render().unwrap())
}

async fn get_root() -> Html<String> {
    println!("GET /");
    let doggo_src = reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let dog: DogApi = serde_json::from_str(&doggo_src).unwrap();
    let template = MainTemplate {
        doggo_src: dog.message,
    };
    // "Hello World".to_string()
    Html(template.render().unwrap())
}
