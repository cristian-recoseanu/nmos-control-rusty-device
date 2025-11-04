use crate::data_types::{
    IdArgs, IdArgsValue, NcBlockMemberDescriptor, NcClassDescriptor, NcElementId,
    NcMethodDescriptor, NcMethodStatus, NcParameterDescriptor, NcPropertyChangeType,
    NcPropertyDescriptor, PropertyChangedEvent, PropertyChangedEventData,
};
use crate::nc_object::{NcMember, NcObject};
use itertools::Itertools;
use serde_json::{Value, json};
use std::any::Any;
use tokio::sync::mpsc;

pub struct NcBlock {
    pub base: NcObject,
    pub is_root: bool,
    pub enabled: bool,
    pub members: Vec<Box<dyn NcMember>>,
}

impl NcMember for NcBlock {
    fn member_type(&self) -> &'static str {
        "NcBlock"
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
        if oid == self.base.oid {
            match (id_args.id.level, id_args.id.index) {
                (2, 1) => (None, json!(self.enabled), NcMethodStatus::Ok),
                (2, 2) => (
                    None,
                    json!(self.generate_members_descriptors()),
                    NcMethodStatus::Ok,
                ),
                _ => self.base.get_property(oid, id_args),
            }
        } else if let Some(member) = self.find_member(oid) {
            member.get_property(oid, id_args)
        } else {
            (
                Some("Member not found".to_string()),
                json!(null),
                NcMethodStatus::BadOid,
            )
        }
    }

    fn set_property(
        &mut self,
        oid: u64,
        id_args_value: IdArgsValue,
    ) -> (Option<String>, NcMethodStatus) {
        if oid == self.base.oid {
            match id_args_value.id.level {
                2 => (
                    Some("Could not find the property".to_string()),
                    NcMethodStatus::PropertyNotImplemented,
                ),
                _ => self.base.set_property(oid, id_args_value),
            }
        } else if let Some(member) = self.find_member_mut(oid) {
            member.set_property(oid, id_args_value)
        } else {
            (Some("Member not found".to_string()), NcMethodStatus::BadOid)
        }
    }

    fn invoke_method(
        &self,
        oid: u64,
        method_id: NcElementId,
        args: Value,
    ) -> (Option<String>, Option<Value>, NcMethodStatus) {
        if oid == self.base.oid {
            match (method_id.level, method_id.index) {
                (1, 3) => {
                    match args.as_object() {
                        Some(args_obj) => {
                            match args_obj.get("id") {
                                Some(id_val) => {
                                    if let Some(id_obj) = id_val.as_object() {
                                        let level = id_obj
                                            .get("level")
                                            .and_then(|v| v.as_u64())
                                            .map(|v| v as u32);
                                        let index = id_obj
                                            .get("index")
                                            .and_then(|v| v.as_u64())
                                            .map(|v| v as u32);

                                        if let (Some(2), Some(2)) = (level, index) {
                                            // Handle members property (2p2)
                                            let index = match args_obj
                                                .get("index")
                                                .and_then(|v| v.as_u64())
                                            {
                                                Some(idx) => idx as usize,
                                                None => {
                                                    return (
                                                        Some("Invalid index argument".to_string()),
                                                        None,
                                                        NcMethodStatus::ParameterError,
                                                    );
                                                }
                                            };

                                            let members = self.generate_members_descriptors();

                                            // Check if index is within bounds
                                            if index >= members.len() {
                                                return (
                                                    Some(format!(
                                                        "Index {} out of bounds for sequence",
                                                        index
                                                    )),
                                                    None,
                                                    NcMethodStatus::IndexOutOfBounds,
                                                );
                                            }

                                            if let Some(member) = members.get(index) {
                                                return (
                                                    None,
                                                    Some(json!(member)),
                                                    NcMethodStatus::Ok,
                                                );
                                            }
                                        }
                                    }
                                }
                                None => {
                                    return (
                                        Some("Missing 'id' argument".to_string()),
                                        None,
                                        NcMethodStatus::ParameterError,
                                    );
                                }
                            }
                        }
                        None => {
                            return (
                                Some("Invalid arguments".to_string()),
                                None,
                                NcMethodStatus::ParameterError,
                            );
                        }
                    }
                    // If not the members property, delegate to base class
                    self.base.invoke_method(oid, method_id, args)
                }
                (1, 7) => {
                    match args.as_object() {
                        Some(args_obj) => {
                            match args_obj.get("id") {
                                Some(id_val) => {
                                    if let Some(id_obj) = id_val.as_object() {
                                        let level = id_obj
                                            .get("level")
                                            .and_then(|v| v.as_u64())
                                            .map(|v| v as u32);
                                        let index = id_obj
                                            .get("index")
                                            .and_then(|v| v.as_u64())
                                            .map(|v| v as u32);

                                        if let (Some(2), Some(2)) = (level, index) {
                                            // Handle members property (2p2)
                                            let members = self.generate_members_descriptors();
                                            return (
                                                None,
                                                Some(json!(members.len())),
                                                NcMethodStatus::Ok,
                                            );
                                        }
                                    }
                                }
                                None => {
                                    return (
                                        Some("Invalid id argument".to_string()),
                                        None,
                                        NcMethodStatus::ParameterError,
                                    );
                                }
                            }
                        }
                        None => {
                            return (
                                Some("Invalid arguments".to_string()),
                                None,
                                NcMethodStatus::ParameterError,
                            );
                        }
                    }
                    // If not the members property, delegate to base class
                    self.base.invoke_method(oid, method_id, args)
                }
                (2, 1) => (
                    None,
                    Some(json!(self.get_member_descriptors(args))),
                    NcMethodStatus::Ok,
                ), // 2m1
                (2, 2) => (
                    None,
                    Some(json!(self.find_members_by_path(args))),
                    NcMethodStatus::Ok,
                ), // 2m2
                (2, 3) => (
                    None,
                    Some(json!(self.find_members_by_role(args))),
                    NcMethodStatus::Ok,
                ), // 2m3
                (2, 4) => (
                    None,
                    Some(json!(self.find_members_by_class_id(args))),
                    NcMethodStatus::Ok,
                ), // 2m4
                _ => self.base.invoke_method(oid, method_id, args),
            }
        } else if let Some(member) = self.find_member(oid) {
            member.invoke_method(oid, method_id, args)
        } else {
            (
                Some("Member not found".to_string()),
                None,
                NcMethodStatus::BadOid,
            )
        }
    }
}

