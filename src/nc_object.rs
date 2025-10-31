use crate::data_types::{
    IdArgs, IdArgsValue, NcClassDescriptor, NcElementId, NcEventDescriptor, NcMethodDescriptor,
    NcMethodStatus, NcParameterDescriptor, NcPropertyChangeType, NcPropertyConstraints,
    NcPropertyDescriptor, NcTouchpoint, PropertyChangedEvent, PropertyChangedEventData,
};
use serde_json::Value;
use serde_json::json;
use std::any::Any;
use tokio::sync::mpsc;

// Define a trait that all member types will implement
pub trait NcMember: Send {
    // Type identification
    fn member_type(&self) -> &'static str;
    // Common accessors
    fn get_role(&self) -> &str;
    fn get_oid(&self) -> u64;
    fn get_constant_oid(&self) -> bool;
    fn get_class_id(&self) -> &[u32];
    fn get_user_label(&self) -> Option<&str>;
    // For downcasting when you need the concrete type
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_property(&self, oid: u64, id_args: &IdArgs) -> (Option<String>, Value, NcMethodStatus);
    fn set_property(
        &mut self,
        _oid: u64,
        id_args_value: IdArgsValue,
    ) -> (Option<String>, NcMethodStatus);
    fn invoke_method(
        &self,
        _oid: u64,
        _method_id: NcElementId,
        _args: Value,
    ) -> (Option<String>, Option<Value>, NcMethodStatus);
}

#[derive(Debug, Clone)]
pub struct NcObject {
    pub class_id: Vec<u32>,
    pub oid: u64,
    pub constant_oid: bool,
    pub owner: Option<u64>,
    pub role: String,
    pub user_label: Option<String>,
    pub touchpoints: Option<Vec<NcTouchpoint>>,
    pub runtime_property_constraints: Option<Vec<NcPropertyConstraints>>,
    pub notifier: mpsc::UnboundedSender<PropertyChangedEvent>,
}

impl NcMember for NcObject {
    fn member_type(&self) -> &'static str {
        "NcObject"
    }
    fn get_role(&self) -> &str {
        &self.role
    }
    fn get_oid(&self) -> u64 {
        self.oid
    }
    fn get_constant_oid(&self) -> bool {
        self.constant_oid
    }
    fn get_class_id(&self) -> &[u32] {
        &self.class_id
    }
    fn get_user_label(&self) -> Option<&str> {
        self.user_label.as_deref()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn get_property(&self, _oid: u64, id_args: &IdArgs) -> (Option<String>, Value, NcMethodStatus) {
        match (id_args.id.level, id_args.id.index) {
            (1, 1) => (None, json!(self.class_id), NcMethodStatus::Ok),
            (1, 2) => (None, json!(self.oid), NcMethodStatus::Ok),
            (1, 3) => (None, json!(self.constant_oid), NcMethodStatus::Ok),
            (1, 4) => (None, json!(self.owner), NcMethodStatus::Ok),
            (1, 5) => (None, json!(self.role), NcMethodStatus::Ok),
            (1, 6) => (None, json!(self.user_label), NcMethodStatus::Ok),
            (1, 7) => (None, json!(self.touchpoints), NcMethodStatus::Ok),
            (1, 8) => (
                None,
                json!(self.runtime_property_constraints),
                NcMethodStatus::Ok,
            ),
            _ => (
                Some("Could not find the property".to_string()),
                json!(null),
                NcMethodStatus::PropertyNotImplemented,
            ),
        }
    }
    fn set_property(
        &mut self,
        _oid: u64,
        id_args_value: IdArgsValue,
    ) -> (Option<String>, NcMethodStatus) {
        if id_args_value.id.level == 1 {
            match id_args_value.id.index {
                6 => {
                    // Set userLabel (accepts string or null)
                    match &id_args_value.value {
                        serde_json::Value::String(s) => {
                            self.user_label = Some(s.clone());
                            let _ = self.notifier.send(PropertyChangedEvent::new(
                                self.oid,
                                PropertyChangedEventData {
                                    property_id: id_args_value.id,
                                    change_type: NcPropertyChangeType::ValueChanged,
                                    value: serde_json::json!(s),
                                    sequence_item_index: None,
                                },
                            ));
                            (None, NcMethodStatus::Ok)
                        }
                        serde_json::Value::Null => {
                            self.user_label = None;
                            let _ = self.notifier.send(PropertyChangedEvent::new(
                                self.oid,
                                PropertyChangedEventData {
                                    property_id: id_args_value.id,
                                    change_type: NcPropertyChangeType::ValueChanged,
                                    value: serde_json::json!(null),
                                    sequence_item_index: None,
                                },
                            ));
                            (None, NcMethodStatus::Ok)
                        }
                        _ => (
                            Some("Property value was invalid".to_string()),
                            NcMethodStatus::ParameterError,
                        ),
                    }
                }
                1 | 2 | 3 | 4 | 5 | 7 | 8 => (
                    Some("Property is readonly".to_string()),
                    NcMethodStatus::Readonly,
                ),
                _ => (
                    Some("Could not find the property".to_string()),
                    NcMethodStatus::PropertyNotImplemented,
                ),
            }
        } else {
            (
                Some("Could not find the property".to_string()),
                NcMethodStatus::PropertyNotImplemented,
            )
        }
    }
    fn invoke_method(
        &self,
        _oid: u64,
        _method_id: NcElementId,
        _args: Value,
    ) -> (Option<String>, Option<Value>, NcMethodStatus) {
        //TODO: This is where we can add treatment for other methods in NcObject
        (
            Some("Method not yet implemented".to_string()),
            None,
            NcMethodStatus::MethodNotImplemented,
        )
    }
}

