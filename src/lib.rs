use bitcoin::address::{NetworkChecked, NetworkUnchecked};
use bitcoin::consensus::{deserialize, serialize};

pub use bitcoin::BlockHash;
pub use bitcoin::Txid;

use error::EncodeError;
use error::ExtractTxError;
use error::FeeRateError;
use error::FromScriptError;
use error::ParseAmountError;
use error::PsbtError;
use error::{AddressParseError, PsbtParseError};

use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;

#[macro_use]
mod macros;
pub mod error;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Object)]
#[uniffi::export(Display)]
pub struct Address(bitcoin::Address<NetworkChecked>);

#[uniffi::export]
impl Address {
    #[uniffi::constructor]
    pub fn new(address: String, network: Network) -> Result<Self, AddressParseError> {
        let parsed_address =
            bitcoin::Address::from_str(&address).map_err(AddressParseError::from)?;
        let network_checked_address = parsed_address.require_network(network.into())?;
        Ok(Address(network_checked_address))
    }

    #[uniffi::constructor]
    pub fn from_script(script: Arc<Script>, network: Network) -> Result<Self, FromScriptError> {
        let address = bitcoin::Address::from_script(
            &script.0.clone(),
            Into::<bitcoin::Network>::into(network),
        )?;
        Ok(Address(address))
    }

    pub fn script_pubkey(&self) -> Arc<Script> {
        Arc::new(Script(self.0.script_pubkey()))
    }

    pub fn to_qr_uri(&self) -> String {
        self.0.to_qr_uri()
    }

