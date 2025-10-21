use crate::data_types::{
    ElementId, IdArgs, IdArgsValue, NcDeviceGenericState, NcDeviceOperationalState, NcManufacturer,
    NcMethodStatus, NcProduct, NcPropertyChangeType, NcResetCause, PropertyChangedEvent,
    PropertyChangedEventData,
};
use crate::nc_object::{NcMember, NcObject};
use serde_json::{Value, json};
use std::any::Any;
use tokio::sync::mpsc;

pub struct NcDeviceManager {
    pub base: NcObject,
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
        &self.base.role
    }

    fn get_oid(&self) -> u64 {
        self.base.oid
    }

    fn get_constant_oid(&self) -> bool {
        self.base.constant_oid
    }

    fn get_class_id(&self) -> &[u32] {
        &self.base.class_id
    }

    fn get_user_label(&self) -> Option<&str> {
        self.base.user_label.as_deref()
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

                    let _ = self.base.notifier.send(PropertyChangedEvent::new(
                        self.base.oid,
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

                    let _ = self.base.notifier.send(PropertyChangedEvent::new(
                        self.base.oid,
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

                    let _ = self.base.notifier.send(PropertyChangedEvent::new(
                        self.base.oid,
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
                    NcMethodStatus::BadOid,
                ),
            }
        } else {
            self.base.set_property(_oid, id_args_value)
        }
    }

    fn invoke_method(
        &self,
        _oid: u64,
        method_id: ElementId,
        args: Value,
    ) -> (Option<String>, Option<Value>, NcMethodStatus) {
        // NcDeviceManager has no methods, delegate to base
        self.base.invoke_method(_oid, method_id, args)
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
        nc_version: String,
        manufacturer: NcManufacturer,
        product: NcProduct,
        serial_number: String,
        notifier: mpsc::UnboundedSender<PropertyChangedEvent>,
    ) -> Self {
        NcDeviceManager {
            base: NcObject::new(
                vec![1, 3, 1], // Class ID for NcDeviceManager
                oid,
                constant_oid,
                owner,
                role,
                user_label,
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
