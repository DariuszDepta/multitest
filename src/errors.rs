//! # Error definitions used across this library

use cosmwasm_std::{WasmMsg, WasmQuery};
use thiserror::Error;

/// An enumeration of error variants used across this library.
///
/// To create a specific error variant, please use any of the associated functions.
///
/// # Example
///
/// ```
/// use multitest::Error;
///
/// let err = Error::empty_attribute_key("MultiTest");
///
/// // use the error instance
///
/// assert_eq!("Empty attribute key. Value: MultiTest", err.to_string());
///
/// ```
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// [Error](Self) variant used for signalling an empty attribute key.
    #[error("Empty attribute key. Value: {0}")]
    EmptyAttributeKey(String),

    /// [Error](Self) variant used for signalling an empty attribute value.
    #[error("Empty attribute value. Key: {0}")]
    EmptyAttributeValue(String),

    #[error("Attribute key starts with reserved prefix _: {0}")]
    ReservedAttributeKey(String),

    #[error("Event type too short: {0}")]
    EventTypeTooShort(String),

    #[error("Unsupported wasm query: {0:?}")]
    UnsupportedWasmQuery(WasmQuery),

    #[error("Unsupported wasm message: {0:?}")]
    UnsupportedWasmMsg(WasmMsg),

    #[error("code id: invalid")]
    InvalidCodeId,

    #[error("code id {0}: no such code")]
    UnregisteredCodeId(u64),

    #[error("Contract with this address already exists: {0}")]
    DuplicatedContractAddress(String),
}

impl Error {
    /// Creates an [Error](Self) instance for empty attribute key.
    ///
    /// # Example
    ///
    /// ```
    /// use multitest::Error;
    ///
    /// let err = Error::empty_attribute_key("December");
    /// assert_eq!("Empty attribute key. Value: December", err.to_string());
    ///
    /// ```
    pub fn empty_attribute_key(value: impl Into<String>) -> Self {
        Self::EmptyAttributeKey(value.into())
    }

    pub fn empty_attribute_value(value: impl Into<String>) -> Self {
        Self::EmptyAttributeValue(value.into())
    }

    pub fn reserved_attribute_key(value: impl Into<String>) -> Self {
        Self::ReservedAttributeKey(value.into())
    }

    pub fn event_type_too_short(value: impl Into<String>) -> Self {
        Self::EventTypeTooShort(value.into())
    }

    pub fn unsupported_wasm_query(value: impl Into<WasmQuery>) -> Self {
        Self::UnsupportedWasmQuery(value.into())
    }

    pub fn unsupported_wasm_msg(value: impl Into<WasmMsg>) -> Self {
        Self::UnsupportedWasmMsg(value.into())
    }

    pub fn invalid_code_id() -> Self {
        Self::InvalidCodeId
    }

    pub fn unregistered_code_id(value: u64) -> Self {
        Self::UnregisteredCodeId(value)
    }

    pub fn duplicated_contract_address(value: impl Into<String>) -> Self {
        Self::DuplicatedContractAddress(value.into())
    }
}
