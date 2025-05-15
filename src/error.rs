pub use bitcoin::address::FromScriptError as BitcoinFromScriptError;
pub use bitcoin::address::ParseError as BitcoinParseError;
use bitcoin::amount::ParseAmountError as BitcoinParseAmountError;
use bitcoin::consensus::encode::Error as BitcoinEncodeError;
use bitcoin::hex::DisplayHex;
use bitcoin::psbt::Error as BitcoinPsbtError;
use bitcoin::psbt::ExtractTxError as BitcoinExtractTxError;
use bitcoin::psbt::PsbtParseError as BitcoinPsbtParseError;

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum AddressParseError {
    #[error("base58 address encoding error")]
    Base58,
    #[error("bech32 address encoding error")]
    Bech32,
    #[error("witness version conversion/parsing error: {error_message}")]
    WitnessVersion { error_message: String },
    #[error("witness program error: {error_message}")]
    WitnessProgram { error_message: String },
    #[error("tried to parse an unknown hrp")]
    UnknownHrp,
    #[error("legacy address base58 string")]
    LegacyAddressTooLong,
    #[error("legacy address base58 data")]
    InvalidBase58PayloadLength,
    #[error("segwit address bech32 string")]
    InvalidLegacyPrefix,
    #[error("validation error")]
    NetworkValidation,
    #[error("other address parse error")]
    OtherAddressParseErr,
}

