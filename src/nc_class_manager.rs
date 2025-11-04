use crate::data_types::{
    IdArgs, IdArgsValue, NcAnyDatatypeDescriptor, NcClassDescriptor, NcDatatypeDescriptor,
    NcDatatypeDescriptorStruct as DtStruct, NcDatatypeDescriptorTypeDef as DtTypeDef,
    NcDatatypeType, NcElementId, NcMethodStatus, NcParameterDescriptor, NcPropertyConstraints,
    NcPropertyDescriptor, NcTouchpoint, PropertyChangedEvent,
};
use crate::nc_manager::NcManager;
use crate::nc_object::NcMember;
use serde_json::{Value, json};
use std::any::Any;
use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct NcClassManager {
    pub base: NcManager,
    control_classes_register: HashMap<String, NcClassDescriptor>,
    data_types_register: HashMap<String, NcAnyDatatypeDescriptor>,
}

impl NcMember for NcClassManager {
    fn member_type(&self) -> &'static str {
        "NcClassManager"
    }
    fn get_role(&self) -> &str {
        self.base.get_role()
    }
    fn get_oid(&self) -> u64 {
        self.base.get_oid()
    }
    fn get_constant_oid(&self) -> bool {
        self.base.get_constant_oid()
    }
    fn get_class_id(&self) -> &[u32] {
        self.base.get_class_id()
    }
    fn get_user_label(&self) -> Option<&str> {
        self.base.get_user_label()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_property(&self, _oid: u64, id_args: &IdArgs) -> (Option<String>, Value, NcMethodStatus) {
        match (id_args.id.level, id_args.id.index) {
            (3, 1) => {
                let list: Vec<NcClassDescriptor> =
                    self.control_classes_register.values().cloned().collect();
                (None, json!(list), NcMethodStatus::Ok)
            }
            (3, 2) => {
                let list: Vec<NcAnyDatatypeDescriptor> =
                    self.data_types_register.values().cloned().collect();
                (None, json!(list), NcMethodStatus::Ok)
            }
            _ => self.base.get_property(_oid, id_args),
        }
    }

    fn set_property(
        &mut self,
        _oid: u64,
        id_args_value: IdArgsValue,
    ) -> (Option<String>, NcMethodStatus) {
        if id_args_value.id.level == 3 {
            match id_args_value.id.index {
                1 | 2 => (
                    Some("Property is readonly".to_string()),
                    NcMethodStatus::Readonly,
                ),
                _ => (
                    Some("Could not find the property or it is read-only".to_string()),
                    NcMethodStatus::PropertyNotImplemented,
                ),
            }
        } else {
            self.base.set_property(_oid, id_args_value)
        }
    }

    fn invoke_method(
        &self,
        oid: u64,
        method_id: NcElementId,
        args: Value,
    ) -> (Option<String>, Option<Value>, NcMethodStatus) {
        if oid == self.base.get_oid() {
            match (method_id.level, method_id.index) {
                (1, 3) => {
                    match args.as_object() {
                        Some(args_obj) => {
                            let index = match args_obj.get("index").and_then(|v| v.as_u64()) {
                                Some(idx) => idx as usize,
                                None => {
                                    return (
                                        Some("Invalid index argument".to_string()),
                                        None,
                                        NcMethodStatus::ParameterError,
                                    );
                                }
                            };

                            match args_obj.get("id").and_then(|v| v.as_object()) {
                                Some(id_obj) => {
                                    let level = id_obj
                                        .get("level")
                                        .and_then(|v| v.as_u64())
                                        .map(|v| v as u32);
                                    let index_field = id_obj
                                        .get("index")
                                        .and_then(|v| v.as_u64())
                                        .map(|v| v as u32);

                                    match (level, index_field) {
                                        (Some(3), Some(1)) => {
                                            // Handle controlClasses (3p1)
                                            let sequence: Vec<NcClassDescriptor> = self
                                                .control_classes_register
                                                .values()
                                                .cloned()
                                                .collect();

                                            // Check if index is within bounds
                                            if index >= sequence.len() {
                                                return (
                                                    Some(format!(
                                                        "Index {} out of bounds for controlClasses sequence",
                                                        index
                                                    )),
                                                    None,
                                                    NcMethodStatus::IndexOutOfBounds,
                                                );
                                            }

                                            (
                                                None,
                                                Some(json!(sequence[index].clone())),
                                                NcMethodStatus::Ok,
                                            )
                                        }
                                        (Some(3), Some(2)) => {
                                            // Handle datatypes (3p2)
                                            let sequence: Vec<NcAnyDatatypeDescriptor> = self
                                                .data_types_register
                                                .values()
                                                .cloned()
                                                .collect();

                                            // Check if index is within bounds
                                            if index >= sequence.len() {
                                                return (
                                                    Some(format!(
                                                        "Index {} out of bounds for datatypes sequence",
                                                        index
                                                    )),
                                                    None,
                                                    NcMethodStatus::IndexOutOfBounds,
                                                );
                                            }

                                            (
                                                None,
                                                Some(json!(sequence[index].clone())),
                                                NcMethodStatus::Ok,
                                            )
                                        }
                                        _ => (
                                            Some("Invalid id argument".to_string()),
                                            None,
                                            NcMethodStatus::ParameterError,
                                        ),
                                    }
                                }
                                None => (
                                    Some("Invalid id argument".to_string()),
                                    None,
                                    NcMethodStatus::ParameterError,
                                ),
                            }
                        }
                        None => (
                            Some("Invalid arguments".to_string()),
                            None,
                            NcMethodStatus::ParameterError,
                        ),
                    }
                }
                (1, 7) => {
                    match args.as_object() {
                        Some(args_obj) => {
                            match args_obj.get("id").and_then(|v| v.as_object()) {
                                Some(id_obj) => {
                                    let level = id_obj
                                        .get("level")
                                        .and_then(|v| v.as_u64())
                                        .map(|v| v as u32);
                                    let index = id_obj
                                        .get("index")
                                        .and_then(|v| v.as_u64())
                                        .map(|v| v as u32);

                                    match (level, index) {
                                        (Some(3), Some(1)) => {
                                            // Return length of controlClasses (3p1)
                                            let length = self.control_classes_register.len() as u64;
                                            (None, Some(json!(length)), NcMethodStatus::Ok)
                                        }
                                        (Some(3), Some(2)) => {
                                            // Return length of datatypes (3p2)
                                            let length = self.data_types_register.len() as u64;
                                            (None, Some(json!(length)), NcMethodStatus::Ok)
                                        }
                                        _ => (
                                            Some("Invalid id argument".to_string()),
                                            None,
                                            NcMethodStatus::ParameterError,
                                        ),
                                    }
                                }
                                None => (
                                    Some("Invalid id argument".to_string()),
                                    None,
                                    NcMethodStatus::ParameterError,
                                ),
                            }
                        }
                        None => (
                            Some("Invalid arguments".to_string()),
                            None,
                            NcMethodStatus::ParameterError,
                        ),
                    }
                }
                (3, 1) => {
                    let class_id = args.get("classId").and_then(|v| v.as_array()).map(|arr| {
                        arr.iter()
                            .filter_map(|n| n.as_u64().map(|x| x as u32))
                            .collect::<Vec<u32>>()
                    });
                    if class_id.is_none() {
                        return (
                            Some("No class identity has been provided".to_string()),
                            None,
                            NcMethodStatus::InvalidRequest,
                        );
                    }
                    let include_inherited = args.get("includeInherited").and_then(|v| v.as_bool());
                    if include_inherited.is_none() {
                        return (
                            Some("No includeInherited argument provided".to_string()),
                            None,
                            NcMethodStatus::InvalidRequest,
                        );
                    }

                    let class_id = class_id.unwrap();
                    let include_inherited = include_inherited.unwrap();
                    match self.get_control_class_descriptor(&class_id, include_inherited) {
                        Some(desc) => (None, Some(json!(desc)), NcMethodStatus::Ok),
                        None => (
                            Some("Descriptor for class could not be found".to_string()),
                            None,
                            NcMethodStatus::InvalidRequest,
                        ),
                    }
                }
                (3, 2) => {
                    let name = args
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    if name.is_none() {
                        return (
                            Some("No name argument has been provided".to_string()),
                            None,
                            NcMethodStatus::InvalidRequest,
                        );
                    }
                    let include_inherited = args.get("includeInherited").and_then(|v| v.as_bool());
                    if include_inherited.is_none() {
                        return (
                            Some("No includeInherited argument provided".to_string()),
                            None,
                            NcMethodStatus::InvalidRequest,
                        );
                    }

                    let name = name.unwrap();
                    let include_inherited = include_inherited.unwrap();
                    match self.get_datatype_descriptor(&name, include_inherited) {
                        Some(desc) => (None, Some(json!(desc)), NcMethodStatus::Ok),
                        None => (
                            Some("Descriptor for type could not be found".to_string()),
                            None,
                            NcMethodStatus::InvalidRequest,
                        ),
                    }
                }
                _ => self.base.invoke_method(oid, method_id, args),
            }
        } else {
            self.base.invoke_method(oid, method_id, args)
        }
    }
}

