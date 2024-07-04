/* This file has been automatically generated. */

//! # Multi-bit logic
//!
//! This module defines jets that operate on strings of bits.

#![allow(unused)]
#![allow(clippy::complexity)]

use super::*;

/// Check if the value is [`u8::MAX`].
pub fn all_8(a: u8) -> bool {
    todo!()
}

/// Check if the value is [`u16::MAX`].
pub fn all_16(a: u16) -> bool {
    todo!()
}

/// Check if the value is [`u32::MAX`].
pub fn all_32(a: u32) -> bool {
    todo!()
}

/// Check if the value is [`u64::MAX`].
pub fn all_64(a: u64) -> bool {
    todo!()
}

/// Bitwise AND of two 1-bit values.
pub fn and_1(a: u1, b: u1) -> u1 {
    todo!()
}

/// Bitwise AND of two 8-bit values.
pub fn and_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Bitwise AND of two 16-bit values.
pub fn and_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Bitwise AND of two 32-bit values
pub fn and_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Bitwise AND of two 64-bit values
pub fn and_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Bitwise CHOICE of a bit and two 1-bit values.  If the bit is true, then take the first value, else take the second value.
pub fn ch_1(a: u1, b: (u1, u1)) -> u1 {
    todo!()
}

/// Bitwise CHOICE of a bit and two 8-bit values.  If the bit is true, then take the first value, else take the second value.
pub fn ch_8(a: u8, b: (u8, u8)) -> u8 {
    todo!()
}

/// Bitwise CHOICE of a bit and two 16-bit values. If the bit is true, then take the first value, else take the second value.
pub fn ch_16(a: u16, b: (u16, u16)) -> u16 {
    todo!()
}

/// Bitwise CHOICE of a bit and two 32-bit values. If the bit is true, then take the first value, else take the second value.
pub fn ch_32(a: u32, b: (u32, u32)) -> u32 {
    todo!()
}

/// Bitwise CHOICE of a bit and two 64-bit values. If the bit is true, then take the first value, else take the second value.
pub fn ch_64(a: u64, b: (u64, u64)) -> u64 {
    todo!()
}

/// Bitwise NOT of a 1-bit  value.
pub fn complement_1(a: u1) -> u1 {
    todo!()
}

/// Bitwise NOT of an 8-bit value.
pub fn complement_8(a: u8) -> u8 {
    todo!()
}

/// Bitwise NOT of a 16-bit value.
pub fn complement_16(a: u16) -> u16 {
    todo!()
}

/// Bitwise NOT of a 32-bit value.
pub fn complement_32(a: u32) -> u32 {
    todo!()
}

/// Bitwise NOT of a 64-bit value.
pub fn complement_64(a: u64) -> u64 {
    todo!()
}

/// Check if two 1-bit values are equal.
pub fn eq_1(a: u1, b: u1) -> bool {
    todo!()
}

/// Check if two 8-bit values are equal.
pub fn eq_8(a: u8, b: u8) -> bool {
    todo!()
}

/// Check if two 16-bit values are equal.
pub fn eq_16(a: u16, b: u16) -> bool {
    todo!()
}

/// Check if two 32-bit values are equal.
pub fn eq_32(a: u32, b: u32) -> bool {
    todo!()
}

/// Check if two 64-bit values are equal.
pub fn eq_64(a: u64, b: u64) -> bool {
    todo!()
}

