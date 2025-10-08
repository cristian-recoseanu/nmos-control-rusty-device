use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const MESSAGE_TYPE_COMMAND: u16 = 0;
pub const MESSAGE_TYPE_COMMAND_RESPONSE: u16 = 1;
pub const MESSAGE_TYPE_NOTIFICATION: u16 = 2;
pub const MESSAGE_TYPE_SUBSCRIPTION: u16 = 3;
pub const MESSAGE_TYPE_SUBSCRIPTION_RESPONSE: u16 = 4;
pub const MESSAGE_TYPE_ERROR: u16 = 5;

#[derive(Serialize, Clone)]
pub struct DeviceControl {
    #[serde(rename = "type")]
    pub type_: String,
    pub href: String,
    pub authorization: bool,
}

#[derive(Serialize, Clone)]
pub struct NmosDevice {
    pub id: String,
    pub label: String,
    pub description: String,
    pub senders: Vec<String>,
    pub receivers: Vec<String>,
    pub node_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub version: String,
    pub controls: Vec<DeviceControl>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementId {
    pub level: u32,
    pub index: u32,
}

#[derive(Deserialize, Debug)]
pub struct Command {
    pub handle: u64,
    pub oid: u64,
    #[serde(rename = "methodId")]
    pub method_id: ElementId,
    pub arguments: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdArgs {
    pub id: ElementId,
}

#[derive(Deserialize, Debug)]
pub struct IdArgsValue {
    pub id: ElementId,
    pub value: Value,
}

#[derive(Deserialize, Debug)]
pub struct WsCommandMessage {
    pub commands: Vec<Command>,
    #[serde(rename = "messageType")]
    pub message_type: u16,
}

#[derive(Serialize, Debug)]
pub struct ResponseResult {
    pub status: u64,
    pub value: Value,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub handle: u64,
    pub result: ResponseResult,
}

#[derive(Serialize, Debug)]
pub struct WsCommandResponseMessage {
    pub responses: Vec<Response>,
    #[serde(rename = "messageType")]
    pub message_type: u16,
}

#[derive(Serialize, Debug)]
pub struct WsErrorMessage {
    pub status: u64,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "messageType")]
    pub message_type: u16,
}

#[derive(Deserialize, Debug)]
pub struct WsSubscriptionMessage {
    pub subscriptions: Vec<u64>,
    #[serde(rename = "messageType")]
    pub message_type: u16,
}

#[derive(Serialize, Debug)]
pub struct WsSubscriptionResponseMessage {
    pub subscriptions: Vec<u64>,
    #[serde(rename = "messageType")]
    pub message_type: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NcPropertyChangeType {
    ValueChanged = 0,
    SequenceItemAdded = 1,
    SequenceItemChanged = 2,
    SequenceItemRemoved = 3,
}

#[derive(Serialize, Debug, Clone)]
pub struct PropertyChangedEventData {
    #[serde(rename = "propertyId")]
    pub property_id: ElementId,
    #[serde(rename = "changeType")]
    pub change_type: NcPropertyChangeType,
    pub value: Value,
    #[serde(rename = "sequenceItemIndex")]
    pub sequence_item_index: Option<u64>,
}

#[derive(Serialize, Debug, Clone)]
pub struct PropertyChangedEvent {
    pub oid: u64,
    #[serde(rename = "eventId")]
    pub event_id_: ElementId,
    #[serde(rename = "eventData")]
    pub event_data: PropertyChangedEventData,
}

impl PropertyChangedEvent {
    pub fn new(oid: u64, event_data: PropertyChangedEventData) -> Self {
        PropertyChangedEvent {
            oid,
            event_id_: ElementId { level: 1, index: 1 },
            event_data,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct WsNotificationMessage {
    pub notifications: Vec<PropertyChangedEvent>,
    #[serde(rename = "messageType")]
    pub message_type: u16,
}

#[derive(Serialize, Debug)]
pub struct NcBlockMemberDescriptor {
    pub role: String,
    pub oid: u64,
    #[serde(rename = "constantOid")]
    pub constant_oid: bool,
    #[serde(rename = "classId")]
    pub class_id: Vec<u32>,
    #[serde(rename = "userLabel")]
    pub user_label: String,
    pub owner: u64,
}
