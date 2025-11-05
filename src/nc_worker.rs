use serde_json::{Value, json};
use std::any::Any;
use tokio::sync::mpsc;

use crate::data_types::{
    IdArgs, IdArgsValue, NcClassDescriptor, NcDescriptor, NcElementId, NcMethodStatus,
    NcPropertyChangeType, NcPropertyDescriptor, PropertyChangedEvent, PropertyChangedEventData,
};
use crate::nc_object::{NcMember, NcObject};

#[derive(Debug, Clone)]
pub struct NcWorker {
    pub base: NcObject,
    pub enabled: bool,
}

impl NcWorker {
    pub fn get_class_descriptor(include_inherited: bool) -> NcClassDescriptor {
        let mut desc = NcClassDescriptor {
            base: crate::data_types::NcDescriptor {
                description: Some("NcWorker class descriptor".to_string()),
            },
            class_id: vec![1, 2],
            name: "NcWorker".to_string(),
            fixed_role: None,
            properties: vec![NcPropertyDescriptor {
                base: NcDescriptor {
                    description: Some("Indicates if the worker is enabled".to_string()),
                },
                id: NcElementId { level: 2, index: 1 },
                name: "enabled".to_string(),
                type_name: Some("NcBoolean".to_string()),
                is_read_only: false,
                is_nullable: false,
                is_sequence: false,
                is_deprecated: false,
                constraints: None,
            }],
            methods: vec![],
            events: vec![],
        };

        if include_inherited {
            let base_desc = crate::nc_object::NcObject::get_class_descriptor(true);
            desc.properties.extend(base_desc.properties);
            desc.methods.extend(base_desc.methods);
            desc.events.extend(base_desc.events);
        }

        desc
    }
}

impl NcWorker {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        class_id: Vec<u32>,
        oid: u64,
        constant_oid: bool,
        owner: Option<u64>,
        role: &str,
        user_label: Option<&str>,
        touchpoints: Option<Vec<crate::data_types::NcTouchpoint>>,
        runtime_property_constraints: Option<Vec<crate::data_types::NcPropertyConstraints>>,
        notifier: mpsc::UnboundedSender<crate::data_types::PropertyChangedEvent>,
    ) -> Self {
        NcWorker {
            base: NcObject::new(
                class_id,
                oid,
                constant_oid,
                owner,
                role,
                user_label,
                touchpoints,
                runtime_property_constraints,
                notifier,
            ),
            enabled: true,
        }
    }
}

impl NcMember for NcWorker {
    fn member_type(&self) -> &'static str {
        "NcWorker"
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

    fn get_property(&self, oid: u64, id_args: &IdArgs) -> (Option<String>, Value, NcMethodStatus) {
        // Handle NcWorker specific properties
        if id_args.id.level == 2 && id_args.id.index == 1 {
            // 2.1 for enabled property
            return (None, json!(self.enabled), NcMethodStatus::Ok);
        }

        // Delegate to base class for other properties
        self.base.get_property(oid, id_args)
    }

    fn set_property(
        &mut self,
        oid: u64,
        id_args_value: IdArgsValue,
    ) -> (Option<String>, NcMethodStatus) {
        // Handle NcWorker specific properties
        if id_args_value.id.level == 2 && id_args_value.id.index == 1 {
            // 2.1 for enabled property
            if let Value::Bool(enabled) = id_args_value.value {
                let old_value = self.enabled;
                self.enabled = enabled;

                // Notify about the property change
                if old_value != enabled {
                    let _ = self.base.notifier.send(PropertyChangedEvent::new(
                        oid,
                        PropertyChangedEventData {
                            property_id: id_args_value.id,
                            change_type: NcPropertyChangeType::ValueChanged,
                            value: json!(enabled),
                            sequence_item_index: None,
                        },
                    ));
                }

                return (None, NcMethodStatus::Ok);
            } else {
                return (
                    Some("Invalid arguments".to_string()),
                    NcMethodStatus::ParameterError,
                );
            }
        }

        // Delegate to base class for other properties
        self.base.set_property(oid, id_args_value)
    }

    fn invoke_method(
        &self,
        oid: u64,
        method_id: NcElementId,
        args: Value,
    ) -> (Option<String>, Option<Value>, NcMethodStatus) {
        // No methods specific to NcWorker, delegate to base class
        self.base.invoke_method(oid, method_id, args)
    }
}
