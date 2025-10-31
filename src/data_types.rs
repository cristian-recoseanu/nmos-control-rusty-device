use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub const MESSAGE_TYPE_COMMAND: u16 = 0;
pub const MESSAGE_TYPE_COMMAND_RESPONSE: u16 = 1;
pub const MESSAGE_TYPE_NOTIFICATION: u16 = 2;
pub const MESSAGE_TYPE_SUBSCRIPTION: u16 = 3;
pub const MESSAGE_TYPE_SUBSCRIPTION_RESPONSE: u16 = 4;
pub const MESSAGE_TYPE_ERROR: u16 = 5;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointResourceBase {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

impl NcTouchpointResourceBase {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Touchpoint resource".to_string()),
                },
                name: "NcTouchpointResource".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "resourceType".to_string(),
                type_name: Some("NcString".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointResourceNmos {
    #[serde(flatten)]
    pub base: NcTouchpointResourceBase,
    pub id: String,
}

impl NcTouchpointResourceNmos {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Touchpoint NMOS resource".to_string()),
                },
                name: "NcTouchpointResourceNmos".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "id".to_string(),
                type_name: Some("NcUuid".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: Some("NcTouchpointResource".to_string()),
        };
        if include_inherited {
            let base = NcTouchpointResourceBase::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointResourceNmosChannelMapping {
    #[serde(flatten)]
    pub base: NcTouchpointResourceNmos,
    #[serde(rename = "ioId")]
    pub io_id: String,
}

impl NcTouchpointResourceNmosChannelMapping {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Touchpoint NMOS channel mapping resource".to_string()),
                },
                name: "NcTouchpointResourceNmosChannelMapping".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "ioId".to_string(),
                type_name: Some("NcString".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: Some("NcTouchpointResourceNmos".to_string()),
        };
        if include_inherited {
            let base = NcTouchpointResourceNmos::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NcTouchpoint {
    Nmos(NcTouchpointNmos),
    NmosChannelMapping(NcTouchpointNmosChannelMapping),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointBase {
    #[serde(rename = "contextNamespace")]
    pub context_namespace: String,
}

impl NcTouchpointBase {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Touchpoint".to_string()),
                },
                name: "NcTouchpoint".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "contextNamespace".to_string(),
                type_name: Some("NcString".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointNmos {
    #[serde(flatten)]
    pub base: NcTouchpointBase,
    pub resource: NcTouchpointResourceNmos,
}

impl NcTouchpointNmos {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Touchpoint NMOS".to_string()),
                },
                name: "NcTouchpointNmos".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "resource".to_string(),
                type_name: Some("NcTouchpointResourceNmos".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: Some("NcTouchpoint".to_string()),
        };
        if include_inherited {
            let base = NcTouchpointBase::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcTouchpointNmosChannelMapping {
    #[serde(flatten)]
    pub base: NcTouchpointBase,
    pub resource: NcTouchpointResourceNmosChannelMapping,
}

impl NcTouchpointNmosChannelMapping {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Touchpoint NMOS channel mapping".to_string()),
                },
                name: "NcTouchpointNmosChannelMapping".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "resource".to_string(),
                type_name: Some("NcTouchpointResourceNmosChannelMapping".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: Some("NcTouchpoint".to_string()),
        };
        if include_inherited {
            let base = NcTouchpointBase::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcDescriptor {
    pub description: Option<String>,
}

impl NcDescriptor {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                name: "NcDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
                base: NcDescriptor {
                    description: Some("Base descriptor".to_string()),
                },
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor {
                    description: Some("Optional user facing description".to_string()),
                },
                name: "description".to_string(),
                type_name: Some("NcString".to_string()),
                is_nullable: true,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct NcBlockMemberDescriptor {
    #[serde(flatten)]
    pub base: NcDescriptor,
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

impl NcBlockMemberDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Block member descriptor".to_string()),
                },
                name: "NcBlockMemberDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "role".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "oid".to_string(),
                    type_name: Some("NcOid".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "constantOid".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "classId".to_string(),
                    type_name: Some("NcClassId".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "userLabel".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "owner".to_string(),
                    type_name: Some("NcOid".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcClassDescriptor {
    #[serde(flatten)]
    pub base: NcDescriptor,
    #[serde(rename = "classId")]
    pub class_id: Vec<u32>,
    pub name: String,
    #[serde(rename = "fixedRole")]
    pub fixed_role: Option<String>,
    pub properties: Vec<NcPropertyDescriptor>,
    pub methods: Vec<NcMethodDescriptor>,
    pub events: Vec<NcEventDescriptor>,
}

impl NcClassDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Descriptor of a class".to_string()),
                },
                name: "NcClassDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("Numeric class identity".to_string()),
                    },
                    name: "classId".to_string(),
                    type_name: Some("NcClassId".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("Class name".to_string()),
                    },
                    name: "name".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("Fixed role for manager classes".to_string()),
                    },
                    name: "fixedRole".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("Property descriptors".to_string()),
                    },
                    name: "properties".to_string(),
                    type_name: Some("NcPropertyDescriptor".to_string()),
                    is_nullable: false,
                    is_sequence: true,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("Method descriptors".to_string()),
                    },
                    name: "methods".to_string(),
                    type_name: Some("NcMethodDescriptor".to_string()),
                    is_nullable: false,
                    is_sequence: true,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("Event descriptors".to_string()),
                    },
                    name: "events".to_string(),
                    type_name: Some("NcEventDescriptor".to_string()),
                    is_nullable: false,
                    is_sequence: true,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(into = "u32", try_from = "u32")]
#[repr(u32)]
pub enum NcDatatypeType {
    Primitive = 0,
    Typedef = 1,
    Struct = 2,
    Enum = 3,
}

impl From<NcDatatypeType> for u32 {
    fn from(v: NcDatatypeType) -> Self {
        v as u32
    }
}

impl From<u32> for NcDatatypeType {
    fn from(value: u32) -> Self {
        match value {
            0 => NcDatatypeType::Primitive,
            1 => NcDatatypeType::Typedef,
            2 => NcDatatypeType::Struct,
            3 => NcDatatypeType::Enum,
            _ => NcDatatypeType::Primitive,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcDatatypeDescriptor {
    #[serde(flatten)]
    pub base: NcDescriptor,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: NcDatatypeType,
    pub constraints: Option<NcParameterConstraintsUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NcAnyDatatypeDescriptor {
    Primitive(NcDatatypeDescriptor),
    TypeDef(NcDatatypeDescriptorTypeDef),
    Struct(NcDatatypeDescriptorStruct),
    Enum(NcDatatypeDescriptorEnum),
}

impl NcDatatypeDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Datatype descriptor base".to_string()),
                },
                name: "NcDatatypeDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "name".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "type".to_string(),
                    type_name: Some("NcDatatypeType".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "constraints".to_string(),
                    type_name: Some("NcParameterConstraints".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcDatatypeDescriptorPrimitive {
    #[serde(flatten)]
    pub base: NcDatatypeDescriptor,
}

impl NcDatatypeDescriptorPrimitive {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Primitive datatype descriptor".to_string()),
                },
                name: "NcDatatypeDescriptorPrimitive".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![],
            parent_type: Some("NcDatatypeDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDatatypeDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcDatatypeDescriptorTypeDef {
    #[serde(flatten)]
    pub base: NcDatatypeDescriptor,
    #[serde(rename = "parentType")]
    pub parent_type: String,
    #[serde(rename = "isSequence")]
    pub is_sequence: bool,
}

impl NcDatatypeDescriptorTypeDef {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Typedef datatype descriptor".to_string()),
                },
                name: "NcDatatypeDescriptorTypeDef".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "parentType".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "isSequence".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDatatypeDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDatatypeDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcDatatypeDescriptorStruct {
    #[serde(flatten)]
    pub base: NcDatatypeDescriptor,
    pub fields: Vec<NcFieldDescriptor>,
    #[serde(rename = "parentType")]
    pub parent_type: Option<String>,
}

impl NcDatatypeDescriptorStruct {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Struct datatype descriptor".to_string()),
                },
                name: "NcDatatypeDescriptorStruct".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "fields".to_string(),
                    type_name: Some("NcFieldDescriptor".to_string()),
                    is_nullable: false,
                    is_sequence: true,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "parentType".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDatatypeDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDatatypeDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcDatatypeDescriptorEnum {
    #[serde(flatten)]
    pub base: NcDatatypeDescriptor,
    pub items: Vec<NcEnumItemDescriptor>,
}

impl NcDatatypeDescriptorEnum {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Enum datatype descriptor".to_string()),
                },
                name: "NcDatatypeDescriptorEnum".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "items".to_string(),
                type_name: Some("NcEnumItemDescriptor".to_string()),
                is_nullable: false,
                is_sequence: true,
                constraints: None,
            }],
            parent_type: Some("NcDatatypeDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDatatypeDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcFieldDescriptor {
    #[serde(flatten)]
    pub base: NcDescriptor,
    pub name: String,
    #[serde(rename = "typeName")]
    pub type_name: Option<String>,
    #[serde(rename = "isNullable")]
    pub is_nullable: bool,
    #[serde(rename = "isSequence")]
    pub is_sequence: bool,
    pub constraints: Option<NcParameterConstraintsUnion>,
}

impl NcFieldDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Descriptor of a field of a struct".to_string()),
                },
                name: "NcFieldDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("Name of field".to_string()),
                    },
                    name: "name".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some(
                            "Name of field's datatype. Can only ever be null if the type is any"
                                .to_string(),
                        ),
                    },
                    name: "typeName".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("TRUE iff field is nullable".to_string()),
                    },
                    name: "isNullable".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("TRUE iff field is a sequence".to_string()),
                    },
                    name: "isSequence".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some(
                            "Optional constraints on top of the underlying data type".to_string(),
                        ),
                    },
                    name: "constraints".to_string(),
                    type_name: Some("NcParameterConstraints".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcEnumItemDescriptor {
    #[serde(flatten)]
    pub base: NcDescriptor,
    pub name: String,
    pub value: u16,
}

impl NcEnumItemDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Descriptor of an enum item".to_string()),
                },
                name: "NcEnumItemDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("Name of option".to_string()),
                    },
                    name: "name".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor {
                        description: Some("Enum item numerical value".to_string()),
                    },
                    name: "value".to_string(),
                    type_name: Some("NcUint16".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcEventDescriptor {
    #[serde(flatten)]
    pub base: NcDescriptor,
    pub id: NcElementId,
    pub name: String,
    #[serde(rename = "eventDatatype")]
    pub event_datatype: String,
    #[serde(rename = "isDeprecated")]
    pub is_deprecated: bool,
}

impl NcEventDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Descriptor of an event".to_string()),
                },
                name: "NcEventDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "id".to_string(),
                    type_name: Some("NcEventId".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "name".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "eventDatatype".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "isDeprecated".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcMethodDescriptor {
    #[serde(flatten)]
    pub base: NcDescriptor,
    pub id: NcElementId,
    pub name: String,
    #[serde(rename = "resultDatatype")]
    pub result_datatype: String,
    pub parameters: Vec<NcParameterDescriptor>,
    #[serde(rename = "isDeprecated")]
    pub is_deprecated: bool,
}

impl NcMethodDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Descriptor of a method".to_string()),
                },
                name: "NcMethodDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "id".to_string(),
                    type_name: Some("NcMethodId".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "name".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "resultDatatype".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "parameters".to_string(),
                    type_name: Some("NcParameterDescriptor".to_string()),
                    is_nullable: false,
                    is_sequence: true,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "isDeprecated".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcParameterDescriptor {
    #[serde(flatten)]
    pub base: NcDescriptor,
    pub name: String,
    #[serde(rename = "typeName")]
    pub type_name: Option<String>,
    #[serde(rename = "isNullable")]
    pub is_nullable: bool,
    #[serde(rename = "isSequence")]
    pub is_sequence: bool,
    pub constraints: Option<NcParameterConstraintsUnion>,
}

impl NcParameterDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Descriptor of a method parameter".to_string()),
                },
                name: "NcParameterDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "name".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "typeName".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "isNullable".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "isSequence".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "constraints".to_string(),
                    type_name: Some("NcParameterConstraints".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcPropertyDescriptor {
    #[serde(flatten)]
    pub base: NcDescriptor,
    pub id: NcElementId,
    pub name: String,
    #[serde(rename = "typeName")]
    pub type_name: Option<String>,
    #[serde(rename = "isReadOnly")]
    pub is_read_only: bool,
    #[serde(rename = "isNullable")]
    pub is_nullable: bool,
    #[serde(rename = "isSequence")]
    pub is_sequence: bool,
    #[serde(rename = "isDeprecated")]
    pub is_deprecated: bool,
    pub constraints: Option<NcParameterConstraintsUnion>,
}

impl NcPropertyDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Descriptor of a property".to_string()),
                },
                name: "NcPropertyDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "id".to_string(),
                    type_name: Some("NcPropertyId".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "name".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "typeName".to_string(),
                    type_name: Some("NcName".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "isReadOnly".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "isNullable".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "isSequence".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "isDeprecated".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "constraints".to_string(),
                    type_name: Some("NcParameterConstraints".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcDescriptor".to_string()),
        };
        if include_inherited {
            let base = NcDescriptor::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NcParameterConstraintsUnion {
    Number(NcParameterConstraintsNumber),
    String(NcParameterConstraintsString),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcParameterConstraints {
    #[serde(rename = "defaultValue")]
    pub default_value: Option<Value>,
}

impl NcParameterConstraints {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Base parameter constraints".to_string()),
                },
                name: "NcParameterConstraints".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "defaultValue".to_string(),
                type_name: None,
                is_nullable: true,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcParameterConstraintsNumber {
    #[serde(flatten)]
    pub base: NcParameterConstraints,
    pub maximum: Option<f64>,
    pub minimum: Option<f64>,
    pub step: Option<f64>,
}

impl NcParameterConstraintsNumber {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Numeric parameter constraints".to_string()),
                },
                name: "NcParameterConstraintsNumber".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "maximum".to_string(),
                    type_name: None,
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "minimum".to_string(),
                    type_name: None,
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "step".to_string(),
                    type_name: None,
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcParameterConstraints".to_string()),
        };
        if include_inherited {
            let base = NcParameterConstraints::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcParameterConstraintsString {
    #[serde(flatten)]
    pub base: NcParameterConstraints,
    #[serde(rename = "maxCharacters")]
    pub max_characters: Option<u32>,
    pub pattern: Option<String>,
}

impl NcParameterConstraintsString {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("String parameter constraints".to_string()),
                },
                name: "NcParameterConstraintsString".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "maxCharacters".to_string(),
                    type_name: Some("NcUint32".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "pattern".to_string(),
                    type_name: Some("NcRegex".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcParameterConstraints".to_string()),
        };
        if include_inherited {
            let base = NcParameterConstraints::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NcElementId {
    pub level: u32,
    pub index: u32,
}

impl NcElementId {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Element identifier".to_string()),
                },
                name: "NcElementId".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "level".to_string(),
                    type_name: Some("NcUint16".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "index".to_string(),
                    type_name: Some("NcUint16".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NcPropertyId {
    pub level: u32,
    pub index: u32,
}

impl NcPropertyId {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Property identifier".to_string()),
                },
                name: "NcPropertyId".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![],
            parent_type: Some("NcElementId".to_string()),
        };
        if include_inherited {
            let base = NcElementId::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NcMethodId {
    pub level: u32,
    pub index: u32,
}

impl NcMethodId {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Method identifier".to_string()),
                },
                name: "NcMethodId".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![],
            parent_type: Some("NcElementId".to_string()),
        };
        if include_inherited {
            let base = NcElementId::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NcEventId {
    pub level: u32,
    pub index: u32,
}

impl NcEventId {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Event identifier".to_string()),
                },
                name: "NcEventId".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![],
            parent_type: Some("NcElementId".to_string()),
        };
        if include_inherited {
            let base = NcElementId::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcManufacturer {
    pub name: String,
    #[serde(rename = "organizationId")]
    pub organization_id: Option<i32>,
    pub website: Option<String>,
}

impl NcManufacturer {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Manufacturer descriptor".to_string()),
                },
                name: "NcManufacturer".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "name".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "organizationId".to_string(),
                    type_name: Some("NcOrganizationId".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "website".to_string(),
                    type_name: Some("NcUri".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: None,
        }
    }
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

impl NcProduct {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Product descriptor".to_string()),
                },
                name: "NcProduct".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "name".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "key".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "revisionLevel".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "brandName".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "uuid".to_string(),
                    type_name: Some("NcUuid".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "description".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcDeviceOperationalState {
    pub generic: NcDeviceGenericState,
    #[serde(rename = "deviceSpecificDetails")]
    pub device_specific_details: Option<String>,
}

impl NcDeviceOperationalState {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Device operational state".to_string()),
                },
                name: "NcDeviceOperationalState".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "generic".to_string(),
                    type_name: Some("NcDeviceGenericState".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "deviceSpecificDetails".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: None,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct PropertyChangedEvent {
    pub oid: u64,
    #[serde(rename = "eventId")]
    pub event_id_: NcElementId,
    #[serde(rename = "eventData")]
    pub event_data: PropertyChangedEventData,
}

impl PropertyChangedEvent {
    pub fn new(oid: u64, event_data: PropertyChangedEventData) -> Self {
        PropertyChangedEvent {
            oid,
            event_id_: NcElementId { level: 1, index: 1 },
            event_data,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct PropertyChangedEventData {
    #[serde(rename = "propertyId")]
    pub property_id: NcElementId,
    #[serde(rename = "changeType")]
    pub change_type: NcPropertyChangeType,
    pub value: Value,
    #[serde(rename = "sequenceItemIndex")]
    pub sequence_item_index: Option<u64>,
}

impl PropertyChangedEventData {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Property changed event data".to_string()),
                },
                name: "NcPropertyChangedEventData".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "propertyId".to_string(),
                    type_name: Some("NcPropertyId".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "changeType".to_string(),
                    type_name: Some("NcPropertyChangeType".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "value".to_string(),
                    type_name: None,
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "sequenceItemIndex".to_string(),
                    type_name: Some("NcId".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: None,
        }
    }
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

#[derive(Deserialize, Debug)]
pub struct Command {
    pub handle: u64,
    pub oid: u64,
    #[serde(rename = "methodId")]
    pub method_id: NcElementId,
    pub arguments: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdArgs {
    pub id: NcElementId,
}

#[derive(Deserialize, Debug)]
pub struct IdArgsValue {
    pub id: NcElementId,
    pub value: Value,
}

#[derive(Deserialize, Debug)]
pub struct WsCommandMessage {
    pub commands: Vec<Command>,
    #[serde(rename = "messageType")]
    pub message_type: u16,
}

#[derive(Serialize, Debug)]
pub struct NcMethodResult {
    pub status: NcMethodStatus,
}

impl NcMethodResult {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Method result base".to_string()),
                },
                name: "NcMethodResult".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "status".to_string(),
                type_name: Some("NcMethodStatus".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct NcMethodResultPropertyValue {
    #[serde(flatten)]
    pub base: NcMethodResult,
    pub value: Value,
}

impl NcMethodResultPropertyValue {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Method result with property value".to_string()),
                },
                name: "NcMethodResultPropertyValue".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "value".to_string(),
                type_name: None,
                is_nullable: true,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: Some("NcMethodResult".to_string()),
        };
        if include_inherited {
            let base = NcMethodResult::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Debug)]
pub struct NcMethodResultBlockMemberDescriptors {
    #[serde(flatten)]
    pub base: NcMethodResult,
    pub value: Vec<NcBlockMemberDescriptor>,
}

impl NcMethodResultBlockMemberDescriptors {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some(
                        "Method result containing block member descriptors".to_string(),
                    ),
                },
                name: "NcMethodResultBlockMemberDescriptors".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "value".to_string(),
                type_name: Some("NcBlockMemberDescriptor".to_string()),
                is_nullable: false,
                is_sequence: true,
                constraints: None,
            }],
            parent_type: Some("NcMethodResult".to_string()),
        };
        if include_inherited {
            let base = NcMethodResult::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Debug)]
pub struct NcMethodResultClassDescriptor {
    #[serde(flatten)]
    pub base: NcMethodResult,
    pub value: NcClassDescriptor,
}

impl NcMethodResultClassDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Method result containing a class descriptor".to_string()),
                },
                name: "NcMethodResultClassDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "value".to_string(),
                type_name: Some("NcClassDescriptor".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: Some("NcMethodResult".to_string()),
        };
        if include_inherited {
            let base = NcMethodResult::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Debug)]
pub struct NcMethodResultDatatypeDescriptor {
    #[serde(flatten)]
    pub base: NcMethodResult,
    pub value: NcDatatypeDescriptor,
}

impl NcMethodResultDatatypeDescriptor {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Method result containing a datatype descriptor".to_string()),
                },
                name: "NcMethodResultDatatypeDescriptor".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "value".to_string(),
                type_name: Some("NcDatatypeDescriptor".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: Some("NcMethodResult".to_string()),
        };
        if include_inherited {
            let base = NcMethodResult::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Debug)]
pub struct NcMethodResultId {
    #[serde(flatten)]
    pub base: NcMethodResult,
    #[serde(rename = "value")]
    pub value: u64,
}

impl NcMethodResultId {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Id method result".to_string()),
                },
                name: "NcMethodResultId".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "value".to_string(),
                type_name: Some("NcId".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: Some("NcMethodResult".to_string()),
        };
        if include_inherited {
            let base = NcMethodResult::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Debug)]
pub struct NcMethodResultLength {
    #[serde(flatten)]
    pub base: NcMethodResult,
    #[serde(rename = "value")]
    pub value: Option<u32>,
}

impl NcMethodResultLength {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Length method result".to_string()),
                },
                name: "NcMethodResultLength".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "value".to_string(),
                type_name: Some("NcUint32".to_string()),
                is_nullable: true,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: Some("NcMethodResult".to_string()),
        };
        if include_inherited {
            let base = NcMethodResult::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Debug)]
pub struct NcMethodResultError {
    #[serde(flatten)]
    pub base: NcMethodResult,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

impl NcMethodResultError {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Method result error".to_string()),
                },
                name: "NcMethodResultError".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![NcFieldDescriptor {
                base: NcDescriptor { description: None },
                name: "errorMessage".to_string(),
                type_name: Some("NcString".to_string()),
                is_nullable: false,
                is_sequence: false,
                constraints: None,
            }],
            parent_type: Some("NcMethodResult".to_string()),
        };
        if include_inherited {
            let base = NcMethodResult::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ResponsePayload {
    Result(NcMethodResultPropertyValue),
    Error(NcMethodResultError),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NcPropertyConstraints {
    Number(NcPropertyConstraintsNumber),
    String(NcPropertyConstraintsString),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcPropertyConstraintsBase {
    #[serde(rename = "propertyId")]
    pub property_id: NcElementId,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<Value>,
}

impl NcPropertyConstraintsBase {
    pub fn get_type_descriptor(_include_inherited: bool) -> NcDatatypeDescriptorStruct {
        NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Base property constraints".to_string()),
                },
                name: "NcPropertyConstraints".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "propertyId".to_string(),
                    type_name: Some("NcPropertyId".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "defaultValue".to_string(),
                    type_name: None,
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcPropertyConstraintsNumber {
    #[serde(flatten)]
    pub base: NcPropertyConstraintsBase,
    pub maximum: Option<f64>,
    pub minimum: Option<f64>,
    pub step: Option<f64>,
}

impl NcPropertyConstraintsNumber {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("Numeric property constraints".to_string()),
                },
                name: "NcPropertyConstraintsNumber".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "maximum".to_string(),
                    type_name: None,
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "minimum".to_string(),
                    type_name: None,
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "step".to_string(),
                    type_name: None,
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcPropertyConstraints".to_string()),
        };
        if include_inherited {
            let base = NcPropertyConstraintsBase::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcPropertyConstraintsString {
    #[serde(flatten)]
    pub base: NcPropertyConstraintsBase,
    #[serde(rename = "maxCharacters")]
    pub max_characters: Option<u32>,
    pub pattern: Option<String>,
}

impl NcPropertyConstraintsString {
    pub fn get_type_descriptor(include_inherited: bool) -> NcDatatypeDescriptorStruct {
        let mut current = NcDatatypeDescriptorStruct {
            base: NcDatatypeDescriptor {
                base: NcDescriptor {
                    description: Some("String property constraints".to_string()),
                },
                name: "NcPropertyConstraintsString".to_string(),
                type_: NcDatatypeType::Struct,
                constraints: None,
            },
            fields: vec![
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "maxCharacters".to_string(),
                    type_name: Some("NcUint32".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
                NcFieldDescriptor {
                    base: NcDescriptor { description: None },
                    name: "pattern".to_string(),
                    type_name: Some("NcRegex".to_string()),
                    is_nullable: true,
                    is_sequence: false,
                    constraints: None,
                },
            ],
            parent_type: Some("NcPropertyConstraints".to_string()),
        };
        if include_inherited {
            let base = NcPropertyConstraintsBase::get_type_descriptor(true);
            current.fields.extend(base.fields);
        }
        current
    }
}

#[derive(Serialize, Debug)]
pub struct WsNotificationMessage {
    pub notifications: Vec<PropertyChangedEvent>,
    #[serde(rename = "messageType")]
    pub message_type: u16,
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
