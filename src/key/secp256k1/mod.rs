pub mod address;
pub mod keychain;
pub mod kms;
pub mod private_key;
pub mod public_key;
pub mod signature;
pub mod txs;

#[cfg(feature = "libsecp256k1")]
pub mod libsecp256k1;

#[cfg(feature = "mnemonic")]
pub mod mnemonic;

use std::{
    collections::HashMap,
    fmt,
    fs::{self, File},
    io::{self, Error, ErrorKind, Write},
    path::Path,
};

use crate::{codec::serde::hex_0x_primitive_types_h160::Hex0xH160, ids::short};
use async_trait::async_trait;
use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Key interface that "only" allows "sign" operations.
/// Trait is used here to limit access to the underlying private/secret key.
/// or to enable secure remote key management service integration (e.g., KMS ECC_SECG_P256K1).
#[async_trait]
pub trait SignOnly {
    type Error: std::error::Error;

    fn signing_key(&self) -> io::Result<k256::ecdsa::SigningKey>;

    /// Signs the 32-byte SHA256 output message with the ECDSA private key and the recoverable code.
    /// "github.com/decred/dcrd/dcrec/secp256k1/v3/ecdsa.SignCompact" outputs 65-byte signature.
    /// ref. "avalanchego/utils/crypto.PrivateKeySECP256K1R.SignHash"
    /// ref. <https://github.com/rust-bitcoin/rust-secp256k1/blob/master/src/ecdsa/recovery.rs>
    /// ref. <https://docs.rs/secp256k1/latest/secp256k1/struct.SecretKey.html#method.sign_ecdsa>
    /// ref. <https://docs.rs/secp256k1/latest/secp256k1/struct.Message.html>
    /// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/crypto#PrivateKeyED25519.SignHash>
    async fn sign_digest(&self, digest: &[u8]) -> Result<[u8; 65], Self::Error>;
}

/// Key interface that "only" allows "read" operations.
pub trait ReadOnly {
    fn key_type(&self) -> KeyType;
    /// Implements "crypto.PublicKeySECP256K1R.Address()" and "formatting.FormatAddress".
    /// "human readable part" (hrp) must be valid output from "constants.GetHRP(networkID)".
    /// ref. <https://pkg.go.dev/github.com/ava-labs/avalanchego/utils/constants>
    fn hrp_address(&self, network_id: u32, chain_id_alias: &str) -> io::Result<String>;
    fn short_address(&self) -> io::Result<short::Id>;
    fn short_address_bytes(&self) -> io::Result<Vec<u8>>;
    fn eth_address(&self) -> String;
    fn h160_address(&self) -> primitive_types::H160;
}

lazy_static! {
    /// Test keys generated by "avalanchego/utils/crypto.FactorySECP256K1R".
    pub static ref TEST_KEYS: Vec<crate::key::secp256k1::private_key::Key> = {
        #[derive(RustEmbed)]
        #[folder = "artifacts/"]
        #[prefix = "artifacts/"]
        struct Asset;

        let key_file = Asset::get("artifacts/test.insecure.secp256k1.key.infos.json").unwrap();

        let key_infos: Vec<Info> = serde_json::from_slice(&key_file.data).unwrap();
        let mut keys: Vec<crate::key::secp256k1::private_key::Key> = Vec::new();
        for ki in key_infos.iter() {
            keys.push(ki.to_private_key());
        }
        keys
    };

    /// Test key infos in the same order of "TEST_KEYS".
    pub static ref TEST_INFOS: Vec<Info> = {
        #[derive(RustEmbed)]
        #[folder = "artifacts/"]
        #[prefix = "artifacts/"]
        struct Asset;

        let key_file = Asset::get("artifacts/test.insecure.secp256k1.key.infos.json").unwrap();
        serde_json::from_slice(&key_file.data).unwrap()
    };
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- key::secp256k1::test_keys --exact --show-output
#[test]
fn test_keys() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    for k in TEST_KEYS.iter() {
        log::info!(
            "[KEY] test key eth address {:?}",
            k.to_public_key().eth_address()
        );
    }
    for ki in TEST_INFOS.iter() {
        log::info!("[INFO] test key eth address {:?}", ki.eth_address);
    }
    assert_eq!(TEST_KEYS.len(), TEST_INFOS.len());

    log::info!("total {} test keys are found", TEST_KEYS.len());
}

// test random keys generated by "avalanchego/utils/crypto.FactorySECP256K1R"
// and make sure both generate the same addresses
// use "avalanche-rust/avalanchego-conformance/key/secp256k1"
// to generate keys and addresses with "avalanchego"
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Info {
    /// Optional key identifier (e.g., name, AWS KMS Id/Arn).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    pub key_type: KeyType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnemonic_phrase: Option<String>,
    /// CB58-encoded private key with the prefix "PrivateKey-" (e.g., Avalanche).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_cb58: Option<String>,
    /// Hex-encoded private key without the prefix "0x" (e.g., Ethereum).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_hex: Option<String>,

    #[serde(default)]
    pub addresses: HashMap<u32, ChainAddresses>,
    #[serde(default)]
    pub short_address: short::Id,
    #[serde(default)]
    pub eth_address: String,
    #[serde_as(as = "Hex0xH160")]
    pub h160_address: primitive_types::H160,
}

impl Default for Info {
    fn default() -> Self {
        Self::default()
    }
}

