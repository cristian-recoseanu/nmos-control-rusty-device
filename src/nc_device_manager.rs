use crate::data_types::{
    IdArgs, IdArgsValue, NcClassDescriptor, NcDeviceGenericState, NcDeviceOperationalState,
    NcElementId, NcManufacturer, NcMethodStatus, NcProduct, NcPropertyChangeType,
    NcPropertyConstraints, NcPropertyDescriptor, NcResetCause, NcTouchpoint, PropertyChangedEvent,
    PropertyChangedEventData,
};
use crate::nc_manager::NcManager;
use crate::nc_object::NcMember;
use serde_json::{Value, json};
use std::any::Any;
use tokio::sync::mpsc;

pub struct NcDeviceManager {
    pub base: NcManager,
    pub nc_version: String,
    pub manufacturer: NcManufacturer,
    pub product: NcProduct,
    pub serial_number: String,
    pub user_inventory_code: Option<String>,
    pub device_name: Option<String>,
    pub device_role: Option<String>,
    pub operational_state: NcDeviceOperationalState,
    pub reset_cause: NcResetCause,
    pub message: Option<String>,
}

impl NcMember for NcDeviceManager {
    fn member_type(&self) -> &'static str {
        "NcDeviceManager"
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
            (3, 1) => (None, json!(self.nc_version), NcMethodStatus::Ok),
            (3, 2) => (None, json!(self.manufacturer), NcMethodStatus::Ok),
            (3, 3) => (None, json!(self.product), NcMethodStatus::Ok),
            (3, 4) => (None, json!(self.serial_number), NcMethodStatus::Ok),
            (3, 5) => (None, json!(self.user_inventory_code), NcMethodStatus::Ok),
            (3, 6) => (None, json!(self.device_name), NcMethodStatus::Ok),
            (3, 7) => (None, json!(self.device_role), NcMethodStatus::Ok),
            (3, 8) => (None, json!(self.operational_state), NcMethodStatus::Ok),
            (3, 9) => (None, json!(self.reset_cause), NcMethodStatus::Ok),
            (3, 10) => (None, json!(self.message), NcMethodStatus::Ok),
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
                5 => {
                    // userInventoryCode
                    if id_args_value.value.is_null() {
                        self.user_inventory_code = None;
                    } else if let Some(code) = id_args_value.value.as_str() {
                        self.user_inventory_code = Some(code.to_string());
                    } else {
                        return (
                            Some("Property value was invalid".to_string()),
                            NcMethodStatus::ParameterError,
                        );
                    }

                    let _ = self.base.base.notifier.send(PropertyChangedEvent::new(
                        self.base.base.oid,
                        PropertyChangedEventData {
                            property_id: id_args_value.id,
                            change_type: NcPropertyChangeType::ValueChanged,
                            value: json!(self.user_inventory_code),
                            sequence_item_index: None,
                        },
                    ));
                    (None, NcMethodStatus::Ok)
                }
                6 => {
                    // deviceName
                    if id_args_value.value.is_null() {
                        self.device_name = None;
                    } else if let Some(name) = id_args_value.value.as_str() {
                        self.device_name = Some(name.to_string());
                    } else {
                        return (
                            Some("Property value was invalid".to_string()),
                            NcMethodStatus::ParameterError,
                        );
                    }

                    let _ = self.base.base.notifier.send(PropertyChangedEvent::new(
                        self.base.base.oid,
                        PropertyChangedEventData {
                            property_id: id_args_value.id,
                            change_type: NcPropertyChangeType::ValueChanged,
                            value: json!(self.device_name),
                            sequence_item_index: None,
                        },
                    ));
                    (None, NcMethodStatus::Ok)
                }
                7 => {
                    // deviceRole
                    if id_args_value.value.is_null() {
                        self.device_role = None;
                    } else if let Some(role) = id_args_value.value.as_str() {
                        self.device_role = Some(role.to_string());
                    } else {
                        return (
                            Some("Property value was invalid".to_string()),
                            NcMethodStatus::ParameterError,
                        );
                    }

                    let _ = self.base.base.notifier.send(PropertyChangedEvent::new(
                        self.base.base.oid,
                        PropertyChangedEventData {
                            property_id: id_args_value.id,
                            change_type: NcPropertyChangeType::ValueChanged,
                            value: json!(self.device_role),
                            sequence_item_index: None,
                        },
                    ));
                    (None, NcMethodStatus::Ok)
                }
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
        _oid: u64,
        method_id: NcElementId,
        args: Value,
    ) -> (Option<String>, Option<Value>, NcMethodStatus) {
        self.base.invoke_method(_oid, method_id, args)
    }
}

