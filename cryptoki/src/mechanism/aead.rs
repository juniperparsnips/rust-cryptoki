// Copyright 2023 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! AEAD block cipher mechanism types

use crate::types::Ulong;

/// Parameters for AES-GCM.
#[derive(Debug, Clone, Copy)]
pub struct GcmParams<'a> {
    iv: &'a [u8],
    aad: &'a [u8],
    tag_bits: Ulong,
}

impl<'a> GcmParams<'a> {
    /// Construct GCM parameters.
    ///
    /// # Arguments
    ///
    /// `iv` - The initialization vector.  This must be non-empty.  In PKCS#11
    /// 2.40, the maximum length of the IV is 256 bytes.  A 12-byte IV may be
    /// processed more efficiently than other lengths.
    ///
    /// `aad` - The additional authenticated data.  This data is authenticated
    /// but not encrypted.  This may be between 0 and 2^32-1 bytes.
    ///
    /// `tag_bits` - The length, in **bits**, of the authentication tag.  Must
    /// be between 0 and 128.  The tag is appended to the end of the
    /// ciphertext.
    ///
    /// # Panics
    ///
    /// This function panics if the length of `iv` or `aad` does not
    /// fit into an [Ulong].
    pub fn new(iv: &'a [u8], aad: &'a [u8], tag_bits: Ulong) -> Self {
        // The ulIvBits parameter seems to be missing from the 2.40 spec,
        // although it is included in the header file.  In [1], OASIS clarified
        // that the header file is normative.  In 3.0, they added the parameter
        // to the spec, but it seems to be unused:
        //
        // > Do not use ulIvBits to specify the length of the initialization
        // > vector, but ulIvLen instead.
        //
        // Further, in v3.0, the IV is permitted to be up to 2^32-1 bytes,
        // which would cause ulIvBits to overflow on platforms where
        // sizeof(CK_ULONG) = 4.
        //
        // In light of all this, we include ulIvBits in the struct, but always
        // set it to zero.
        //
        // [1]: https://www.oasis-open.org/committees/document.php?document_id=58032&wg_abbrev=pkcs11
        GcmParams { iv, aad, tag_bits }
    }

    /// The initialization vector.
    pub fn iv(&self) -> &'a [u8] {
        self.iv
    }

    /// The additional authenticated data.
    pub fn aad(&self) -> &'a [u8] {
        self.aad
    }

    /// The length, in bits, of the authentication tag.
    pub fn tag_bits(&self) -> Ulong {
        self.tag_bits
    }
}
