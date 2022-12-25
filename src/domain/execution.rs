use hex::FromHex;
use serde::de::Deserialize;
use serde::ser::{Serialize, Serializer};
use serde::Deserializer;

use std::fmt;

#[derive(PartialEq, Eq, Clone, Default)]
pub struct ExecutionAddress(pub [u8; 20]);

impl fmt::Display for ExecutionAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = format!("0x{}", hex::encode(self.0.to_vec()));
        write!(f, "{}", hex)
    }
}

impl fmt::Debug for ExecutionAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = format!("0x{}", hex::encode(self.0.to_vec()));
        write!(f, "{}", hex)
    }
}

impl Serialize for ExecutionAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex = format!("0x{}", hex::encode(self.0.to_vec()));
        serializer.serialize_str(&hex.as_str())
    }
}

impl<'de> Deserialize<'de> for ExecutionAddress {
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

impl std::str::FromStr for ExecutionAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix("0x") {
            let bytes = <[u8; 20]>::from_hex(stripped).map_err(|e| e.to_string())?;
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
    fn execution_address1() {
        let body = "00".repeat(19);
        let addr_str = format!("\"0x{}01\"", body);
        let mut value = [0; 20];
        value[19] = 1;
        let addr = ExecutionAddress(value);

        let serialized = serde_json::to_string(&addr).unwrap();
        let deserialized: ExecutionAddress = serde_json::from_str(&addr_str).unwrap();

        assert_eq!(serialized, addr_str);
        assert_eq!(deserialized, addr);
    }

    #[test]
    fn execution_address255() {
        let body = "00".repeat(18);
        let addr_str = format!("\"0x0a{}ff\"", body);
        let mut value = [0; 20];
        value[0] = 10;
        value[19] = 255;
        let addr = ExecutionAddress(value);
        let serialized = serde_json::to_string(&addr).unwrap();
        let deserialized: ExecutionAddress = serde_json::from_str(&addr_str).unwrap();

        assert_eq!(serialized, addr_str);
        assert_eq!(deserialized, addr);
    }

    #[test]
    #[should_panic(expected = "Odd number of digits")]
    fn execution_address_wrong1() {
        let body = "00".repeat(19);
        let addr_str = format!("\"0x{}1\"", body);
        let _: ExecutionAddress = serde_json::from_str(&addr_str).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid string length")]
    fn execution_address_wrong2() {
        let body = "00".repeat(19);
        let addr_str = format!("\"0x{}\"", body);
        let _: ExecutionAddress = serde_json::from_str(&addr_str).unwrap();
    }

    #[test]
    #[should_panic(expected = "Must start with 0x")]
    fn execution_address_wrong_start() {
        let body = "00".repeat(19);
        let addr_str = format!("\"{}01\"", body);
        let _: ExecutionAddress = serde_json::from_str(&addr_str).unwrap();
    }
}
