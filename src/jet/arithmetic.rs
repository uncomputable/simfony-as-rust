/* This file has been automatically generated. */

//! # Arithmetic
//!
//! This module defines jets that compute arithmetic functions.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Add two integers and return the carry.
///
/// ## Cost
///
/// 112 mWU _(milli weight units)_
pub fn add_8(a: u8, b: u8) -> (bool, u8) {
    todo!()
}

/// Add two integers and return the carry.
///
/// ## Cost
///
/// 108 mWU _(milli weight units)_
pub fn add_16(a: u16, b: u16) -> (bool, u16) {
    todo!()
}

/// Add two integers and return the carry.
///
/// ## Cost
///
/// 117 mWU _(milli weight units)_
pub fn add_32(a: u32, b: u32) -> (bool, u32) {
    todo!()
}

/// Add two integers and return the carry.
///
/// ## Cost
///
/// 109 mWU _(milli weight units)_
pub fn add_64(a: u64, b: u64) -> (bool, u64) {
    todo!()
}

/// Decrement an integer by one and return the borrow bit.
///
/// ## Cost
///
/// 79 mWU _(milli weight units)_
pub fn decrement_8(a: u8) -> (bool, u8) {
    todo!()
}

/// Decrement an integer by one and return the borrow bit.
///
/// ## Cost
///
/// 85 mWU _(milli weight units)_
pub fn decrement_16(a: u16) -> (bool, u16) {
    todo!()
}

/// Decrement an integer by one and return the borrow bit.
///
/// ## Cost
///
/// 91 mWU _(milli weight units)_
pub fn decrement_32(a: u32) -> (bool, u32) {
    todo!()
}

/// Decrement an integer by one and return the borrow bit.
///
/// ## Cost
///
/// 89 mWU _(milli weight units)_
pub fn decrement_64(a: u64) -> (bool, u64) {
    todo!()
}

/// Divide the first integer by the second integer, and return the remainder.
///
/// ## Cost
///
/// 128 mWU _(milli weight units)_
pub fn div_mod_8(a: u8, b: u8) -> (u8, u8) {
    todo!()
}

/// Divide the first integer by the second integer, and return the remainder.
///
/// ## Cost
///
/// 118 mWU _(milli weight units)_
pub fn div_mod_16(a: u16, b: u16) -> (u16, u16) {
    todo!()
}

/// Divide the first integer by the second integer, and return the remainder.
///
/// ## Cost
///
/// 115 mWU _(milli weight units)_
pub fn div_mod_32(a: u32, b: u32) -> (u32, u32) {
    todo!()
}

/// Divide the first integer by the second integer, and return the remainder.
///
/// ## Cost
///
/// 86 mWU _(milli weight units)_
pub fn div_mod_64(a: u64, b: u64) -> (u64, u64) {
    todo!()
}

/// Divide the 128-bit integer `a` by the 64-bit integer `b`.
/// Return a tuple of the quotient `q` and the remainer `r`.
///
/// Use this jet to recursively define wide integer divisions.
///
/// ## Preconditions
/// 1. `q` < 2^64
/// 2. 2^63 ≤ `b`
///
/// Return `(u64::MAX, u64::MAX)` when the preconditions are not satisfied.
///
/// ## Cost
///
/// 208 mWU _(milli weight units)_
pub fn div_mod_128_64(a: u128, b: u64) -> (u64, u64) {
    todo!()
}

/// Divide the first integer by the second integer.
///
/// ## Cost
///
/// 108 mWU _(milli weight units)_
pub fn divide_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Divide the first integer by the second integer.
///
/// ## Cost
///
/// 98 mWU _(milli weight units)_
pub fn divide_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Divide the first integer by the second integer.
///
/// ## Cost
///
/// 100 mWU _(milli weight units)_
pub fn divide_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Divide the first integer by the second integer.
///
/// ## Cost
///
/// 101 mWU _(milli weight units)_
pub fn divide_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Check if the first integer is divisible by the second integer.
///
/// ## Cost
///
/// 98 mWU _(milli weight units)_
pub fn divides_8(a: u8, b: u8) -> bool {
    todo!()
}

