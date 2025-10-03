use axum::{
    extract::{
        ws::{Message},
        Path, State,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get},
    Json, Router,
};
use serde_json::json;
use std::{sync::{Arc, Mutex}};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use tokio::sync::{RwLock, mpsc};
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

// Declare modules
mod data_types;
mod nc_object;
mod nc_block;
mod websocket;

// Import from modules
use data_types::*;
use nc_object::*;
use nc_block::*;
use websocket::*;

// A shared state for our application
struct AppState {
    connections: RwLock<HashMap<Uuid, ConnectionState>>,
    device: NmosDevice,
    event_rx: RwLock<mpsc::UnboundedReceiver<PropertyChangedEvent>>,
    root_block: Mutex<NcBlock>
}

/// Holds the WebSocket connection state including subscriptions and the sender for notifications
#[derive(Debug)]
struct ConnectionState {
    subscribed_oids: HashSet<u64>,
    sender: mpsc::UnboundedSender<Message>
}

impl AppState {
    /// Means of notifying subscribers if the oid is contained in their subscriptions list
    async fn notify_subscribers(&self, event_data: PropertyChangedEvent) {
        let conns = self.connections.read().await;

        for conn in conns.values() {
            if conn.subscribed_oids.contains(&event_data.oid) {
                let response = WsNotificationMessage {
                    message_type: MESSAGE_TYPE_NOTIFICATION,
                    notifications: vec![event_data.clone()]
                };
                if let Ok(resp_text) = serde_json::to_string(&response) {
                    let _ = conn.sender.send(Message::Text(resp_text));
                }
            }
        }
    }
}

/// Returns a TAI timestamp in the format "<seconds>:<nanoseconds>"
pub fn tai_timestamp() -> String {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH)
        .expect("System time is before Unix epoch");

    let seconds = since_epoch.as_secs();
    let nanos = since_epoch.subsec_nanos();

    // Current TAI–UTC offset is +37s
    let tai_seconds = seconds + 37;

    format!("{}:{}", tai_seconds, nanos)
}

#[tokio::main]
async fn main() {
    // Set up logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create a minimal NmosDevice so we can advertise the IS-12 control
    let device = NmosDevice {
        id: "67c25159-ce25-4000-a66c-f31fff890265".to_string(), //Fixed for easy persistence, replace with for generated uuid: Uuid::new_v4().to_string(),
        label: "Example Device".to_string(),
        description: "An example NMOS device".to_string(),
        senders: vec![],
        receivers: vec![],
        node_id: Uuid::new_v4().to_string(),
        type_: "urn:x-nmos:device:generic".to_string(),
        version: tai_timestamp(),
        controls: vec![
            DeviceControl {
                type_: "urn:x-nmos:control:ncp/v1.0".to_string(),
                href: "ws://127.0.0.1:3000/ws".to_string(),
                authorization: false
            }
        ]
    };

    println!("Device ID: {}", device.id);

    // Create a channel where our device model objects can notify
    let (tx, rx) = mpsc::unbounded_channel::<PropertyChangedEvent>();

    let mut root_block = NcBlock::new(true, vec![1, 1], 1, true, None,"root", None, true, tx.clone());
    
    let obj_1 = NcObject::new(vec![1], 2, true, Some(1), "test", Some("test"), tx.clone());
    
    root_block.add_member(obj_1);

    let app_state = Arc::new(AppState {
        device, 
        root_block: Mutex::new(root_block),
        connections: RwLock::new(HashMap::new()),
        event_rx: RwLock::new(rx)
    });

    tokio::spawn(run_event_loop(app_state.clone()));

    // Build our web application with the routes
    let app = Router::new()
        .route("/x-nmos/node/v1.3/devices/", get(devices_rest_api_handler)) // REST API endpoint for all devices
        .route("/x-nmos/node/v1.3/devices/:id", get(device_rest_api_handler)) // REST API endpoint for specific device
        .route("/ws", get(websocket_handler)) // WebSocket endpoint
        .with_state(app_state);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Run event loop to funnel any events received from device model objects to subscribes via notifications
async fn run_event_loop(state: Arc<AppState>) {
    loop {
        let maybe_event = {
            let mut rx = state.event_rx.write().await;
            rx.recv().await
        };

        if let Some(event_data   ) = maybe_event {
            state.notify_subscribers(event_data).await;
        } else {
            break; // channel closed, exit
        }
    }
}

// --- REST API Handler ---

async fn devices_rest_api_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut response = Vec::new();
    response.push(state.device.clone());
    (StatusCode::OK, Json(json!(response)))
}

async fn device_rest_api_handler(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> impl IntoResponse {
    if state.device.id == id {
        (StatusCode::OK, Json(json!(state.device)))
    } else {
        (StatusCode::NOT_FOUND, Json(json!({ "error": "device not found" })))
    }
}