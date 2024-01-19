// Copyright 2024 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
//! Custom vendor-defined mechanisms

use std::fmt::Debug;

use super::MechanismType;

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
pub trait MechanismParams: Debug {}

impl MechanismParams for () {}