/// Check if the first integer is divisible by the second integer.
///
/// ## Cost
///
/// 93 mWU _(milli weight units)_
pub fn divides_16(a: u16, b: u16) -> bool {
    todo!()
}

/// Check if the first integer is divisible by the second integer.
///
/// ## Cost
///
/// 87 mWU _(milli weight units)_
pub fn divides_32(a: u32, b: u32) -> bool {
    todo!()
}

/// Check if the first integer is divisible by the second integer.
///
/// ## Cost
///
/// 91 mWU _(milli weight units)_
pub fn divides_64(a: u64, b: u64) -> bool {
    todo!()
}

/// Add two integers. Take a carry-in and return a carry-out.
///
/// ## Cost
///
/// 127 mWU _(milli weight units)_
pub fn full_add_8(a: bool, b: u8, c: u8) -> (bool, u8) {
    todo!()
}

/// Add two integers. Take a carry-in and return a carry-out.
///
/// ## Cost
///
/// 121 mWU _(milli weight units)_
pub fn full_add_16(a: bool, b: u16, c: u16) -> (bool, u16) {
    todo!()
}

/// Add two integers. Take a carry-in and return a carry-out.
///
/// ## Cost
///
/// 119 mWU _(milli weight units)_
pub fn full_add_32(a: bool, b: u32, c: u32) -> (bool, u32) {
    todo!()
}

/// Add two integers. Take a carry-in and return a carry-out.
///
/// ## Cost
///
/// 121 mWU _(milli weight units)_
pub fn full_add_64(a: bool, b: u64, c: u64) -> (bool, u64) {
    todo!()
}

/// Decrement an integer by one. Take a borrow-in and return a borrow-out.
///
/// ## Cost
///
/// 91 mWU _(milli weight units)_
pub fn full_decrement_8(a: bool, b: u8) -> (bool, u8) {
    todo!()
}

/// Decrement an integer by one. Take a borrow-in and return a borrow-out.
///
/// ## Cost
///
/// 92 mWU _(milli weight units)_
pub fn full_decrement_16(a: bool, b: u16) -> (bool, u16) {
    todo!()
}

/// Decrement an integer by one. Take a borrow-in and return a borrow-out.
///
/// ## Cost
///
/// 107 mWU _(milli weight units)_
pub fn full_decrement_32(a: bool, b: u32) -> (bool, u32) {
    todo!()
}

/// Decrement an integer by one. Take a borrow-in and return a borrow-out.
///
/// ## Cost
///
/// 81 mWU _(milli weight units)_
pub fn full_decrement_64(a: bool, b: u64) -> (bool, u64) {
    todo!()
}

/// Increment an integer by one. Take a carry-in and return a carry-out.
///
/// ## Cost
///
/// 72 mWU _(milli weight units)_
pub fn full_increment_8(a: bool, b: u8) -> (bool, u8) {
    todo!()
}

/// Increment an integer by one. Take a carry-in and return a carry-out.
///
/// ## Cost
///
/// 89 mWU _(milli weight units)_
pub fn full_increment_16(a: bool, b: u16) -> (bool, u16) {
    todo!()
}

/// Increment an integer by one. Take a carry-in and return a carry-out.
///
/// ## Cost
///
/// 104 mWU _(milli weight units)_
pub fn full_increment_32(a: bool, b: u32) -> (bool, u32) {
    todo!()
}

/// Increment an integer by one. Take a carry-in and return a carry-out.
///
/// ## Cost
///
/// 99 mWU _(milli weight units)_
pub fn full_increment_64(a: bool, b: u64) -> (bool, u64) {
    todo!()
}

/// Helper for multiplying integers. Take the product of the first pair of integers and add the sum of the second pair.
///
/// ## Cost
///
/// 109 mWU _(milli weight units)_
pub fn full_multiply_8(a: (u8, u8), b: (u8, u8)) -> u16 {
    todo!()
}

