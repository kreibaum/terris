use std::fmt::Display;

use log::debug;

/// Identifier for players, this way we can play without accounts.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct UserUuid(String);

impl Display for UserUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UserUuid {
    /** Login happens via /ws?uuid=... this parses the "uuid=..." part for you. */
    pub fn from_query_string(query_string: &str) -> Option<Self> {
        use lazy_static::lazy_static;
        use regex::Regex;
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "UUID=([0-9A-F]{8}-[0-9A-F]{4}-4[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12})"
            )
            .unwrap();
        }
        debug!("{}", query_string);

        if let Some(cap) = RE.captures_iter(&query_string.to_uppercase()).next() {
            if let Some(uuid) = cap.get(1) {
                return Some(UserUuid(uuid.as_str().to_owned()));
            }
        }

        None
    }
}
