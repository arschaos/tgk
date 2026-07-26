//! Implementation of the `tgk init` subcommand.
//!
//! Guides the user through an interactive terminal workflow to collect personal details
//! needed for data broker removal requests and builds a [`UserProfile`].

use tgk_core::{Address, FullName, UserProfile};

use crate::prompt::{confirm, prompt, prompt_list, prompt_optional};

/// Executes the profile initialization wizard.
///
/// Interactively prompts the user for identity attributes (name, aliases, DOB, emails,
/// phone numbers, current address, previous addresses, and relatives) and constructs
/// a [`UserProfile`].
pub fn run() {
    println!("Initializing User Profile...");
    println!(
        "TGK needs some of the same information data brokers already have on you, \
         so it can find and later request removal of it. Please enter the following \
         information below, and hit Enter to confirm. Fields marked with (opt) or \
         (blank to stop) are optional fields, and are not required for use of TGK. \n"
    );

    let full_name = FullName {
        first: prompt("First Name"),
        middle: prompt_optional("Middle Name"),
        last: prompt("Last Name"),
    };

    let aliases = prompt_list("Alias / maiden name / nickname");
    let date_of_birth = prompt_optional("Date of birth (YYYY-MM-DD)");
    let emails = prompt_list("Email address");
    let phone_numbers = prompt_list("Phone number (X-XXX-XXX-XXXX)");

    println!("\nCurrent address:");
    let current_address = Some(prompt_address());

    let mut previous_addresses = Vec::new();
    while confirm("Add a previous address?") {
        previous_addresses.push(prompt_address());
    }

    let relatives = prompt_list("Relative's name");

    let profile = UserProfile {
        full_name,
        aliases,
        date_of_birth,
        emails,
        phone_numbers,
        current_address,
        previous_addresses,
        relatives,
    };

    // TODO: hand `profile` to tgk-vault to be encrypted and persisted to
    // disk once that crate has real storage logic. For now, print it back
    // to confirm the collected data round-trips correctly.
    println!("\nProfile captured:\n{profile:#?}");
}

/// Prompts the user for all constituent fields of an [`Address`].
fn prompt_address() -> Address {
    Address {
        street: prompt("\tStreet"),
        city: prompt("\tCity"),
        state_or_region: prompt("\tState / region"),
        postal_code: prompt("\tPostal code"),
        country: prompt("\tCountry"),
    }
}


