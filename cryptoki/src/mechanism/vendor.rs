// Copyright 2024 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! Custom vendor-defined mechanisms

use std::fmt::Debug;

use super::MechanismType;

// The current problem with this that remains to be solved is enforcing
// a single type of params for a new `MechanismType`
// Something like
// ```
// pub trait VendorMechanism {
//     const TYPE: MechanismType;
// }
// ```
// Would be nice, but it's not object safe, which we really need for dynamic vendor types
// Another option could be
// ```
// pub trait VendorMechanism {
//     fn mechanism_type(&self) -> MechanismType;
// }
// ```
// But that still allows the implementor to change the mechanism types for a param
// It would require otherwise unnecessary unit structs such as
// ```
// pub const CKA_EXPORT: CK_ATTRIBUTE_TYPE = 0x10000001;
// let custom_type = MechanismType::new_vendor_defined(CKA_CUSTOM);
// pub struct CustomMechanism;
//
// impl VendorMechanism for CustomMechanism {
//      fn mechanism_type(&self) -> MechanismType {
//          custom_type
//      }
// }
// ```
// though it's the inverse of what I actually want. It sets a single MechanismType for each param
// not single param type for each MechanismType

#[derive(Debug, Clone, Copy)]
/// A generic vendor defined mechanism
pub struct VendorDefinedMechanism<'a> {
    mechanism_type: MechanismType,
    pub(crate) params: &'a Box<dyn MechanismParams>,
}

impl<'a> VendorDefinedMechanism<'a> {
    /// Create a new vendor defined mechanism
    pub fn new(
        mechanism_type: MechanismType,
        params: &'a Box<dyn MechanismParams>,
    ) -> VendorDefinedMechanism<'a> {
        VendorDefinedMechanism {
            mechanism_type,
            params: params,
        }
    }

    /// The vendor mechanism type
    pub fn mechanism_type(&self) -> MechanismType {
        self.mechanism_type
    }
}

/// Parameters for a custom mechanism
/// TODO: Safety about mutating in custom mechanisms
pub trait MechanismParams: Debug + Send + Sync {}

impl MechanismParams for () {}