impl NcDeviceManager {
    pub fn get_class_descriptor(include_inherited: bool) -> NcClassDescriptor {
        let mut desc = NcClassDescriptor {
            base: crate::data_types::NcDescriptor { description: Some("NcDeviceManager class descriptor".to_string()) },
            class_id: vec![1, 3, 1],
            name: "NcDeviceManager".to_string(),
            fixed_role: Some("DeviceManager".to_string()),
            properties: vec![
                NcPropertyDescriptor { // 3p1
                    base: crate::data_types::NcDescriptor { description: Some("Version of nc this dev uses".to_string()) },
                    id: NcElementId { level: 3, index: 1 },
                    name: "ncVersion".to_string(),
                    type_name: Some("NcVersionCode".to_string()),
                    is_read_only: true,
                    is_nullable: false,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor { // 3p2
                    base: crate::data_types::NcDescriptor { description: Some("Manufacturer descriptor".to_string()) },
                    id: NcElementId { level: 3, index: 2 },
                    name: "manufacturer".to_string(),
                    type_name: Some("NcManufacturer".to_string()),
                    is_read_only: true,
                    is_nullable: false,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor { // 3p3
                    base: crate::data_types::NcDescriptor { description: Some("Product descriptor".to_string()) },
                    id: NcElementId { level: 3, index: 3 },
                    name: "product".to_string(),
                    type_name: Some("NcProduct".to_string()),
                    is_read_only: true,
                    is_nullable: false,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor { // 3p4
                    base: crate::data_types::NcDescriptor { description: Some("Serial number".to_string()) },
                    id: NcElementId { level: 3, index: 4 },
                    name: "serialNumber".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_read_only: true,
                    is_nullable: false,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor { // 3p5
                    base: crate::data_types::NcDescriptor { description: Some("Asset tracking identifier (user specified)".to_string()) },
                    id: NcElementId { level: 3, index: 5 },
                    name: "userInventoryCode".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_read_only: false,
                    is_nullable: true,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor { // 3p6
                    base: crate::data_types::NcDescriptor { description: Some("Name of this device in the application. Instance name, not product name.".to_string()) },
                    id: NcElementId { level: 3, index: 6 },
                    name: "deviceName".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_read_only: false,
                    is_nullable: true,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor { // 3p7
                    base: crate::data_types::NcDescriptor { description: Some("Role of this device in the application.".to_string()) },
                    id: NcElementId { level: 3, index: 7 },
                    name: "deviceRole".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_read_only: false,
                    is_nullable: true,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor { // 3p8
                    base: crate::data_types::NcDescriptor { description: Some("Device operational state".to_string()) },
                    id: NcElementId { level: 3, index: 8 },
                    name: "operationalState".to_string(),
                    type_name: Some("NcDeviceOperationalState".to_string()),
                    is_read_only: true,
                    is_nullable: false,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor { // 3p9
                    base: crate::data_types::NcDescriptor { description: Some("Reason for most recent reset".to_string()) },
                    id: NcElementId { level: 3, index: 9 },
                    name: "resetCause".to_string(),
                    type_name: Some("NcResetCause".to_string()),
                    is_read_only: true,
                    is_nullable: false,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor { // 3p10
                    base: crate::data_types::NcDescriptor { description: Some("Arbitrary message from dev to controller".to_string()) },
                    id: NcElementId { level: 3, index: 10 },
                    name: "message".to_string(),
                    type_name: Some("NcString".to_string()),
                    is_read_only: true,
                    is_nullable: true,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
            ],
            methods: vec![],
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
impl NcDeviceManager {
    pub fn new(
        oid: u64,
        constant_oid: bool,
        owner: Option<u64>,
        role: &str,
        user_label: Option<&str>,
        touchpoints: Option<Vec<NcTouchpoint>>,
        runtime_property_constraints: Option<Vec<NcPropertyConstraints>>,
        nc_version: String,
        manufacturer: NcManufacturer,
        product: NcProduct,
        serial_number: String,
        notifier: mpsc::UnboundedSender<PropertyChangedEvent>,
    ) -> Self {
        NcDeviceManager {
            base: NcManager::new(
                vec![1, 3, 1], // Class ID for NcDeviceManager
                oid,
                constant_oid,
                owner,
                role,
                user_label,
                touchpoints,
                runtime_property_constraints,
                notifier,
            ),
            nc_version,
            manufacturer,
            product,
            serial_number,
            user_inventory_code: None,
            device_name: None,
            device_role: None,
            operational_state: NcDeviceOperationalState {
                generic: NcDeviceGenericState::NormalOperation,
                device_specific_details: None,
            },
            reset_cause: NcResetCause::PowerOn,
            message: None,
        }
    }
}
