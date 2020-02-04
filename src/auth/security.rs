use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::secretbox;

pub struct Crypto {
    key: secretbox::Key,
}

impl Crypto {
    pub fn new() -> Crypto {
        Crypto {
            key: secretbox::gen_key(),
        }
    }

    pub fn decode(&self, value: String) -> Option<String> {
        let serialized = value.chars().map(|x| x as u8).collect::<Vec<u8>>();

        match bincode::deserialize::<EncodedData>(&serialized) {
            Ok(deserialized) => {
                let nonce = deserialized.nonce;
                if let Ok(unsealed) = secretbox::open(&deserialized.data, &nonce, &self.key) {
                    Some(unsealed.iter().map(|&x| x as char).collect::<String>())
                } else {
                    None
                }
            }
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }
    pub fn encode(&self, value: String) -> String {
        let nonce = secretbox::gen_nonce();
        let sealed = secretbox::seal(value.as_bytes(), &nonce, &self.key);
        let cookie_data = EncodedData::new(nonce, sealed);
        let serialized = bincode::serialize(&cookie_data).unwrap();
        serialized.iter().map(|&x| x as char).collect::<String>()
    }
}

#[derive(Serialize, Deserialize)]
struct EncodedData {
    nonce: secretbox::Nonce,
    data: Vec<u8>,
}

impl EncodedData {
    fn new(nonce: secretbox::Nonce, data: Vec<u8>) -> EncodedData {
        EncodedData { nonce, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        let crypto = Crypto::new();

        let msg = "ODBIOR".to_string();
        let encoded = crypto.encode(msg.clone());
        let decoded = crypto.decode(encoded);
        assert_eq!(msg, decoded.unwrap());
    }

    #[test]
    fn diffrent_key() {
        let crypto = Crypto::new();

        let msg1 = "ODBIOR".to_string();
        let msg2 = "TEST".to_string();

        let encoded1 = crypto.encode(msg1.clone());
        let encoded2 = crypto.encode(msg2.clone());

        let decoded1 = crypto.decode(encoded1).unwrap();
        let decoded2 = crypto.decode(encoded2).unwrap();

        assert_ne!(decoded1, decoded2);
        assert_eq!(msg1, decoded1);
        assert_eq!(msg2, decoded2);
    }
}