impl NcClassManager {
    fn class_id_to_key(id: &[u32]) -> String {
        id.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(".")
    }

    fn generate_class_descriptors() -> HashMap<String, NcClassDescriptor> {
        let mut reg: HashMap<String, NcClassDescriptor> = HashMap::new();

        let classes = vec![
            crate::nc_object::NcObject::get_class_descriptor(false),
            crate::nc_block::NcBlock::get_class_descriptor(false),
            crate::nc_manager::NcManager::get_class_descriptor(false),
            crate::nc_device_manager::NcDeviceManager::get_class_descriptor(false),
            crate::nc_class_manager::NcClassManager::get_class_descriptor(false),
        ];

        for desc in classes {
            let key = NcClassManager::class_id_to_key(&desc.class_id);
            reg.insert(key, desc);
        }

        reg
    }

    pub fn get_datatype_descriptor(
        &self,
        name: &str,
        include_inherited: bool,
    ) -> Option<NcAnyDatatypeDescriptor> {
        if !include_inherited {
            return self.data_types_register.get(name).cloned();
        }

        // For include_inherited = true, rebuild known struct descriptors using their static builders

        let rebuilt: Option<DtStruct> = match name {
            "NcFieldDescriptor" => Some(crate::data_types::NcFieldDescriptor::get_type_descriptor(
                true,
            )),
            "NcEnumItemDescriptor" => {
                Some(crate::data_types::NcEnumItemDescriptor::get_type_descriptor(true))
            }
            "NcPropertyDescriptor" => {
                Some(crate::data_types::NcPropertyDescriptor::get_type_descriptor(true))
            }
            "NcMethodDescriptor" => Some(
                crate::data_types::NcMethodDescriptor::get_type_descriptor(true),
            ),
            "NcEventDescriptor" => Some(crate::data_types::NcEventDescriptor::get_type_descriptor(
                true,
            )),
            "NcParameterDescriptor" => {
                Some(crate::data_types::NcParameterDescriptor::get_type_descriptor(true))
            }
            "NcClassDescriptor" => Some(crate::data_types::NcClassDescriptor::get_type_descriptor(
                true,
            )),
            "NcDescriptor" => Some(crate::data_types::NcDescriptor::get_type_descriptor(true)),
            "NcDatatypeDescriptor" => {
                Some(crate::data_types::NcDatatypeDescriptor::get_type_descriptor(true))
            }
            "NcDatatypeDescriptorPrimitive" => {
                Some(crate::data_types::NcDatatypeDescriptorPrimitive::get_type_descriptor(true))
            }
            "NcDatatypeDescriptorEnum" => {
                Some(crate::data_types::NcDatatypeDescriptorEnum::get_type_descriptor(true))
            }
            "NcDatatypeDescriptorTypeDef" => {
                Some(crate::data_types::NcDatatypeDescriptorTypeDef::get_type_descriptor(true))
            }
            "NcDatatypeDescriptorStruct" => {
                Some(crate::data_types::NcDatatypeDescriptorStruct::get_type_descriptor(true))
            }
            "NcParameterConstraints" => {
                Some(crate::data_types::NcParameterConstraints::get_type_descriptor(true))
            }
            "NcParameterConstraintsNumber" => {
                Some(crate::data_types::NcParameterConstraintsNumber::get_type_descriptor(true))
            }
            "NcParameterConstraintsString" => {
                Some(crate::data_types::NcParameterConstraintsString::get_type_descriptor(true))
            }
            "NcPropertyConstraints" => {
                Some(crate::data_types::NcPropertyConstraintsBase::get_type_descriptor(true))
            }
            "NcPropertyConstraintsNumber" => {
                Some(crate::data_types::NcPropertyConstraintsNumber::get_type_descriptor(true))
            }
            "NcPropertyConstraintsString" => {
                Some(crate::data_types::NcPropertyConstraintsString::get_type_descriptor(true))
            }
            "NcElementId" => Some(crate::data_types::NcElementId::get_type_descriptor(true)),
            "NcPropertyId" => Some(crate::data_types::NcPropertyId::get_type_descriptor(true)),
            "NcMethodId" => Some(crate::data_types::NcMethodId::get_type_descriptor(true)),
            "NcEventId" => Some(crate::data_types::NcEventId::get_type_descriptor(true)),
            "NcManufacturer" => Some(crate::data_types::NcManufacturer::get_type_descriptor(true)),
            "NcProduct" => Some(crate::data_types::NcProduct::get_type_descriptor(true)),
            "NcDeviceOperationalState" => {
                Some(crate::data_types::NcDeviceOperationalState::get_type_descriptor(true))
            }
            "PropertyChangedEventData" | "NcPropertyChangedEventData" => {
                Some(crate::data_types::PropertyChangedEventData::get_type_descriptor(true))
            }
            "NcBlockMemberDescriptor" => {
                Some(crate::data_types::NcBlockMemberDescriptor::get_type_descriptor(true))
            }
            "NcTouchpoint" => Some(crate::data_types::NcTouchpointBase::get_type_descriptor(
                true,
            )),
            "NcTouchpointNmos" => Some(crate::data_types::NcTouchpointNmos::get_type_descriptor(
                true,
            )),
            "NcTouchpointNmosChannelMapping" => {
                Some(crate::data_types::NcTouchpointNmosChannelMapping::get_type_descriptor(true))
            }
            "NcTouchpointResource" => {
                Some(crate::data_types::NcTouchpointResourceBase::get_type_descriptor(true))
            }
            "NcTouchpointResourceNmos" => {
                Some(crate::data_types::NcTouchpointResourceNmos::get_type_descriptor(true))
            }
            "NcTouchpointResourceNmosChannelMapping" => Some(
                crate::data_types::NcTouchpointResourceNmosChannelMapping::get_type_descriptor(
                    true,
                ),
            ),
            "NcMethodResult" => Some(crate::data_types::NcMethodResult::get_type_descriptor(true)),
            "NcMethodResultPropertyValue" => {
                Some(crate::data_types::NcMethodResultPropertyValue::get_type_descriptor(true))
            }
            "NcMethodResultError" => Some(
                crate::data_types::NcMethodResultError::get_type_descriptor(true),
            ),
            "NcMethodResultBlockMemberDescriptors" => Some(
                crate::data_types::NcMethodResultBlockMemberDescriptors::get_type_descriptor(true),
            ),
            "NcMethodResultClassDescriptor" => {
                Some(crate::data_types::NcMethodResultClassDescriptor::get_type_descriptor(true))
            }
            "NcMethodResultDatatypeDescriptor" => {
                Some(crate::data_types::NcMethodResultDatatypeDescriptor::get_type_descriptor(true))
            }
            "NcMethodResultId" => Some(crate::data_types::NcMethodResultId::get_type_descriptor(
                true,
            )),
            "NcMethodResultLength" => {
                Some(crate::data_types::NcMethodResultLength::get_type_descriptor(true))
            }

            // Types registered directly or non-structs: fall back to registry
            _ => None,
        };

        if let Some(dt) = rebuilt {
            return Some(NcAnyDatatypeDescriptor::Struct(dt));
        }

        // For primitives, typedefs, enums, or unknowns, return the cached version
        self.data_types_register.get(name).cloned()
    }

