//! Core data types and shared models for the TGK (Privacy Toolkit) ecosystem.
//!
//! Provides fundamental domain structs such as [`UserProfile`], [`FullName`], and [`Address`]
//! used across frontends (CLI, GUI) and backend crates (`tgk-vault`, `tgk-net`).

use serde::{Deserialize, Serialize};

/// The personal information TGK needs on file to find (and eventually request
/// removal of) a user's exposure across data brokers and people-search sites.
///
/// This type lives in `tgk-core` rather than `tgk-cli` on purpose: it's the shared
/// contract between every front end TGK ships (this CLI today, a desktop UI
/// later). Nothing in here should assume it was collected via a terminal —
/// treat it as plain data, built however the caller likes.
///
/// Field choices are based on what data removal services (`DeleteMe`, `Incogni`,
/// `Aura`, etc.) match against, since that's the same information the broker
/// sites TGK targets index on: name, address history, phone, email, plus
/// aliases and relatives to help disambiguate people with common names.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserProfile {
    /// The primary legal full name of the user.
    pub full_name: FullName,

    /// Maiden names, nicknames, or past legal names a broker might have
    /// indexed you under.
    pub aliases: Vec<String>,

    /// ISO 8601 formatted date of birth (YYYY-MM-DD). Optional, but people-search
    /// sites commonly use this to tell apart two people with the same name.
    pub date_of_birth: Option<String>,

    /// List of email addresses associated with the user.
    pub emails: Vec<String>,

    /// List of phone numbers associated with the user.
    pub phone_numbers: Vec<String>,

    /// The user's current residential address, if available.
    pub current_address: Option<Address>,

    /// Brokers frequently retain years of address history, so past addresses
    /// are often needed to find (and remove) every record.
    pub previous_addresses: Vec<Address>,

    /// Names of relatives. Broker sites often list "possible relatives" next
    /// to a record; having these on hand helps confirm a match is really you.
    pub relatives: Vec<String>,
}

/// Represents a individual's full name component breakdown.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FullName {
    /// Given or first name.
    pub first: String,

    /// Middle name or middle initial, if applicable.
    pub middle: Option<String>,

    /// Family name or surname.
    pub last: String,
}

/// Represents a physical postal address.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Address {
    /// Street address, including line 1 and line 2 (e.g. house number, street name, apartment/suite).
    pub street: String,

    /// City or municipality name.
    pub city: String,

    /// State, province, or region code/name.
    pub state_or_region: String,

    /// Postal code or ZIP code.
    pub postal_code: String,

    /// Country name or two-letter ISO country code.
    pub country: String,
}