    pub fn is_valid_for_network(&self, network: Network) -> bool {
        let address_str = self.0.to_string();
        if let Ok(unchecked_address) = address_str.parse::<bitcoin::Address<NetworkUnchecked>>() {
            unchecked_address.is_valid_for_network(network.into())
        } else {
            false
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl_from_core_type!(Address, bitcoin::Address<NetworkChecked>);
impl_from_ffi_type!(Address, bitcoin::Address<NetworkChecked>);

#[derive(Clone, Debug, uniffi::Object)]
pub struct FeeRate(pub bitcoin::FeeRate);

#[uniffi::export]
impl FeeRate {
    #[uniffi::constructor]
    pub fn from_sat_per_vb(sat_per_vb: u64) -> Result<Self, FeeRateError> {
        let fee_rate: Option<bitcoin::FeeRate> = bitcoin::FeeRate::from_sat_per_vb(sat_per_vb);
        match fee_rate {
            Some(fee_rate) => Ok(FeeRate(fee_rate)),
            None => Err(FeeRateError::ArithmeticOverflow),
        }
    }

    #[uniffi::constructor]
    pub fn from_sat_per_kwu(sat_per_kwu: u64) -> Self {
        FeeRate(bitcoin::FeeRate::from_sat_per_kwu(sat_per_kwu))
    }

    pub fn to_sat_per_vb_ceil(&self) -> u64 {
        self.0.to_sat_per_vb_ceil()
    }

    pub fn to_sat_per_vb_floor(&self) -> u64 {
        self.0.to_sat_per_vb_floor()
    }

    pub fn to_sat_per_kwu(&self) -> u64 {
        self.0.to_sat_per_kwu()
    }
}

impl_from_core_type!(FeeRate, bitcoin::FeeRate);
impl_from_ffi_type!(FeeRate, bitcoin::FeeRate);

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl From<bitcoin::OutPoint> for OutPoint {
    fn from(outpoint: bitcoin::OutPoint) -> Self {
        OutPoint {
            txid: outpoint.txid,
            vout: outpoint.vout,
        }
    }
}

impl From<OutPoint> for bitcoin::OutPoint {
    fn from(outpoint: OutPoint) -> Self {
        bitcoin::OutPoint::new(outpoint.txid, outpoint.vout)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct Script(pub bitcoin::ScriptBuf);

#[uniffi::export]
impl Script {
    #[uniffi::constructor]
    pub fn new(raw_output_script: Vec<u8>) -> Self {
        let script: bitcoin::ScriptBuf = raw_output_script.into();
        Script(script)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl_from_core_type!(Script, bitcoin::ScriptBuf);
impl_from_ffi_type!(Script, bitcoin::ScriptBuf);

#[derive(Debug, Clone, uniffi::Record)]
pub struct TxOut {
    pub value: Arc<Amount>,
    pub script_pubkey: Arc<Script>,
}

impl From<bitcoin::TxOut> for TxOut {
    fn from(tx_out: bitcoin::TxOut) -> Self {
        TxOut {
            value: Arc::new(Amount(tx_out.value)),
            script_pubkey: Arc::new(Script(tx_out.script_pubkey)),
        }
    }
}

impl From<TxOut> for bitcoin::TxOut {
    fn from(tx_out: TxOut) -> Self {
        bitcoin::TxOut {
            value: tx_out.value.0,
            script_pubkey: tx_out.script_pubkey.0.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Object)]
pub struct Amount(pub bitcoin::Amount);

#[uniffi::export]
impl Amount {
    #[uniffi::constructor]
    pub fn from_sat(sat: u64) -> Self {
        Amount(bitcoin::Amount::from_sat(sat))
    }

    #[uniffi::constructor]
    pub fn from_btc(btc: f64) -> Result<Self, ParseAmountError> {
        let bitcoin_amount = bitcoin::Amount::from_btc(btc).map_err(ParseAmountError::from)?;
        Ok(Amount(bitcoin_amount))
    }

    pub fn to_sat(&self) -> u64 {
        self.0.to_sat()
    }

    pub fn to_btc(&self) -> f64 {
        self.0.to_btc()
    }
}

impl_from_core_type!(Amount, bitcoin::Amount);
impl_from_ffi_type!(Amount, bitcoin::Amount);

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct TxIn {
    pub previous_output: OutPoint,
    pub script_sig: Arc<Script>,
    pub sequence: u32,
    pub witness: Vec<Vec<u8>>,
}

impl From<bitcoin::TxIn> for TxIn {
    fn from(tx_in: bitcoin::TxIn) -> Self {
        TxIn {
            previous_output: tx_in.previous_output.into(),
            script_sig: Arc::new(tx_in.script_sig.into()),
            sequence: tx_in.sequence.0,
            witness: tx_in.witness.to_vec(),
        }
    }
}

impl From<TxIn> for bitcoin::TxIn {
    fn from(tx_in: TxIn) -> Self {
        bitcoin::TxIn {
            previous_output: tx_in.previous_output.into(),
            script_sig: tx_in.script_sig.0.clone(),
            sequence: bitcoin::Sequence(tx_in.sequence),
            witness: tx_in.witness.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Object)]
pub struct Transaction(pub bitcoin::Transaction);

#[uniffi::export]
impl Transaction {
    #[uniffi::constructor]
    pub fn deserialize(transaction_bytes: &[u8]) -> Result<Self, EncodeError> {
        let transaction: bitcoin::Transaction = deserialize(transaction_bytes)?;
        Ok(Transaction(transaction))
    }

    pub fn serialize(&self) -> Vec<u8> {
        serialize(&self.0)
    }

    pub fn compute_txid(&self) -> String {
        self.0.compute_txid().to_string()
    }

    pub fn weight(&self) -> u64 {
        self.0.weight().to_wu()
    }

    pub fn total_size(&self) -> u64 {
        self.0.total_size() as u64
    }

    pub fn vsize(&self) -> u64 {
        self.0.vsize() as u64
    }

    pub fn is_coinbase(&self) -> bool {
        self.0.is_coinbase()
    }

    pub fn is_explicitly_rbf(&self) -> bool {
        self.0.is_explicitly_rbf()
    }

    pub fn is_lock_time_enabled(&self) -> bool {
        self.0.is_lock_time_enabled()
    }

    pub fn version(&self) -> i32 {
        self.0.version.0
    }

    pub fn input(&self) -> Vec<TxIn> {
        self.0
            .input
            .clone()
            .into_iter()
            .map(|tx_in| tx_in.into())
            .collect()
    }

    pub fn output(&self) -> Vec<TxOut> {
        self.0
            .output
            .clone()
            .into_iter()
            .map(|tx_out| tx_out.into())
            .collect()
    }

    pub fn lock_time(&self) -> u32 {
        self.0.lock_time.to_consensus_u32()
    }
}

impl_from_core_type!(Transaction, bitcoin::Transaction);
impl_from_ffi_type!(Transaction, bitcoin::Transaction);

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Object)]
pub struct Psbt(bitcoin::Psbt);

#[uniffi::export]
impl Psbt {
    #[uniffi::constructor]
    pub fn from_unsigned_tx(tx: Arc<Transaction>) -> Result<Self, PsbtError> {
        let psbt = bitcoin::Psbt::from_unsigned_tx(tx.0.clone().into())?;
        Ok(Psbt(psbt))
    }

    #[uniffi::constructor]
    pub fn deserialize(psbt_bytes: &[u8]) -> Result<Self, PsbtError> {
        let psbt = bitcoin::Psbt::deserialize(psbt_bytes)?;
        Ok(psbt.into())
    }

    #[uniffi::constructor]
    pub fn deserialize_base64(psbt_base64: String) -> Result<Self, PsbtParseError> {
        let psbt = bitcoin::Psbt::from_str(&psbt_base64)?;
        Ok(psbt.into())
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.0.serialize()
    }

    pub fn serialize_hex(&self) -> String {
        self.0.serialize_hex()
    }

    pub fn serialize_base64(&self) -> String {
        self.0.to_string()
    }

    pub fn extract_tx(&self) -> Result<Arc<Transaction>, ExtractTxError> {
        Ok(Arc::new(self.0.clone().extract_tx()?.into()))
    }

    pub fn combine(&self, other: Arc<Self>) -> Result<Arc<Psbt>, PsbtError> {
        let mut psbt = self.0.clone();
        let other_psbt = other.0.clone();
        psbt.combine(other_psbt)?;
        Ok(Arc::new(psbt.into()))
    }

    pub fn fee(&self) -> Result<Arc<Amount>, PsbtError> {
        Ok(Arc::new(self.0.clone().fee()?.into()))
    }
}

impl_from_core_type!(Psbt, bitcoin::Psbt);
impl_from_ffi_type!(Psbt, bitcoin::Psbt);

#[derive(Clone, Default, uniffi::Enum)]
#[non_exhaustive]
pub enum Network {
    #[default]
    Bitcoin,
    Testnet,
    Testnet4,
    Signet,
    Regtest,
}

impl From<Network> for bitcoin::Network {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => bitcoin::Network::Bitcoin,
            Network::Testnet => bitcoin::Network::Testnet,
            Network::Testnet4 => bitcoin::Network::Testnet4,
            Network::Signet => bitcoin::Network::Signet,
            Network::Regtest => bitcoin::Network::Regtest,
        }
    }
}

define_custom_string_type!(Txid);
define_custom_string_type!(BlockHash);

uniffi::setup_scaffolding!("bitcoin");
