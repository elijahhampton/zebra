//! Zebra script opcodes.

/// Supported opcodes
///
/// <https://github.com/zcash/zcash/blob/8b16094f6672d8268ff25b2d7bddd6a6207873f7/src/script/script.h#L39>
pub enum OpCode {
    /// Opcodes used to generate P2SH scripts.
    /// Returns 1 if the inputs are exactly equal, 0 otherwise.
    Equal = 0x87,
    /// The input is hashed twice: first with SHA-256 and then with RIPEMD-160.
    Hash160 = 0xa9,
    /// Pushes the next 20 bytes onto the stack.
    Push20Bytes = 0x14,
    // Additional opcodes used to generate P2PKH scripts.
    /// Duplicates the top stack item.
    Dup = 0x76,
    /// Same as OP_EQUAL, but runs OP_VERIFY afterward.
    EqualVerify = 0x88,
    /// Verifies a signature against a public key
    /// The signature used by OP_CHECKSIG must be a valid signature for the
    /// hash and public key. If it is, 1 is returned, 0 otherwise.
    CheckSig = 0xac,
}
