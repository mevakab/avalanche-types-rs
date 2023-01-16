use std::{
    collections::BTreeMap,
    convert::TryFrom,
    io::{self, Error, ErrorKind},
    str::FromStr,
};

use crate::codec::serde::hex_0x_bytes::Hex0xBytes;
use ethers_core::{
    abi::{Function, Param, ParamType, StateMutability, Token},
    types::{
        transaction::eip712::{
            EIP712Domain, Eip712, Eip712DomainType, TypedData, Types, EIP712_DOMAIN_TYPE_HASH,
            EIP712_DOMAIN_TYPE_HASH_WITH_SALT,
        },
        RecoveryMessage, Signature, H160, H256, U256,
    },
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use zerocopy::AsBytes;

/// ref. <https://eips.ethereum.org/EIPS/eip-712>
/// ref. <https://eips.ethereum.org/EIPS/eip-2770>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/contracts/src/forwarder/IForwarder.sol>
pub struct RelayTransactionRequestBuilder {
    /// EIP-712 domain name.
    /// Used for domain separator hash.
    /// ref. "ethers_core::types::transaction::eip712::Eip712::domain_separator"
    pub domain_name: String,
    /// EIP-712 domain version.
    /// Used for domain separator hash.
    /// ref. "ethers_core::types::transaction::eip712::Eip712::domain_separator"
    pub domain_version: String,
    /// EIP-712 domain chain id.
    /// Used for domain separator hash.
    /// ref. "ethers_core::types::transaction::eip712::Eip712::domain_separator"
    pub domain_chain_id: U256,
    /// EIP-712 domain verifying contract name.
    /// Used for domain separator hash.
    /// Address of the contract that will verify the signature.
    /// ref. "ethers_core::types::transaction::eip712::Eip712::domain_separator"
    pub domain_verifying_contract: H160,

    /// Forward request "from" field.
    /// ref. <https://eips.ethereum.org/EIPS/eip-2770>
    pub from: H160,
    /// Forward request "to" field.
    /// ref. <https://eips.ethereum.org/EIPS/eip-2770>
    pub to: H160,
    /// Forward request "value" field.
    /// ref. <https://eips.ethereum.org/EIPS/eip-2770>
    pub value: U256,
    /// Forward request "gas" field.
    /// ref. <https://eips.ethereum.org/EIPS/eip-2770>
    pub gas: U256,
    /// Forward request "nonce" field.
    /// ref. <https://eips.ethereum.org/EIPS/eip-2770>
    pub nonce: U256,
    /// Forward request "data" field.
    /// ref. <https://eips.ethereum.org/EIPS/eip-2770>
    pub data: Vec<u8>,
    /// Forward request "validUntil" field.
    /// ref. <https://eips.ethereum.org/EIPS/eip-2770>
    pub valid_until_time: U256,
}

impl RelayTransactionRequestBuilder {
    pub fn new() -> Self {
        Self {
            domain_name: String::new(),
            domain_version: String::new(),
            domain_chain_id: U256::zero(),
            domain_verifying_contract: H160::zero(),
            from: H160::zero(),
            to: H160::zero(),
            value: U256::zero(),
            gas: U256::zero(),
            nonce: U256::zero(),
            data: Vec::new(),
            valid_until_time: U256::zero(),
        }
    }

    #[must_use]
    pub fn domain_name(mut self, domain_name: impl Into<String>) -> Self {
        self.domain_name = domain_name.into();
        self
    }

    #[must_use]
    pub fn domain_version(mut self, domain_version: impl Into<String>) -> Self {
        self.domain_version = domain_version.into();
        self
    }

    #[must_use]
    pub fn domain_chain_id(mut self, domain_chain_id: impl Into<U256>) -> Self {
        self.domain_chain_id = domain_chain_id.into();
        self
    }

    #[must_use]
    pub fn domain_verifying_contract(mut self, domain_verifying_contract: impl Into<H160>) -> Self {
        self.domain_verifying_contract = domain_verifying_contract.into();
        self
    }

    #[must_use]
    pub fn from(mut self, from: impl Into<H160>) -> Self {
        self.from = from.into();
        self
    }

    #[must_use]
    pub fn to(mut self, to: impl Into<H160>) -> Self {
        self.to = to.into();
        self
    }

    #[must_use]
    pub fn value(mut self, value: impl Into<U256>) -> Self {
        self.value = value.into();
        self
    }

    #[must_use]
    pub fn gas(mut self, gas: impl Into<U256>) -> Self {
        self.gas = gas.into();
        self
    }

    #[must_use]
    pub fn nonce(mut self, nonce: impl Into<U256>) -> Self {
        self.nonce = nonce.into();
        self
    }

    #[must_use]
    pub fn data(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.data = data.into();
        self
    }

    #[must_use]
    pub fn valid_until_time(mut self, valid_until_time: impl Into<U256>) -> Self {
        self.valid_until_time = valid_until_time.into();
        self
    }

    pub fn build_typed_data(&self) -> TypedData {
        let mut message = BTreeMap::new();
        message.insert(
            String::from("from"),
            serde_json::to_value(self.from).unwrap(),
        );
        message.insert(String::from("to"), serde_json::to_value(self.to).unwrap());
        message.insert(
            String::from("value"),
            serde_json::to_value(self.value).unwrap(),
        );
        message.insert(String::from("gas"), serde_json::to_value(self.gas).unwrap());
        message.insert(
            String::from("nonce"),
            serde_json::to_value(self.nonce).unwrap(),
        );
        message.insert(
            String::from("data"),
            serde_json::to_value(hex::encode(&self.data)).unwrap(),
        );
        message.insert(
            String::from("validUntilTime"),
            serde_json::to_value(self.valid_until_time).unwrap(),
        );

        TypedData {
            domain: EIP712Domain {
                name: Some(self.domain_name.clone()),
                version: Some(self.domain_version.clone()),
                chain_id: Some(self.domain_chain_id),
                verifying_contract: Some(self.domain_verifying_contract),
                salt: None,
            },
            types: foward_request_types(),
            primary_type: "Message".to_string(),
            message,
        }
    }

    /// Builds and signs the typed data with the signer and returns the
    /// "RelayTransactionRequest" with the signature attached in the relay metadata.
    /// Use "serde_json::to_vec" to encode to "ethers_core::types::Bytes"
    /// and send the request via "eth_sendRawTransaction".
    pub async fn build_and_sign(
        &self,
        eth_signer: impl ethers_signers::Signer + Clone,
    ) -> io::Result<RelayTransactionRequest> {
        let forward_request = self.build_typed_data();
        RelayTransactionRequest::sign(forward_request, eth_signer).await
    }
}

/// ref. <https://eips.ethereum.org/EIPS/eip-2770>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/contracts/src/forwarder/IForwarder.sol>
pub fn foward_request_types() -> Types {
    let mut types = BTreeMap::new();
    types.insert(
        "EIP712Domain".to_string(),
        vec![
            Eip712DomainType {
                name: String::from("name"),
                r#type: String::from("string"),
            },
            Eip712DomainType {
                name: String::from("version"),
                r#type: String::from("string"),
            },
            Eip712DomainType {
                name: String::from("chainId"),
                r#type: String::from("uint256"),
            },
            Eip712DomainType {
                name: String::from("verifyingContract"),
                r#type: String::from("address"),
            },
        ],
    );
    types.insert(
        "Message".to_string(),
        vec![
            Eip712DomainType {
                name: String::from("from"),
                r#type: String::from("address"),
            },
            Eip712DomainType {
                name: String::from("to"),
                r#type: String::from("address"),
            },
            Eip712DomainType {
                name: String::from("value"),
                r#type: String::from("uint256"),
            },
            Eip712DomainType {
                name: String::from("gas"),
                r#type: String::from("uint256"),
            },
            Eip712DomainType {
                name: String::from("nonce"),
                r#type: String::from("uint256"),
            },
            Eip712DomainType {
                name: String::from("data"),
                r#type: String::from("bytes"),
            },
            Eip712DomainType {
                name: String::from("validUntilTime"),
                r#type: String::from("uint256"),
            },
        ],
    );
    return types;
}

/// Parsed function of "execute((address,address,uint256,uint256,uint256,bytes,uint256) req,bytes32 domainSeparator,bytes32 requestTypeHash,bytes suffixData,bytes sig) (bool success, bytes memory ret)".
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/contracts/src/forwarder/IForwarder.sol> "execute"
/// ref. <https://github.com/gakonst/ethers-rs/blob/master/ethers-core/src/abi/human_readable/mod.rs> "HumanReadableParser::parse_function"
pub fn forwarder_execute_func() -> Function {
    #![allow(deprecated)]
    Function {
        name: "execute".to_string(),
        inputs: vec![
            Param {
                name: "req".to_string(),
                kind: ParamType::Tuple(vec![
                    ParamType::Address,   // "from"
                    ParamType::Address,   // "to"
                    ParamType::Uint(256), // "value"
                    ParamType::Uint(256), // "gas"
                    ParamType::Uint(256), // "nonce"
                    ParamType::Bytes,     // "data"
                    ParamType::Uint(256), // "validUntilTime"
                ]),
                internal_type: None,
            },
            Param {
                name: "domainSeparator".to_string(),
                kind: ParamType::FixedBytes(32),
                internal_type: None,
            },
            Param {
                name: "requestTypeHash".to_string(),
                kind: ParamType::FixedBytes(32),
                internal_type: None,
            },
            Param {
                name: "suffixData".to_string(),
                kind: ParamType::Bytes,
                internal_type: None,
            },
            Param {
                name: "sig".to_string(),
                kind: ParamType::Bytes,
                internal_type: None,
            },
        ],
        outputs: vec![
            Param {
                name: "success".to_string(),
                kind: ParamType::Bool,
                internal_type: None,
            },
            Param {
                name: "ret".to_string(),
                kind: ParamType::Bytes,
                internal_type: None,
            },
        ],
        constant: None,
        state_mutability: StateMutability::NonPayable,
    }
}

/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/types/RelayTransactionRequest.ts>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/EIP712/RelayRequest.ts>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/EIP712/ForwardRequest.ts>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/contracts/src/forwarder/IForwarder.sol>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/EIP712/RelayData.ts>
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayTransactionRequest {
    pub forward_request: TypedData,
    pub relay_metadata: RelayMetadata,
}

/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/types/RelayTransactionRequest.ts>
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelayMetadata {
    #[serde_as(as = "Option<Hex0xBytes>")]
    pub signature: Option<Vec<u8>>,
}

impl RelayTransactionRequest {
    /// Signs the typed data with the signer and returns the "RelayTransactionRequest"
    /// with the signature attached in the relay metadata.
    /// Use "serde_json::to_vec" to encode to "ethers_core::types::Bytes"
    /// and send the request via "eth_sendRawTransaction".
    pub async fn sign(
        forward_request: TypedData,
        signer: impl ethers_signers::Signer + Clone,
    ) -> io::Result<Self> {
        let sig = signer
            .sign_typed_data(&forward_request)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed sign_typed_data '{}'", e)))?;

        Ok(Self {
            forward_request,
            relay_metadata: RelayMetadata {
                signature: Some(sig.to_vec()),
            },
        })
    }

    /// Decodes the EIP-712 encoded typed data and signature in the relay metadata.
    /// ref. <https://ethereum.org/en/developers/docs/apis/json-rpc/#eth_sendrawtransaction>
    pub fn decode_signed(b: impl AsRef<[u8]>) -> io::Result<Self> {
        serde_json::from_slice(b.as_ref()).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed serde_json::from_slice '{}'", e),
            )
        })
    }

    /// Recovers the signature and signer address from its relay metadata signature field.
    pub fn recover_signature(&self) -> io::Result<(Signature, H160)> {
        if let Some(sig) = &self.relay_metadata.signature {
            let sig = Signature::try_from(sig.to_owned().as_bytes()).map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("failed Signature::try_from '{}'", e),
                )
            })?;

            let fwd_req_hash = self.forward_request.encode_eip712().map_err(|e| {
                Error::new(ErrorKind::Other, format!("failed encode_eip712 '{}'", e))
            })?;
            let fwd_req_hash = H256::from_slice(&fwd_req_hash.to_vec());

            let signer_addr = sig.recover(RecoveryMessage::Hash(fwd_req_hash)).map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!(
                        "failed to recover signer address from signature and forward request hash '{}'",
                        e
                    ),
                )
            })?;
            Ok((sig, signer_addr))
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "relay_metadata.signature not found",
            ));
        }
    }

    /// Computes the domain separator hash from the signed relay transaction request.
    /// ref. <https://eips.ethereum.org/EIPS/eip-712>
    pub fn domain_separator(&self) -> H256 {
        let domain_separator = self.forward_request.domain.separator();
        H256::from_slice(&domain_separator)
    }

    /// Computes the type hash from the signed relay transaction request.
    /// ref. <https://eips.ethereum.org/EIPS/eip-712>
    pub fn request_type_hash(&self) -> H256 {
        let request_type_hash = if self.forward_request.domain.salt.is_none() {
            EIP712_DOMAIN_TYPE_HASH
        } else {
            EIP712_DOMAIN_TYPE_HASH_WITH_SALT
        };
        H256::from_slice(&request_type_hash)
    }

    /// Returns the calldata based on the arguments to the forwarder "execute" function.
    /// ref. "HumanReadableParser::parse_function"
    /// ref. "execute((address,address,uint256,uint256,uint256,bytes,uint256) req,bytes32 domainSeparator,bytes32 requestTypeHash,bytes suffixData,bytes sig) (bool success, bytes memory ret)"
    /// ref. ["(0x52C84043CD9c865236f11d9Fc9F56aa003c1f922,0x52C84043CD9c865236f11d9Fc9F56aa003c1f922,0,0,0,0x11,0)", "0x11", "0x11", "0x11", "0x11", "0x11"]
    /// ref. <https://github.com/opengsn/gsn/blob/master/packages/contracts/src/forwarder/IForwarder.sol>
    pub fn encode_execute(&self, suffix_data: Vec<u8>) -> io::Result<Vec<u8>> {
        let from = if let Some(from) = self.forward_request.message.get("from") {
            if let Some(v) = from.as_str() {
                H160::from_str(v).unwrap()
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    "self.forward_request.message expected type 'from'",
                ));
            }
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "self.forward_request.message missing 'from' field",
            ));
        };

        let to = if let Some(to) = self.forward_request.message.get("to") {
            if let Some(v) = to.as_str() {
                H160::from_str(v).unwrap()
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    "self.forward_request.message expected type 'to'",
                ));
            }
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "self.forward_request.message missing 'to' field",
            ));
        };

        let value = if let Some(value) = self.forward_request.message.get("value") {
            if let Some(v) = value.as_str() {
                if v.starts_with("0x") {
                    U256::from_str_radix(v, 16).unwrap()
                } else {
                    U256::from_str(v).unwrap()
                }
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    "self.forward_request.message expected type 'value'",
                ));
            }
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "self.forward_request.message missing 'value' field",
            ));
        };

        let gas = if let Some(gas) = self.forward_request.message.get("gas") {
            if let Some(v) = gas.as_str() {
                if v.starts_with("0x") {
                    U256::from_str_radix(v, 16).unwrap()
                } else {
                    U256::from_str(v).unwrap()
                }
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    "self.forward_request.message expected type 'gas'",
                ));
            }
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "self.forward_request.message missing 'gas' field",
            ));
        };

        let nonce = if let Some(nonce) = self.forward_request.message.get("nonce") {
            if let Some(v) = nonce.as_str() {
                if v.starts_with("0x") {
                    U256::from_str_radix(v, 16).unwrap()
                } else {
                    U256::from_str(v).unwrap()
                }
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    "self.forward_request.message expected type 'nonce'",
                ));
            }
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "self.forward_request.message missing 'nonce' field",
            ));
        };

        let data = if let Some(data) = self.forward_request.message.get("data") {
            if let Some(v) = data.as_str() {
                hex::decode(v.trim_start_matches("0x")).map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("failed hex::decode on 'data' field '{}'", e),
                    )
                })?
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    "self.forward_request.message expected type 'data'",
                ));
            }
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "self.forward_request.message missing 'data' field",
            ));
        };

        let valid_until_time =
            if let Some(valid_until_time) = self.forward_request.message.get("validUntilTime") {
                if let Some(v) = valid_until_time.as_str() {
                    if v.starts_with("0x") {
                        U256::from_str_radix(v, 16).unwrap()
                    } else {
                        U256::from_str(v).unwrap()
                    }
                } else {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "self.forward_request.message expected type 'validUntilTime'",
                    ));
                }
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    "self.forward_request.message missing 'validUntilTime' field",
                ));
            };

        // TODO: do not use "encode_args" from str
        // "LenientTokenizer::tokenize" cannot handle hex encode
        // "Uint parse error: InvalidCharacter"
        // ref. <https://github.com/foundry-rs/foundry/blob/master/common/src/abi.rs> "encode_args"
        let mut tokens = vec![
            Token::Tuple(vec![
                Token::Address(from),
                Token::Address(to),
                Token::Uint(value),
                Token::Uint(gas),
                Token::Uint(nonce),
                Token::Bytes(data),
                Token::Uint(valid_until_time),
            ]),
            Token::FixedBytes(self.domain_separator().as_bytes().to_vec()),
            Token::FixedBytes(self.request_type_hash().as_bytes().to_vec()),
            Token::Bytes(suffix_data),
        ];
        if let Some(sig) = &self.relay_metadata.signature {
            tokens.push(Token::Bytes(sig.clone()));
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "self.relay_metadata.signature missing",
            ));
        }

        let func = forwarder_execute_func();
        func.encode_input(&tokens)
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed to encode_input {}", e)))
    }
}

