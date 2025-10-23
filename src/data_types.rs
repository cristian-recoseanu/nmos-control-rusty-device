use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub const MESSAGE_TYPE_COMMAND: u16 = 0;
pub const MESSAGE_TYPE_COMMAND_RESPONSE: u16 = 1;
pub const MESSAGE_TYPE_NOTIFICATION: u16 = 2;
pub const MESSAGE_TYPE_SUBSCRIPTION: u16 = 3;
pub const MESSAGE_TYPE_SUBSCRIPTION_RESPONSE: u16 = 4;
pub const MESSAGE_TYPE_ERROR: u16 = 5;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(into = "u16", try_from = "u16")]
#[repr(u16)]
pub enum NcMethodStatus {
    Ok = 200,
    PropertyDeprecated = 298,
    MethodDeprecated = 299,
    BadCommandFormat = 400,
    Unauthorized = 401,
    BadOid = 404,
    Readonly = 405,
    InvalidRequest = 406,
    Conflict = 409,
    BufferOverflow = 413,
    IndexOutOfBounds = 414,
    ParameterError = 417,
    Locked = 423,
    DeviceError = 500,
    MethodNotImplemented = 501,
    PropertyNotImplemented = 502,
    NotReady = 503,
    Timeout = 504,
}

impl From<NcMethodStatus> for u16 {
    fn from(status: NcMethodStatus) -> Self {
        status as u16
    }
}