impl NcBlock {
    pub fn get_class_descriptor(include_inherited: bool) -> NcClassDescriptor {
        let mut descriptor = NcClassDescriptor {
            base: crate::data_types::NcDescriptor { description: Some("NcBlock class descriptor".to_string()) },
            class_id: vec![1, 1],
            name: "NcBlock".to_string(),
            fixed_role: None,
            properties: vec![
                NcPropertyDescriptor {
                    base: crate::data_types::NcDescriptor { description: Some("TRUE if block is functional".to_string()) },
                    id: NcElementId { level: 2, index: 1 },
                    name: "enabled".to_string(),
                    type_name: Some("NcBoolean".to_string()),
                    is_read_only: true,
                    is_nullable: false,
                    is_sequence: false,
                    is_deprecated: false,
                    constraints: None,
                },
                NcPropertyDescriptor {
                    base: crate::data_types::NcDescriptor { description: Some("Descriptors of this block's members".to_string()) },
                    id: NcElementId { level: 2, index: 2 },
                    name: "members".to_string(),
                    type_name: Some("NcBlockMemberDescriptor".to_string()),
                    is_read_only: true,
                    is_nullable: false,
                    is_sequence: true,
                    is_deprecated: false,
                    constraints: None,
                },
            ],
            methods: vec![
                NcMethodDescriptor {
                    base: crate::data_types::NcDescriptor { description: Some("Gets descriptors of members of the block".to_string()) },
                    id: NcElementId { level: 2, index: 1 },
                    name: "GetMemberDescriptors".to_string(),
                    result_datatype: "NcMethodResultBlockMemberDescriptors".to_string(),
                    parameters: vec![NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor { description: Some("If recurse is set to true, nested members can be retrieved".to_string()) },
                        name: "recurse".to_string(),
                        type_name: Some("NcBoolean".to_string()),
                        is_nullable: false,
                        is_sequence: false,
                        constraints: None,
                    }],
                    is_deprecated: false,
                },
                NcMethodDescriptor {
                    base: crate::data_types::NcDescriptor { description: Some("Finds member(s) by path".to_string()) },
                    id: NcElementId { level: 2, index: 2 },
                    name: "FindMembersByPath".to_string(),
                    result_datatype: "NcMethodResultBlockMemberDescriptors".to_string(),
                    parameters: vec![NcParameterDescriptor {
                        base: crate::data_types::NcDescriptor { description: Some("Relative path to search for (MUST not include the role of the block targeted by oid)".to_string()) },
                        name: "path".to_string(),
                        type_name: Some("NcRolePath".to_string()),
                        is_nullable: false,
                        is_sequence: false,
                        constraints: None,
                    }],
                    is_deprecated: false,
                },
                NcMethodDescriptor {
                    base: crate::data_types::NcDescriptor { description: Some("Finds members with given role name or fragment".to_string()) },
                    id: NcElementId { level: 2, index: 3 },
                    name: "FindMembersByRole".to_string(),
                    result_datatype: "NcMethodResultBlockMemberDescriptors".to_string(),
                    parameters: vec![
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("Role text to search for".to_string()) },
                            name: "role".to_string(),
                            type_name: Some("NcString".to_string()),
                            is_nullable: false,
                            is_sequence: false,
                            constraints: None,
                        },
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("Signals if the comparison should be case sensitive".to_string()) },
                            name: "caseSensitive".to_string(),
                            type_name: Some("NcBoolean".to_string()),
                            is_nullable: false,
                            is_sequence: false,
                            constraints: None,
                        },
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("TRUE to only return exact matches".to_string()) },
                            name: "matchWholeString".to_string(),
                            type_name: Some("NcBoolean".to_string()),
                            is_nullable: false,
                            is_sequence: false,
                            constraints: None,
                        },
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("TRUE to search nested blocks".to_string()) },
                            name: "recurse".to_string(),
                            type_name: Some("NcBoolean".to_string()),
                            is_nullable: false,
                            is_sequence: false,
                            constraints: None,
                        },
                    ],
                    is_deprecated: false,
                },
                NcMethodDescriptor {
                    base: crate::data_types::NcDescriptor { description: Some("Finds members with given class id".to_string()) },
                    id: NcElementId { level: 2, index: 4 },
                    name: "FindMembersByClassId".to_string(),
                    result_datatype: "NcMethodResultBlockMemberDescriptors".to_string(),
                    parameters: vec![
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("Class id to search for".to_string()) },
                            name: "classId".to_string(),
                            type_name: Some("NcClassId".to_string()),
                            is_nullable: false,
                            is_sequence: false,
                            constraints: None,
                        },
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("If TRUE it will also include derived class descriptors".to_string()) },
                            name: "includeDerived".to_string(),
                            type_name: Some("NcBoolean".to_string()),
                            is_nullable: false,
                            is_sequence: false,
                            constraints: None,
                        },
                        NcParameterDescriptor {
                            base: crate::data_types::NcDescriptor { description: Some("TRUE to search nested blocks".to_string()) },
                            name: "recurse".to_string(),
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
            let base_desc = crate::nc_object::NcObject::get_class_descriptor(true);
            descriptor.properties.extend(base_desc.properties);
            descriptor.methods.extend(base_desc.methods);
            descriptor.events.extend(base_desc.events);
        }

        descriptor
    }
}

