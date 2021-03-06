// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use string::UAString;
use node_id::NodeId;
use basic_types::LocalizedText;

/// An argument for a method.
#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: UAString,
    pub data_type: NodeId,
    pub value_rank: Int32,
    pub array_dimensions: Option<Vec<UInt32>>,
    pub description: LocalizedText,
}

impl MessageInfo for Argument {
    fn object_id(&self) -> ObjectId {
        ObjectId::Argument_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<Argument> for Argument {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.name.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();
        size += byte_len_array(&self.array_dimensions);
        size += self.description.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.name.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        size += write_array(stream, &self.array_dimensions)?;
        size += self.description.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let name = UAString::decode(stream)?;
        let data_type = NodeId::decode(stream)?;
        let value_rank = Int32::decode(stream)?;
        let array_dimensions: Option<Vec<UInt32>> = read_array(stream)?;
        let description = LocalizedText::decode(stream)?;
        Ok(Argument {
            name,
            data_type,
            value_rank,
            array_dimensions,
            description,
        })
    }
}
