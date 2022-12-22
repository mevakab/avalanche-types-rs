// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMinimumHeightResponse {
    #[prost(uint64, tag="1")]
    pub height: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentHeightResponse {
    #[prost(uint64, tag="1")]
    pub height: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSubnetIdRequest {
    #[prost(bytes="bytes", tag="1")]
    pub chain_id: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSubnetIdResponse {
    #[prost(bytes="bytes", tag="1")]
    pub subnet_id: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorSetRequest {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(bytes="bytes", tag="2")]
    pub subnet_id: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(bytes="bytes", tag="1")]
    pub node_id: ::prost::bytes::Bytes,
    #[prost(uint64, tag="2")]
    pub weight: u64,
    #[prost(bytes="bytes", tag="3")]
    pub public_key: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorSetResponse {
    #[prost(message, repeated, tag="1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
}
/// Encoded file descriptor set for the `validatorstate` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb7, 0x12, 0x0a, 0x24, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x74,
    0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x73, 0x74,
    0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x76, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x6f, 0x72, 0x73, 0x74, 0x61, 0x74, 0x65, 0x1a, 0x1b, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x65, 0x6d, 0x70, 0x74, 0x79,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x32, 0x0a, 0x18, 0x47, 0x65, 0x74, 0x4d, 0x69, 0x6e,
    0x69, 0x6d, 0x75, 0x6d, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x22, 0x32, 0x0a, 0x18, 0x47, 0x65,
    0x74, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x22, 0x2f,
    0x0a, 0x12, 0x47, 0x65, 0x74, 0x53, 0x75, 0x62, 0x6e, 0x65, 0x74, 0x49, 0x44, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x63, 0x68, 0x61, 0x69, 0x6e, 0x49, 0x64, 0x22,
    0x32, 0x0a, 0x13, 0x47, 0x65, 0x74, 0x53, 0x75, 0x62, 0x6e, 0x65, 0x74, 0x49, 0x44, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x75, 0x62, 0x6e, 0x65, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x73, 0x75, 0x62, 0x6e, 0x65,
    0x74, 0x49, 0x64, 0x22, 0x4d, 0x0a, 0x16, 0x47, 0x65, 0x74, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x6f, 0x72, 0x53, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a,
    0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x68,
    0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x75, 0x62, 0x6e, 0x65, 0x74, 0x5f,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x73, 0x75, 0x62, 0x6e, 0x65, 0x74,
    0x49, 0x64, 0x22, 0x5b, 0x0a, 0x09, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x12,
    0x17, 0x0a, 0x07, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x06, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x77, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x77, 0x65, 0x69, 0x67, 0x68, 0x74,
    0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x22,
    0x54, 0x0a, 0x17, 0x47, 0x65, 0x74, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x53,
    0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x39, 0x0a, 0x0a, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19,
    0x2e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e,
    0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x52, 0x0a, 0x76, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x6f, 0x72, 0x73, 0x32, 0xf8, 0x02, 0x0a, 0x0e, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x6f, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x54, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x4d,
    0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x16, 0x2e, 0x67,
    0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x45,
    0x6d, 0x70, 0x74, 0x79, 0x1a, 0x28, 0x2e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72,
    0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x47, 0x65, 0x74, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d,
    0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x54,
    0x0a, 0x10, 0x47, 0x65, 0x74, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x48, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x12, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x1a, 0x28, 0x2e, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x47, 0x65, 0x74, 0x43,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x56, 0x0a, 0x0b, 0x47, 0x65, 0x74, 0x53, 0x75, 0x62, 0x6e, 0x65,
    0x74, 0x49, 0x44, 0x12, 0x22, 0x2e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x75, 0x62, 0x6e, 0x65, 0x74, 0x49, 0x44,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x23, 0x2e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x6f, 0x72, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x75, 0x62, 0x6e,
    0x65, 0x74, 0x49, 0x44, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x62, 0x0a, 0x0f,
    0x47, 0x65, 0x74, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x53, 0x65, 0x74, 0x12,
    0x26, 0x2e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x2e, 0x47, 0x65, 0x74, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x53, 0x65, 0x74,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x27, 0x2e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x6f, 0x72, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x47, 0x65, 0x74, 0x56, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x6f, 0x72, 0x53, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x42, 0x39, 0x5a, 0x37, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61,
    0x76, 0x61, 0x2d, 0x6c, 0x61, 0x62, 0x73, 0x2f, 0x61, 0x76, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x68,
    0x65, 0x67, 0x6f, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x70, 0x62, 0x2f, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x74, 0x61, 0x74, 0x65, 0x4a, 0xd4, 0x0a, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x32, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x04, 0x00, 0x25, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x4e, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x06, 0x00, 0x4e, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00,
    0x12, 0x04, 0x08, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x08,
    0x08, 0x16, 0x0a, 0x69, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x51, 0x1a,
    0x5c, 0x20, 0x47, 0x65, 0x74, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x48, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x69, 0x6e, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6d, 0x61, 0x6c, 0x0a, 0x20, 0x70, 0x72, 0x6f, 0x70,
    0x6f, 0x73, 0x61, 0x6c, 0x20, 0x77, 0x69, 0x6e, 0x64, 0x6f, 0x77, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x06, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0b, 0x17, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0b, 0x37, 0x4f, 0x0a, 0x4a, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x0d, 0x02, 0x51, 0x1a, 0x3d, 0x20, 0x47, 0x65, 0x74, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x48, 0x65, 0x69, 0x67, 0x68, 0x74, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x68, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x50, 0x2d, 0x63, 0x68, 0x61, 0x69,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x06,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0d, 0x17, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x37, 0x4f, 0x0a, 0x46, 0x0a,
    0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x02, 0x44, 0x1a, 0x39, 0x20, 0x47, 0x65, 0x74,
    0x53, 0x75, 0x62, 0x6e, 0x65, 0x74, 0x49, 0x44, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x75, 0x62, 0x6e, 0x65, 0x74, 0x49, 0x44, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20, 0x63, 0x68,
    0x61, 0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x0f, 0x06, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x12,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x2f, 0x42, 0x0a,
    0x7b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x03, 0x12, 0x02, 0x50, 0x1a, 0x6e, 0x20, 0x47,
    0x65, 0x74, 0x56, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f, 0x72, 0x53, 0x65, 0x74, 0x20, 0x72,
    0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x65, 0x69, 0x67, 0x68,
    0x74, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x44,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64,
    0x65, 0x64, 0x0a, 0x20, 0x73, 0x75, 0x62, 0x6e, 0x65, 0x74, 0x20, 0x61, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x50, 0x2d, 0x63, 0x68,
    0x61, 0x69, 0x6e, 0x20, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x12, 0x06, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x12, 0x16, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x12, 0x37, 0x4e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x15, 0x00,
    0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x15, 0x08, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x16, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x16, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x16, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x19, 0x00, 0x1b,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x19, 0x08, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1a, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1a, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1d, 0x00, 0x1f, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x1e, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x1e, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x21, 0x00, 0x23, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x21, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x22, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x22, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x22, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22,
    0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x25, 0x00, 0x28, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x25, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x26, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x26, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x26, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x12,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x27, 0x02, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x27, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x2a, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x02, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x2b, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12,
    0x03, 0x2c, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2c,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x09, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2c, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x2d, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x2d, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x30, 0x00, 0x32,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x30, 0x08, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x31, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x31, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x31, 0x15, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31,
    0x22, 0x23, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("validatorstate.tonic.rs");
// @@protoc_insertion_point(module)