use hex::FromHex;
use serde::de::Deserialize;
use serde::ser::{Serialize, Serializer};
use serde::Deserializer;

use std::fmt;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct ProposerAddress(pub [u8; 48]);

impl fmt::Display for ProposerAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = format!("0x{}", hex::encode(self.0.to_vec()));
        write!(f, "{}", hex)
    }
}

impl Default for ProposerAddress {
    fn default() -> Self {
        Self([0; 48])
    }
}

impl fmt::Debug for ProposerAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = format!("0x{}", hex::encode(self.0.to_vec()));
        write!(f, "{}", hex)
    }
}

impl Serialize for ProposerAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex = format!("0x{}", hex::encode(self.0.to_vec()));
        serializer.serialize_str(&hex.as_str())
    }
}

impl<'de> Deserialize<'de> for ProposerAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct StringVisitor;

        impl<'de> serde::de::Visitor<'de> for StringVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a hex string with 0x prefix")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(value.to_string())
            }
        }

        let string = deserializer.deserialize_str(StringVisitor)?;
        <Self as std::str::FromStr>::from_str(&string).map_err(serde::de::Error::custom)
    }
}

impl std::str::FromStr for ProposerAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix("0x") {
            let bytes = <[u8; 48]>::from_hex(stripped).map_err(|e| e.to_string())?;
            Ok(Self(bytes))
        } else {
            Err("Must start with 0x".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposer_address1() {
        let body = "00".repeat(47);
        let addr_str = format!("\"0x{}01\"", body);
        let mut value = [0; 48];
        value[47] = 1;
        let addr = ProposerAddress(value);

        let serialized = serde_json::to_string(&addr).unwrap();
        let deserialized: ProposerAddress = serde_json::from_str(&addr_str).unwrap();

        assert_eq!(serialized, addr_str);
        assert_eq!(deserialized, addr);
    }

    #[test]
    fn proposer_address255() {
        let body = "00".repeat(46);
        let addr_str = format!("\"0x0a{}ff\"", body);
        let mut value = [0; 48];
        value[0] = 10;
        value[47] = 255;
        let addr = ProposerAddress(value);
        let serialized = serde_json::to_string(&addr).unwrap();
        let deserialized: ProposerAddress = serde_json::from_str(&addr_str).unwrap();

        assert_eq!(serialized, addr_str);
        assert_eq!(deserialized, addr);
    }

    #[test]
    #[should_panic(expected = "Odd number of digits")]
    fn proposer_address_wrong1() {
        let body = "00".repeat(47);
        let addr_str = format!("\"0x{}1\"", body);
        let _: ProposerAddress = serde_json::from_str(&addr_str).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid string length")]
    fn proposer_address_wrong2() {
        let body = "00".repeat(47);
        let addr_str = format!("\"0x{}\"", body);
        let _: ProposerAddress = serde_json::from_str(&addr_str).unwrap();
    }

    #[test]
    #[should_panic(expected = "Must start with 0x")]
    fn execution_address_wrong_start() {
        let body = "00".repeat(47);
        let addr_str = format!("\"{}01\"", body);
        let _: ProposerAddress = serde_json::from_str(&addr_str).unwrap();
    }
}
