mod platform;

use bip39::{Mnemonic, Language, Seed};
use crate::platform::{PlatformFactory, PlatformType};

fn main() {
    println!("Hello, world!");

    let phrase = "fragile harvest require asthma busy find home school horn garlic kite ivory awkward monkey pupil zoo ability gossip senior recycle produce silly leisure suggest";
    let seed = parse_mnemonic(phrase);

    let platform = PlatformFactory::new_platform(&PlatformType::Monero, &seed);
    platform.get_address()
}

fn parse_mnemonic(mnemonic_phrase: &str) -> Vec<u8> {
    let mnemonic = Mnemonic::from_phrase(mnemonic_phrase, Language::English).unwrap();

    return Seed::new(&mnemonic, "").as_bytes().to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mnemonic() {
        let mnemonic = parse_mnemonic("fragile harvest require asthma busy find home school horn garlic kite ivory awkward monkey pupil zoo ability gossip senior recycle produce silly leisure suggest");
        assert_eq!(
            hex::encode(mnemonic),
            "7a1c3edd859098837e1413f8511c3e212ee4413238b44480e39f6f8617b36a7641deccdcf3c50e4d6cf2842920b354c976cefabd5f9f38946aa33fb7226490a9"
        )
    }
}