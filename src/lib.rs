use bitcoin::Amount as BitcoinAmount;
use bitcoin::FeeRate as BitcoinFeeRate;
use bitcoin::ScriptBuf as BitcoinScriptBuf;
pub use bitcoin::OutPoint as BitcoinOutPoint;
pub use bitcoin::Txid as BitcoinTxid;

use error::FeeRateError;
use error::ParseAmountError;

#[macro_use]
mod macros;
pub mod error;
pub use bitcoin::Network;

#[derive(Clone, Debug, uniffi::Object)]
pub struct FeeRate(pub BitcoinFeeRate);

#[uniffi::export]
impl FeeRate {
    #[uniffi::constructor(name = "from_sat_per_vb")]
    pub fn from_sat_per_vb(sat_per_vb: u64) -> Result<Self, FeeRateError> {
        let fee_rate: Option<BitcoinFeeRate> = BitcoinFeeRate::from_sat_per_vb(sat_per_vb);
        match fee_rate {
            Some(fee_rate) => Ok(FeeRate(fee_rate)),
            None => Err(FeeRateError::ArithmeticOverflow),
        }
    }

    #[uniffi::constructor(name = "from_sat_per_kwu")]
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

#[derive(uniffi::Object)]
pub struct Amount(pub BitcoinAmount);

#[uniffi::export]
impl Amount {    
    #[uniffi::constructor(name = "from_sat")]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Txid(pub bitcoin::Txid);

uniffi::custom_type!(Txid, String);

impl UniffiCustomTypeConverter for Txid {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Txid(val.parse::<bitcoin::Txid>().unwrap()))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

uniffi::include_scaffolding!("bitcoin");
