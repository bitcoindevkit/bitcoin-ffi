use bitcoin::address::{NetworkChecked, NetworkUnchecked};
use bitcoin::Address as BitcoinAddress;
use bitcoin::Amount as BitcoinAmount;
use bitcoin::FeeRate as BitcoinFeeRate;
use bitcoin::ScriptBuf as BitcoinScriptBuf;
use bitcoin::Sequence;
use bitcoin::TxIn as BitcoinTxIn;
use bitcoin::TxOut as BitcoinTxOut;

pub use bitcoin::BlockHash;
pub use bitcoin::OutPoint;
pub use bitcoin::Txid;

use error::AddressParseError;
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

    pub fn from_script(script: Arc<ScriptBuf>, network: Network) -> Result<Self, FromScriptError> {
        let address = BitcoinAddress::from_script(&script.0.clone(), network)?;
        Ok(Address(address))
    }

    pub fn script_pubkey(&self) -> Arc<ScriptBuf> {
        Arc::new(ScriptBuf(self.0.script_pubkey()))
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
pub struct ScriptBuf(pub BitcoinScriptBuf);

impl ScriptBuf {
    pub fn new(raw_output_script: Vec<u8>) -> Self {
        let script: BitcoinScriptBuf = raw_output_script.into();
        ScriptBuf(script)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl_from_core_type!(ScriptBuf, BitcoinScriptBuf);
impl_from_ffi_type!(ScriptBuf, BitcoinScriptBuf);

#[derive(Debug, Clone)]
pub struct TxOut {
    pub value: Arc<Amount>,
    pub script_pubkey: Arc<ScriptBuf>,
}

impl From<BitcoinTxOut> for TxOut {
    fn from(tx_out: BitcoinTxOut) -> Self {
        TxOut {
            value: Arc::new(Amount(tx_out.value)),
            script_pubkey: Arc::new(ScriptBuf(tx_out.script_pubkey)),
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
    pub script_sig: Arc<ScriptBuf>,
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

impl_string_custom_typedef!(BlockHash);
impl_string_custom_typedef!(Txid);

uniffi::include_scaffolding!("bitcoin");
