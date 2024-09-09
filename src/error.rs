use bitcoin::amount::ParseAmountError as BitcoinParseAmountError;

#[derive(Debug, thiserror::Error)]
pub enum FeeRateError {
    #[error("arithmetic overflow on feerate")]
    ArithmeticOverflow,
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