#[allow(clippy::too_many_arguments)]
impl NcBlock {
    pub fn new(
        is_root: bool,
        class_id: Vec<u32>,
        oid: u64,
        constant_oid: bool,
        owner: Option<u64>,
        role: &str,
        user_label: Option<&str>,
        enabled: bool,
        touchpoints: Option<Vec<crate::data_types::NcTouchpoint>>,
        runtime_property_constraints: Option<Vec<crate::data_types::NcPropertyConstraints>>,
        notifier: mpsc::UnboundedSender<PropertyChangedEvent>,
    ) -> Self {
        NcBlock {
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
            is_root,
            enabled,
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: Box<dyn NcMember>) {
        self.members.push(member);

        let members_descriptors = self.generate_members_descriptors();

        let _ = self.base.notifier.send(PropertyChangedEvent::new(
            self.base.oid,
            PropertyChangedEventData {
                property_id: NcElementId { level: 2, index: 2 },
                change_type: NcPropertyChangeType::ValueChanged,
                value: json!(members_descriptors),
                sequence_item_index: None,
            },
        ));
    }

    pub fn find_member(&self, oid: u64) -> Option<&dyn NcMember> {
        for member in &self.members {
            if member.get_oid() == oid {
                return Some(member.as_ref());
            }
            if member.member_type() == "NcBlock"
                && let Some(block) = member.as_any().downcast_ref::<NcBlock>()
                && let Some(found) = block.find_member(oid)
            {
                return Some(found);
            }
        }
        None
    }

    pub fn find_member_mut(&mut self, oid: u64) -> Option<&mut dyn NcMember> {
        for member in &mut self.members {
            if member.get_oid() == oid {
                return Some(member.as_mut());
            }
            if member.member_type() == "NcBlock"
                && let Some(block) = member.as_any_mut().downcast_mut::<NcBlock>()
                && let Some(found) = block.find_member_mut(oid)
            {
                return Some(found);
            }
        }
        None
    }

    pub fn generate_members_descriptors(&self) -> Vec<NcBlockMemberDescriptor> {
        self.members
            .iter()
            .map(|m| NcBlockMemberDescriptor {
                base: crate::data_types::NcDescriptor { description: None },
                role: m.get_role().to_owned(),
                oid: m.get_oid(),
                constant_oid: m.get_constant_oid(),
                class_id: m.get_class_id().to_vec(),
                user_label: m.get_user_label().unwrap_or_default().to_owned(),
                owner: self.base.oid,
            })
            .collect()
    }

    pub fn make_member_descriptor(m: &dyn NcMember, owner: u64) -> NcBlockMemberDescriptor {
        NcBlockMemberDescriptor {
            base: crate::data_types::NcDescriptor { description: None },
            role: m.get_role().to_owned(),
            oid: m.get_oid(),
            constant_oid: m.get_constant_oid(),
            class_id: m.get_class_id().to_vec(),
            user_label: m.get_user_label().unwrap_or_default().to_owned(),
            owner,
        }
    }

    pub fn get_member_descriptors(&self, args: Value) -> Vec<NcBlockMemberDescriptor> {
        let recurse = args
            .get("recurse")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let mut results: Vec<_> = self
            .members
            .iter()
            .map(|m| NcBlock::make_member_descriptor(m.as_ref(), self.base.oid))
            .collect();

        if recurse {
            for member in &self.members {
                if let Some(block) = member.as_any().downcast_ref::<NcBlock>() {
                    results.extend(block.get_member_descriptors(args.clone()));
                }
            }
        }

        results
    }

    pub fn find_members_by_path(&self, args: Value) -> Vec<NcBlockMemberDescriptor> {
        let Some(path_array) = args.get("path").and_then(|v| v.as_array()) else {
            return Vec::new();
        };

        let segments: Vec<String> = path_array
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.trim().to_string()))
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() {
            return Vec::new();
        }

