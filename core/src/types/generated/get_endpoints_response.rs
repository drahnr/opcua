// This file was autogenerated from Opc.Ua.Types.bsd.xml

use std::io::{Read, Write, Result};

use types::*;
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct GetEndpointsResponse {
    pub response_header: ResponseHeader,
    pub endpoints: Option<Vec<EndpointDescription>>,
}

impl BinaryEncoder<GetEndpointsResponse> for GetEndpointsResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.endpoints);
        size
    }
    
    fn encode<S: Write>(&self, stream: &mut S) -> Result<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.endpoints)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> Result<GetEndpointsResponse> {
        let response_header = ResponseHeader::decode(stream)?;
        let endpoints: Option<Vec<EndpointDescription>> = read_array(stream)?;
        Ok(GetEndpointsResponse {
            response_header: response_header,
            endpoints: endpoints,
        })
    }
}