/// Helper for multiplying integers. Take the product of the first pair of integers and add the sum of the second pair.
///
/// ## Cost
///
/// 112 mWU _(milli weight units)_
pub fn full_multiply_16(a: (u16, u16), b: (u16, u16)) -> u32 {
    todo!()
}

/// Helper for multiplying integers. Take the product of the first pair of integers and add the sum of the second pair.
///
/// ## Cost
///
/// 96 mWU _(milli weight units)_
pub fn full_multiply_32(a: (u32, u32), b: (u32, u32)) -> u64 {
    todo!()
}

/// Helper for multiplying integers. Take the product of the first pair of integers and add the sum of the second pair.
///
/// ## Cost
///
/// 127 mWU _(milli weight units)_
pub fn full_multiply_64(a: (u64, u64), b: (u64, u64)) -> u128 {
    todo!()
}

/// Subtract the second integer from the first integer. Take a borrow-in and return a borrow-out.
///
/// ## Cost
///
/// 126 mWU _(milli weight units)_
pub fn full_subtract_8(a: bool, b: u8, c: u8) -> (bool, u8) {
    todo!()
}

/// Subtract the second integer from the first integer. Take a borrow-in and return a borrow-out.
///
/// ## Cost
///
/// 121 mWU _(milli weight units)_
pub fn full_subtract_16(a: bool, b: u16, c: u16) -> (bool, u16) {
    todo!()
}

/// Subtract the second integer from the first integer. Take a borrow-in and return a borrow-out.
///
/// ## Cost
///
/// 116 mWU _(milli weight units)_
pub fn full_subtract_32(a: bool, b: u32, c: u32) -> (bool, u32) {
    todo!()
}

/// Subtract the second integer from the first integer. Take a borrow-in and return a borrow-out.
///
/// ## Cost
///
/// 98 mWU _(milli weight units)_
pub fn full_subtract_64(a: bool, b: u64, c: u64) -> (bool, u64) {
    todo!()
}

/// Increment an integer by one and return the carry.
///
/// ## Cost
///
/// 85 mWU _(milli weight units)_
pub fn increment_8(a: u8) -> (bool, u8) {
    todo!()
}

/// Increment an integer by one and return the carry.
///
/// ## Cost
///
/// 69 mWU _(milli weight units)_
pub fn increment_16(a: u16) -> (bool, u16) {
    todo!()
}

/// Increment an integer by one and return the carry.
///
/// ## Cost
///
/// 92 mWU _(milli weight units)_
pub fn increment_32(a: u32) -> (bool, u32) {
    todo!()
}

/// Increment an integer by one and return the carry.
///
/// ## Cost
///
/// 87 mWU _(milli weight units)_
pub fn increment_64(a: u64) -> (bool, u64) {
    todo!()
}

/// Check if an integer is one.
///
/// ## Cost
///
/// 91 mWU _(milli weight units)_
pub fn is_one_8(a: u8) -> bool {
    todo!()
}

/// Check if an integer is one.
///
/// ## Cost
///
/// 82 mWU _(milli weight units)_
pub fn is_one_16(a: u16) -> bool {
    todo!()
}

/// Check if an integer is one.
///
/// ## Cost
///
/// 65 mWU _(milli weight units)_
pub fn is_one_32(a: u32) -> bool {
    todo!()
}

/// Check if an integer is one.
///
/// ## Cost
///
/// 83 mWU _(milli weight units)_
pub fn is_one_64(a: u64) -> bool {
    todo!()
}

/// Check if an integer is zero.
///
/// ## Cost
///
/// 77 mWU _(milli weight units)_
pub fn is_zero_8(a: u8) -> bool {
    todo!()
}

/// Check if an integer is zero.
///
/// ## Cost
///
/// 75 mWU _(milli weight units)_
pub fn is_zero_16(a: u16) -> bool {
    todo!()
}

