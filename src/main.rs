use axum::{
    Json, Router,
    extract::{Path, State, ws::Message},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use gethostname::gethostname;
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
mod nc_class_manager;
mod nc_device_manager;
mod nc_manager;
mod nc_object;
mod websocket;

// Imports
use crate::{
    data_types::{
        DeviceControl, NcManufacturer, NcProduct, NmosApi, NmosClock, NmosDevice, NmosEndpoint,
        NmosInterface, NmosNode, PropertyChangedEvent,
    },
    nc_block::NcBlock,
    nc_class_manager::NcClassManager,
    nc_device_manager::NcDeviceManager,
    nc_object::NcObject,
    websocket::{run_event_loop, websocket_handler},
};

// === AppState ===

pub struct AppState {
    pub connections: RwLock<HashMap<Uuid, ConnectionState>>,
    pub node: NmosNode,
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

        for (_conn_id, conn) in conns.iter() {
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

    let hostname = gethostname();

    // Create node
    let node = NmosNode::new(
        Uuid::new_v4().to_string(),
        "Example Node".into(),
        "An example NMOS node".into(),
        tai_timestamp(),
        HashMap::new(),
        "http://127.0.0.1:3000".into(),
        hostname.to_string_lossy().into(),
        vec![NmosClock {
            name: "clk0".into(),
            ref_type: "internal".into(),
        }],
        vec![
            NmosInterface {
                chassis_id: "00-15-5d-67-c3-4e".into(),
                name: "eth0".into(),
                port_id: "00-15-5d-67-c3-4e".into(),
            },
            NmosInterface {
                chassis_id: "96-1c-70-61-b1-54".into(),
                name: "eth1".into(),
                port_id: "96-1c-70-61-b1-54".into(),
            },
        ],
        NmosApi {
            endpoints: vec![NmosEndpoint {
                host: "127.0.0.1".into(),
                port: 3000,
                protocol: "http".into(),
            }],
            versions: vec!["v1.3".into()],
        },
    );

    // Create device
    let device = NmosDevice::new(
        "67c25159-ce25-4000-a66c-f31fff890265".into(), // id - Use: Uuid::new_v4().to_string() to generate new uuid
        "Example Device".into(),
        "An example NMOS device".into(),
        tai_timestamp(),
        HashMap::new(),
        vec![],
        vec![],
        node.base.id.clone(),
        "urn:x-nmos:device:generic".into(),
        vec![DeviceControl {
            type_: "urn:x-nmos:control:ncp/v1.0".into(),
            href: "ws://127.0.0.1:3000/ws".into(),
            authorization: false,
        }],
    );

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
        None,
        None,
        tx.clone(),
    );

    let device_manager = NcDeviceManager::new(
        2,
        true,
        Some(1),
        "DeviceManager",
        Some("Device Manager"),
        Some(vec![crate::data_types::NcTouchpoint::Nmos(
            crate::data_types::NcTouchpointNmos {
                base: crate::data_types::NcTouchpointBase {
                    context_namespace: "x-nmos".into(),
                },
                resource: crate::data_types::NcTouchpointResourceNmos {
                    base: crate::data_types::NcTouchpointResourceBase {
                        resource_type: "device".into(),
                    },
                    id: device.base.id.clone(),
                },
            },
        )]),
        None,
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

    let class_manager = NcClassManager::new(
        3,
        true,
        Some(1),
        "ClassManager",
        Some("Class Manager"),
        None,
        None,
        tx.clone(),
    );
    root.add_member(Box::new(class_manager));

    // Add NcObject member
    let obj_1 = NcObject::new(
        vec![1],
        4,
        true,
        Some(1),
        "my-obj-01",
        Some("My object 01"),
        None,
        None,
        tx.clone(),
    );
    root.add_member(Box::new(obj_1));

    // Add NcBlock member
    let mut block_1 = NcBlock::new(
        false,
        vec![1, 1],
        5,
        true,
        Some(1),
        "my-block-01",
        None,
        true,
        None,
        None,
        tx.clone(),
    );
    let obj_2 = NcObject::new(
        vec![1],
        6,
        true,
        Some(5),
        "my-nested-block-obj",
        None,
        None,
        None,
        tx.clone(),
    );
    block_1.add_member(Box::new(obj_2));
    root.add_member(Box::new(block_1));

    let app_state = Arc::new(AppState {
        node,
        device,
        connections: RwLock::new(HashMap::new()),
        root_block: Mutex::new(root),
        event_rx: Mutex::new(rx),
    });

    // Event loop background task
    tokio::spawn(run_event_loop(app_state.clone()));

    // Routes
    let app = Router::new()
        .route("/x-nmos/node/v1.3", get(base_is_04_rest_api_handler))
        .route("/x-nmos/node/v1.3/", get(base_is_04_rest_api_handler))
        .route("/x-nmos/node/v1.3/self", get(node_self_rest_api_handler))
        .route("/x-nmos/node/v1.3/sources", get(sources_rest_api_handler))
        .route("/x-nmos/node/v1.3/sources/", get(sources_rest_api_handler))
        .route(
            "/x-nmos/node/v1.3/sources/{id}",
            get(source_rest_api_handler),
        )
        .route("/x-nmos/node/v1.3/flows", get(flows_rest_api_handler))
        .route("/x-nmos/node/v1.3/flows/", get(flows_rest_api_handler))
        .route("/x-nmos/node/v1.3/flows/{id}", get(flow_rest_api_handler))
        .route("/x-nmos/node/v1.3/senders", get(senders_rest_api_handler))
        .route("/x-nmos/node/v1.3/senders/", get(senders_rest_api_handler))
        .route(
            "/x-nmos/node/v1.3/senders/{id}",
            get(sender_rest_api_handler),
        )
        .route(
            "/x-nmos/node/v1.3/receivers",
            get(receivers_rest_api_handler),
        )
        .route(
            "/x-nmos/node/v1.3/receivers/",
            get(receivers_rest_api_handler),
        )
        .route(
            "/x-nmos/node/v1.3/receivers/{id}",
            get(receiver_rest_api_handler),
        )
        .route("/x-nmos/node/v1.3/devices", get(devices_rest_api_handler))
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

async fn base_is_04_rest_api_handler(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!([
            "self/",
            "sources/",
            "flows/",
            "devices/",
            "senders/",
            "receivers/"
        ])),
    )
}

async fn node_self_rest_api_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!(state.node)))
}

async fn sources_rest_api_handler(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!([])))
}

async fn source_rest_api_handler(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

async fn flows_rest_api_handler(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!([])))
}

async fn flow_rest_api_handler(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

async fn senders_rest_api_handler(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!([])))
}

async fn sender_rest_api_handler(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

async fn receivers_rest_api_handler(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!([])))
}

async fn receiver_rest_api_handler(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

async fn devices_rest_api_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    (StatusCode::OK, Json(json!([state.device])))
}

async fn device_rest_api_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if state.device.base.id == id {
        (StatusCode::OK, Json(json!(state.device)))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "device not found" })),
        )
    }
}
