use std;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use opcua_types::*;
use opcua_types::status_codes::StatusCode;
use opcua_types::status_codes::StatusCode::*;
use opcua_types::service_types::*;

use address_space::{AttributeGetter, AttributeSetter};
use address_space::node::Node;

// This should match size of AttributeId
const NUM_ATTRIBUTES: usize = 22;

/// This is a sanity saving macro that adds Node trait methods to all types that have a base
/// member.
#[macro_export]
macro_rules! find_attribute_mandatory {
    ( $sel:expr, $attr: ident ) => {
        let attribute_id = AttributeId::$attr;
        if let Some(attribute) = $sel.find_attribute(&attribute_id) {
            if let Attribute::$attr(value) = attribute.clone() {
                return value;
            }
        }
        panic!("Mandatory attribute {:?} is missing", attribute_id);
    }
}

macro_rules! is_valid_value_type {
    ( $data_value: expr, $variant_type: ident ) => {
        if let Some(ref value) = $data_value.value {
            if let Variant::$variant_type(_) = *value {
                true
            } else {
                error!("Cannot set data value as its value is of the wrong type");
                false
            }
        }
        else {
            error!("Cannot set data value as its value is None");
            false
        }
    }
}

/// Base is the functionality that all kinds of nodes need. Part 3, diagram B.4
pub struct Base {
    /// Attributes
    attributes: Vec<Option<DataValue>>,
    /// Attribute getters - if None, handled by Base
    attribute_getters: HashMap<AttributeId, Arc<Mutex<AttributeGetter + Send>>>,
    /// Attribute setters - if None, handled by Base
    attribute_setters: HashMap<AttributeId, Arc<Mutex<AttributeSetter + Send>>>,
}

impl Debug for Base {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // This impl will not write out the key, but it exists to keep structs happy
        // that contain a key as a field
        write!(f, "Base {{ base: {:?} }}", self.attributes)
    }
}

impl Node for Base {
    /// Returns the node class
    fn node_class(&self) -> NodeClass {
        let result = find_attribute_value_mandatory!(self, NodeClass, Int32);
        NodeClass::from_i32(result).unwrap()
    }

    fn node_id(&self) -> NodeId {
        let result = find_attribute_value_mandatory!(self, NodeId, NodeId);
        result.as_ref().clone()
    }

    fn browse_name(&self) -> QualifiedName {
        let result = find_attribute_value_mandatory!(self, BrowseName, QualifiedName);
        result.as_ref().clone()
    }

    fn display_name(&self) -> LocalizedText {
        let result = find_attribute_value_mandatory!(self, DisplayName, LocalizedText);
        result.as_ref().clone()
    }

    fn description(&self) -> Option<LocalizedText> {
        let result = find_attribute_value_optional!(self, Description, LocalizedText);
        if result.is_none() {
            None
        } else {
            Some(result.unwrap().as_ref().clone())
        }
    }

    fn write_mask(&self) -> Option<UInt32> {
        find_attribute_value_optional!(self, WriteMask, UInt32)
    }

    fn set_write_mask(&mut self, write_mask: UInt32) {
        let _ = self.set_attribute(AttributeId::WriteMask, DataValue::new(write_mask as UInt32));
    }

    fn user_write_mask(&self) -> Option<UInt32> {
        find_attribute_value_optional!(self, UserWriteMask, UInt32)
    }

    fn set_user_write_mask(&mut self, write_mask: UInt32) {
        let _ = self.set_attribute(AttributeId::UserWriteMask, DataValue::new(write_mask as UInt32));
    }

    fn find_attribute(&self, attribute_id: AttributeId) -> Option<DataValue> {
        if let Some(getter) = self.attribute_getters.get(&attribute_id) {
            let mut getter = getter.lock().unwrap();
            let value = getter.get(self.node_id(), attribute_id);
            if value.is_ok() {
                value.unwrap()
            } else {
                None
            }
        } else {
            let attribute_idx = Self::attribute_idx(attribute_id);
            if attribute_idx >= self.attributes.len() {
                warn!("Attribute id {:?} is out of range and invalid", attribute_id);
                None
            } else {
                self.attributes[attribute_idx].clone()
            }
        }
    }

