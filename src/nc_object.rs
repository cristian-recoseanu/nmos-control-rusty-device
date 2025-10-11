use crate::data_types::{
    ElementId, IdArgs, IdArgsValue, NcPropertyChangeType, PropertyChangedEvent,
    PropertyChangedEventData,
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
    fn get_property(&self, oid: u64, id_args: &IdArgs) -> (Option<String>, Value);
    fn set_property(&mut self, _oid: u64, id_args_value: IdArgsValue) -> (Option<String>, bool);
    fn invoke_method(
        &self,
        _oid: u64,
        _method_id: ElementId,
        _args: Value,
    ) -> (Option<String>, Option<Value>);
}

#[derive(Debug, Clone)]
pub struct NcObject {
    pub class_id: Vec<u32>,
    pub oid: u64,
    pub constant_oid: bool,
    pub owner: Option<u64>,
    pub role: String,
    pub user_label: Option<String>,
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
    fn get_property(&self, _oid: u64, id_args: &IdArgs) -> (Option<String>, Value) {
        match (id_args.id.level, id_args.id.index) {
            (1, 1) => (None, json!(self.class_id)),
            (1, 2) => (None, json!(self.oid)),
            (1, 3) => (None, json!(self.constant_oid)),
            (1, 4) => (None, json!(self.owner)),
            (1, 5) => (None, json!(self.role)),
            (1, 6) => (None, json!(self.user_label)),
            _ => (None, json!(null)),
        }
    }
    fn set_property(&mut self, _oid: u64, id_args_value: IdArgsValue) -> (Option<String>, bool) {
        if id_args_value.id.level == 1 && id_args_value.id.index == 6 {
            // Set userLabel
            if let Some(new_label) = id_args_value.value.as_str() {
                self.user_label = Some(new_label.to_string());
                let _ = self.notifier.send(PropertyChangedEvent::new(
                    self.oid,
                    PropertyChangedEventData {
                        property_id: id_args_value.id,
                        change_type: NcPropertyChangeType::ValueChanged,
                        value: serde_json::json!(new_label),
                        sequence_item_index: None,
                    },
                ));
                return (None, true);
            }
            (Some("Property value was invalid".to_string()), false)
        } else {
            (Some("Could not find the property".to_string()), false)
        }
    }
    fn invoke_method(
        &self,
        _oid: u64,
        _method_id: ElementId,
        _args: Value,
    ) -> (Option<String>, Option<Value>) {
        //TODO: This is where we can add treatment for other methods in NcObject
        (Some("Methods not yet implemented".to_string()), None)
    }
}

impl NcObject {
    pub fn new(
        class_id: Vec<u32>,
        oid: u64,
        constant_oid: bool,
        owner: Option<u64>,
        role: &str,
        user_label: Option<&str>,
        notifier: mpsc::UnboundedSender<PropertyChangedEvent>,
    ) -> Self {
        NcObject {
            class_id,
            constant_oid,
            owner,
            oid,
            role: role.to_string(),
            user_label: user_label.map(|s| s.to_string()),
            notifier,
        }
    }
}
