/* This file has been automatically generated. */

//! # Time locks
//!
//! This module defines jets for checking time locks of Elements transactions.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Assert that the value returned by [`tx_lock_distance`] is greater than or equal to the given value.
///
/// ## Panics
/// The assertion fails.
///
/// ## Cost
///
/// 105 mWU _(milli weight units)_
pub fn check_lock_distance(a: Distance) {
    todo!()
}

/// Assert that the value returned by [`tx_lock_duration`] is greater than or equal to the given value.
///
/// ## Panics
/// The assertion fails
///
/// ## Cost
///
/// 102 mWU _(milli weight units)_
pub fn check_lock_duration(a: Duration) {
    todo!()
}

/// Assert that the value returned by [`tx_lock_height`]   is greater than or equal to the given value.
///
/// ## Panics
/// The assertion fails.
///
/// ## Cost
///
/// 77 mWU _(milli weight units)_
pub fn check_lock_height(a: Height) {
    todo!()
}

/// Assert that the value returned by [`tx_lock_time`]     is greater than or equal to the given value.
///
/// ## Panics
/// The assertion fails.
///
/// ## Cost
///
/// 93 mWU _(milli weight units)_
pub fn check_lock_time(a: Time) {
    todo!()
}

/// Check if the sequence numbers of all transaction inputs are at their maximum value.
///
/// ## Cost
///
/// 71 mWU _(milli weight units)_
pub fn tx_is_final() -> bool {
    todo!()
}

/// If [`version`] returns 2 or greater, then return the greatest valid [`Distance`] value of any transaction input. Return zeroes otherwise.
///
/// ## Cost
///
/// 91 mWU _(milli weight units)_
pub fn tx_lock_distance() -> Distance {
    todo!()
}

/// If [`version`] returns 2 or greater, then return the greatest valid [`Duration`] value of any transaction input. Return zeroes otherwise.
///
/// ## Cost
///
/// 84 mWU _(milli weight units)_
pub fn tx_lock_duration() -> Duration {
    todo!()
}

/// If [`tx_is_final`] returns false, then try to parse the transaction's lock time as a [`Height`] value. Return zeroes otherwise.
///
/// ## Cost
///
/// 80 mWU _(milli weight units)_
pub fn tx_lock_height() -> Height {
    todo!()
}

/// If [`tx_is_final`] returns false, then try to parse the transaction's lock time as a [`Time`] value. Return zeroes otherwise.
///
/// ## Cost
///
/// 80 mWU _(milli weight units)_
pub fn tx_lock_time() -> Time {
    todo!()
}
