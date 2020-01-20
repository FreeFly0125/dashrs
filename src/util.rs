//! Module containing various utility functions related to processing Geometry Dash data

/// Performs RobTop's XOR en-/decoding routine on `encoded` using `key`
///
/// Note that although both `encrypted` and `key` are `str`s, the decryption
/// is done directly on the bytes, and the result of each byte-wise XOR
/// operation is casted to `char`, meaning this function only works for
/// ASCII strings.
pub fn cyclic_xor(encoded: &mut [u8], key: &str) {
    for (data_byte, key_byte) in encoded.iter_mut().zip(key.bytes().cycle()) {
        *data_byte = *data_byte ^ key_byte
    }
}
