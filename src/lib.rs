use bitcoin::address::{NetworkChecked, NetworkUnchecked};
use bitcoin::consensus::{deserialize, serialize};
use bitcoin::Address as BitcoinAddress;
use bitcoin::Amount as BitcoinAmount;
use bitcoin::FeeRate as BitcoinFeeRate;
use bitcoin::ScriptBuf as BitcoinScriptBuf;
use bitcoin::Transaction as BitcoinTransaction;
use bitcoin::TxIn as BitcoinTxIn;
use bitcoin::TxOut as BitcoinTxOut;
use bitcoin::Sequence;

pub use bitcoin::BlockHash;
pub use bitcoin::OutPoint;
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
pub use bitcoin::Network;

#[derive(Debug, PartialEq, Eq)]
pub struct Address(BitcoinAddress<NetworkChecked>);

impl Address {
    pub fn new(address: String, network: Network) -> Result<Self, AddressParseError> {
        let parsed_address = BitcoinAddress::from_str(&address).map_err(AddressParseError::from)?;
        let network_checked_address = parsed_address.require_network(network)?;
        Ok(Address(network_checked_address))
    }

    pub fn from_script(script: Arc<Script>, network: Network) -> Result<Self, FromScriptError> {
        let address = BitcoinAddress::from_script(&script.0.clone(), network)?;
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
            unchecked_address.is_valid_for_network(network)
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

#[derive(Clone, Debug)]
pub struct FeeRate(pub BitcoinFeeRate);

impl FeeRate {
    pub fn from_sat_per_vb(sat_per_vb: u64) -> Result<Self, FeeRateError> {
        let fee_rate: Option<BitcoinFeeRate> = BitcoinFeeRate::from_sat_per_vb(sat_per_vb);
        match fee_rate {
            Some(fee_rate) => Ok(FeeRate(fee_rate)),
            None => Err(FeeRateError::ArithmeticOverflow),
        }
    }

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Script(pub BitcoinScriptBuf);

impl Script {
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

#[derive(Debug, Clone)]
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
        let value = match Arc::try_unwrap(tx_out.value) {
            Ok(val) => val.0,
            Err(arc) => arc.0,
        };

        let script_pubkey = match Arc::try_unwrap(tx_out.script_pubkey) {
            Ok(val) => val.0,
            Err(arc) => arc.0.clone(),
        };

        BitcoinTxOut {
            value,
            script_pubkey,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Amount(pub BitcoinAmount);

impl Amount {
    pub fn from_sat(sat: u64) -> Self {
        Amount(BitcoinAmount::from_sat(sat))
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxIn {
    pub previous_output: OutPoint,
    pub script_sig: Arc<Script>,
    pub sequence: u32,
    pub witness: Vec<Vec<u8>>,
}

impl From<BitcoinTxIn> for TxIn {
    fn from(value: BitcoinTxIn) -> Self {
        Self {
            previous_output: value.previous_output,
            script_sig: Arc::new(value.script_sig.into()),
            sequence: value.sequence.0,
            witness: value.witness.to_vec(),
        }
    }
}

impl From<TxIn> for BitcoinTxIn {
    fn from(value: TxIn) -> Self {
        Self {
            previous_output: value.previous_output,
            script_sig: value.script_sig.0.clone(),
            sequence: Sequence(value.sequence),
            witness: value.witness.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction(pub BitcoinTransaction);

impl Transaction {
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
        self.0.input.clone().into_iter().map(|tx_in| tx_in.into()).collect()
    }

    pub fn output(&self) -> Vec<TxOut> {
        self.0.output.clone().into_iter().map(|tx_out| tx_out.into()).collect()
    }

    pub fn lock_time(&self) -> u32 {
        self.0.lock_time.to_consensus_u32()
    }
}

impl_from_core_type!(Transaction, BitcoinTransaction);
impl_from_ffi_type!(Transaction, BitcoinTransaction);

impl_string_custom_typedef!(BlockHash);
impl_string_custom_typedef!(Txid);

uniffi::include_scaffolding!("bitcoin");
