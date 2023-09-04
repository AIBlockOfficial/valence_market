use rand::Rng;
use crate::constants::{DRUID_CHARSET, DRUID_LENGTH};

/// Constructs a 16 byte DRUID string
pub fn construct_druid() -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = (0..DRUID_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..DRUID_CHARSET.len());
            DRUID_CHARSET[idx] as char
        })
        .collect();

    random_string
}