    fn generate_type_descriptors() -> HashMap<String, NcAnyDatatypeDescriptor> {
        let mut reg: HashMap<String, NcAnyDatatypeDescriptor> = HashMap::new();

        let mut add_prim = |name: &str, description: &str| {
            reg.insert(
                name.to_string(),
                NcAnyDatatypeDescriptor::Primitive(NcDatatypeDescriptor {
                    base: crate::data_types::NcDescriptor {
                        description: Some(description.to_string()),
                    },
                    name: name.to_string(),
                    type_: NcDatatypeType::Primitive,
                    constraints: None,
                }),
            );
        };

        add_prim("NcBoolean", "Boolean primitive");
        add_prim("NcInt16", "Signed 16-bit integer");
        add_prim("NcInt32", "Signed 32-bit integer");
        add_prim("NcInt64", "Signed 64-bit integer");
        add_prim("NcUint16", "Unsigned 16-bit integer");
        add_prim("NcUint32", "Unsigned 32-bit integer");
        add_prim("NcUint64", "Unsigned 64-bit integer");
        add_prim("NcFloat32", "32-bit floating point");
        add_prim("NcFloat64", "64-bit floating point");
        add_prim("NcString", "String primitive");

        let mut add_struct = |dt: DtStruct| {
            let name = dt.base.name.clone();
            reg.insert(name, NcAnyDatatypeDescriptor::Struct(dt));
        };
        add_struct(crate::data_types::NcElementId::get_type_descriptor(false));
        add_struct(crate::data_types::NcEventId::get_type_descriptor(false));
        add_struct(crate::data_types::NcMethodId::get_type_descriptor(false));
        add_struct(crate::data_types::NcPropertyId::get_type_descriptor(false));
        add_struct(crate::data_types::NcManufacturer::get_type_descriptor(
            false,
        ));
        add_struct(crate::data_types::NcProduct::get_type_descriptor(false));
        add_struct(crate::data_types::NcDeviceOperationalState::get_type_descriptor(false));
        add_struct(crate::data_types::PropertyChangedEventData::get_type_descriptor(false));
        add_struct(crate::data_types::NcBlockMemberDescriptor::get_type_descriptor(false));
        add_struct(crate::data_types::NcDatatypeDescriptor::get_type_descriptor(false));
        add_struct(crate::data_types::NcDatatypeDescriptorPrimitive::get_type_descriptor(false));
        add_struct(crate::data_types::NcDatatypeDescriptorEnum::get_type_descriptor(false));
        add_struct(crate::data_types::NcDatatypeDescriptorTypeDef::get_type_descriptor(false));
        add_struct(crate::data_types::NcDatatypeDescriptorStruct::get_type_descriptor(false));
        add_struct(crate::data_types::NcFieldDescriptor::get_type_descriptor(
            false,
        ));
        add_struct(crate::data_types::NcEnumItemDescriptor::get_type_descriptor(false));
        add_struct(crate::data_types::NcDescriptor::get_type_descriptor(false));
        add_struct(crate::data_types::NcPropertyDescriptor::get_type_descriptor(false));
        add_struct(crate::data_types::NcMethodDescriptor::get_type_descriptor(
            false,
        ));
        add_struct(crate::data_types::NcEventDescriptor::get_type_descriptor(
            false,
        ));
        add_struct(crate::data_types::NcParameterDescriptor::get_type_descriptor(false));
        add_struct(crate::data_types::NcClassDescriptor::get_type_descriptor(
            false,
        ));
        add_struct(crate::data_types::NcDescriptor::get_type_descriptor(false));
        add_struct(crate::data_types::NcTouchpointBase::get_type_descriptor(
            false,
        ));
        add_struct(crate::data_types::NcTouchpointNmos::get_type_descriptor(
            false,
        ));
        add_struct(crate::data_types::NcTouchpointNmosChannelMapping::get_type_descriptor(false));
        add_struct(crate::data_types::NcTouchpointResourceBase::get_type_descriptor(false));
        add_struct(crate::data_types::NcTouchpointResourceNmos::get_type_descriptor(false));
        add_struct(
            crate::data_types::NcTouchpointResourceNmosChannelMapping::get_type_descriptor(false),
        );
        add_struct(crate::data_types::NcParameterConstraints::get_type_descriptor(false));
        add_struct(crate::data_types::NcParameterConstraintsNumber::get_type_descriptor(false));
        add_struct(crate::data_types::NcParameterConstraintsString::get_type_descriptor(false));
        add_struct(crate::data_types::NcPropertyConstraintsBase::get_type_descriptor(false));
        add_struct(crate::data_types::NcPropertyConstraintsNumber::get_type_descriptor(false));
        add_struct(crate::data_types::NcPropertyConstraintsString::get_type_descriptor(false));
        add_struct(crate::data_types::NcMethodResult::get_type_descriptor(
            false,
        ));
        add_struct(crate::data_types::NcMethodResultPropertyValue::get_type_descriptor(false));
        add_struct(crate::data_types::NcMethodResultError::get_type_descriptor(
            false,
        ));
        add_struct(
            crate::data_types::NcMethodResultBlockMemberDescriptors::get_type_descriptor(false),
        );
        add_struct(crate::data_types::NcMethodResultClassDescriptor::get_type_descriptor(false));
        add_struct(crate::data_types::NcMethodResultDatatypeDescriptor::get_type_descriptor(false));
        add_struct(crate::data_types::NcMethodResultId::get_type_descriptor(
            false,
        ));
        add_struct(crate::data_types::NcMethodResultLength::get_type_descriptor(false));

        let mut add_enum = |name: &str, items: Vec<(&str, u16, &str)>, description: &str| {
            let enum_desc = crate::data_types::NcDatatypeDescriptorEnum {
                base: NcDatatypeDescriptor {
                    base: crate::data_types::NcDescriptor {
                        description: Some(description.to_string()),
                    },
                    name: name.to_string(),
                    type_: NcDatatypeType::Enum,
                    constraints: None,
                },
                items: items
                    .into_iter()
                    .map(|(n, v, d)| crate::data_types::NcEnumItemDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some(d.to_string()),
                        },
                        name: n.to_string(),
                        value: v,
                    })
                    .collect(),
            };
            reg.insert(name.to_string(), NcAnyDatatypeDescriptor::Enum(enum_desc));
        };

        add_enum(
            "NcDeviceGenericState",
            vec![
                ("Unknown", 0, "Unknown"),
                ("NormalOperation", 1, "Normal operation"),
                ("Initializing", 2, "Initializing"),
                ("Updating", 3, "Updating"),
                ("LicensingError", 4, "Licensing error"),
                ("InternalError", 5, "Internal error"),
            ],
            "Device generic state enumeration",
        );

        add_enum(
            "NcPropertyChangeType",
            vec![
                ("ValueChanged", 0, "Value changed"),
                ("SequenceItemAdded", 1, "Sequence item added"),
                ("SequenceItemChanged", 2, "Sequence item changed"),
                ("SequenceItemRemoved", 3, "Sequence item removed"),
            ],
            "Property change type enumeration",
        );

        add_enum(
            "NcResetCause",
            vec![
                ("Unknown", 0, "Unknown"),
                ("PowerOn", 1, "Power on"),
                ("InternalError", 2, "Internal error"),
                ("Upgrade", 3, "Upgrade"),
                ("ControllerRequest", 4, "Controller request"),
                ("ManualReset", 5, "Manual reset"),
            ],
            "Device reset cause enumeration",
        );

        add_enum(
            "NcMethodStatus",
            vec![
                ("Ok", 200, "Ok"),
                ("PropertyDeprecated", 298, "Property deprecated"),
                ("MethodDeprecated", 299, "Method deprecated"),
                ("BadCommandFormat", 400, "Bad command format"),
                ("Unauthorized", 401, "Unauthorized"),
                ("BadOid", 404, "Bad OID"),
                ("Readonly", 405, "Readonly"),
                ("InvalidRequest", 406, "Invalid request"),
                ("Conflict", 409, "Conflict"),
                ("BufferOverflow", 413, "Buffer overflow"),
                ("IndexOutOfBounds", 414, "Index out of bounds"),
                ("ParameterError", 417, "Parameter error"),
                ("Locked", 423, "Locked"),
                ("DeviceError", 500, "Device error"),
                ("MethodNotImplemented", 501, "Method not implemented"),
                ("PropertyNotImplemented", 502, "Property not implemented"),
                ("NotReady", 503, "Not ready"),
                ("Timeout", 504, "Timeout"),
            ],
            "Method status enumeration",
        );

        add_enum(
            "NcDatatypeType",
            vec![
                ("Primitive", 0, "Primitive"),
                ("Typedef", 1, "Typedef"),
                ("Struct", 2, "Struct"),
                ("Enum", 3, "Enum"),
            ],
            "Datatype kind enumeration",
        );

        let mut add_typedef = |name: &str, parent: &str, is_sequence: bool, description: &str| {
            reg.insert(
                name.to_string(),
                NcAnyDatatypeDescriptor::TypeDef(DtTypeDef {
                    base: NcDatatypeDescriptor {
                        base: crate::data_types::NcDescriptor {
                            description: Some(description.to_string()),
                        },
                        name: name.to_string(),
                        type_: NcDatatypeType::Typedef,
                        constraints: None,
                    },
                    parent_type: parent.to_string(),
                    is_sequence,
                }),
            );
        };

        add_typedef(
            "NcName",
            "NcString",
            false,
            "Programmatically significant name, alphanumerics + underscore, no spaces",
        );
        add_typedef("NcRolePath", "NcString", true, "Role path");
        add_typedef("NcRegex", "NcString", false, "Regex pattern");
        add_typedef("NcRole", "NcString", false, "Role string");
        add_typedef("NcClassId", "NcInt32", true, "Sequence of class ID fields.");
        add_typedef("NcId", "NcUint32", false, "Identifier handler");
        add_typedef("NcOid", "NcUint32", false, "Object id");
        add_typedef(
            "NcOrganizationId",
            "NcInt32",
            false,
            "Unique 24-bit organization id",
        );
        add_typedef("NcUri", "NcString", false, "Uniform resource identifier");
        add_typedef(
            "NcVersionCode",
            "NcString",
            false,
            "Version code in semantic versioning format",
        );
        add_typedef("NcUuid", "NcString", false, "UUID");
        add_typedef(
            "NcTimeInterval",
            "NcInt64",
            false,
            "Time interval described in nanoseconds",
        );

        reg
    }
    pub fn get_control_class_descriptor(
        &self,
        class_id: &[u32],
        include_inherited: bool,
    ) -> Option<NcClassDescriptor> {
        let key = NcClassManager::class_id_to_key(class_id);
        if !include_inherited {
            return self.control_classes_register.get(&key).cloned();
        }

        match class_id {
            [1] => Some(crate::nc_object::NcObject::get_class_descriptor(true)),
            [1, 1] => Some(crate::nc_block::NcBlock::get_class_descriptor(true)),
            [1, 3] => Some(crate::nc_manager::NcManager::get_class_descriptor(true)),
            [1, 3, 1] => {
                Some(crate::nc_device_manager::NcDeviceManager::get_class_descriptor(true))
            }
            [1, 3, 2] => Some(crate::nc_class_manager::NcClassManager::get_class_descriptor(true)),
            _ => None,
        }
    }
    pub fn get_class_descriptor(include_inherited: bool) -> NcClassDescriptor {
        let mut desc = NcClassDescriptor {
            base: crate::data_types::NcDescriptor { description: Some("NcClassManager class descriptor".to_string()) },
            class_id: vec![1, 3, 2],
            name: "NcClassManager".to_string(),
            fixed_role: Some("ClassManager".to_string()),
            properties: vec![
                NcPropertyDescriptor { // 3p1
                    base: crate::data_types::NcDescriptor { description: Some("Descriptions of all control classes in the device (descriptors do not contain inherited elements)".to_string()) },
                    id: NcElementId { level: 3, index: 1 },
                    name: "controlClasses".to_string(),
                    type_name: Some("NcClassDescriptor".to_string()),
                    is_read_only: true,
                    is_nullable: false,
                    is_sequence: true,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor { // 3p2
                    base: crate::data_types::NcDescriptor { description: Some("Descriptions of all data types in the device (descriptors do not contain inherited elements)".to_string()) },
                    id: NcElementId { level: 3, index: 2 },
                    name: "datatypes".to_string(),
                    type_name: Some("NcDatatypeDescriptor".to_string()),
                    is_read_only: true,
                    is_nullable: false,
                    is_sequence: true,
                    is_deprecated: false,
                    constraints: None,
                },
            ],
            methods: vec![
                // 3m1 GetControlClass
                crate::data_types::NcMethodDescriptor {
                    base: crate::data_types::NcDescriptor { description: None },
                    id: NcElementId { level: 3, index: 1 },
                    name: "GetControlClass".to_string(),
                    result_datatype: "NcMethodResultClassDescriptor".to_string(),
                    parameters: vec![
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("class ID".to_string()) },
                            name: "classId".to_string(),
                            type_name: Some("NcClassId".to_string()),
                            is_nullable: false,
                            is_sequence: false,
                            constraints: None,
                        },
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("If set the descriptor would contain all inherited elements".to_string()) },
                            name: "includeInherited".to_string(),
                            type_name: Some("NcBoolean".to_string()),
                            is_nullable: false,
                            is_sequence: false,
                            constraints: None,
                        },
                    ],
                    is_deprecated: false,
                },
                // 3m2 GetDatatype
                crate::data_types::NcMethodDescriptor {
                    base: crate::data_types::NcDescriptor { description: None },
                    id: NcElementId { level: 3, index: 2 },
                    name: "GetDatatype".to_string(),
                    result_datatype: "NcMethodResultDatatypeDescriptor".to_string(),
                    parameters: vec![
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("name of datatype".to_string()) },
                            name: "name".to_string(),
                            type_name: Some("NcName".to_string()),
                            is_nullable: false,
                            is_sequence: false,
                            constraints: None,
                        },
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("If set the descriptor would contain all inherited elements".to_string()) },
                            name: "includeInherited".to_string(),
                            type_name: Some("NcBoolean".to_string()),
                            is_nullable: false,
                            is_sequence: false,
                            constraints: None,
                        },
                    ],
                    is_deprecated: false,
                },
            ],
            events: vec![],
        };

        if include_inherited {
            let base_desc = crate::nc_manager::NcManager::get_class_descriptor(true);
            desc.properties.extend(base_desc.properties);
            desc.methods.extend(base_desc.methods);
            desc.events.extend(base_desc.events);
        }

        desc
    }
}

#[allow(clippy::too_many_arguments)]
impl NcClassManager {
    pub fn new(
        oid: u64,
        constant_oid: bool,
        owner: Option<u64>,
        role: &str,
        user_label: Option<&str>,
        touchpoints: Option<Vec<NcTouchpoint>>,
        runtime_property_constraints: Option<Vec<NcPropertyConstraints>>,
        notifier: mpsc::UnboundedSender<PropertyChangedEvent>,
    ) -> Self {
        let base = NcManager::new(
            vec![1, 3, 2], // Class ID for NcClassManager
            oid,
            constant_oid,
            owner,
            role,
            user_label,
            touchpoints,
            runtime_property_constraints,
            notifier,
        );

        let control_classes_register = NcClassManager::generate_class_descriptors();
        let data_types_register = NcClassManager::generate_type_descriptors();

        NcClassManager {
            base,
            control_classes_register,
            data_types_register,
        }
    }
}
