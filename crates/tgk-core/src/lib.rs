//! Core domain model for the TGK encrypted profile maker.
//!
//! This crate holds the in-memory `Profile` and `Account` types that the CLI
//! (and, later, the `serve` daemon and dashboard) share. No storage, crypto,
//! or network concerns live here in Step 1.

use serde::{Deserialize, Serialize};

/// A user's identity plus their registered accounts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    /// Human-readable display name.
    pub display_name: String,

    /// Email addresses associated with the identity.
    pub emails: Vec<String>,

    /// Usernames associated with the identity.
    pub usernames: Vec<String>,

    /// Services the user has accounts on.
    pub accounts: Vec<Account>,
}

/// A single registered account on a service.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    /// The service name, e.g. `"github"`.
    pub service: String,

    /// The email or username used on that service.
    pub login: String,

    /// Whether this service is a data broker.
    pub is_broker: bool,

    /// How the account can be removed, e.g. `"api"`, `"email"`, `"web_form"`, `"manual"`.
    pub removal_method: Option<String>,

    /// Removal status, e.g. `"none"`, `"requested"`, `"confirmed"`.
    pub status: Option<String>,
    
    /// RFC 3339 timestamp of the last run (string in Step 1).
    pub last_run: Option<String>,
}

impl Account {
    /// Build a new account with the Step 1 defaults (not a broker, no
    /// removal metadata yet).
    #[must_use]
    pub fn new(service: impl Into<String>, login: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            login: login.into(),
            is_broker: false,
            removal_method: None,
            status: None,
            last_run: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn populated_profile() -> Profile {
        Profile {
            display_name: "Ethan Clark".to_string(),
            emails: vec!["ethan@example.com".to_string()],
            usernames: vec!["ethan".to_string()],
            accounts: vec![Account {
                service: "github".to_string(),
                login: "ethan".to_string(),
                is_broker: false,
                removal_method: Some("api".to_string()),
                status: Some("requested".to_string()),
                last_run: Some("2026-01-01T00:00:00Z".to_string()),
            }],
        }
    }

    #[test]
    fn populated_profile_roundtrips() {
        let profile = populated_profile();
        let json = serde_json::to_string(&profile).unwrap();
        let back: Profile = serde_json::from_str(&json).unwrap();
        assert_eq!(profile, back);
    }

    #[test]
    fn empty_profile_roundtrips() {
        let profile = Profile {
            display_name: String::new(),
            emails: Vec::new(),
            usernames: Vec::new(),
            accounts: Vec::new(),
        };
        let json = serde_json::to_string(&profile).unwrap();
        let back: Profile = serde_json::from_str(&json).unwrap();
        assert_eq!(profile, back);
    }
}
