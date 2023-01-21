use std::{
    convert::TryFrom,
    io::{self, Error, ErrorKind},
    str::FromStr,
};

use ethers_core::types::{
    transaction::eip712::{Eip712, TypedData},
    RecoveryMessage, Signature, H160, H256, U256,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use zerocopy::AsBytes;

impl super::Tx {
    /// Builds and signs the typed data with the signer and returns the
    /// "RelayTransactionRequest" with the signature attached in the relay metadata.
    /// Use "serde_json::to_vec" to encode to "ethers_core::types::Bytes"
    /// and send the request via "eth_sendRawTransaction".
    pub async fn sign_to_request(
        &self,
        eth_signer: impl ethers_signers::Signer + Clone,
    ) -> io::Result<Request> {
        Request::sign(self, eth_signer).await
    }
}

/// Used for gas relayer server.
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/types/RelayTransactionRequest.ts>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/EIP712/RelayRequest.ts>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/EIP712/ForwardRequest.ts>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/contracts/src/forwarder/IForwarder.sol>
/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/EIP712/RelayData.ts>
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub forward_request: TypedData,
    pub metadata: Metadata,
}

/// ref. <https://github.com/opengsn/gsn/blob/master/packages/common/src/types/RelayTransactionRequest.ts>
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde_as(as = "serde_with::hex::Hex")]
    pub signature: Vec<u8>,
}

impl Request {
    /// Signs the typed data with the signer and returns the "RelayTransactionRequest"
    /// with the signature attached in the relay metadata.
    /// Use "serde_json::to_vec" to encode to "ethers_core::types::Bytes"
    /// and send the request via "eth_sendRawTransaction".
    pub async fn sign(
        tx: &super::Tx,
        signer: impl ethers_signers::Signer + Clone,
    ) -> io::Result<Self> {
        let sig = signer
            .sign_typed_data(tx)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed sign_typed_data '{}'", e)))?;

        Ok(Self {
            forward_request: tx.typed_data(),
            metadata: Metadata {
                signature: sig.to_vec(),
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

    /// Recovers the GSN transaction object based on the raw typed data and given type name and suffix data.
    pub fn recover_tx(&self, type_name: &str, type_suffix_data: &str) -> io::Result<super::Tx> {
        let domain_name = if let Some(name) = &self.forward_request.domain.name {
            name.clone()
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "self.forward_request.domain missing 'name' field",
            ));
        };

        let domain_version = if let Some(version) = &self.forward_request.domain.version {
            version.clone()
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "self.forward_request.domain missing 'version' field",
            ));
        };

        let domain_chain_id = if let Some(chain_id) = &self.forward_request.domain.chain_id {
            chain_id
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "self.forward_request.domain missing 'chain_id' field",
            ));
        };

        let domain_verifying_contract =
            if let Some(verifying_contract) = &self.forward_request.domain.verifying_contract {
                verifying_contract.clone()
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    "self.forward_request.domain missing 'verifying_contract' field",
                ));
            };

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

        Ok(super::Tx::new()
            .domain_name(domain_name)
            .domain_version(domain_version)
            .domain_chain_id(domain_chain_id)
            .domain_verifying_contract(domain_verifying_contract)
            .from(from)
            .to(to)
            .value(value)
            .gas(gas)
            .nonce(nonce)
            .data(data)
            .valid_until_time(valid_until_time)
            .type_name(type_name)
            .type_suffix_data(type_suffix_data))
    }

    /// Recovers the signature and signer address from its relay metadata signature field.
    pub fn recover_signature(
        &self,
        type_name: &str,
        type_suffix_data: &str,
    ) -> io::Result<(Signature, H160)> {
        let sig = Signature::try_from(self.metadata.signature.as_bytes()).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed Signature::try_from '{}'", e),
            )
        })?;

        let tx = self.recover_tx(type_name, type_suffix_data)?;
        let fwd_req_hash = tx
            .encode_eip712()
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed encode_eip712 '{}'", e)))?;
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
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib --features="evm" -- evm::eip712::gsn::relay::test_build_relay_transaction_request --exact --show-output
#[test]
fn test_build_relay_transaction_request() {
    use ethers_core::{
        abi::{Function, Param, ParamType, StateMutability, Token},
        types::U256,
    };
    use ethers_signers::{LocalWallet, Signer};

    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();

    // parsed function of "register(string name)"
    let func = Function {
        name: "register".to_string(),
        inputs: vec![Param {
            name: "name".to_string(),
            kind: ParamType::String,
            internal_type: None,
        }],
        outputs: Vec::new(),
        constant: None,
        state_mutability: StateMutability::NonPayable,
    };
    let arg_tokens = vec![Token::String("aaaaa".to_string())];
    let calldata = crate::evm::abi::encode_calldata(func, &arg_tokens).unwrap();
    log::info!("calldata: 0x{}", hex::encode(calldata.clone()));

    macro_rules! ab {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    let domain_name = random_manager::string(20);
    let domain_version = format!("{}", random_manager::u16());

    let my_type = random_manager::string(20);
    let my_suffix_data = random_manager::string(20);

    let tx = super::Tx::new()
        .domain_name(domain_name)
        .domain_version(domain_version)
        .domain_chain_id(U256::from(random_manager::u64()))
        .domain_verifying_contract(H160::random())
        .from(H160::random())
        .to(H160::random())
        .value(U256::zero())
        .nonce(U256::from(random_manager::u64()))
        .data(calldata)
        .valid_until_time(U256::from(random_manager::u64()))
        .type_name(&my_type)
        .type_suffix_data(&my_suffix_data);

    let k = crate::key::secp256k1::private_key::Key::generate().unwrap();
    let signer: LocalWallet = k.signing_key().into();

    let rr = ab!(tx.sign_to_request(signer.clone())).unwrap();
    log::info!("request: {}", serde_json::to_string_pretty(&rr).unwrap());

    let (sig1, signer_addr) = rr.recover_signature(&my_type, &my_suffix_data).unwrap();
    assert_eq!(k.to_public_key().to_h160(), signer_addr);

    // default TypeData has different "struct_hash"
    let sig2 = ab!(signer.sign_typed_data(&rr.forward_request)).unwrap();
    assert_ne!(sig1, sig2);

    // Tx implements its own "struct_hash", must match with the recovered signature
    let sig3 =
        ab!(signer.sign_typed_data(&rr.recover_tx(&my_type, &my_suffix_data).unwrap())).unwrap();
    assert_eq!(sig1, sig3);

    let d = tx.encode_execute_call(sig1.to_vec()).unwrap();
    log::info!("encode_execute_call: {}", hex::encode(d));
}