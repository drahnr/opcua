// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use service_types::impls::MessageInfo;
use node_ids::ObjectId;
use status_codes::StatusCode;
use basic_types::DiagnosticInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct ContentFilterElementResult {
    pub status_code: StatusCode,
    pub operand_status_codes: Option<Vec<StatusCode>>,
    pub operand_diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for ContentFilterElementResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::ContentFilterElementResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ContentFilterElementResult> for ContentFilterElementResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += byte_len_array(&self.operand_status_codes);
        size += byte_len_array(&self.operand_diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += write_array(stream, &self.operand_status_codes)?;
        size += write_array(stream, &self.operand_diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let operand_status_codes: Option<Vec<StatusCode>> = read_array(stream)?;
        let operand_diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream)?;
        Ok(ContentFilterElementResult {
            status_code,
            operand_status_codes,
            operand_diagnostic_infos,
        })
    }
}