/// Check if two 256-bit values are equal.
pub fn eq_256(a: u256, b: u256) -> bool {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 1-bit  value into the 16-bit value. Return the shifted value and the 1  bit  that was  shifted out.
pub fn full_left_shift_16_1(a: u16, b: u1) -> (u1, u16) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 2-bit  value into the 16-bit value. Return the shifted value and the 2  bits that were shifted out.
pub fn full_left_shift_16_2(a: u16, b: u2) -> (u2, u16) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 4-bit  value into the 16-bit value. Return the shifted value and the 4  bits that were shifted out.
pub fn full_left_shift_16_4(a: u16, b: u4) -> (u4, u16) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 8-bit  value into the 16-bit value. Return the shifted value and the 8  bits that were shifted out.
pub fn full_left_shift_16_8(a: u16, b: u8) -> (u8, u16) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 1-bit  value into the 32-bit value. Return the shifted value and the 1  bit  that was  shifted out.
pub fn full_left_shift_32_1(a: u32, b: u1) -> (u1, u32) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 2-bit  value into the 32-bit value. Return the shifted value and the 2  bits that were shifted out.
pub fn full_left_shift_32_2(a: u32, b: u2) -> (u2, u32) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 4-bit  value into the 32-bit value. Return the shifted value and the 4  bits that were shifted out.
pub fn full_left_shift_32_4(a: u32, b: u4) -> (u4, u32) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 8-bit  value into the 32-bit value. Return the shifted value and the 8  bits that were shifted out.
pub fn full_left_shift_32_8(a: u32, b: u8) -> (u8, u32) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 16-bit value into the 32-bit value. Return the shifted value and the 16 bits that were shifted out.
pub fn full_left_shift_32_16(a: u32, b: u16) -> (u16, u32) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 1-bit  value into the 64-bit value. Return the shifted value and the 1  bit  that was  shifted out.
pub fn full_left_shift_64_1(a: u64, b: u1) -> (u1, u64) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 2-bit  value into the 64-bit value. Return the shifted value and the 2  bits that were shifted out.
pub fn full_left_shift_64_2(a: u64, b: u2) -> (u2, u64) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 4-bit  value into the 64-bit value. Return the shifted value and the 4  bits that were shifted out.
pub fn full_left_shift_64_4(a: u64, b: u4) -> (u4, u64) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 8-bit  value into the 64-bit value. Return the shifted value and the 8  bits that were shifted out.
pub fn full_left_shift_64_8(a: u64, b: u8) -> (u8, u64) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 16-bit value into the 64-bit value. Return the shifted value and the 16 bits that were shifted out.
pub fn full_left_shift_64_16(a: u64, b: u16) -> (u16, u64) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 32-bit value into the 64-bit value. Return the shifted value and the 32 bits that were shifted out.
pub fn full_left_shift_64_32(a: u64, b: u32) -> (u32, u64) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 1-bit  value into the 8-bit  value. Return the shifted value and the 1  bit  that was  shifted out.
pub fn full_left_shift_8_1(a: u8, b: u1) -> (u1, u8) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 2-bit  value into the 8-bit  value. Return the shifted value and the 2  bits that were shifted out.
pub fn full_left_shift_8_2(a: u8, b: u2) -> (u2, u8) {
    todo!()
}

/// Helper for left-shifting  bits. The bits are shifted from the 4-bit  value into the 8-bit  value. Return the shifted value and the 4  bits that were shifted out.
pub fn full_left_shift_8_4(a: u8, b: u4) -> (u4, u8) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 1-bit  value into the 16-bit value. Return the shifted value and the 1  bit  that was  shifted out.
pub fn full_right_shift_16_1(a: u1, b: u16) -> (u16, u1) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 2-bit  value into the 16-bit value. Return the shifted value and the 2  bits that were shifted out.
pub fn full_right_shift_16_2(a: u2, b: u16) -> (u16, u2) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 4-bit  value into the 16-bit value. Return the shifted value and the 4  bits that were shifted out.
pub fn full_right_shift_16_4(a: u4, b: u16) -> (u16, u4) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 8-bit  value into the 16-bit value. Return the shifted value and the 8  bits that were shifted out.
pub fn full_right_shift_16_8(a: u8, b: u16) -> (u16, u8) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 1-bit  value into the 32-bit value. Return the shifted value and the 1  bit  that was  shifted out.
pub fn full_right_shift_32_1(a: u1, b: u32) -> (u32, u1) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 2-bit  value into the 32-bit value. Return the shifted value and the 2  bits that were shifted out.
pub fn full_right_shift_32_2(a: u2, b: u32) -> (u32, u2) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 4-bit  value into the 32-bit value. Return the shifted value and the 4  bits that were shifted out.
pub fn full_right_shift_32_4(a: u4, b: u32) -> (u32, u4) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 8-bit  value into the 32-bit value. Return the shifted value and the 8  bits that were shifted out.
pub fn full_right_shift_32_8(a: u8, b: u32) -> (u32, u8) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 16-bit value into the 32-bit value. Return the shifted value and the 16 bits that were shifted out.
pub fn full_right_shift_32_16(a: u16, b: u32) -> (u32, u16) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 1-bit  value into the 64-bit value. Return the shifted value and the 1  bit  that was  shifted out.
pub fn full_right_shift_64_1(a: u1, b: u64) -> (u64, u1) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 2-bit  value into the 64-bit value. Return the shifted value and the 2  bits that were shifted out.
pub fn full_right_shift_64_2(a: u2, b: u64) -> (u64, u2) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 4-bit  value into the 64-bit value. Return the shifted value and the 4  bits that were shifted out.
pub fn full_right_shift_64_4(a: u4, b: u64) -> (u64, u4) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 8-bit  value into the 64-bit value. Return the shifted value and the 8  bits that were shifted out.
pub fn full_right_shift_64_8(a: u8, b: u64) -> (u64, u8) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 16-bit value into the 64-bit value. Return the shifted value and the 16 bits that were shifted out.
pub fn full_right_shift_64_16(a: u16, b: u64) -> (u64, u16) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 32-bit value into the 64-bit value. Return the shifted value and the 32 bits that were shifted out.
pub fn full_right_shift_64_32(a: u32, b: u64) -> (u64, u32) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 1-bit  value into the 8-bit  value. Return the shifted value and the 1  bit  that was  shifted out.
pub fn full_right_shift_8_1(a: u1, b: u8) -> (u8, u1) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 2-bit  value into the 8-bit  value. Return the shifted value and the 2  bits that were shifted out.
pub fn full_right_shift_8_2(a: u2, b: u8) -> (u8, u2) {
    todo!()
}

/// Helper for right-shifting bits. The bits are shifted from the 4-bit  value into the 8-bit  value. Return the shifted value and the 4  bits that were shifted out.
pub fn full_right_shift_8_4(a: u4, b: u8) -> (u8, u4) {
    todo!()
}

/// Return `u1::MAX` = 1.
pub fn high_1() -> u1 {
    todo!()
}

/// Return [`u8::MAX`].
pub fn high_8() -> u8 {
    todo!()
}

/// Return [`u16::MAX`].
pub fn high_16() -> u16 {
    todo!()
}

/// Return [`u32::MAX`].
pub fn high_32() -> u32 {
    todo!()
}

/// Return [`u64::MAX`].
pub fn high_64() -> u64 {
    todo!()
}

/// Extend a 16-bit value to a 32-bit value by padding its left with the MSB.
pub fn left_extend_16_32(a: u16) -> u32 {
    todo!()
}

/// Extend a 16-bit value to a 64-bit value by padding its left with the MSB.
pub fn left_extend_16_64(a: u16) -> u64 {
    todo!()
}

/// Extend a 1-bit  value to an 8-bit value by padding its left with the MSB.
pub fn left_extend_1_8(a: u1) -> u8 {
    todo!()
}

/// Extend a 1-bit  value to a 16-bit value by padding its left with the MSB.
pub fn left_extend_1_16(a: u1) -> u16 {
    todo!()
}

/// Extend a 1-bit  value to a 32-bit value by padding its left with the MSB.
pub fn left_extend_1_32(a: u1) -> u32 {
    todo!()
}

/// Extend a 1-bit  value to a 64-bit value by padding its left with the MSB.
pub fn left_extend_1_64(a: u1) -> u64 {
    todo!()
}

/// Extend a 16-bit value to a 64-bit value by padding its left with the MSB.
pub fn left_extend_32_64(a: u32) -> u64 {
    todo!()
}

/// Extend an 8-bit value to a 16-bit value by padding its left with the MSB.
pub fn left_extend_8_16(a: u8) -> u16 {
    todo!()
}

/// Extend an 8-bit value to a 32-bit value by padding its left with the MSB.
pub fn left_extend_8_32(a: u8) -> u32 {
    todo!()
}

/// Extend an 8-bit value to a 64-bit value by padding its left with the MSB.
pub fn left_extend_8_64(a: u8) -> u64 {
    todo!()
}

/// Extend a 16-bit value to a 32-bit value by padding its left with ones.
pub fn left_pad_high_16_32(a: u16) -> u32 {
    todo!()
}

/// Extend a 16-bit value to a 64-bit value by padding its left with ones.
pub fn left_pad_high_16_64(a: u16) -> u64 {
    todo!()
}

/// Extend a 1-bit  value to an 8-bit value by padding its left with ones.
pub fn left_pad_high_1_8(a: u1) -> u8 {
    todo!()
}

/// Extend a 1-bit  value to a 16-bit value by padding its left with ones.
pub fn left_pad_high_1_16(a: u1) -> u16 {
    todo!()
}

/// Extend a 1-bit  value to a 32-bit value by padding its left with ones.
pub fn left_pad_high_1_32(a: u1) -> u32 {
    todo!()
}

/// Extend a 1-bit  value to a 64-bit value by padding its left with ones.
pub fn left_pad_high_1_64(a: u1) -> u64 {
    todo!()
}

/// Extend a 32-bit value to a 64-bit value by padding its left with ones.
pub fn left_pad_high_32_64(a: u32) -> u64 {
    todo!()
}

/// Extend an 8-bit value to a 16-bit value by padding its left with ones.
pub fn left_pad_high_8_16(a: u8) -> u16 {
    todo!()
}

/// Extend an 8-bit value to a 32-bit value by padding its left with ones.
pub fn left_pad_high_8_32(a: u8) -> u32 {
    todo!()
}

/// Extend a 1-bit  value to a 64-bit value by padding its left with ones.
pub fn left_pad_high_8_64(a: u8) -> u64 {
    todo!()
}

/// Extend a 16-bit value to a 32-bit value by padding its left with zeroes.
pub fn left_pad_low_16_32(a: u16) -> u32 {
    todo!()
}

/// Extend a 16-bit value to a 64-bit value by padding its left with zeroes.
pub fn left_pad_low_16_64(a: u16) -> u64 {
    todo!()
}

/// Extend a 1-bit  value to an 8-bit value by padding its left with zeroes.
pub fn left_pad_low_1_8(a: u1) -> u8 {
    todo!()
}

/// Extend a 1-bit  value to a 16-bit value by padding its left with zeroes.
pub fn left_pad_low_1_16(a: u1) -> u16 {
    todo!()
}

/// Extend a 1-bit  value to a 32-bit value by padding its left with zeroes.
pub fn left_pad_low_1_32(a: u1) -> u32 {
    todo!()
}

/// Extend a 1-bit  value to a 64-bit value by padding its left with zeroes.
pub fn left_pad_low_1_64(a: u1) -> u64 {
    todo!()
}

/// Extend a 32-bit value to a 64-bit value by padding its left with zeroes.
pub fn left_pad_low_32_64(a: u32) -> u64 {
    todo!()
}

/// Extend an 8-bit value to a 16-bit value by padding its left with zeroes.
pub fn left_pad_low_8_16(a: u8) -> u16 {
    todo!()
}

/// Extend an 8-bit value to a 32-bit value by padding its left with zeroes.
pub fn left_pad_low_8_32(a: u8) -> u32 {
    todo!()
}

/// Extend an 8-bit value to a 64-bit value by padding its left with zeroes.
pub fn left_pad_low_8_64(a: u8) -> u64 {
    todo!()
}

/// Left-rotate an 8-bit value by the given amount.
pub fn left_rotate_8(a: u4, b: u8) -> u8 {
    todo!()
}

/// Left-rotate a 16-bit value by the given amount.
pub fn left_rotate_16(a: u4, b: u16) -> u16 {
    todo!()
}

/// Left-rotate a 32-bit value by the given amount.
pub fn left_rotate_32(a: u8, b: u32) -> u32 {
    todo!()
}

/// Left-rotate a 64-bit value by the given amount.
pub fn left_rotate_64(a: u8, b: u64) -> u64 {
    todo!()
}

/// Left-shift an 8-bit value by the given amount. Bits are filled with zeroes.
pub fn left_shift_8(a: u4, b: u8) -> u8 {
    todo!()
}

/// Left-shift a 16-bit value by the given amount. Bits are filled with zeroes.
pub fn left_shift_16(a: u4, b: u16) -> u16 {
    todo!()
}

/// Left-shift a 32-bit value by the given amount. Bits are filled with zeroes.
pub fn left_shift_32(a: u8, b: u32) -> u32 {
    todo!()
}

/// Left-shift a 64-bit value by the given amount. Bits are filled with zeroes.
pub fn left_shift_64(a: u8, b: u64) -> u64 {
    todo!()
}

/// Left-shift an 8-bit value by the given amount. Bits are filled with the given bit.
pub fn left_shift_with_8(a: u1, b: (u4, u8)) -> u8 {
    todo!()
}

/// Left-shift a 16-bit value by the given amount. Bits are filled with the given bit.
pub fn left_shift_with_16(a: u1, b: (u4, u16)) -> u16 {
    todo!()
}

/// Left-shift a 32-bit value by the given amount. Bits are filled with the given bit.
pub fn left_shift_with_32(a: u1, b: (u8, u32)) -> u32 {
    todo!()
}

/// Left-shift a 64-bit value by the given amount. Bits are filled with the given bit.
pub fn left_shift_with_64(a: u1, b: (u8, u64)) -> u64 {
    todo!()
}

/// Return the most significant 1  bit  of a 16-bit value.
pub fn leftmost_16_1(a: u16) -> u1 {
    todo!()
}

/// Return the most significant 2  bits of a 16-bit value.
pub fn leftmost_16_2(a: u16) -> u2 {
    todo!()
}

/// Return the most significant 4  bits of a 16-bit value.
pub fn leftmost_16_4(a: u16) -> u4 {
    todo!()
}

/// Return the most significant 8  bits of a 16-bit value.
pub fn leftmost_16_8(a: u16) -> u8 {
    todo!()
}

/// Return the most significant 1  bit  of a 32-bit value.
pub fn leftmost_32_1(a: u32) -> u1 {
    todo!()
}

/// Return the most significant 2  bits of a 32-bit value.
pub fn leftmost_32_2(a: u32) -> u2 {
    todo!()
}

/// Return the most significant 4  bits of a 32-bit value.
pub fn leftmost_32_4(a: u32) -> u4 {
    todo!()
}

/// Return the most significant 8  bits of a 32-bit value.
pub fn leftmost_32_8(a: u32) -> u8 {
    todo!()
}

/// Return the most significant 16 bits of a 32-bit value.
pub fn leftmost_32_16(a: u32) -> u16 {
    todo!()
}

/// Return the most significant 1  bits of a 64-bit value.
pub fn leftmost_64_1(a: u64) -> u1 {
    todo!()
}

/// Return the most significant 2  bits of a 64-bit value.
pub fn leftmost_64_2(a: u64) -> u2 {
    todo!()
}

/// Return the most significant 4  bits of a 64-bit value.
pub fn leftmost_64_4(a: u64) -> u4 {
    todo!()
}

/// Return the most significant 8  bits of a 64-bit value.
pub fn leftmost_64_8(a: u64) -> u8 {
    todo!()
}

/// Return the most significant 16 bits of a 64-bit value.
pub fn leftmost_64_16(a: u64) -> u16 {
    todo!()
}

/// Return the most significant 32 bits of a 64-bit value.
pub fn leftmost_64_32(a: u64) -> u32 {
    todo!()
}

/// Return the most significant 1  bits of an 8-bit value.
pub fn leftmost_8_1(a: u8) -> u1 {
    todo!()
}

/// Return the most significant 1  bits of an 8-bit value.
pub fn leftmost_8_2(a: u8) -> u2 {
    todo!()
}

/// Return the most significant 1  bits of an 8-bit value.
pub fn leftmost_8_4(a: u8) -> u4 {
    todo!()
}

/// Return `u1::MIN` = 1.
pub fn low_1() -> u1 {
    todo!()
}

/// Return [`u8::MIN`].
pub fn low_8() -> u8 {
    todo!()
}

/// Return [`u16::MIN`].
pub fn low_16() -> u16 {
    todo!()
}

/// Return [`u32::MIN`].
pub fn low_32() -> u32 {
    todo!()
}

/// Return [`u64::MIN`].
pub fn low_64() -> u64 {
    todo!()
}

/// Bitwise MAJORITY of three 1-bit values. The output bit is false if two or more input bits are false, and true otherwise.
pub fn maj_1(a: u1, b: (u1, u1)) -> u1 {
    todo!()
}

/// Bitwise MAJORITY of three 1-bit values. The output bit is false if two or more input bits are false, and true otherwise.
pub fn maj_8(a: u8, b: (u8, u8)) -> u8 {
    todo!()
}

/// Bitwise MAJORITY of three 1-bit values. The output bit is false if two or more input bits are false, and true otherwise.
pub fn maj_16(a: u16, b: (u16, u16)) -> u16 {
    todo!()
}

/// Bitwise MAJORITY of three 1-bit values. The output bit is false if two or more input bits are false, and true otherwise.
pub fn maj_32(a: u32, b: (u32, u32)) -> u32 {
    todo!()
}

/// Bitwise MAJORITY of three 1-bit values. The output bit is false if two or more input bits are false, and true otherwise.
pub fn maj_64(a: u64, b: (u64, u64)) -> u64 {
    todo!()
}

/// Bitwise OR of two 1-bit values.
pub fn or_1(a: u1, b: u1) -> u1 {
    todo!()
}

/// Bitwise OR of two 8-bit values.
pub fn or_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Bitwise OR of two 16-bit values.
pub fn or_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Bitwise OR of two 32-bit values.
pub fn or_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Bitwise OR of two 64-bit values.
pub fn or_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Extend a 16-bit value to a 32-bit value by padding its right with the MSB.
pub fn right_extend_16_32(a: u16) -> u32 {
    todo!()
}

/// Extend a 16-bit value to a 64-bit value by padding its right with the MSB.
pub fn right_extend_16_64(a: u16) -> u64 {
    todo!()
}

/// Extend a 16-bit value to a 64-bit value by padding its right with the MSB.
pub fn right_extend_32_64(a: u32) -> u64 {
    todo!()
}

/// Extend an 8-bit value to a 16-bit value by padding its right with the MSB.
pub fn right_extend_8_16(a: u8) -> u16 {
    todo!()
}

/// Extend an 8-bit value to a 32-bit value by padding its right with the MSB.
pub fn right_extend_8_32(a: u8) -> u32 {
    todo!()
}

/// Extend an 8-bit value to a 64-bit value by padding its right with the MSB.
pub fn right_extend_8_64(a: u8) -> u64 {
    todo!()
}

/// Extend a 16-bit value to a 32-bit value by padding its right with ones.
pub fn right_pad_high_16_32(a: u16) -> u32 {
    todo!()
}

/// Extend a 16-bit value to a 64-bit value by padding its right with ones.
pub fn right_pad_high_16_64(a: u16) -> u64 {
    todo!()
}

/// Extend a 1-bit  value to an 8-bit value by padding its right with ones.
pub fn right_pad_high_1_8(a: u1) -> u8 {
    todo!()
}

/// Extend a 1-bit  value to a 16-bit value by padding its right with ones.
pub fn right_pad_high_1_16(a: u1) -> u16 {
    todo!()
}

/// Extend a 1-bit  value to a 32-bit value by padding its right with ones.
pub fn right_pad_high_1_32(a: u1) -> u32 {
    todo!()
}

/// Extend a 1-bit  value to a 64-bit value by padding its right with ones.
pub fn right_pad_high_1_64(a: u1) -> u64 {
    todo!()
}

/// Extend a 32-bit value to a 64-bit value by padding its right with ones.
pub fn right_pad_high_32_64(a: u32) -> u64 {
    todo!()
}

/// Extend an 8-bit  value to a 16-bit value by padding its right with ones.
pub fn right_pad_high_8_16(a: u8) -> u16 {
    todo!()
}

/// Extend an 8-bit  value to a 32-bit value by padding its right with ones.
pub fn right_pad_high_8_32(a: u8) -> u32 {
    todo!()
}

/// Extend a 1-bit  value to a 64-bit value by padding its right with ones.
pub fn right_pad_high_8_64(a: u8) -> u64 {
    todo!()
}

/// Extend a 16-bit value to a 32-bit value by padding its right with zeroes.
pub fn right_pad_low_16_32(a: u16) -> u32 {
    todo!()
}

/// Extend a 16-bit value to a 64-bit value by padding its right with zeroes.
pub fn right_pad_low_16_64(a: u16) -> u64 {
    todo!()
}

/// Extend a 1-bit  value to an 8-bit value by padding its right with zeroes.
pub fn right_pad_low_1_8(a: u1) -> u8 {
    todo!()
}

/// Extend a 1-bit  value to a 16-bit value by padding its right with zeroes.
pub fn right_pad_low_1_16(a: u1) -> u16 {
    todo!()
}

/// Extend a 1-bit  value to a 32-bit value by padding its right with zeroes.
pub fn right_pad_low_1_32(a: u1) -> u32 {
    todo!()
}

/// Extend a 1-bit  value to a 64-bit value by padding its right with zeroes.
pub fn right_pad_low_1_64(a: u1) -> u64 {
    todo!()
}

/// Extend a 32-bit value to a 64-bit value by padding its right with zeroes.
pub fn right_pad_low_32_64(a: u32) -> u64 {
    todo!()
}

/// Extend an 8-bit value to a 16-bit value by padding its right with zeroes.
pub fn right_pad_low_8_16(a: u8) -> u16 {
    todo!()
}

/// Extend an 8-bit value to a 32-bit value by padding its right with zeroes.
pub fn right_pad_low_8_32(a: u8) -> u32 {
    todo!()
}

/// Extend an 8-bit value to a 64-bit value by padding its right with zeroes.
pub fn right_pad_low_8_64(a: u8) -> u64 {
    todo!()
}

/// Right-rotate an 8-bit value by the given amount.
pub fn right_rotate_8(a: u4, b: u8) -> u8 {
    todo!()
}

/// Right-rotate a 16-bit value by the given amount.
pub fn right_rotate_16(a: u4, b: u16) -> u16 {
    todo!()
}

/// Right-rotate a 32-bit value by the given amount.
pub fn right_rotate_32(a: u8, b: u32) -> u32 {
    todo!()
}

/// Right-rotate a 64-bit value by the given amount.
pub fn right_rotate_64(a: u8, b: u64) -> u64 {
    todo!()
}

/// Right-shift an 8-bit value by the given amount. Bits are filled with zeroes.
pub fn right_shift_8(a: u4, b: u8) -> u8 {
    todo!()
}

/// Right-shift a 16-bit value by the given amount. Bits are filled with zeroes.
pub fn right_shift_16(a: u4, b: u16) -> u16 {
    todo!()
}

/// Right-shift a 32-bit value by the given amount. Bits are filled with zeroes.
pub fn right_shift_32(a: u8, b: u32) -> u32 {
    todo!()
}

/// Right-shift a 64-bit value by the given amount. Bits are filled with zeroes.
pub fn right_shift_64(a: u8, b: u64) -> u64 {
    todo!()
}

/// Right-shift an 8-bit value by the given amount. Bits are filled with the given bit.
pub fn right_shift_with_8(a: u1, b: (u4, u8)) -> u8 {
    todo!()
}

/// Right-shift a 16-bit value by the given amount. Bits are filled with the given bit.
pub fn right_shift_with_16(a: u1, b: (u4, u16)) -> u16 {
    todo!()
}

/// Right-shift a 32-bit value by the given amount. Bits are filled with the given bit.
pub fn right_shift_with_32(a: u1, b: (u8, u32)) -> u32 {
    todo!()
}

/// Right-shift a 64-bit value by the given amount. Bits are filled with the given bit.
pub fn right_shift_with_64(a: u1, b: (u8, u64)) -> u64 {
    todo!()
}

/// Return the least significant 1  bit  of a 16-bit value.
pub fn rightmost_16_1(a: u16) -> u1 {
    todo!()
}

/// Return the least significant 2  bits of a 16-bit value.
pub fn rightmost_16_2(a: u16) -> u2 {
    todo!()
}

/// Return the least significant 4  bits of a 16-bit value.
pub fn rightmost_16_4(a: u16) -> u4 {
    todo!()
}

/// Return the least significant 8  bits of a 16-bit value.
pub fn rightmost_16_8(a: u16) -> u8 {
    todo!()
}

/// Return the least significant 1  bit  of a 32-bit value.
pub fn rightmost_32_1(a: u32) -> u1 {
    todo!()
}

/// Return the least significant 2  bits of a 32-bit value.
pub fn rightmost_32_2(a: u32) -> u2 {
    todo!()
}

/// Return the least significant 4  bits of a 32-bit value.
pub fn rightmost_32_4(a: u32) -> u4 {
    todo!()
}

/// Return the least significant 8  bits of a 32-bit value.
pub fn rightmost_32_8(a: u32) -> u8 {
    todo!()
}

/// Return the least significant 16 bits of a 32-bit value.
pub fn rightmost_32_16(a: u32) -> u16 {
    todo!()
}

/// Return the least significant 1  bits of a 64-bit value.
pub fn rightmost_64_1(a: u64) -> u1 {
    todo!()
}

/// Return the least significant 2  bits of a 64-bit value.
pub fn rightmost_64_2(a: u64) -> u2 {
    todo!()
}

/// Return the least significant 4  bits of a 64-bit value.
pub fn rightmost_64_4(a: u64) -> u4 {
    todo!()
}

/// Return the least significant 8  bits of a 64-bit value.
pub fn rightmost_64_8(a: u64) -> u8 {
    todo!()
}

/// Return the least significant 16 bits of a 64-bit value.
pub fn rightmost_64_16(a: u64) -> u16 {
    todo!()
}

/// Return the least significant 32 bits of a 64-bit value.
pub fn rightmost_64_32(a: u64) -> u32 {
    todo!()
}

/// Return the least significant 1  bits of an 8-bit value.
pub fn rightmost_8_1(a: u8) -> u1 {
    todo!()
}

/// Return the least significant 1  bits of an 8-bit value.
pub fn rightmost_8_2(a: u8) -> u2 {
    todo!()
}

/// Return the least significant 1  bits of an 8-bit value.
pub fn rightmost_8_4(a: u8) -> u4 {
    todo!()
}

/// Check if a 1-bit  value is nonzero.
pub fn some_1(a: u1) -> bool {
    todo!()
}

/// Check if an 8-bit value is nonzero.
pub fn some_8(a: u8) -> bool {
    todo!()
}

/// Check if a 16-bit value is nonzero.
pub fn some_16(a: u16) -> bool {
    todo!()
}

/// Check if a 32-bit value is nonzero.
pub fn some_32(a: u32) -> bool {
    todo!()
}

/// Check if a 64-bit value is nonzero.
pub fn some_64(a: u64) -> bool {
    todo!()
}

/// Assert that a bit is true or panic otherwise.
pub fn verify(a: bool) {
    todo!()
}

/// Bitwise XOR of two 1-bit  values.
pub fn xor_1(a: u1, b: u1) -> u1 {
    todo!()
}

/// Bitwise XOR of two 8-bit  values.
pub fn xor_8(a: u8, b: u8) -> u8 {
    todo!()
}

/// Bitwise XOR of two 16-bit values.
pub fn xor_16(a: u16, b: u16) -> u16 {
    todo!()
}

/// Bitwise XOR of two 32-bit values.
pub fn xor_32(a: u32, b: u32) -> u32 {
    todo!()
}

/// Bitwise XOR of two 64-bit values.
pub fn xor_64(a: u64, b: u64) -> u64 {
    todo!()
}

/// Bitwise XOR of three 1-bit  values.
pub fn xor_xor_1(a: u1, b: (u1, u1)) -> u1 {
    todo!()
}

/// Bitwise XOR of three 8-bit  values.
pub fn xor_xor_8(a: u8, b: (u8, u8)) -> u8 {
    todo!()
}

/// Bitwise XOR of three 16-bit values.
pub fn xor_xor_16(a: u16, b: (u16, u16)) -> u16 {
    todo!()
}

/// Bitwise XOR of three 32-bit values.
pub fn xor_xor_32(a: u32, b: (u32, u32)) -> u32 {
    todo!()
}

/// Bitwise XOR of three 64-bit values.
pub fn xor_xor_64(a: u64, b: (u64, u64)) -> u64 {
    todo!()
}