    fn set_attribute(&mut self, attribute_id: AttributeId, value: DataValue) -> Result<(), StatusCode> {
        // Check the type of the datavalue
        let type_is_valid = match attribute_id {
            AttributeId::NodeId | AttributeId::NodeClass => {
                false
            }
            AttributeId::BrowseName => {
                is_valid_value_type!(value, String)
            }
            AttributeId::DisplayName | AttributeId::Description | AttributeId::InverseName => {
                is_valid_value_type!(value, LocalizedText)
            }
            AttributeId::WriteMask | AttributeId::UserWriteMask => {
                is_valid_value_type!(value, UInt32)
            }
            AttributeId::IsAbstract | AttributeId::Symmetric | AttributeId::ContainsNoLoops | AttributeId::Historizing | AttributeId::Executable | AttributeId::UserExecutable => {
                is_valid_value_type!(value, Boolean)
            }
            AttributeId::EventNotifier | AttributeId::AccessLevel | AttributeId::UserAccessLevel => {
                is_valid_value_type!(value, Byte)
            }
            AttributeId::DataType => {
                is_valid_value_type!(value, NodeId)
            }
            AttributeId::ValueRank => {
                is_valid_value_type!(value, Int32)
            }
            AttributeId::ArrayDimensions => {
                if !is_valid_value_type!(value, Array) {
                    false
                } else {
                    if let &Variant::Array(ref array) = value.value.as_ref().unwrap() {
                        // check that array of variants are all UInt32s
                        let non_u32_value = array.iter().find(|v| if let &Variant::UInt32(_) = v { false } else { true });
                        if non_u32_value.is_some() {
                            error!("Array contains non UInt32 values, cannot use as array dimensions");
                            false
                        } else {
                            true
                        }
                    } else {
                        panic!("The value should be an Array");
                    }
                }
            }
            AttributeId::MinimumSamplingInterval => {
                is_valid_value_type!(value, Double)
            }
            AttributeId::Value => {
                // Anything is permitted
                true
            }
        };
        if !type_is_valid {
            Err(BadTypeMismatch)
        } else {
            let attribute_idx = Self::attribute_idx(attribute_id);
            if let Some(setter) = self.attribute_setters.get(&attribute_id) {
                let mut setter = setter.lock().unwrap();
                setter.set(self.node_id(), attribute_id, value)?;
            } else {
                self.attributes[attribute_idx] = Some(value);
            }
            Ok(())
        }
    }
}

impl Base {
    pub fn new(node_class: NodeClass, node_id: &NodeId, browse_name: &str, display_name: &str, description: &str, mut attributes: Vec<(AttributeId, Variant)>) -> Base {
        // Mandatory attributes
        let mut attributes_to_add = vec![
            (AttributeId::NodeClass, Variant::Int32(node_class as Int32)),
            (AttributeId::NodeId, Variant::new(node_id.clone())),
            (AttributeId::DisplayName, Variant::new(LocalizedText::new("", display_name))),
            (AttributeId::BrowseName, Variant::new(QualifiedName::new(0, browse_name))),
            (AttributeId::Description, Variant::new(LocalizedText::new("", description))),
            (AttributeId::WriteMask, Variant::UInt32(0)),
            (AttributeId::UserWriteMask, Variant::UInt32(0)),
        ];
        attributes_to_add.append(&mut attributes);

        // Make attributes from their initial values
        let now = DateTime::now();
        let mut attributes = vec![None; NUM_ATTRIBUTES];
        for (attribute_id, value) in attributes_to_add {
            let attribute_idx = Base::attribute_idx(attribute_id);
            attributes[attribute_idx] = Some(DataValue {
                value: Some(value),
                status: Some(Good),
                server_timestamp: Some(now.clone()),
                server_picoseconds: Some(0),
                source_timestamp: Some(now.clone()),
                source_picoseconds: Some(0),
            });
        }

        Base {
            attributes,
            attribute_getters: HashMap::new(),
            attribute_setters: HashMap::new(),
        }
    }

    pub fn set_attribute_getter(&mut self, attribute_id: AttributeId, getter: Arc<Mutex<AttributeGetter + Send>>) {
        self.attribute_getters.insert(attribute_id, getter);
    }

    pub fn set_attribute_setter(&mut self, attribute_id: AttributeId, setter: Arc<Mutex<AttributeSetter + Send>>) {
        self.attribute_setters.insert(attribute_id, setter);
    }

    pub fn set_attribute_value(&mut self, attribute_id: AttributeId, value: Variant, server_timestamp: &DateTime, source_timestamp: &DateTime) -> Result<(), StatusCode> {
        self.set_attribute(attribute_id, DataValue {
            value: Some(value),
            status: Some(Good),
            server_timestamp: Some(server_timestamp.clone()),
            server_picoseconds: Some(0),
            source_timestamp: Some(source_timestamp.clone()),
            source_picoseconds: Some(0),
        })
    }

    #[inline]
    pub fn attribute_idx(attribute_id: AttributeId) -> usize {
        attribute_id as usize - 1
    }
}