impl From<BitcoinParseError> for AddressParseError {
    fn from(error: BitcoinParseError) -> Self {
        match error {
            BitcoinParseError::Base58(_) => AddressParseError::Base58,
            BitcoinParseError::Bech32(_) => AddressParseError::Bech32,
            BitcoinParseError::WitnessVersion(e) => AddressParseError::WitnessVersion {
                error_message: e.to_string(),
            },
            BitcoinParseError::WitnessProgram(e) => AddressParseError::WitnessProgram {
                error_message: e.to_string(),
            },
            BitcoinParseError::UnknownHrp(_) => AddressParseError::UnknownHrp,
            BitcoinParseError::LegacyAddressTooLong(_) => AddressParseError::LegacyAddressTooLong,
            BitcoinParseError::InvalidBase58PayloadLength(_) => {
                AddressParseError::InvalidBase58PayloadLength
            }
            BitcoinParseError::InvalidLegacyPrefix(_) => AddressParseError::InvalidLegacyPrefix,
            BitcoinParseError::NetworkValidation(_) => AddressParseError::NetworkValidation,
            _ => AddressParseError::OtherAddressParseErr,
        }
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FeeRateError {
    #[error("arithmetic overflow on feerate")]
    ArithmeticOverflow,
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FromScriptError {
    #[error("script is not a p2pkh, p2sh or witness program")]
    UnrecognizedScript,

    #[error("witness program error: {error_message}")]
    WitnessProgram { error_message: String },

    #[error("witness version construction error: {error_message}")]
    WitnessVersion { error_message: String },

    // This error is required because the bitcoin::address::FromScriptError is non-exhaustive
    #[error("other from script error")]
    OtherFromScriptErr,
}

impl From<BitcoinFromScriptError> for FromScriptError {
    fn from(error: BitcoinFromScriptError) -> Self {
        match error {
            BitcoinFromScriptError::UnrecognizedScript => FromScriptError::UnrecognizedScript,
            BitcoinFromScriptError::WitnessProgram(e) => FromScriptError::WitnessProgram {
                error_message: e.to_string(),
            },
            BitcoinFromScriptError::WitnessVersion(e) => FromScriptError::WitnessVersion {
                error_message: e.to_string(),
            },
            _ => FromScriptError::OtherFromScriptErr,
        }
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ParseAmountError {
    #[error("amount out of range")]
    OutOfRange,

    #[error("amount has a too high precision")]
    TooPrecise,

    #[error("the input has too few digits")]
    MissingDigits,

    #[error("the input is too large")]
    InputTooLarge,

    #[error("invalid character: {error_message}")]
    InvalidCharacter { error_message: String },

    // Has to handle non-exhaustive
    #[error("unknown parse amount error")]
    OtherParseAmountErr,
}

impl From<BitcoinParseAmountError> for ParseAmountError {
    fn from(error: BitcoinParseAmountError) -> Self {
        match error {
            BitcoinParseAmountError::OutOfRange(_) => ParseAmountError::OutOfRange,
            BitcoinParseAmountError::TooPrecise(_) => ParseAmountError::TooPrecise,
            BitcoinParseAmountError::MissingDigits(_) => ParseAmountError::MissingDigits,
            BitcoinParseAmountError::InputTooLarge(_) => ParseAmountError::InputTooLarge,
            BitcoinParseAmountError::InvalidCharacter(c) => ParseAmountError::InvalidCharacter {
                error_message: c.to_string(),
            },
            _ => ParseAmountError::OtherParseAmountErr,
        }
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum EncodeError {
    #[error("io error")]
    Io,
    #[error("allocation of oversized vector")]
    OversizedVectorAllocation,
    #[error("invalid checksum: expected={expected} actual={actual}")]
    InvalidChecksum { expected: String, actual: String },
    #[error("non-minimal var int")]
    NonMinimalVarInt,
    #[error("parse failed")]
    ParseFailed,
    #[error("unsupported segwit version: {flag}")]
    UnsupportedSegwitFlag { flag: u8 },
    // This is required because the bdk::bitcoin::consensus::encode::Error is non-exhaustive
    #[error("other encoding error")]
    OtherEncodeErr,
}

impl From<BitcoinEncodeError> for EncodeError {
    fn from(error: BitcoinEncodeError) -> Self {
        match error {
            BitcoinEncodeError::Io(_) => EncodeError::Io,
            BitcoinEncodeError::OversizedVectorAllocation { .. } => {
                EncodeError::OversizedVectorAllocation
            }
            BitcoinEncodeError::InvalidChecksum { expected, actual } => {
                EncodeError::InvalidChecksum {
                    expected: DisplayHex::to_lower_hex_string(&expected),
                    actual: DisplayHex::to_lower_hex_string(&actual),
                }
            }
            BitcoinEncodeError::NonMinimalVarInt => EncodeError::NonMinimalVarInt,
            BitcoinEncodeError::ParseFailed(_) => EncodeError::ParseFailed,
            BitcoinEncodeError::UnsupportedSegwitFlag(flag) => {
                EncodeError::UnsupportedSegwitFlag { flag }
            }
            _ => EncodeError::OtherEncodeErr,
        }
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum PsbtError {
    #[error("invalid magic")]
    InvalidMagic,
    #[error("UTXO information is not present in PSBT")]
    MissingUtxo,
    #[error("invalid separator")]
    InvalidSeparator,
    #[error("output index is out of bounds of non witness script output array")]
    PsbtUtxoOutOfBounds,
    #[error("invalid key: {key}")]
    InvalidKey { key: String },
    #[error("non-proprietary key type found when proprietary key was expected")]
    InvalidProprietaryKey,
    #[error("duplicate key: {key}")]
    DuplicateKey { key: String },
    #[error("the unsigned transaction has script sigs")]
    UnsignedTxHasScriptSigs,
    #[error("the unsigned transaction has script witnesses")]
    UnsignedTxHasScriptWitnesses,
    #[error("partially signed transactions must have an unsigned transaction")]
    MustHaveUnsignedTx,
    #[error("no more key-value pairs for this psbt map")]
    NoMorePairs,
    #[error("different unsigned transaction")]
    UnexpectedUnsignedTx,
    #[error("non-standard sighash type: {sighash}")]
    NonStandardSighashType { sighash: u32 },
    #[error("invalid hash when parsing slice: {hash}")]
    InvalidHash { hash: String },
    #[error("preimage does not match")]
    InvalidPreimageHashPair,
    #[error("combine conflict: {xpub}")]
    CombineInconsistentKeySources { xpub: String },
    #[error("bitcoin consensus encoding error: {encoding_error}")]
    ConsensusEncoding { encoding_error: String },
    #[error("PSBT has a negative fee which is not allowed")]
    NegativeFee,
    #[error("integer overflow in fee calculation")]
    FeeOverflow,
    #[error("invalid public key {error_message}")]
    InvalidPublicKey { error_message: String },
    #[error("invalid secp256k1 public key: {secp256k1_error}")]
    InvalidSecp256k1PublicKey { secp256k1_error: String },
    #[error("invalid xonly public key")]
    InvalidXOnlyPublicKey,
    #[error("invalid ECDSA signature: {error_message}")]
    InvalidEcdsaSignature { error_message: String },
    #[error("invalid taproot signature: {error_message}")]
    InvalidTaprootSignature { error_message: String },
    #[error("invalid control block")]
    InvalidControlBlock,
    #[error("invalid leaf version")]
    InvalidLeafVersion,
    #[error("taproot error")]
    Taproot,
    #[error("taproot tree error: {error_message}")]
    TapTree { error_message: String },
    #[error("xpub key error")]
    XPubKey,
    #[error("version error: {error_message}")]
    Version { error_message: String },
    #[error("data not consumed entirely when explicitly deserializing")]
    PartialDataConsumption,
    #[error("I/O error: {error_message}")]
    Io { error_message: String },
    #[error("other PSBT error")]
    OtherPsbtErr,
}

impl From<BitcoinPsbtError> for PsbtError {
    fn from(error: BitcoinPsbtError) -> Self {
        match error {
            BitcoinPsbtError::InvalidMagic => PsbtError::InvalidMagic,
            BitcoinPsbtError::MissingUtxo => PsbtError::MissingUtxo,
            BitcoinPsbtError::InvalidSeparator => PsbtError::InvalidSeparator,
            BitcoinPsbtError::PsbtUtxoOutOfbounds => PsbtError::PsbtUtxoOutOfBounds,
            BitcoinPsbtError::InvalidKey(key) => PsbtError::InvalidKey {
                key: key.to_string(),
            },
            BitcoinPsbtError::InvalidProprietaryKey => PsbtError::InvalidProprietaryKey,
            BitcoinPsbtError::DuplicateKey(key) => PsbtError::DuplicateKey {
                key: key.to_string(),
            },
            BitcoinPsbtError::UnsignedTxHasScriptSigs => PsbtError::UnsignedTxHasScriptSigs,
            BitcoinPsbtError::UnsignedTxHasScriptWitnesses => {
                PsbtError::UnsignedTxHasScriptWitnesses
            }
            BitcoinPsbtError::MustHaveUnsignedTx => PsbtError::MustHaveUnsignedTx,
            BitcoinPsbtError::NoMorePairs => PsbtError::NoMorePairs,
            BitcoinPsbtError::UnexpectedUnsignedTx { .. } => PsbtError::UnexpectedUnsignedTx,
            BitcoinPsbtError::NonStandardSighashType(sighash) => {
                PsbtError::NonStandardSighashType { sighash }
            }
            BitcoinPsbtError::InvalidHash(hash) => PsbtError::InvalidHash {
                hash: hash.to_string(),
            },
            BitcoinPsbtError::InvalidPreimageHashPair { .. } => PsbtError::InvalidPreimageHashPair,
            BitcoinPsbtError::CombineInconsistentKeySources(xpub) => {
                PsbtError::CombineInconsistentKeySources {
                    xpub: xpub.to_string(),
                }
            }
            BitcoinPsbtError::ConsensusEncoding(encoding_error) => PsbtError::ConsensusEncoding {
                encoding_error: encoding_error.to_string(),
            },
            BitcoinPsbtError::NegativeFee => PsbtError::NegativeFee,
            BitcoinPsbtError::FeeOverflow => PsbtError::FeeOverflow,
            BitcoinPsbtError::InvalidPublicKey(e) => PsbtError::InvalidPublicKey {
                error_message: e.to_string(),
            },
            BitcoinPsbtError::InvalidSecp256k1PublicKey(e) => {
                PsbtError::InvalidSecp256k1PublicKey {
                    secp256k1_error: e.to_string(),
                }
            }
            BitcoinPsbtError::InvalidXOnlyPublicKey => PsbtError::InvalidXOnlyPublicKey,
            BitcoinPsbtError::InvalidEcdsaSignature(e) => PsbtError::InvalidEcdsaSignature {
                error_message: e.to_string(),
            },
            BitcoinPsbtError::InvalidTaprootSignature(e) => PsbtError::InvalidTaprootSignature {
                error_message: e.to_string(),
            },
            BitcoinPsbtError::InvalidControlBlock => PsbtError::InvalidControlBlock,
            BitcoinPsbtError::InvalidLeafVersion => PsbtError::InvalidLeafVersion,
            BitcoinPsbtError::Taproot(_) => PsbtError::Taproot,
            BitcoinPsbtError::TapTree(e) => PsbtError::TapTree {
                error_message: e.to_string(),
            },
            BitcoinPsbtError::XPubKey(_) => PsbtError::XPubKey,
            BitcoinPsbtError::Version(e) => PsbtError::Version {
                error_message: e.to_string(),
            },
            BitcoinPsbtError::PartialDataConsumption => PsbtError::PartialDataConsumption,
            BitcoinPsbtError::Io(e) => PsbtError::Io {
                error_message: e.to_string(),
            },
            _ => PsbtError::OtherPsbtErr,
        }
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum PsbtParseError {
    #[error("error in internal psbt data structure: {error_message}")]
    PsbtEncoding { error_message: String },
    #[error("error in psbt base64 encoding: {error_message}")]
    Base64Encoding { error_message: String },
}

impl From<BitcoinPsbtParseError> for PsbtParseError {
    fn from(error: BitcoinPsbtParseError) -> Self {
        match error {
            BitcoinPsbtParseError::PsbtEncoding(e) => PsbtParseError::PsbtEncoding {
                error_message: e.to_string(),
            },
            BitcoinPsbtParseError::Base64Encoding(e) => PsbtParseError::Base64Encoding {
                error_message: e.to_string(),
            },
            _ => {
                unreachable!("this is required because of the non-exhaustive enum in rust-bitcoin")
            }
        }
    }
}

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum ExtractTxError {
    #[error("feerate is too high {fee_rate}")]
    AbsurdFeeRate { fee_rate: String },
    #[error("input[s] are missing information")]
    MissingInputValue,
    #[error("input is less than the output value")]
    SendingTooMuch,
    #[error("other extract tx error")]
    OtherExtractTxErr,
}

impl From<BitcoinExtractTxError> for ExtractTxError {
    fn from(error: BitcoinExtractTxError) -> Self {
        match error {
            BitcoinExtractTxError::AbsurdFeeRate { fee_rate, .. } => {
                ExtractTxError::AbsurdFeeRate {
                    fee_rate: fee_rate.to_string(),
                }
            }
            BitcoinExtractTxError::MissingInputValue { .. } => ExtractTxError::MissingInputValue,
            BitcoinExtractTxError::SendingTooMuch { .. } => ExtractTxError::SendingTooMuch,
            _ => ExtractTxError::OtherExtractTxErr,
        }
    }
}
