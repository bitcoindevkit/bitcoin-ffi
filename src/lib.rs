uniffi::setup_scaffolding!();

use bitcoin::Amount as BitcoinAmount;
use bitcoin::FeeRate as BitcoinFeeRate;
use bitcoin::ScriptBuf as BitcoinScriptBuf;
use bitcoin::TxIn as BitcoinTxIn;
use bitcoin::Weight as BitcoinWeight;
use bitcoin::Sequence as BitcoinSequence;
use bitcoin::Witness as BitcoinWitness;
use bitcoin::Transaction as BitcoinTransaction;
use bitcoin::transaction::Version as BitcoinTxVersion;
use bitcoin::locktime::absolute::LockTime as BitcoinLockTime;

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

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Script {
    buffer: Vec<u8>,
}

impl From<BitcoinScriptBuf> for Script {
    fn from(script: BitcoinScriptBuf) -> Self {
        Script {
            buffer: script.to_bytes(),
        }
    }
}

impl From<Script> for BitcoinScriptBuf {
    fn from(script: Script) -> Self {
        BitcoinScriptBuf::from(script.buffer)
    }
}

#[derive(Debug, PartialEq, Eq, uniffi::Object)]
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

impl From<bitcoin::OutPoint> for OutPoint {
    fn from(outpoint: bitcoin::OutPoint) -> Self {
        OutPoint {
            txid: Txid(outpoint.txid),
            vout: outpoint.vout,
        }
    }
}

impl From<OutPoint> for bitcoin::OutPoint {
    fn from(outpoint: OutPoint) -> Self {
        bitcoin::OutPoint {
            txid: outpoint.txid.0,
            vout: outpoint.vout,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Enum)]
#[non_exhaustive]
pub enum NetworkType {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

impl From<Network> for NetworkType {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => NetworkType::Mainnet,
            Network::Testnet => NetworkType::Testnet,
            Network::Signet => NetworkType::Signet,
            Network::Regtest => NetworkType::Regtest,
            _ => unreachable!(),
        }
    }
}

impl From<NetworkType> for Network {
    fn from(network_type: NetworkType) -> Self {
        match network_type {
            NetworkType::Mainnet => Network::Bitcoin,
            NetworkType::Testnet => Network::Testnet,
            NetworkType::Signet => Network::Signet,
            NetworkType::Regtest => Network::Regtest,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct Weight(pub BitcoinWeight);

impl_from_core_type!(Weight, BitcoinWeight);
impl_from_ffi_type!(Weight, BitcoinWeight);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sequence(pub BitcoinSequence);
uniffi::custom_type!(Sequence, u32);

impl UniffiCustomTypeConverter for Sequence {
    type Builtin = u32;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Sequence(BitcoinSequence::from_consensus(val)))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0.to_consensus_u32()
    }
}

impl_from_core_type!(Sequence, BitcoinSequence);
impl_from_ffi_type!(Sequence, BitcoinSequence);


#[derive(Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct Witness(pub BitcoinWitness);

impl_from_core_type!(Witness, BitcoinWitness);
impl_from_ffi_type!(Witness, BitcoinWitness);


#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct TxIn {
    pub previous_output: OutPoint,
    pub sequence: Sequence,
    pub script_sig: Script,
    pub witness: Vec<Vec<u8>>,
}

impl From<BitcoinTxIn> for TxIn {
    fn from(txin: BitcoinTxIn) -> Self {
        TxIn {
            previous_output: txin.previous_output.into(),
            sequence: txin.sequence.into(),
            script_sig: txin.script_sig.into(),
            witness: txin.witness.iter().map(|w| w.to_vec()).collect(),
        }
    }
}

impl From<TxIn> for BitcoinTxIn {
    fn from(txin: TxIn) -> Self {
        BitcoinTxIn {
            previous_output: txin.previous_output.into(),
            sequence: txin.sequence.into(),
            script_sig: txin.script_sig.into(),
            witness: BitcoinWitness::from(txin.witness),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct TxOut {
    pub amount: Arc<Amount>,
    pub script_pubkey: Script,
}

impl From<bitcoin::TxOut> for TxOut {
    fn from(txout: bitcoin::TxOut) -> Self {
        TxOut {
            amount: Arc::new(Amount(txout.value)),
            script_pubkey: txout.script_pubkey.into(),
        }
    }
}

impl From<TxOut> for bitcoin::TxOut {
    fn from(txout: TxOut) -> Self {
        bitcoin::TxOut {
            value: txout.amount.0,
            script_pubkey: txout.script_pubkey.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version(pub i32);

uniffi::custom_type!(Version, i32);

impl UniffiCustomTypeConverter for Version {
    type Builtin = i32;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Version(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

impl From<BitcoinTxVersion> for Version {
    fn from(version: BitcoinTxVersion) -> Self {
        Version(version.0)
    }
}

impl From<Version> for BitcoinTxVersion {
    fn from(version: Version) -> Self {
        BitcoinTxVersion(version.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LockTime(pub u32);
uniffi::custom_type!(LockTime, u32);

impl UniffiCustomTypeConverter for LockTime {
    type Builtin = u32;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(LockTime(val))
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.0
    }
}

impl From<BitcoinLockTime> for LockTime {
    fn from(locktime: BitcoinLockTime) -> Self {
        LockTime(locktime.to_consensus_u32())
    }
}

impl From<LockTime> for BitcoinLockTime {
    fn from(locktime: LockTime) -> Self {
        BitcoinLockTime::from_consensus(locktime.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct Transaction(pub BitcoinTransaction);

impl From<BitcoinTransaction> for Transaction {
    fn from(tx: BitcoinTransaction) -> Self {
        Transaction( BitcoinTransaction {
            version: tx.version.into(),
            lock_time: tx.lock_time.into(),
            input: tx.input.into_iter().map(|i| i.into()).collect(),
            output: tx.output.into_iter().map(|o| o.into()).collect(),
        })
    }
}

impl From<Transaction> for BitcoinTransaction {
    fn from(tx: Transaction) -> Self {
        BitcoinTransaction {
            version: tx.0.version.into(),
            lock_time: tx.0.lock_time.into(),
            input: tx.0.input.into_iter().map(|i| i.into()).collect(),
            output: tx.0.output.into_iter().map(|o| o.into()).collect(),
        }
    }
}


#[uniffi::export]
impl Transaction {
    #[uniffi::constructor]
    pub fn new(version: Version, lock_time: LockTime, input: Vec<TxIn>, output: Vec<TxOut>) -> Self {
        Transaction(BitcoinTransaction {
            version: version.into(),
            lock_time: lock_time.into(),
            input: input.into_iter().map(|i| i.into()).collect(),
            output: output.into_iter().map(|o| o.into()).collect(),
        })
    }

    pub fn version(&self) -> Version {
        self.0.version.into()
    }

    pub fn lock_time(&self) -> LockTime {
        self.0.lock_time.into()
    }

    pub fn input(&self) -> Vec<TxIn> {
        self.0.input.iter().map(|i| i.clone().into()).collect()
    }

    pub fn output(&self) -> Vec<TxOut> {
        self.0.output.iter().map(|o| o.clone().into()).collect()
    }

    pub fn txid(&self) -> Txid {
        Txid(self.0.compute_txid())
    }

    pub fn weight(&self) -> Weight {
        self.0.weight().into()
    }

    pub fn vsize(&self) -> u32 {
        self.0.vsize() as u32
    }
}