impl Info {
    pub fn default() -> Self {
        Info {
            id: None,
            key_type: KeyType::Unknown(String::new()),
            mnemonic_phrase: None,
            private_key_cb58: None,
            private_key_hex: None,
            addresses: HashMap::new(),
            short_address: short::Id::empty(),
            eth_address: String::new(),
            h160_address: primitive_types::H160::zero(),
        }
    }
}

impl From<&crate::key::secp256k1::private_key::Key> for Info {
    fn from(sk: &crate::key::secp256k1::private_key::Key) -> Self {
        sk.to_info(1).unwrap()
    }
}

impl Info {
    pub fn load(file_path: &str) -> io::Result<Self> {
        log::info!("loading Info from {}", file_path);

        if !Path::new(file_path).exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("file {} does not exists", file_path),
            ));
        }

        let f = File::open(&file_path).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("failed to open {} ({})", file_path, e),
            )
        })?;
        serde_yaml::from_reader(f)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("invalid YAML: {}", e)))
    }

    pub fn sync(&self, file_path: String) -> io::Result<()> {
        log::info!("syncing key info to '{}'", file_path);
        let path = Path::new(&file_path);
        let parent_dir = path.parent().unwrap();
        fs::create_dir_all(parent_dir)?;

        let d = serde_json::to_vec(self)
            .map_err(|e| Error::new(ErrorKind::Other, format!("failed to serialize JSON {}", e)))?;

        let mut f = File::create(&file_path)?;
        f.write_all(&d)?;

        Ok(())
    }

    pub fn to_private_key(&self) -> crate::key::secp256k1::private_key::Key {
        crate::key::secp256k1::private_key::Key::from_cb58(self.private_key_cb58.clone().unwrap())
            .unwrap()
    }
}

/// ref. <https://doc.rust-lang.org/std/string/trait.ToString.html>
/// ref. <https://doc.rust-lang.org/std/fmt/trait.Display.html>
/// Use "Self.to_string()" to directly invoke this
impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = serde_yaml::to_string(&self).unwrap();
        write!(f, "{}", s)
    }
}

/// Defines the key type.
#[derive(
    Deserialize,
    Serialize,
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum KeyType {
    Hot,
    AwsKms,
    Unknown(String),
}

impl std::convert::From<&str> for KeyType {
    fn from(s: &str) -> Self {
        match s {
            "hot" => KeyType::Hot,
            "aws-kms" => KeyType::AwsKms,
            "aws_kms" => KeyType::AwsKms,

            other => KeyType::Unknown(other.to_owned()),
        }
    }
}

impl std::str::FromStr for KeyType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(KeyType::from(s))
    }
}

/// ref. <https://doc.rust-lang.org/std/string/trait.ToString.html>
/// ref. <https://doc.rust-lang.org/std/fmt/trait.Display.html>
/// Use "Self.to_string()" to directly invoke this
impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl KeyType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            KeyType::Hot => "hot",
            KeyType::AwsKms => "aws-kms",

            KeyType::Unknown(s) => s.as_ref(),
        }
    }

    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &[
            "hot",     //
            "aws-kms", //
        ]
    }
}

impl AsRef<str> for KeyType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ChainAddresses {
    pub x_address: String,
    pub p_address: String,
    pub c_address: String,
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- key::secp256k1::test_keys_address --exact --show-output
#[test]
fn test_keys_address() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();

    #[derive(RustEmbed)]
    #[folder = "artifacts/"]
    #[prefix = "artifacts/"]
    struct Asset;

    for asset in vec!["artifacts/test.insecure.secp256k1.key.infos.json"] {
        let key_file = Asset::get(asset).unwrap();
        let key_contents = std::str::from_utf8(key_file.data.as_ref()).unwrap();
        let key_infos: Vec<Info> = serde_json::from_slice(&key_contents.as_bytes()).unwrap();
        log::info!("loaded {}", asset);

        for (pos, ki) in key_infos.iter().enumerate() {
            log::info!("checking the key info at {}", pos);

            let sk = crate::key::secp256k1::private_key::Key::from_cb58(
                &ki.private_key_cb58.clone().unwrap(),
            )
            .unwrap();
            assert_eq!(
                sk,
                crate::key::secp256k1::private_key::Key::from_hex(
                    ki.private_key_hex.clone().unwrap()
                )
                .unwrap(),
            );
            let pubkey = sk.to_public_key();

            assert_eq!(
                pubkey.hrp_address(1, "X").unwrap(),
                ki.addresses.get(&1).unwrap().x_address
            );
            assert_eq!(
                pubkey.hrp_address(1, "P").unwrap(),
                ki.addresses.get(&1).unwrap().p_address
            );
            assert_eq!(
                pubkey.hrp_address(1, "C").unwrap(),
                ki.addresses.get(&1).unwrap().c_address
            );

            assert_eq!(
                pubkey.hrp_address(9999, "X").unwrap(),
                ki.addresses.get(&9999).unwrap().x_address
            );
            assert_eq!(
                pubkey.hrp_address(9999, "P").unwrap(),
                ki.addresses.get(&9999).unwrap().p_address
            );
            assert_eq!(
                pubkey.hrp_address(9999, "C").unwrap(),
                ki.addresses.get(&9999).unwrap().c_address
            );

            assert_eq!(pubkey.to_short_id().unwrap(), ki.short_address);
            assert_eq!(pubkey.eth_address(), ki.eth_address);
        }
    }
}