impl From<u16> for NcMethodStatus {
    fn from(value: u16) -> Self {
        match value {
            200 => NcMethodStatus::Ok,
            298 => NcMethodStatus::PropertyDeprecated,
            299 => NcMethodStatus::MethodDeprecated,
            400 => NcMethodStatus::BadCommandFormat,
            401 => NcMethodStatus::Unauthorized,
            404 => NcMethodStatus::BadOid,
            405 => NcMethodStatus::Readonly,
            406 => NcMethodStatus::InvalidRequest,
            409 => NcMethodStatus::Conflict,
            413 => NcMethodStatus::BufferOverflow,
            414 => NcMethodStatus::IndexOutOfBounds,
            417 => NcMethodStatus::ParameterError,
            423 => NcMethodStatus::Locked,
            500 => NcMethodStatus::DeviceError,
            501 => NcMethodStatus::MethodNotImplemented,
            502 => NcMethodStatus::PropertyNotImplemented,
            503 => NcMethodStatus::NotReady,
            504 => NcMethodStatus::Timeout,
            _ => NcMethodStatus::DeviceError,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct NmosResource {
    pub id: String,
    pub label: String,
    pub description: String,
    pub version: String,
    pub tags: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Clone)]
pub struct DeviceControl {
    #[serde(rename = "type")]
    pub type_: String,
    pub href: String,
    pub authorization: bool,
}

#[derive(Serialize, Clone)]
pub struct NmosDevice {
    #[serde(flatten)]
    pub base: NmosResource,
    pub senders: Vec<String>,
    pub receivers: Vec<String>,
    pub node_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub controls: Vec<DeviceControl>,
}

#[allow(clippy::too_many_arguments)]
impl NmosDevice {
    pub fn new(
        id: String,
        label: String,
        description: String,
        version: String,
        tags: HashMap<String, Vec<String>>,
        senders: Vec<String>,
        receivers: Vec<String>,
        node_id: String,
        type_: String,
        controls: Vec<DeviceControl>,
    ) -> Self {
        NmosDevice {
            base: NmosResource {
                id,
                label,
                description,
                version,
                tags,
            },
            senders,
            receivers,
            node_id,
            type_,
            controls,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct NmosClock {
    pub name: String,
    pub ref_type: String,
}

#[derive(Serialize, Clone)]
pub struct NmosInterface {
    pub chassis_id: String,
    pub name: String,
    pub port_id: String,
}

#[derive(Serialize, Clone)]
pub struct NmosEndpoint {
    pub host: String,
    pub port: u32,
    pub protocol: String,
}

#[derive(Serialize, Clone)]
pub struct NmosApi {
    pub endpoints: Vec<NmosEndpoint>,
    pub versions: Vec<String>,
}

#[derive(Serialize, Clone)]
pub struct NmosNode {
    #[serde(flatten)]
    pub base: NmosResource,
    pub href: String,
    pub hostname: String,
    pub caps: Map<String, Value>,
    pub services: Vec<Map<String, Value>>,
    pub clocks: Vec<NmosClock>,
    pub interfaces: Vec<NmosInterface>,
    pub api: NmosApi,
}

#[allow(clippy::too_many_arguments)]
impl NmosNode {
    pub fn new(
        id: String,
        label: String,
        description: String,
        version: String,
        tags: HashMap<String, Vec<String>>,
        href: String,
        hostname: String,
        clocks: Vec<NmosClock>,
        interfaces: Vec<NmosInterface>,
        api: NmosApi,
    ) -> Self {
        NmosNode {
            base: NmosResource {
                id,
                label,
                description,
                version,
                tags,
            },
            href,
            hostname,
            caps: Map::new(),
            services: Vec::new(),
            clocks,
            interfaces,
            api,
        }
    }
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
pub struct ResponseBase {
    pub status: NcMethodStatus,
}

#[derive(Serialize, Debug)]
pub struct ResponseResult {
    #[serde(flatten)]
    pub base: ResponseBase,
    pub value: Value,
}

#[derive(Serialize, Debug)]
pub struct ResponseError {
    #[serde(flatten)]
    pub base: ResponseBase,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ResponsePayload {
    Result(ResponseResult),
    Error(ResponseError),
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub handle: u64,
    pub result: ResponsePayload,
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
#[serde(into = "u32", try_from = "u32")]
#[repr(u32)]
pub enum NcPropertyChangeType {
    ValueChanged = 0,
    SequenceItemAdded = 1,
    SequenceItemChanged = 2,
    SequenceItemRemoved = 3,
}

impl From<NcPropertyChangeType> for u32 {
    fn from(change_type: NcPropertyChangeType) -> Self {
        change_type as u32
    }
}

impl From<u32> for NcPropertyChangeType {
    fn from(value: u32) -> Self {
        match value {
            0 => NcPropertyChangeType::ValueChanged,
            1 => NcPropertyChangeType::SequenceItemAdded,
            2 => NcPropertyChangeType::SequenceItemChanged,
            3 => NcPropertyChangeType::SequenceItemRemoved,
            _ => NcPropertyChangeType::ValueChanged,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointResourceNmos {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointResourceNmosChannelMapping {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    #[serde(rename = "ioId")]
    pub io_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointBase {
    #[serde(rename = "contextNamespace")]
    pub context_namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointNmos {
    #[serde(flatten)]
    pub base: NcTouchpointBase,
    pub resource: NcTouchpointResourceNmos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointNmosChannelMapping {
    #[serde(flatten)]
    pub base: NcTouchpointBase,
    pub resource: NcTouchpointResourceNmosChannelMapping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NcTouchpoint {
    Nmos(NcTouchpointNmos),
    NmosChannelMapping(NcTouchpointNmosChannelMapping),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcPropertyConstraintsBase {
    #[serde(rename = "propertyId")]
    pub property_id: ElementId,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcPropertyConstraintsNumber {
    #[serde(flatten)]
    pub base: NcPropertyConstraintsBase,
    pub maximum: Option<f64>,
    pub minimum: Option<f64>,
    pub step: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcPropertyConstraintsString {
    #[serde(flatten)]
    pub base: NcPropertyConstraintsBase,
    #[serde(rename = "maxCharacters")]
    pub max_characters: Option<u32>,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NcPropertyConstraints {
    Number(NcPropertyConstraintsNumber),
    String(NcPropertyConstraintsString),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcManufacturer {
    pub name: String,
    #[serde(rename = "organizationId")]
    pub organization_id: Option<i32>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcProduct {
    pub name: String,
    pub key: String,
    #[serde(rename = "revisionLevel")]
    pub revision_level: String,
    #[serde(rename = "brandName")]
    pub brand_name: Option<String>,
    pub uuid: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "u32", try_from = "u32")]
#[repr(u32)]
pub enum NcDeviceGenericState {
    Unknown = 0,
    NormalOperation = 1,
    Initializing = 2,
    Updating = 3,
    LicensingError = 4,
    InternalError = 5,
}

impl From<NcDeviceGenericState> for u32 {
    fn from(state: NcDeviceGenericState) -> Self {
        state as u32
    }
}

impl From<u32> for NcDeviceGenericState {
    fn from(value: u32) -> Self {
        match value {
            0 => NcDeviceGenericState::Unknown,
            1 => NcDeviceGenericState::NormalOperation,
            2 => NcDeviceGenericState::Initializing,
            3 => NcDeviceGenericState::Updating,
            4 => NcDeviceGenericState::LicensingError,
            5 => NcDeviceGenericState::InternalError,
            _ => NcDeviceGenericState::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcDeviceOperationalState {
    pub generic: NcDeviceGenericState,
    #[serde(rename = "deviceSpecificDetails")]
    pub device_specific_details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "u32", try_from = "u32")]
#[repr(u32)]
pub enum NcResetCause {
    Unknown = 0,
    PowerOn = 1,
    InternalError = 2,
    Upgrade = 3,
    ControllerRequest = 4,
    ManualReset = 5,
}

impl From<NcResetCause> for u32 {
    fn from(cause: NcResetCause) -> Self {
        cause as u32
    }
}

impl From<u32> for NcResetCause {
    fn from(value: u32) -> Self {
        match value {
            0 => NcResetCause::Unknown,
            1 => NcResetCause::PowerOn,
            2 => NcResetCause::InternalError,
            3 => NcResetCause::Upgrade,
            4 => NcResetCause::ControllerRequest,
            5 => NcResetCause::ManualReset,
            _ => NcResetCause::Unknown,
        }
    }
}
