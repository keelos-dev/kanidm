//! keelOS fork: the password rules an administrator chose. Kanidm fixes two numbers in code:
//! a single-factor password must have 15 characters (NIST) and must reach zxcvbn score 4. keelOS
//! lets the administrator set both (keelauthd writes `keel_password_min_length` and
//! `keel_password_min_score` to the server configuration): the minimum length (the group
//! policy's `auth_password_minimum_length` may still raise it) and the score a password must
//! reach (0 = length only, 4 = Kanidm's own rule). Unset = Kanidm's rules.

use kanidm_lib_crypto::PW_SFA_MIN_LENGTH_NIST;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug)]
pub struct KeelPasswordPolicy {
    pub min_length: u32,
    pub min_score: u8,
}

impl Default for KeelPasswordPolicy {
    fn default() -> Self {
        KeelPasswordPolicy {
            min_length: PW_SFA_MIN_LENGTH_NIST,
            min_score: 4,
        }
    }
}

static POLICY: OnceLock<KeelPasswordPolicy> = OnceLock::new();

/// Set once at start (before the IDM server exists); a second call is ignored.
pub fn set(p: KeelPasswordPolicy) {
    let _ = POLICY.set(p);
}

pub fn get() -> KeelPasswordPolicy {
    POLICY.get().copied().unwrap_or_default()
}

/// Is a zxcvbn score below what the policy asks for?
pub fn too_weak(score: zxcvbn::Score) -> bool {
    (score as u8) < get().min_score
}
