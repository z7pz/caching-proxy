use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use clap::{Parser, command};
use redis::AsyncCommands;
use tokio::net::TcpListener;
use tokio::sync::OnceCell;
use tower_http::{compression::CompressionLayer, limit::RequestBodyLimitLayer};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use url::Url;

static REDIS_CLIENT: OnceCell<redis::Client> = OnceCell::const_new();

#[derive(Parser, Debug)]
#[command(name = "cache-proxy")]
#[command(version = "0.2.0")]
#[command(about = "A caching proxy with Redis, compression, and rate limiting")]
pub struct Args {
    #[clap(short, long, default_value = "3000")]
    pub port: u16,

    #[clap(short, long, default_value = "http://localhost")]
    pub origin: String,

    #[clap(long, default_value = "60")]
    pub cache_ttl: u64,

    #[clap(long, default_value = "redis://127.0.0.1/")]
    pub redis_url: String,

    #[clap(long("clear-cache"))]
    pub clear_cache: bool,
}

async fn get_redis_client(url: &str) -> &'static redis::Client {
    REDIS_CLIENT
        .get_or_init(|| async { redis::Client::open(url).unwrap() })
        .await
}

pub async fn handle_request(origin: String, cache_ttl: u64) -> Result<impl IntoResponse, StatusCode> {
    let parsed_url = Url::parse(&origin).map_err(|_| StatusCode::BAD_REQUEST)?;
    let cache_key = parsed_url.as_str().to_string();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));

    let client = get_redis_client("redis://127.0.0.1/").await;
    let mut con = client.get_multiplexed_async_connection().await.unwrap();

    if let Ok(cached_response) = con.get::<_, String>(&cache_key).await {
        headers.insert("X-Cache", HeaderValue::from_static("HIT"));
        return Ok((headers, cached_response));
    }

    let response = reqwest::get(&origin).await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let body = response.text().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    headers.insert("X-Cache", HeaderValue::from_static("MISS"));
    let _: () = con.set_ex(&cache_key, &body, cache_ttl.try_into().unwrap()).await.unwrap();

    Ok((headers, body))
}

pub async fn clear_cache(redis_url: &str) {
    let client = get_redis_client(redis_url).await;
    let mut con = client.get_connection().unwrap();
    redis::cmd("FLUSHDB").execute(&mut con);
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    if args.clear_cache {
        clear_cache(&args.redis_url).await;
        info!("Cache cleared successfully.");
        return;
    }

    let app = Router::new()
        .route("/", get(move || async move { handle_request(args.origin.clone(), args.cache_ttl).await }))
        .layer(CompressionLayer::new()) // Gzip compression
        .layer(RequestBodyLimitLayer::new(1024 * 1024)); // Limit request body to 1MB

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    info!("Proxy server running at {}", addr);
    axum::serve(listener, app).await.unwrap();
}
