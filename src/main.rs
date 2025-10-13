use axum::{
    Json, Router,
    extract::{Path, State, ws::Message},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde_json::json;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::{Mutex, RwLock, mpsc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

// Declare modules
mod data_types;
mod nc_block;
mod nc_device_manager;
mod nc_object;
mod websocket;

// Imports
use crate::{
    data_types::{DeviceControl, NcManufacturer, NcProduct, NmosDevice, PropertyChangedEvent},
    nc_block::NcBlock,
    nc_device_manager::NcDeviceManager,
    nc_object::NcObject,
    websocket::{run_event_loop, websocket_handler},
};

// === AppState ===

pub struct AppState {
    pub connections: RwLock<HashMap<Uuid, ConnectionState>>,
    pub device: NmosDevice,
    pub root_block: Mutex<NcBlock>,
    pub event_rx: Mutex<mpsc::UnboundedReceiver<PropertyChangedEvent>>,
}

/// Each WebSocket connection’s state
#[derive(Debug)]
pub struct ConnectionState {
    pub subscribed_oids: std::collections::HashSet<u64>,
    pub sender: mpsc::UnboundedSender<Message>,
}

impl AppState {
    /// Broadcasts a property change to all subscribed clients.
    pub async fn notify_subscribers(&self, event_data: PropertyChangedEvent) {
        let conns = self.connections.read().await;
        let payload = serde_json::to_string(&crate::data_types::WsNotificationMessage {
            message_type: crate::data_types::MESSAGE_TYPE_NOTIFICATION,
            notifications: vec![event_data.clone()],
        })
        .unwrap();

        for conn in conns.values() {
            if conn.subscribed_oids.contains(&event_data.oid) {
                let _ = conn.sender.send(Message::Text(payload.clone().into()));
            }
        }
    }
}

/// Returns a TAI timestamp in `<seconds>:<nanoseconds>` format.
pub fn tai_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before Unix epoch");
    format!("{}:{}", now.as_secs() + 37, now.subsec_nanos())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logging setup
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create device
    let device = NmosDevice {
        id: "67c25159-ce25-4000-a66c-f31fff890265".into(), //Use: Uuid::new_v4().to_string() to generate new uuid
        label: "Example Device".into(),
        description: "An example NMOS device".into(),
        senders: vec![],
        receivers: vec![],
        node_id: Uuid::new_v4().to_string(),
        type_: "urn:x-nmos:device:generic".into(),
        version: tai_timestamp(),
        controls: vec![DeviceControl {
            type_: "urn:x-nmos:control:ncp/v1.0".into(),
            href: "ws://127.0.0.1:3000/ws".into(),
            authorization: false,
        }],
    };

    // Model setup
    let (tx, rx) = mpsc::unbounded_channel::<PropertyChangedEvent>();

    // Create the root block
    let mut root = NcBlock::new(
        true,
        vec![1, 1],
        1,
        true,
        None,
        "root",
        None,
        true,
        tx.clone(),
    );

    let device_manager = NcDeviceManager::new(
        2,
        true,
        Some(1),
        "DeviceManager",
        Some("Device Manager"),
        "v1.0.0".to_string(),
        NcManufacturer {
            name: "Your Company".to_string(),
            organization_id: None,
            website: Some("https://example.com".to_string()),
        },
        NcProduct {
            name: "Your Product".to_string(),
            key: "MODEL-XYZ-2000".to_string(),
            revision_level: "1.0".to_string(),
            brand_name: Some("Your Brand".to_string()),
            uuid: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
            description: Some("Professional device".to_string()),
        },
        "SN-123456789".to_string(),
        tx.clone(),
    );
    root.add_member(Box::new(device_manager));

    // Add NcObject member
    let obj_1 = NcObject::new(
        vec![1],
        3,
        true,
        Some(1),
        "my-obj-01",
        Some("My object 01"),
        tx.clone(),
    );
    root.add_member(Box::new(obj_1));

    // Add NcBlock member
    let mut block_1 = NcBlock::new(
        false,
        vec![1, 1],
        4,
        true,
        None,
        "my-block-01",
        None,
        true,
        tx.clone(),
    );
    let obj_2 = NcObject::new(
        vec![1],
        5,
        true,
        Some(1),
        "my-nested-block-obj",
        None,
        tx.clone(),
    );
    block_1.add_member(Box::new(obj_2));
    root.add_member(Box::new(block_1));

    let app_state = Arc::new(AppState {
        device,
        connections: RwLock::new(HashMap::new()),
        root_block: Mutex::new(root),
        event_rx: Mutex::new(rx),
    });

    // Event loop background task
    tokio::spawn(run_event_loop(app_state.clone()));

    // Routes
    let app = Router::new()
        .route("/x-nmos/node/v1.3/devices/", get(devices_rest_api_handler))
        .route(
            "/x-nmos/node/v1.3/devices/{id}",
            get(device_rest_api_handler),
        )
        .route("/ws", get(websocket_handler))
        .with_state(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

// --- REST Handlers ---

async fn devices_rest_api_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!([state.device])))
}

async fn device_rest_api_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if state.device.id == id {
        (StatusCode::OK, Json(json!(state.device)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "device not found" })),
        )
    }
}
