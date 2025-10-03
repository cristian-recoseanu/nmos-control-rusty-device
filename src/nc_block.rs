use itertools::Itertools;
use serde_json::Value;
use serde_json::json;
use tokio::sync::mpsc;
use crate::nc_object::NcObject;
use crate::data_types::{ElementId, IdArgs, IdArgsValue, NcPropertyChangeType, PropertyChangedEvent, PropertyChangedEventData, NcBlockMemberDescriptor};

#[derive(Debug, Clone)]
pub struct NcBlock {
    pub base: NcObject,
    pub is_root: bool,
    pub enabled: bool,
    pub members: Vec<NcObject>
}

impl NcBlock {
    pub fn new(is_root: bool, class_id: Vec<u32>, oid: u64, constant_oid: bool, owner: Option<u64>, role: &str, user_label: Option<&str>, enabled: bool, notifier: mpsc::UnboundedSender<PropertyChangedEvent>) -> Self {
        NcBlock {
            base: NcObject::new(class_id, oid, constant_oid, owner, role, user_label, notifier),
            is_root,
            enabled,
            members: Vec::new()
        }
    }

    pub fn add_member(&mut self, member: NcObject) {
        self.members.push(member);

        let members_descriptors: Vec<NcBlockMemberDescriptor> = self.members
            .iter()
            .map(|p| NcBlockMemberDescriptor {
                role: p.role.clone(),
                oid: p.oid,
                constant_oid: p.constant_oid,
                class_id: p.class_id.clone(),
                user_label: p.user_label.clone().unwrap_or_default(),
                owner: self.base.oid
            })
            .collect();

        let _ = self.base.notifier.send(PropertyChangedEvent::new(self.base.oid, PropertyChangedEventData {
            property_id: ElementId { level: 2, index: 2 }, change_type: NcPropertyChangeType::ValueChanged, value: serde_json::json!(members_descriptors), sequence_item_index: None }));
    }

    pub fn find_member(&self, oid: u64) -> Option<NcObject> {
        if let Some(member) = self.members.iter().find(|obj| obj.oid == oid) {
            Some(member.clone())
        } else {
            for m in &self.members {
                if let Some(found) = m.find_member(oid) {
                    return Some(found);
                }
            }
            None
        }
    }

    pub fn generate_members_descriptors(&self) -> Vec<NcBlockMemberDescriptor> {
        self.members
            .iter()
            .map(|p| NcBlockMemberDescriptor {
                role: p.role.clone(),
                oid: p.oid,
                constant_oid: p.constant_oid,
                class_id: p.class_id.clone(),
                user_label: p.user_label.clone().unwrap_or_default(),
                owner: self.base.oid
            })
            .collect()
    }

    pub fn get_property(&self, oid: u64, id_args: &IdArgs) -> (Option<String>, Value) {
        if oid == self.base.oid {
            match (id_args.id.level, id_args.id.index) {
                (2, 1) => (None, json!(self.enabled)),
                (2, 2) => (None, json!(self.generate_members_descriptors())),
                _ => self.base.get_property(oid, &id_args)
            }
        } else {
            let m = self.find_member(oid);
            if let Some(member) = m {
                member.get_property(oid, &id_args)
            } else {
                (Some("Member not found".to_string()), serde_json::json!(null))
            }
        }
    }

    pub fn set_property(&mut self, oid: u64, id_args_value: IdArgsValue) -> (Option<String>, bool) {
        if oid == self.base.oid {
            match id_args_value.id.level {
                2 => (Some("Could not find the property".to_string()), false),
                _ => self.base.set_property(oid, id_args_value)
            }
        } else {
            let m = self.find_member(oid);
            if let Some(mut member) = m {
                member.set_property(oid, id_args_value)
            } else {
                (Some("Member not found".to_string()), false)
            }
        }
    }

    pub fn invoke_method(&self, oid: u64, method_id: ElementId, args: Value) -> (Option<String>, Option<Value>) {
        if oid == self.base.oid {
            match (method_id.level, method_id.index) {
                (2, 4) => (None, Some(serde_json::json!(self.find_members_by_class_id(args)))),
                _ => self.base.invoke_method(oid, method_id, args)
            }
        } else {
            let m = self.find_member(oid);
            if let Some(member) = m {
                member.invoke_method(oid, method_id, args)
            } else {
                (Some("Member not found".to_string()), None)
            }
        }
    }

    pub fn find_members_by_class_id(&self, args: Value) -> Vec<NcBlockMemberDescriptor> {
        let class_id: Option<Vec<u64>> = args.get("classId")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_u64()).collect());

        let recurse = args.get("recurse").and_then(|v| v.as_bool()).unwrap_or(false);
        let include_derived = args.get("includeDerived").and_then(|v| v.as_bool()).unwrap_or(false);

        let Some(class_id) = class_id else {
            return Vec::new();
        };

        let class_id_str = class_id.iter().join(".");

        // small helper to compare class_ids
        let matches_class_id = |cid: &[u32]| {
            let cid_str = cid.iter().join(".");
            if include_derived {
                cid_str.starts_with(&class_id_str)
            } else {
                cid_str == class_id_str
            }
        };

        // small helper to build descriptor
        let make_descriptor = |role: String, oid, constant_oid, class_id: Vec<u32>, user_label: Option<String>, owner| {
            NcBlockMemberDescriptor {
                role,
                oid,
                constant_oid,
                class_id,
                user_label: user_label.unwrap_or_default(),
                owner,
            }
        };

        let mut results: Vec<_> = self.members.iter()
            .filter(|m| matches_class_id(&m.class_id))
            .map(|m| make_descriptor(
                m.role.clone(),
                m.oid,
                m.constant_oid,
                m.class_id.clone(),
                m.user_label.clone(),
                self.base.oid,
            ))
            .collect();

        if recurse {
            for member in &self.members {
                results.extend(member.find_members_by_class_id(args.clone()));
            }
        }

        if self.is_root && matches_class_id(&self.base.class_id) {
            results.push(make_descriptor(
                self.base.role.clone(),
                self.base.oid,
                self.base.constant_oid,
                self.base.class_id.clone(),
                self.base.user_label.clone(),
                self.base.oid,
            ));
        }

        results
    }
}