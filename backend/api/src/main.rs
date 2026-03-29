use base64::Engine;
use object_store::aws::AmazonS3Builder;
use sea_orm::DbConn;
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid;

mod attachment_service;
mod auth;
mod auth_service;
mod comment_service;
mod db;
mod git_service;
mod init;
mod issue_service;
mod notification_service;
mod project_service;
mod rbac_service;
mod routes;
mod sse_service;
mod webhook_worker;

pub use sse_service::SseEvent;

#[derive(Clone)]
pub struct AppState {
    pub db_conn: DbConn,
    pub aes_key: Arc<[u8; 32]>,
    pub sse_tx: broadcast::Sender<SseEvent>,
    pub storage: Arc<dyn object_store::ObjectStore>,
    pub job_tx: tokio::sync::mpsc::Sender<uuid::Uuid>,
    pub http_client: reqwest::Client,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("api=info".parse().unwrap()),
        )
        .init();

    let aes_key = load_aes_key();

    let db_conn = match db::establish_connection().await {
        Ok(conn) => {
            tracing::info!("database connection established");
            conn
        }
        Err(e) => {
            tracing::error!(error = %e, "failed to connect to database");
            std::process::exit(1);
        }
    };

    if let Err(e) = migration::Migrator::up(&db_conn, None).await {
        tracing::error!(error = %e, "failed to run migrations");
        std::process::exit(1);
    }
    tracing::info!("migrations applied");

    if let Err(e) = init::initialize_database(&db_conn).await {
        tracing::error!(error = %e, "failed to initialize database");
        std::process::exit(1);
    }

    let storage = build_storage();
    let (sse_tx, _) = broadcast::channel(1024);
    let (job_tx, job_rx) = tokio::sync::mpsc::channel::<uuid::Uuid>(512);

    let state = AppState {
        db_conn,
        aes_key: Arc::new(aes_key),
        sse_tx,
        storage,
        job_tx,
        http_client: reqwest::Client::new(),
    };

    let worker_state = state.clone();
    tokio::spawn(async move {
        webhook_worker::run(worker_state, job_rx).await;
    });

    let app = routes::create_router().with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!(addr = %addr, "server listening");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

fn load_aes_key() -> [u8; 32] {
    let raw = std::env::var("AES_SECRET").unwrap_or_else(|_| {
        tracing::warn!("AES_SECRET not set, using insecure default — set it in production");
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string()
    });
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(raw.trim())
        .expect("AES_SECRET must be valid base64");
    assert!(
        bytes.len() == 32,
        "AES_SECRET must decode to exactly 32 bytes, got {}",
        bytes.len()
    );
    let mut key = [0u8; 32];
    key.copy_from_slice(&bytes);
    key
}

fn build_storage() -> Arc<dyn object_store::ObjectStore> {
    let endpoint =
        std::env::var("S3_ENDPOINT").unwrap_or_else(|_| "http://minio:9000".to_string());
    let bucket = std::env::var("S3_BUCKET").unwrap_or_else(|_| "ferroliner".to_string());
    let access_key =
        std::env::var("S3_ACCESS_KEY").unwrap_or_else(|_| "minioadmin".to_string());
    let secret_key =
        std::env::var("S3_SECRET_KEY").unwrap_or_else(|_| "minioadmin".to_string());
    let region = std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());

    let store = AmazonS3Builder::new()
        .with_endpoint(&endpoint)
        .with_bucket_name(&bucket)
        .with_access_key_id(&access_key)
        .with_secret_access_key(&secret_key)
        .with_region(&region)
        .with_allow_http(true)
        .build()
        .expect("failed to build object store");

    Arc::new(store)
}
