use serde::{Deserialize, Serialize};

/// The personal information TGK needs on file to find (and eventually request
/// removal of) a user's exposure across data brokers and people-search sites.
///
/// This type lives in tgk-core rather than tgk-cli on purpose: it's the shared
/// contract between every front end TGK ships (this CLI today, a desktop UI
/// later). Nothing in here should assume it was collected via a terminal —
/// treat it as plain data, built however the caller likes.
///
/// Field choices are based on what data removal services (DeleteMe, Incogni,
/// Aura, etc.) match against, since that's the same information the broker
/// sites TGK targets index on: name, address history, phone, email, plus
/// aliases and relatives to help disambiguate people with common names.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserProfile {
    pub full_name: FullName,

    /// Maiden names, nicknames, or past legal names a broker might have
    /// indexed you under.
    pub aliases: Vec<String>,

    /// ISO 8601 (YYYY-MM-DD). Optional, but people-search sites commonly use
    /// this to tell apart two people with the same name.
    pub date_of_birth: Option<String>,

    pub emails: Vec<String>,
    pub phone_numbers: Vec<String>,

    pub current_address: Option<Address>,

    /// Brokers frequently retain years of address history, so past addresses
    /// are often needed to find (and remove) every record.
    pub previous_addresses: Vec<Address>,

    /// Names of relatives. Broker sites often list "possible relatives" next
    /// to a record; having these on hand helps confirm a match is really you.
    pub relatives: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FullName {
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state_or_region: String,
    pub postal_code: String,
    pub country: String,
}