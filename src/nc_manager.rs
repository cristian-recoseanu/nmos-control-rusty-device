use crate::data_types::{IdArgs, IdArgsValue, NcClassDescriptor, NcElementId, NcMethodStatus};
use crate::nc_object::{NcMember, NcObject};
use serde_json::Value;
use std::any::Any;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct NcManager {
    pub base: NcObject,
}

impl NcManager {
    pub fn get_class_descriptor(include_inherited: bool) -> NcClassDescriptor {
        let mut desc = NcClassDescriptor {
            base: crate::data_types::NcDescriptor {
                description: Some("NcManager class descriptor".to_string()),
            },
            class_id: vec![1, 3],
            name: "NcManager".to_string(),
            fixed_role: None,
            properties: vec![],
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

impl NcManager {
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
        NcManager {
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
        }
    }
}

impl NcMember for NcManager {
    fn member_type(&self) -> &'static str {
        "NcManager"
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
        self.base.get_property(oid, id_args)
    }

    fn set_property(
        &mut self,
        oid: u64,
        id_args_value: IdArgsValue,
    ) -> (Option<String>, NcMethodStatus) {
        self.base.set_property(oid, id_args_value)
    }

    fn invoke_method(
        &self,
        oid: u64,
        method_id: NcElementId,
        args: Value,
    ) -> (Option<String>, Option<Value>, NcMethodStatus) {
        self.base.invoke_method(oid, method_id, args)
    }
}
