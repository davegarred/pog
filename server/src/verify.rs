use crate::error::Error;
use axum::http::HeaderMap;
use ed25519::signature::Verifier;
use ed25519::Signature;
use ed25519_dalek::VerifyingKey;

#[derive(Debug, Clone)]
pub struct VerifyTool {
    key: VerifyingKey,
    debug_key: Option<String>,
}

impl VerifyTool {
    pub fn new(key: &str) -> Self {
        let debug_key: Option<String> = std::env::var("DEBUG_HEADER").ok();
        let bytes = match hex::decode(key) {
            Ok(bytes) => bytes,
            Err(err) => {
                println!("provided key is not in hexadecimal form");
                panic!("{:?}", err);
            }
        };
        let key_bytes: [u8; 32] = match TryInto::<[u8; 32]>::try_into(bytes.as_slice()) {
            Ok(key_bytes) => key_bytes,
            Err(err) => {
                println!("provided key is not the correct length");
                panic!("{:?}", err);
            }
        };
        let key = match VerifyingKey::from_bytes(&key_bytes) {
            Ok(key) => key,
            Err(err) => {
                println!("provided key is not valid");
                panic!("{:?}", err);
            }
        };
        Self { key, debug_key }
    }

    pub fn validate(&self, headers: &HeaderMap, body: &str) -> Result<(), Error> {
        if let Some(debug_key) = &self.debug_key {
            if headers.get(debug_key).is_some() {
                return Ok(());
            }
        }
        let signature = match headers.get("X-Signature-Ed25519") {
            None => {
                println!("missing signature");
                ""
            }
            Some(signature) => signature.to_str().unwrap(),
        };
        let timestamp = match headers.get("X-Signature-Timestamp") {
            None => {
                println!("missing timestamp");
                ""
            }
            Some(timestamp) => timestamp.to_str().unwrap(),
        };
        if self.verify(timestamp, body, signature) {
            Ok(())
        } else {
            Err(Error::NotAuthorized)
        }
    }

    pub fn verify(&self, timestamp: &str, body: &str, signature: &str) -> bool {
        let message = format!("{}{}", timestamp, body);
        let sig_bytes = match hex::decode(signature) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        let signature = match Signature::from_slice(sig_bytes.as_slice()) {
            Ok(signature) => signature,
            Err(_) => return false,
        };
        self.key.verify(message.as_bytes(), &signature).is_ok()
    }
}

#[cfg(test)]
mod test {
    use ed25519_dalek::{Signer, SigningKey};

    use crate::verify::VerifyTool;

    #[test]
    fn test_verifier() {
        use rand_core::OsRng; // Requires the `std` feature of `rand_core`
        let signing_key: SigningKey = ed25519_dalek::SigningKey::generate(&mut OsRng);
        let verifying_key = hex::encode(signing_key.verifying_key().as_bytes());
        let verifier = VerifyTool::new(verifying_key.as_str());

        let timestamp = "1608597133";
        let body = r#"{"id":"my-object-id","name":"an-interaction-name"}`"#;
        let to_sign = format!("{}{}", timestamp, body);
        let signature = signing_key.sign(to_sign.as_bytes());
        let header_sig = signature.to_string();

        assert!(verifier.verify(timestamp, body, header_sig.as_str()));
    }
}
