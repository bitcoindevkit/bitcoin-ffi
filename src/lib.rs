use bitcoin::address::{NetworkChecked, NetworkUnchecked};
use bitcoin::consensus::{deserialize, serialize};
use bitcoin::Address as BitcoinAddress;
use bitcoin::Amount as BitcoinAmount;
use bitcoin::FeeRate as BitcoinFeeRate;
use bitcoin::ScriptBuf as BitcoinScriptBuf;
use bitcoin::Sequence;
use bitcoin::Transaction as BitcoinTransaction;
use bitcoin::TxIn as BitcoinTxIn;
use bitcoin::TxOut as BitcoinTxOut;

pub use bitcoin::BlockHash;
pub use bitcoin::Txid;

use error::AddressParseError;
use error::EncodeError;
use error::FeeRateError;
use error::FromScriptError;
use error::ParseAmountError;

use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;

#[macro_use]
mod macros;
pub mod error;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Object)]
pub struct Address(BitcoinAddress<NetworkChecked>);

#[uniffi::export]
impl Address {
    #[uniffi::constructor]
    pub fn new(address: String, network: Network) -> Result<Self, AddressParseError> {
        let parsed_address = BitcoinAddress::from_str(&address).map_err(AddressParseError::from)?;
        let network_checked_address = parsed_address.require_network(network.into())?;
        Ok(Address(network_checked_address))
    }

    #[uniffi::constructor]
    pub fn from_script(script: Arc<Script>, network: Network) -> Result<Self, FromScriptError> {
        let address = BitcoinAddress::from_script(
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
        if let Ok(unchecked_address) = address_str.parse::<BitcoinAddress<NetworkUnchecked>>() {
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

type CheckedBitcoinAddress = BitcoinAddress<NetworkChecked>;
impl_from_core_type!(Address, CheckedBitcoinAddress);
impl_from_ffi_type!(Address, CheckedBitcoinAddress);

#[derive(Clone, Debug, uniffi::Object)]
pub struct FeeRate(pub BitcoinFeeRate);

#[uniffi::export]
impl FeeRate {
    #[uniffi::constructor]
    pub fn from_sat_per_vb(sat_per_vb: u64) -> Result<Self, FeeRateError> {
        let fee_rate: Option<BitcoinFeeRate> = BitcoinFeeRate::from_sat_per_vb(sat_per_vb);
        match fee_rate {
            Some(fee_rate) => Ok(FeeRate(fee_rate)),
            None => Err(FeeRateError::ArithmeticOverflow),
        }
    }

    #[uniffi::constructor]
    pub fn from_sat_per_kwu(sat_per_kwu: u64) -> Self {
        FeeRate(BitcoinFeeRate::from_sat_per_kwu(sat_per_kwu))
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

impl_from_core_type!(FeeRate, BitcoinFeeRate);
impl_from_ffi_type!(FeeRate, BitcoinFeeRate);

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
pub struct Script(pub BitcoinScriptBuf);

#[uniffi::export]
impl Script {
    #[uniffi::constructor]
    pub fn new(raw_output_script: Vec<u8>) -> Self {
        let script: BitcoinScriptBuf = raw_output_script.into();
        Script(script)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl_from_core_type!(Script, BitcoinScriptBuf);
impl_from_ffi_type!(Script, BitcoinScriptBuf);

#[derive(Debug, Clone, uniffi::Record)]
pub struct TxOut {
    pub value: Arc<Amount>,
    pub script_pubkey: Arc<Script>,
}

impl From<BitcoinTxOut> for TxOut {
    fn from(tx_out: BitcoinTxOut) -> Self {
        TxOut {
            value: Arc::new(Amount(tx_out.value)),
            script_pubkey: Arc::new(Script(tx_out.script_pubkey)),
        }
    }
}

impl From<TxOut> for BitcoinTxOut {
    fn from(tx_out: TxOut) -> Self {
        BitcoinTxOut {
            value: tx_out.value.0,
            script_pubkey: tx_out.script_pubkey.0.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Object)]
pub struct Amount(pub BitcoinAmount);

#[uniffi::export]
impl Amount {
    #[uniffi::constructor]
    pub fn from_sat(sat: u64) -> Self {
        Amount(BitcoinAmount::from_sat(sat))
    }

    #[uniffi::constructor]
    pub fn from_btc(btc: f64) -> Result<Self, ParseAmountError> {
        let bitcoin_amount = BitcoinAmount::from_btc(btc).map_err(ParseAmountError::from)?;
        Ok(Amount(bitcoin_amount))
    }

    pub fn to_sat(&self) -> u64 {
        self.0.to_sat()
    }

    pub fn to_btc(&self) -> f64 {
        self.0.to_btc()
    }
}

impl_from_core_type!(Amount, BitcoinAmount);
impl_from_ffi_type!(Amount, BitcoinAmount);

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct TxIn {
    pub previous_output: OutPoint,
    pub script_sig: Arc<Script>,
    pub sequence: u32,
    pub witness: Vec<Vec<u8>>,
}

impl From<BitcoinTxIn> for TxIn {
    fn from(tx_in: BitcoinTxIn) -> Self {
        TxIn {
            previous_output: tx_in.previous_output.into(),
            script_sig: Arc::new(tx_in.script_sig.into()),
            sequence: tx_in.sequence.0,
            witness: tx_in.witness.to_vec(),
        }
    }
}

impl From<&BitcoinTxIn> for TxIn {
    fn from(tx_in: &BitcoinTxIn) -> Self {
        TxIn {
            previous_output: tx_in.previous_output.into(),
            script_sig: Arc::new(tx_in.script_sig.clone().into()),
            sequence: tx_in.sequence.0,
            witness: tx_in.witness.to_vec(),
        }
    }
}

impl From<TxIn> for BitcoinTxIn {
    fn from(tx_in: TxIn) -> Self {
        BitcoinTxIn {
            previous_output: tx_in.previous_output.into(),
            script_sig: tx_in.script_sig.0.clone(),
            sequence: Sequence(tx_in.sequence),
            witness: tx_in.witness.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Object)]
pub struct Transaction(pub BitcoinTransaction);

#[uniffi::export]
impl Transaction {
    #[uniffi::constructor]
    pub fn deserialize(transaction_bytes: &[u8]) -> Result<Self, EncodeError> {
        let transaction: BitcoinTransaction = deserialize(transaction_bytes)?;
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

impl_from_core_type!(Transaction, BitcoinTransaction);
impl_from_ffi_type!(Transaction, BitcoinTransaction);

#[derive(Clone, Default, uniffi::Enum)]
#[non_exhaustive]
pub enum Network {
    #[default]
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

impl From<Network> for bitcoin::Network {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => bitcoin::Network::Bitcoin,
            Network::Testnet => bitcoin::Network::Testnet,
            Network::Signet => bitcoin::Network::Signet,
            Network::Regtest => bitcoin::Network::Regtest,
        }
    }
}

impl From<bitcoin::Network> for Network {
    fn from(network: bitcoin::Network) -> Self {
        match network {
            bitcoin::Network::Bitcoin => Network::Bitcoin,
            bitcoin::Network::Testnet => Network::Testnet,
            bitcoin::Network::Signet => Network::Signet,
            bitcoin::Network::Regtest => Network::Regtest,
            _ => unreachable!(),
        }
    }
}

uniffi::custom_type!(Txid, String);
impl_string_custom_typedef!(Txid);
uniffi::custom_type!(BlockHash, String);
impl_string_custom_typedef!(BlockHash);

uniffi::setup_scaffolding!("bitcoin");
