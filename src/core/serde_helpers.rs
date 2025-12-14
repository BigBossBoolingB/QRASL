//! Serde helpers for types that are not directly supported by serde.

pub mod hashmap_as_vec {
    use serde::{Deserializer, Serializer, de::SeqAccess, ser::SerializeSeq};
    use std::collections::HashMap;
    use crate::crypto::PublicKeyBytes;

    pub fn serialize<S>(map: &HashMap<PublicKeyBytes, u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(map.len()))?;
        for (k, v) in map {
            seq.serialize_element(&(&k[..], v))?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<PublicKeyBytes, u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HashMapVisitor;

        impl<'de> serde::de::Visitor<'de> for HashMapVisitor {
            type Value = HashMap<PublicKeyBytes, u64>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of (PublicKeyBytes, u64) tuples")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut map = HashMap::new();
                while let Some((key_bytes, value)) = seq.next_element::<(Vec<u8>, u64)>()? {
                    let key: PublicKeyBytes = key_bytes.clone().try_into().map_err(|_| {
                        serde::de::Error::invalid_length(key_bytes.len(), &"PublicKeyBytes")
                    })?;
                    map.insert(key, value);
                }
                Ok(map)
            }
        }

        deserializer.deserialize_seq(HashMapVisitor)
    }
}