/// RUST_LOG=debug cargo test --all-features --package avalanche-types --lib -- evm::eip712::gsn::test_build_relay_transaction_request --exact --show-output
#[test]
fn test_build_relay_transaction_request() {
    use ethers_signers::{LocalWallet, Signer};

    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    macro_rules! ab {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    let k = crate::key::secp256k1::TEST_KEYS[0].clone();
    let key_info = k.to_info(1).unwrap();
    log::info!("{:?}", key_info);
    let signer: LocalWallet = k.signing_key().into();

    let rr = ab!(RelayTransactionRequestBuilder::new()
        .domain_name("hello")
        .domain_version("1")
        .domain_chain_id(U256::from(1))
        .domain_verifying_contract(H160::random())
        .from(H160::random())
        .to(H160::random())
        .value(U256::zero())
        .nonce(U256::from(1))
        .data(vec![1, 2, 3])
        .valid_until_time(U256::MAX)
        .build_and_sign(signer.clone()))
    .unwrap();
    let s = serde_json::to_string_pretty(&rr).unwrap();
    log::info!("typed data: {s}");
    let (sig1, signer_addr) = rr.recover_signature().unwrap();
    assert_eq!(key_info.h160_address, signer_addr);

    let sig2 = ab!(signer.sign_typed_data(&rr.forward_request)).unwrap();
    assert_eq!(sig1, sig2);

    let d = rr.encode_execute(vec![1, 2, 3]).unwrap();
    log::info!("encode_execute: {}", hex::encode(d));
}