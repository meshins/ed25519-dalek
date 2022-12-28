// -*- mode: rust; -*-
//
// This file is part of ed25519-dalek.
// Copyright (c) 2017-2019 isis lovecruft
// See LICENSE for licensing information.

use curve25519_dalek::constants;
use curve25519_dalek::edwards::EdwardsPoint;
use curve25519_dalek::scalar::Scalar;

/// Fixed-base scalar multiplication by the Ed25519 base point.
///
/// Uses basepoint tables when the `basepoint-tables` feature is enabled.
pub(crate) fn mul_base(scalar: &Scalar) -> EdwardsPoint {
    #[cfg(not(feature = "basepoint-tables"))]
    {
        scalar * constants::ED25519_BASEPOINT_POINT
    }

    #[cfg(feature = "basepoint-tables")]
    {
        scalar * constants::ED25519_BASEPOINT_TABLE
    }
}