        self.find_members_by_path_recursive(&segments)
    }

    fn find_members_by_path_recursive(&self, segments: &[String]) -> Vec<NcBlockMemberDescriptor> {
        if segments.is_empty() {
            return Vec::new();
        }

        let first = &segments[0];
        let rest = &segments[1..];

        let mut results = Vec::new();

        for member in &self.members {
            if member.get_role() == first {
                if rest.is_empty() {
                    // Last segment → found match
                    results.push(NcBlock::make_member_descriptor(
                        member.as_ref(),
                        self.base.oid,
                    ));
                } else if let Some(block) = member.as_any().downcast_ref::<NcBlock>() {
                    // Recurse into nested NcBlock
                    results.extend(block.find_members_by_path_recursive(rest));
                }
            }
        }

        results
    }

    pub fn find_members_by_role(&self, args: Value) -> Vec<NcBlockMemberDescriptor> {
        let role = args
            .get("role")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if role.is_empty() {
            return Vec::new();
        }

        let case_sensitive = args
            .get("caseSensitive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let match_whole = args
            .get("matchWholeString")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let recurse = args
            .get("recurse")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Prepare normalized role if not case-sensitive
        let search_role = if case_sensitive {
            role.clone()
        } else {
            role.to_lowercase()
        };

        // Closure for matching logic
        let matches_role = |r: &str| {
            if case_sensitive {
                if match_whole {
                    r == role
                } else {
                    r.contains(&role)
                }
            } else {
                let r_lower = r.to_lowercase();
                if match_whole {
                    r_lower == search_role
                } else {
                    r_lower.contains(&search_role)
                }
            }
        };

        let mut results: Vec<_> = self
            .members
            .iter()
            .filter(|m| matches_role(m.get_role()))
            .map(|m| NcBlock::make_member_descriptor(m.as_ref(), self.base.oid))
            .collect();

        if recurse {
            for member in &self.members {
                if let Some(block) = member.as_any().downcast_ref::<NcBlock>() {
                    results.extend(block.find_members_by_role(args.clone()));
                }
            }
        }

        results
    }

    pub fn find_members_by_class_id(&self, args: Value) -> Vec<NcBlockMemberDescriptor> {
        let class_id: Option<Vec<u32>> =
            args.get("classId").and_then(|v| v.as_array()).map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_u64().map(|x| x as u32))
                    .collect()
            });

        let recurse = args
            .get("recurse")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let include_derived = args
            .get("includeDerived")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let Some(class_id) = class_id else {
            return Vec::new();
        };

        let class_id_str = class_id.iter().join(".");

        let matches_class_id = |cid: &[u32]| {
            let cid_str = cid.iter().join(".");
            if include_derived {
                cid_str.starts_with(&class_id_str)
            } else {
                cid_str == class_id_str
            }
        };

        let mut results: Vec<_> = self
            .members
            .iter()
            .filter(|m| matches_class_id(m.get_class_id()))
            .map(|m| NcBlock::make_member_descriptor(m.as_ref(), self.base.oid))
            .collect();

        if recurse {
            for member in &self.members {
                if let Some(block) = member.as_any().downcast_ref::<NcBlock>() {
                    results.extend(block.find_members_by_class_id(args.clone()));
                }
            }
        }

        results
    }
}
