//! Implementation of the `tgk init` subcommand.
//!
//! Guides the user through an interactive terminal workflow to collect personal details
//! needed for data broker removal requests and builds a [`UserProfile`].

use std::io::{self, BufRead};
use tgk_core::{Address, FullName, UserProfile};

use crate::prompt::{
    confirm_from_reader, prompt_from_reader, prompt_list_from_reader, prompt_optional_from_reader,
};

/// Executes the profile initialization wizard.
pub fn run() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    run_with_reader(&mut handle);
}

/// Executes the profile initialization wizard using a generic reader.
pub fn run_with_reader<R: BufRead>(reader: &mut R) -> UserProfile {
    println!("Initializing User Profile...");
    println!(
        "TGK needs some of the same information data brokers already have on you, \
         so it can find and later request removal of it. Please enter the following \
         information below, and hit Enter to confirm. Fields marked with (opt) or \
         (blank to stop) are optional fields, and are not required for use of TGK. \n"
    );

    let full_name = FullName {
        first: prompt_from_reader(reader, "First Name"),
        middle: prompt_optional_from_reader(reader, "Middle Name"),
        last: prompt_from_reader(reader, "Last Name"),
    };

    let aliases = prompt_list_from_reader(reader, "Alias / maiden name / nickname");
    let date_of_birth = prompt_optional_from_reader(reader, "Date of birth (YYYY-MM-DD)");
    let emails = prompt_list_from_reader(reader, "Email address");
    let phone_numbers = prompt_list_from_reader(reader, "Phone number (X-XXX-XXX-XXXX)");

    println!("\nCurrent address:");
    let current_address = Some(prompt_address(reader));

    let mut previous_addresses = Vec::new();
    while confirm_from_reader(reader, "Add a previous address?") {
        previous_addresses.push(prompt_address(reader));
    }

    let relatives = prompt_list_from_reader(reader, "Relative's name");

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
    profile
}

/// Prompts the user for all constituent fields of an [`Address`].
fn prompt_address<R: BufRead>(reader: &mut R) -> Address {
    Address {
        street: prompt_from_reader(reader, "\tStreet"),
        city: prompt_from_reader(reader, "\tCity"),
        state_or_region: prompt_from_reader(reader, "\tState / region"),
        postal_code: prompt_from_reader(reader, "\tPostal code"),
        country: prompt_from_reader(reader, "\tCountry"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_init_run_with_reader() {
        let input_data = vec![
            "Jane",
            "",
            "Doe",
            "",
            "1990-01-01",
            "jane@example.com",
            "",
            "555-0199",
            "",
            "123 Elm St",
            "Metropolis",
            "NY",
            "10001",
            "US",
            "n",
            "",
        ]
        .join("\n")
            + "\n";

        let mut reader = Cursor::new(input_data);
        let profile = run_with_reader(&mut reader);

        assert_eq!(profile.full_name.first, "Jane");
        assert_eq!(profile.full_name.middle, None);
        assert_eq!(profile.full_name.last, "Doe");
        assert_eq!(profile.date_of_birth, Some("1990-01-01".to_string()));
        assert_eq!(profile.emails, vec!["jane@example.com".to_string()]);
        assert_eq!(profile.phone_numbers, vec!["555-0199".to_string()]);
        let addr = profile.current_address.expect("Current address missing");
        assert_eq!(addr.street, "123 Elm St");
        assert_eq!(addr.city, "Metropolis");
        assert_eq!(addr.state_or_region, "NY");
        assert_eq!(addr.postal_code, "10001");
        assert_eq!(addr.country, "US");
        assert!(profile.previous_addresses.is_empty());
        assert!(profile.relatives.is_empty());
    }
}
