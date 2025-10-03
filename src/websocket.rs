use axum::extract::{ws::*, State};
use axum::response::IntoResponse;
use std::sync::Arc;
use uuid::Uuid;
use serde_json::json;
use serde_json::{from_value};
use tokio::sync::mpsc;
use futures_util::{
    sink::SinkExt,
    stream::{StreamExt},
};
use std::collections::HashSet;
use crate::{AppState, ConnectionState};
use crate::data_types::*;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let conn_id = Uuid::new_v4();
    ws.on_upgrade(move |socket| handle_socket(socket, state, conn_id))
}

// Process the commands received and forward them to the right object within our device model
pub async fn process_command(msg: WsCommandMessage, state: Arc<AppState>) -> WsCommandResponseMessage {
    let mut responses = Vec::new();
    for cmd in msg.commands {
        let _method_id = cmd.method_id;
        if _method_id.level == 1 && _method_id.index == 1 {
            // Generic get property method
            let id_args: IdArgs = match from_value(cmd.arguments) {
                Ok(args) => args,
                Err(e) => {
                    responses.push(Response {
                        handle: cmd.handle,
                        result: ResponseResult {
                            status: 400,
                            error_message: Some(format!("Invalid arguments: {}", e)),
                            value: json!(null)
                        }
                    });
                    continue;
                }
            };

            // Lock, clone the block, release lock
            let (error, payload) = state.root_block.lock().unwrap().clone()
                .get_property(cmd.oid, &id_args);

            let result = match error {
                Some(v) => ResponseResult {
                    status: 400,
                    error_message: Some(v),
                    value: json!(null),
                },
                None => ResponseResult {
                    status: 200,
                    error_message: None,
                    value: payload,
                },
            };

            responses.push(Response {
                handle: cmd.handle,
                result,
            });
        } else if _method_id.level == 1 && _method_id.index == 2 {
            // Generic set property method

            let id_args_value: IdArgsValue = match from_value(cmd.arguments) {
                Ok(args) => args,
                Err(e) => {
                    responses.push(Response {
                        handle: cmd.handle,
                        result: ResponseResult {
                            status: 400,
                            error_message: Some(format!("Invalid arguments: {}", e)),
                            value: json!(null)
                        }
                    });
                    continue;
                }
            };

            // Lock, do the mutation synchronously, then await if needed
            let (error, success) = state.root_block.lock().unwrap()
                .set_property(cmd.oid, id_args_value);

            let result = match success {
                false => ResponseResult {
                    status: 400,
                    error_message: error,
                    value: json!(null)
                },
                true => ResponseResult {
                    status: 200,
                    error_message: None,
                    value: json!(null)
                }
            };

            responses.push(Response {
                handle: cmd.handle,
                result,
            });
        } else {
            //TODO: This is where we will need to support other methods outside Get/Set by calling an InvokeMethod implemented by each object

            let (error, response) = state.root_block.lock().unwrap()
                .invoke_method(cmd.oid, _method_id, cmd.arguments);

            let result = match error {
                Some(v) => ResponseResult {
                    status: 400,
                    error_message: Some(v),
                    value: json!(null),
                },
                None => ResponseResult {
                    status: 200,
                    error_message: None,
                    value: response.unwrap_or_else(|| json!(null)),
                }
            };

            responses.push(Response {
                handle: cmd.handle,
                result,
            });
        }
    }

    WsCommandResponseMessage {
        message_type: MESSAGE_TYPE_COMMAND_RESPONSE,
        responses
    }
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, conn_id: Uuid) {
    println!("New WebSocket connection: {}", conn_id);

    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Create a channel to send messages into this WebSocket
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // Task to forward messages from rx -> actual WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break; // client disconnected
            }
        }
    });

    // Insert into AppState
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

    // Task to receive messages from the WebSocket
    let state_clone = state.clone();
    let tx_clone = tx.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if let Message::Text(text) = msg {
                if let Ok(command) = serde_json::from_str::<WsCommandMessage>(&text) && command.message_type == MESSAGE_TYPE_COMMAND {
                    let response = process_command(command, state_clone.clone()).await;
                    if let Ok(resp_text) = serde_json::to_string(&response) {
                        let _ = tx_clone.send(Message::Text(resp_text));
                    }
                } else if let Ok(command) = serde_json::from_str::<WsSubscriptionMessage>(&text) && command.message_type == MESSAGE_TYPE_SUBSCRIPTION {
                    let mut conns = state_clone.connections.write().await;
                    if let Some(conn_state) = conns.get_mut(&conn_id) {
                        conn_state.subscribed_oids.clear();
                        conn_state.subscribed_oids.extend(command.subscriptions.iter());
                    }
                    let response = WsSubscriptionResponseMessage {
                        message_type: MESSAGE_TYPE_SUBSCRIPTION_RESPONSE,
                        subscriptions: command.subscriptions
                    };
                    if let Ok(resp_text) = serde_json::to_string(&response) {
                        let _ = tx_clone.send(Message::Text(resp_text));
                    }
                } else {
                    println!("Failed to parse command");
                    let _ = tx_clone.send(Message::Text(
                        serde_json::to_string(&WsErrorMessage {
                            message_type: MESSAGE_TYPE_ERROR,
                            status: 400,
                            error_message: "Failed to parse command".to_string(),
                        }).unwrap()));
                }
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = send_task => {}
        _ = recv_task => {}
    }

    // Cleanup on disconnect
    let mut conns = state.connections.write().await;
    conns.remove(&conn_id);
    println!("WebSocket connection closed: {}", conn_id);
}