/// Check if an integer is zero.
///
/// ## Cost
///
/// 85 mWU _(milli weight units)_
pub fn is_zero_32(a: u32) -> bool {
    todo!()
}

/// Check if an integer is zero.
///
/// ## Cost
///
/// 80 mWU _(milli weight units)_
pub fn is_zero_64(a: u64) -> bool {
    todo!()
}

/// Check if an integer is less than or equal to another integer.
///
/// ## Cost
///
/// 109 mWU _(milli weight units)_
pub fn le_8(a: u8, b: u8) -> bool {
    todo!()
}

/// Check if an integer is less than or equal to another integer.
///
/// ## Cost
///
/// 112 mWU _(milli weight units)_
pub fn le_16(a: u16, b: u16) -> bool {
    todo!()
}

/// Check if an integer is less than or equal to another integer.
///
/// ## Cost
///
/// 93 mWU _(milli weight units)_
pub fn le_32(a: u32, b: u32) -> bool {
    todo!()
}

/// Check if an integer is less than or equal to another integer.
///
/// ## Cost
///
/// 93 mWU _(milli weight units)_
pub fn le_64(a: u64, b: u64) -> bool {
    todo!()
}

/// Check if an integer is less than another integer.
///
/// ## Cost
///
/// 107 mWU _(milli weight units)_
pub fn lt_8(a: u8, b: u8) -> bool {
    todo!()
}

/// Check if an integer is less than another integer.
///
/// ## Cost
///
/// 123 mWU _(milli weight units)_
pub fn lt_16(a: u16, b: u16) -> bool {
    todo!()
}

/// Check if an integer is less than another integer.
///
/// ## Cost
///
/// 107 mWU _(milli weight units)_
pub fn lt_32(a: u32, b: u32) -> bool {
    todo!()
}

/// Check if an integer is less than another integer.
///
/// ## Cost
///
/// 76 mWU _(milli weight units)_
pub fn lt_64(a: u64, b: u64) -> bool {
    todo!()
}

/// Return the bigger of two integers.
///
/// ## Cost
///
/// 96 mWU _(milli weight units)_
pub fn max_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Return the bigger of two integers.
///
/// ## Cost
///
/// 114 mWU _(milli weight units)_
pub fn max_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Return the bigger of two integers.
///
/// ## Cost
///
/// 92 mWU _(milli weight units)_
pub fn max_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Return the bigger of two integers.
///
/// ## Cost
///
/// 104 mWU _(milli weight units)_
pub fn max_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Return the median of three integers.
///
/// ## Cost
///
/// 122 mWU _(milli weight units)_
pub fn median_8(a: u8, b: u8, c: u8) -> u8 {
    todo!()
}

/// Return the median of three integers.
///
/// ## Cost
///
/// 123 mWU _(milli weight units)_
pub fn median_16(a: u16, b: u16, c: u16) -> u16 {
    todo!()
}

/// Return the median of three integers.
///
/// ## Cost
///
/// 101 mWU _(milli weight units)_
pub fn median_32(a: u32, b: u32, c: u32) -> u32 {
    todo!()
}

/// Return the median of three integers.
///
/// ## Cost
///
/// 109 mWU _(milli weight units)_
pub fn median_64(a: u64, b: u64, c: u64) -> u64 {
    todo!()
}

