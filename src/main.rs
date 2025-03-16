use std::{collections::HashMap, time::Instant};
use axum::{http::{HeaderMap, HeaderValue, StatusCode}, response::IntoResponse, routing::get, Router};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use tokio::net::TcpListener;
use reqwest::{self, Url};
use clap::{Parser, command};

static CACHE: Lazy<RwLock<HashMap<String, (String, Instant)>>> = Lazy::new(|| RwLock::new(HashMap::new()));
const CACHE_EXPIRATION_SECS: u64 = 60;

#[derive(Parser, Debug)]
#[command(name = "cache-proxy")]
#[command(version = "0.1.0")]
#[command(about = "A simple cache proxy server with expiration and improved concurrency")]
pub struct Args {
    #[clap(short, long, default_value = "3000")]
    pub port: u16,

    #[clap(short, long, default_value = "http://localhost")] 
    pub origin: String,

    #[clap(long("clear-cache"))]
    pub clear_cache: bool,
}

pub async fn handle_request(origin: String) -> Result<impl IntoResponse, StatusCode> {
    let parsed_url = Url::parse(&origin).map_err(|_| StatusCode::BAD_REQUEST)?;
    let cache_key = parsed_url.as_str().to_string();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));

    {
        let cache: tokio::sync::RwLockReadGuard<'_, HashMap<String, (String, Instant)>> = CACHE.read().await;
        if let Some((cached_response, timestamp)) = cache.get(&cache_key) {
            if timestamp.elapsed().as_secs() < CACHE_EXPIRATION_SECS {
                headers.insert("X-Cache", HeaderValue::from_static("HIT"));
                return Ok((headers, cached_response.clone()));
            }
        }
    }

    let response = reqwest::get(&origin).await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let body = response.text().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    headers.insert("X-Cache", HeaderValue::from_static("MISS"));
    CACHE.write().await.insert(cache_key, (body.clone(), Instant::now()));
    
    Ok((headers, body))
}

pub async fn clear_cache() {
    CACHE.write().await.clear();
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if args.clear_cache {
        clear_cache().await;
        return;
    }

    let app = Router::new().route("/", get(move || async move { handle_request(args.origin.clone()).await }));
    let addr = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
