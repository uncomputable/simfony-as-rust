/* This file has been automatically generated. */

//! # Issuance
//!
//! This module defines jets for handling issuance of Elements assets or tokens.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Calculate the issued asset id from a given entropy value.
///
/// ## Cost
///
/// 807 mWU _(milli weight units)_
pub fn calculate_asset(a: u256) -> ExplicitAsset {
    todo!()
}

/// Calculate the reissuance token id from a given entropy value for assets with confidential issued amounts.
///
/// ## Cost
///
/// 707 mWU _(milli weight units)_
pub fn calculate_confidential_token(a: u256) -> ExplicitAsset {
    todo!()
}

/// Calculate the reissuance token id from a given entropy value for assets with explicit issued amounts.
///
/// ## Cost
///
/// 771 mWU _(milli weight units)_
pub fn calculate_explicit_token(a: u256) -> ExplicitAsset {
    todo!()
}

/// Calculate the entropy value from a given outpoint and contract hash.
///
/// This entropy value is used to compute issued asset and token IDs.
///
/// ## Cost
///
/// 2095 mWU _(milli weight units)_
pub fn calculate_issuance_entropy(a: Outpoint, b: u256) -> u256 {
    todo!()
}

/// Return the kind of issuance of the input at the given index:
/// - Return `Some(Some(false))` if the input has new issuance.
/// - Return `Some(Some(true))` if the input as reissuance.
/// - Return `Some(None)` if the input has no issuance.
/// - Return `None` if the input does not exist.
///
/// ## Cost
///
/// 91 mWU _(milli weight units)_
pub fn issuance(a: u32) -> Option<Option<bool>> {
    todo!()
}

/// Return the ID of the issued asset of the input at the given index:
/// - Return `Some(Some(x))` if the input has issuance with asset id `x`.
/// - Return `Some(None)` if the input has no issuance.
/// - Return `None` if the input does not exist.
///
/// ## Cost
///
/// 151 mWU _(milli weight units)_
pub fn issuance_asset(a: u32) -> Option<Option<ExplicitAsset>> {
    todo!()
}

/// Return the issuance entropy of the input at the given index:
/// - Return `Some(Some(x))` if the input has reissuance with entropy `x` or if there is new issuance whose computed entropy is `x`.
/// - Return `Some(Some(x))` if the input has no issuance.
/// - Return `None` if the input does not exist.
///
/// ## Cost
///
/// 153 mWU _(milli weight units)_
pub fn issuance_entropy(a: u32) -> Option<Option<u256>> {
    todo!()
}

/// Return the reissuance token of the input at the given index:
/// - Return `Some(Some(x))` if the input has issuance with the reissuance token ID `x`.
/// - Return `Some(None)` if the input has no issuance.
/// - Return `None` if the input does not exist.
///
/// ## Cost
///
/// 149 mWU _(milli weight units)_
pub fn issuance_token(a: u32) -> Option<Option<ExplicitAsset>> {
    todo!()
}

/// Return the asset for Liquid Bitcoin.
///
/// ## Cost
///
/// 145 mWU _(milli weight units)_
pub fn lbtc_asset() -> u256 {
    todo!()
}
