use axum::extract::{State, ws::*};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use serde_json::{from_value, json};
use std::{collections::HashSet, sync::Arc};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{AppState, ConnectionState, data_types::*, nc_object::NcMember};

/// WebSocket entrypoint
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let conn_id = Uuid::new_v4();
    ws.on_upgrade(move |socket| handle_socket(socket, state, conn_id))
}

/// Background event loop
pub async fn run_event_loop(state: Arc<AppState>) {
    while let Some(event) = state.event_rx.lock().await.recv().await {
        state.notify_subscribers(event).await;
    }
}

/// Processes a WsCommandMessage into a response
async fn process_command(msg: WsCommandMessage, state: Arc<AppState>) -> WsCommandResponseMessage {
    let mut responses = Vec::new();

    for cmd in msg.commands {
        let mut root = state.root_block.lock().await;

        let (status, error_message, value) = match (cmd.method_id.level, cmd.method_id.index) {
            (1, 1) => match from_value::<IdArgs>(cmd.arguments) {
                Ok(id_args) => {
                    let (err, val, status) = root.get_property(cmd.oid, &id_args);
                    (status, err, val)
                }
                Err(e) => (
                    NcMethodStatus::BadCommandFormat,
                    Some(format!("Invalid args: {e}")),
                    json!(null),
                ),
            },
            (1, 2) => match from_value::<IdArgsValue>(cmd.arguments) {
                Ok(id_val) => {
                    let (err, status_code) = root.set_property(cmd.oid, id_val);
                    (status_code, err, json!(null))
                }
                Err(e) => (
                    NcMethodStatus::BadCommandFormat,
                    Some(format!("Invalid args: {e}")),
                    json!(null),
                ),
            },
            _ => {
                let (err, resp, status) = root.invoke_method(cmd.oid, cmd.method_id, cmd.arguments);
                (status, err, resp.unwrap_or(json!(null)))
            }
        };

        let result = if error_message.is_some() {
            ResponsePayload::Error(ResponseError {
                base: ResponseBase { status },
                error_message,
            })
        } else {
            ResponsePayload::Result(ResponseResult {
                base: ResponseBase { status },
                value,
            })
        };

        responses.push(Response {
            handle: cmd.handle,
            result,
        });
    }

    WsCommandResponseMessage {
        message_type: MESSAGE_TYPE_COMMAND_RESPONSE,
        responses,
    }
}

/// Handles a single client connection
async fn handle_socket(socket: WebSocket, state: Arc<AppState>, conn_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // Spawn sending loop
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Register connection
    {
        let mut conns = state.connections.write().await;
        conns.insert(
            conn_id,
            ConnectionState {
                subscribed_oids: HashSet::new(),
                sender: tx.clone(),
            },
        );
    }

    let state_c = state.clone();
    let tx_c = tx.clone();

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // Try command
                if let Ok(cmd) = serde_json::from_str::<WsCommandMessage>(&text)
                    && cmd.message_type == MESSAGE_TYPE_COMMAND
                {
                    let response = process_command(cmd, state_c.clone()).await;
                    if let Ok(txt) = serde_json::to_string(&response) {
                        let _ = tx_c.send(Message::Text(txt.into()));
                    }
                    continue;
                }

                // Try subscription
                if let Ok(sub) = serde_json::from_str::<WsSubscriptionMessage>(&text)
                    && sub.message_type == MESSAGE_TYPE_SUBSCRIPTION
                {
                    let mut conns = state_c.connections.write().await;
                    if let Some(c) = conns.get_mut(&conn_id) {
                        c.subscribed_oids = sub.subscriptions.iter().cloned().collect();
                    }
                    let resp = WsSubscriptionResponseMessage {
                        message_type: MESSAGE_TYPE_SUBSCRIPTION_RESPONSE,
                        subscriptions: sub.subscriptions,
                    };
                    if let Ok(txt) = serde_json::to_string(&resp) {
                        let _ = tx_c.send(Message::Text(txt.into()));
                    }
                    continue;
                }

                // Otherwise, error
                let _ = tx_c.send(Message::Text(
                    serde_json::to_string(&WsErrorMessage {
                        message_type: MESSAGE_TYPE_ERROR,
                        status: 400,
                        error_message: "Invalid message".into(),
                    })
                    .unwrap()
                    .into(),
                ));
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    state.connections.write().await.remove(&conn_id);
}