/// Return the smaller of two integers.
///
/// ## Cost
///
/// 99 mWU _(milli weight units)_
pub fn min_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Return the smaller of two integers.
///
/// ## Cost
///
/// 97 mWU _(milli weight units)_
pub fn min_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Return the smaller of two integers.
///
/// ## Cost
///
/// 113 mWU _(milli weight units)_
pub fn min_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Return the smaller of two integers.
///
/// ## Cost
///
/// 102 mWU _(milli weight units)_
pub fn min_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Compute the remainder after dividing both integers.
///
/// ## Cost
///
/// 102 mWU _(milli weight units)_
pub fn modulo_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Compute the remainder after dividing both integers.
///
/// ## Cost
///
/// 103 mWU _(milli weight units)_
pub fn modulo_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Compute the remainder after dividing both integers.
///
/// ## Cost
///
/// 102 mWU _(milli weight units)_
pub fn modulo_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Compute the remainder after dividing both integers.
///
/// ## Cost
///
/// 85 mWU _(milli weight units)_
pub fn modulo_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Multiply two integers. The output is a 16-bit integer.
///
/// ## Cost
///
/// 93 mWU _(milli weight units)_
pub fn multiply_8(a: u8, b: u8) -> u16 {
    todo!()
}

/// Multiply two integers. The output is a 32-bit integer.
///
/// ## Cost
///
/// 90 mWU _(milli weight units)_
pub fn multiply_16(a: u16, b: u16) -> u32 {
    todo!()
}

/// Multiply two integers. The output is a 64-bit integer.
///
/// ## Cost
///
/// 90 mWU _(milli weight units)_
pub fn multiply_32(a: u32, b: u32) -> u64 {
    todo!()
}

/// Multiply two integers. The output is a 128-bit integer.
///
/// ## Cost
///
/// 85 mWU _(milli weight units)_
pub fn multiply_64(a: u64, b: u64) -> u128 {
    todo!()
}

/// Negate the integer (modulo 2⁸)  and return the borrow bit.
///
/// ## Cost
///
/// 91 mWU _(milli weight units)_
pub fn negate_8(a: u8) -> (bool, u8) {
    todo!()
}

/// Negate the integer (modulo 2¹⁶) and return the borrow bit.
///
/// ## Cost
///
/// 70 mWU _(milli weight units)_
pub fn negate_16(a: u16) -> (bool, u16) {
    todo!()
}

/// Negate the integer (modulo 2³²) and return the borrow bit.
///
/// ## Cost
///
/// 85 mWU _(milli weight units)_
pub fn negate_32(a: u32) -> (bool, u32) {
    todo!()
}

/// Negate the integer (modulo 2⁶⁴) and return the borrow bit.
///
/// ## Cost
///
/// 94 mWU _(milli weight units)_
pub fn negate_64(a: u64) -> (bool, u64) {
    todo!()
}

/// Return 1 as an 8-bit integer.
///
/// ## Cost
///
/// 62 mWU _(milli weight units)_
pub fn one_8() -> u8 {
    todo!()
}

/// Return 1 as a 16-bit integer.
///
/// ## Cost
///
/// 60 mWU _(milli weight units)_
pub fn one_16() -> u16 {
    todo!()
}

/// Return 1 as a 32-bit integer.
///
/// ## Cost
///
/// 59 mWU _(milli weight units)_
pub fn one_32() -> u32 {
    todo!()
}

/// Return 1 as a 64-bit integer.
///
/// ## Cost
///
/// 59 mWU _(milli weight units)_
pub fn one_64() -> u64 {
    todo!()
}

/// Subtract the second integer from the first integer, and return the borrow bit.
///
/// ## Cost
///
/// 109 mWU _(milli weight units)_
pub fn subtract_8(a: u8, b: u8) -> (bool, u8) {
    todo!()
}

/// Subtract the second integer from the first integer, and return the borrow bit.
///
/// ## Cost
///
/// 113 mWU _(milli weight units)_
pub fn subtract_16(a: u16, b: u16) -> (bool, u16) {
    todo!()
}

/// Subtract the second integer from the first integer, and return the borrow bit.
///
/// ## Cost
///
/// 118 mWU _(milli weight units)_
pub fn subtract_32(a: u32, b: u32) -> (bool, u32) {
    todo!()
}

/// Subtract the second integer from the first integer, and return the borrow bit.
///
/// ## Cost
///
/// 115 mWU _(milli weight units)_
pub fn subtract_64(a: u64, b: u64) -> (bool, u64) {
    todo!()
}
