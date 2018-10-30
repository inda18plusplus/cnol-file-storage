use ring::{
    self,
    error::Unspecified,
    aead::{
        SealingKey,
        OpeningKey,
        AES_128_GCM,
        seal_in_place,
        open_in_place,
    },
    rand::{
        SystemRandom,
        SecureRandom,
    },
    pbkdf2,
    digest,
};

static ENCRYPTION_ALGORITHM: &'static ring::aead::Algorithm = &AES_128_GCM;
static KEY_DIGEST_ALGORITM: &'static digest::Algorithm = &digest::SHA256;
static PBKDF2_ITERATIONS: u32 = 47_131;

/// Stores encrypted data
pub struct Secret {
    data: Vec<u8>,
    nonce: Vec<u8>,
}

#[derive(Debug)]
pub enum Error {
    /// Could not verify the authenticity of the data
    AuthenticationFailed,

    /// The supplied data was too short
    InvalidLength,
}


impl Secret {
    pub fn new(password: &[u8], data: &[u8], verification: &[u8]) -> Secret {
        let key = generate_key(password);
        let nonce = generate_nonce();

        let data = encrypt(&key, &nonce, data, verification).unwrap();

        Secret {
            data,
            nonce,
        }
    }


    /// Create a new secret from some bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Secret, Error> {
        if bytes.len() < ENCRYPTION_ALGORITHM.nonce_len() {
            Err(Error::InvalidLength)
        } else {
            let (data, nonce) = bytes.split_at(bytes.len() - ENCRYPTION_ALGORITHM.nonce_len());

            Ok(Secret {
                data: data.to_vec(),
                nonce: nonce.to_vec(),
            })
        }
    }

    /// Get the secret as bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut data = self.data.clone();
        data.extend_from_slice(&self.nonce);
        data
    }


    /// Reveals the secret, consuming it in the process
    pub fn reveal(self, password: &[u8], verification: &[u8]) -> Result<Vec<u8>, Error> {
        let key = generate_key(password);

        match decrypt(&key, &self.nonce, &self.data, verification) {
            Ok(data) => {
                Ok(data)
            }
            Err(_) => {
                Err(Error::AuthenticationFailed)
            }
        }
    }
}


/// Generate a private key from a password
fn generate_key(password: &[u8]) -> Vec<u8> {
    let salt = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut hashed_password = vec![0; ENCRYPTION_ALGORITHM.key_len()];

    pbkdf2::derive(KEY_DIGEST_ALGORITM, PBKDF2_ITERATIONS, &salt, password, &mut hashed_password);

    hashed_password
}

/// Generate a nonce for encryption/decryption
fn generate_nonce() -> Vec<u8> {
    let mut nonce = vec![0; ENCRYPTION_ALGORITHM.nonce_len()];
    SystemRandom::new().fill(&mut nonce).unwrap();
    nonce
}

/// Encrypt and sign some data using a private key, a nonce, and an unique identifier for
/// verification.
fn encrypt(key: &[u8], nonce: &[u8], data: &[u8], verification_data: &[u8]) -> Result<Vec<u8>, Unspecified> {
    let key = SealingKey::new(
        ENCRYPTION_ALGORITHM,
        &key,
    ).unwrap();

    // Allocate space for tag
    let mut buffer = data.to_vec();
    for _ in 0..key.algorithm().tag_len() {
        buffer.push(0);
    }

    // Encrypt and sign
    seal_in_place(&key, nonce, verification_data, &mut buffer, ENCRYPTION_ALGORITHM.tag_len())
        .map(|_| buffer)
}

/// Verify and decrypt some data using a private key, a nonce, and an unique identifier for
/// verifying the authenticity of the data.
fn decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8], verification_data: &[u8]) -> Result<Vec<u8>, Unspecified> {
    let key = OpeningKey::new(
        ENCRYPTION_ALGORITHM,
        &key,
    ).unwrap();

    let mut buffer = ciphertext.to_vec();

    // Authenticate and decrypt
    open_in_place(&key, nonce, verification_data, 0, &mut buffer)
        .map(|a| a.to_vec())
}
