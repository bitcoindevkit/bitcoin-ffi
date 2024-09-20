pub use bitcoin::address::FromScriptError as BitcoinFromScriptError;
pub use bitcoin::address::ParseError as BitcoinParseError;
use bitcoin::amount::ParseAmountError as BitcoinParseAmountError;
use bitcoin::consensus::encode::Error as BitcoinEncodeError;
use bitcoin::hex::DisplayHex;

#[derive(Debug, thiserror::Error)]
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

#[derive(Debug, thiserror::Error)]
pub enum FeeRateError {
    #[error("arithmetic overflow on feerate")]
    ArithmeticOverflow,
}

#[derive(Debug, thiserror::Error)]
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

#[derive(Debug, thiserror::Error)]
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

#[derive(Debug, thiserror::Error)]
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