#[allow(clippy::too_many_arguments)]
impl NcObject {
    pub fn new(
        class_id: Vec<u32>,
        oid: u64,
        constant_oid: bool,
        owner: Option<u64>,
        role: &str,
        user_label: Option<&str>,
        touchpoints: Option<Vec<NcTouchpoint>>,
        runtime_property_constraints: Option<Vec<NcPropertyConstraints>>,
        notifier: mpsc::UnboundedSender<PropertyChangedEvent>,
    ) -> Self {
        NcObject {
            class_id,
            constant_oid,
            owner,
            oid,
            role: role.to_string(),
            user_label: user_label.map(|s| s.to_string()),
            touchpoints,
            runtime_property_constraints,
            notifier,
        }
    }

    pub fn get_class_descriptor(_include_inherited: bool) -> NcClassDescriptor {
        let properties = vec![
            NcPropertyDescriptor {
                base: crate::data_types::NcDescriptor { description: Some("Static value. All instances of the same class will have the same identity value".to_string()) },
                id: NcElementId { level: 1, index: 1 },
                name: "classId".to_string(),
                type_name: Some("NcClassId".to_string()),
                is_read_only: true,
                is_nullable: false,
                is_sequence: false,
                is_deprecated: false,
                constraints: None,
            },
            NcPropertyDescriptor {
                base: crate::data_types::NcDescriptor { description: Some("Object identifier".to_string()) },
                id: NcElementId { level: 1, index: 2 },
                name: "oid".to_string(),
                type_name: Some("NcOid".to_string()),
                is_read_only: true,
                is_nullable: false,
                is_sequence: false,
                is_deprecated: false,
                constraints: None,
            },
            NcPropertyDescriptor {
                base: crate::data_types::NcDescriptor { description: Some("TRUE iff OID is hardwired into device".to_string()) },
                id: NcElementId { level: 1, index: 3 },
                name: "constantOid".to_string(),
                type_name: Some("NcBoolean".to_string()),
                is_read_only: true,
                is_nullable: false,
                is_sequence: false,
                is_deprecated: false,
                constraints: None,
            },
            NcPropertyDescriptor {
                base: crate::data_types::NcDescriptor { description: Some("OID of containing block. Can only ever be null for the root block".to_string()) },
                id: NcElementId { level: 1, index: 4 },
                name: "owner".to_string(),
                type_name: Some("NcOid".to_string()),
                is_read_only: true,
                is_nullable: true,
                is_sequence: false,
                is_deprecated: false,
                constraints: None,
            },
            NcPropertyDescriptor {
                base: crate::data_types::NcDescriptor { description: Some("role of obj in containing block".to_string()) },
                id: NcElementId { level: 1, index: 5 },
                name: "role".to_string(),
                type_name: Some("NcString".to_string()),
                is_read_only: true,
                is_nullable: false,
                is_sequence: false,
                is_deprecated: false,
                constraints: None,
            },
            NcPropertyDescriptor {
                base: crate::data_types::NcDescriptor { description: Some("Scribble strip".to_string()) },
                id: NcElementId { level: 1, index: 6 },
                name: "userLabel".to_string(),
                type_name: Some("NcString".to_string()),
                is_read_only: false,
                is_nullable: true,
                is_sequence: false,
                is_deprecated: false,
                constraints: None,
            },
            NcPropertyDescriptor {
                base: crate::data_types::NcDescriptor { description: Some("Touchpoints to other contexts".to_string()) },
                id: NcElementId { level: 1, index: 7 },
                name: "touchpoints".to_string(),
                type_name: Some("NcTouchpoint".to_string()),
                is_read_only: true,
                is_nullable: true,
                is_sequence: true,
                is_deprecated: false,
                constraints: None,
            },
            NcPropertyDescriptor {
                base: crate::data_types::NcDescriptor { description: Some("Runtime property constraints".to_string()) },
                id: NcElementId { level: 1, index: 8 },
                name: "runtimePropertyConstraints".to_string(),
                type_name: Some("NcPropertyConstraints".to_string()),
                is_read_only: true,
                is_nullable: true,
                is_sequence: true,
                is_deprecated: false,
                constraints: None,
            },
        ];

        let methods = vec![
            NcMethodDescriptor {
                base: crate::data_types::NcDescriptor {
                    description: Some("Get property value".to_string()),
                },
                id: NcElementId { level: 1, index: 1 },
                name: "Get".to_string(),
                result_datatype: "NcMethodResultPropertyValue".to_string(),
                parameters: vec![NcParameterDescriptor {
                    base: crate::data_types::NcDescriptor {
                        description: Some("Property id".to_string()),
                    },
                    name: "id".to_string(),
                    type_name: Some("NcPropertyId".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                }],
                is_deprecated: false,
            },
            NcMethodDescriptor {
                base: crate::data_types::NcDescriptor {
                    description: Some("Set property value".to_string()),
                },
                id: NcElementId { level: 1, index: 2 },
                name: "Set".to_string(),
                result_datatype: "NcMethodResult".to_string(),
                parameters: vec![
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Property id".to_string()),
                        },
                        name: "id".to_string(),
                        type_name: Some("NcPropertyId".to_string()),
                        is_nullable: false,
                        is_sequence: false,
                        constraints: None,
                    },
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Property value".to_string()),
                        },
                        name: "value".to_string(),
                        type_name: None,
                        is_nullable: true,
                        is_sequence: false,
                        constraints: None,
                    },
                ],
                is_deprecated: false,
            },
            NcMethodDescriptor {
                base: crate::data_types::NcDescriptor {
                    description: Some("Get sequence item".to_string()),
                },
                id: NcElementId { level: 1, index: 3 },
                name: "GetSequenceItem".to_string(),
                result_datatype: "NcMethodResultPropertyValue".to_string(),
                parameters: vec![
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Property id".to_string()),
                        },
                        name: "id".to_string(),
                        type_name: Some("NcPropertyId".to_string()),
                        is_nullable: false,
                        is_sequence: false,
                        constraints: None,
                    },
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Index of item in the sequence".to_string()),
                        },
                        name: "index".to_string(),
                        type_name: Some("NcId".to_string()),
                        is_nullable: false,
                        is_sequence: false,
                        constraints: None,
                    },
                ],
                is_deprecated: false,
            },
            NcMethodDescriptor {
                base: crate::data_types::NcDescriptor {
                    description: Some("Set sequence item value".to_string()),
                },
                id: NcElementId { level: 1, index: 4 },
                name: "SetSequenceItem".to_string(),
                result_datatype: "NcMethodResult".to_string(),
                parameters: vec![
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Property id".to_string()),
                        },
                        name: "id".to_string(),
                        type_name: Some("NcPropertyId".to_string()),
                        is_nullable: false,
                        is_sequence: false,
                        constraints: None,
                    },
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Index of item in the sequence".to_string()),
                        },
                        name: "index".to_string(),
                        type_name: Some("NcId".to_string()),
                        is_nullable: false,
                        is_sequence: false,
                        constraints: None,
                    },
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Value".to_string()),
                        },
                        name: "value".to_string(),
                        type_name: None,
                        is_nullable: true,
                        is_sequence: false,
                        constraints: None,
                    },
                ],
                is_deprecated: false,
            },
            NcMethodDescriptor {
                base: crate::data_types::NcDescriptor {
                    description: Some("Add item to sequence".to_string()),
                },
                id: NcElementId { level: 1, index: 5 },
                name: "AddSequenceItem".to_string(),
                result_datatype: "NcMethodResultId".to_string(),
                parameters: vec![
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Property id".to_string()),
                        },
                        name: "id".to_string(),
                        type_name: Some("NcPropertyId".to_string()),
                        is_nullable: false,
                        is_sequence: false,
                        constraints: None,
                    },
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Value".to_string()),
                        },
                        name: "value".to_string(),
                        type_name: None,
                        is_nullable: true,
                        is_sequence: false,
                        constraints: None,
                    },
                ],
                is_deprecated: false,
            },
            NcMethodDescriptor {
                base: crate::data_types::NcDescriptor {
                    description: Some("Delete sequence item".to_string()),
                },
                id: NcElementId { level: 1, index: 6 },
                name: "RemoveSequenceItem".to_string(),
                result_datatype: "NcMethodResult".to_string(),
                parameters: vec![
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Property id".to_string()),
                        },
                        name: "id".to_string(),
                        type_name: Some("NcPropertyId".to_string()),
                        is_nullable: false,
                        is_sequence: false,
                        constraints: None,
                    },
                    NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some("Index of item in the sequence".to_string()),
                        },
                        name: "index".to_string(),
                        type_name: Some("NcId".to_string()),
                        is_nullable: false,
                        is_sequence: false,
                        constraints: None,
                    },
                ],
                is_deprecated: false,
            },
            NcMethodDescriptor {
                base: crate::data_types::NcDescriptor {
                    description: Some("Get sequence length".to_string()),
                },
                id: NcElementId { level: 1, index: 7 },
                name: "GetSequenceLength".to_string(),
                result_datatype: "NcMethodResultLength".to_string(),
                parameters: vec![NcParameterDescriptor {
                    base: crate::data_types::NcDescriptor {
                        description: Some("Property id".to_string()),
                    },
                    name: "id".to_string(),
                    type_name: Some("NcPropertyId".to_string()),
                    is_nullable: false,
                    is_sequence: false,
                    constraints: None,
                }],
                is_deprecated: false,
            },
        ];

        let events = vec![NcEventDescriptor {
            base: crate::data_types::NcDescriptor {
                description: Some("Property changed event".to_string()),
            },
            id: NcElementId { level: 1, index: 1 },
            name: "PropertyChanged".to_string(),
            event_datatype: "NcPropertyChangedEventData".to_string(),
            is_deprecated: false,
        }];

        NcClassDescriptor {
            base: crate::data_types::NcDescriptor {
                description: Some("NcObject class descriptor".to_string()),
            },
            class_id: vec![1],
            name: "NcObject".to_string(),
            fixed_role: None,
            properties,
            methods,
            events,
        }
    }
}
