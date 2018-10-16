use ring::{
    self,
    error::Unspecified,
    aead::{
        SealingKey,
        OpeningKey,
        AES_128_GCM,
        seal_in_place,
        open_in_place
    },
    rand::{
        SystemRandom,
        SecureRandom
    },
    pbkdf2::derive,
    digest::SHA256
};

static ALGORITHM: &'static ring::aead::Algorithm = &AES_128_GCM;


/// Stores encrypted data
pub struct Secret {
    data: Vec<u8>,
    nonce: Vec<u8>
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
            nonce
        }
    }


    /// Create a new secret from some bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Secret, Error> {
        if bytes.len() < ALGORITHM.nonce_len() {
            Err(Error::InvalidLength)
        } else {
            let (data, nonce) = bytes.split_at(bytes.len() - ALGORITHM.nonce_len());

            Ok(Secret {
                data: data.to_vec(),
                nonce: nonce.to_vec()
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
            },
            Err(_) => {
                Err(Error::AuthenticationFailed)
            },
        }
    }
}


fn generate_key(password: &[u8]) -> Vec<u8> {
    let salt = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut hashed_password = vec![0; ALGORITHM.key_len()];
    derive(&SHA256, 100, &salt, &password[..], &mut hashed_password);

    hashed_password
}

fn generate_nonce() -> Vec<u8> {
    let mut nonce = vec![0; ALGORITHM.nonce_len()];
    SystemRandom::new().fill(&mut nonce).unwrap();
    nonce
}

fn encrypt(key: &[u8], nonce: &[u8], data: &[u8], verification_data: &[u8]) -> Result<Vec<u8>, Unspecified> {
    let key = SealingKey::new(
        ALGORITHM,
        &key
    ).unwrap();

    // Allocate space for tag
    let mut buffer = data.to_vec();
    for _ in 0..key.algorithm().tag_len() {
        buffer.push(0);
    }

    // Encrypt and sign
    seal_in_place(&key, nonce, verification_data, &mut buffer, ALGORITHM.tag_len())
        .map(|_| buffer)
}

fn decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8], verification_data: &[u8]) -> Result<Vec<u8>, Unspecified> {
    let key = OpeningKey::new(
        ALGORITHM,
        &key
    ).unwrap();

    let mut buffer = ciphertext.to_vec();

    // Authenticate and decrypt
    open_in_place(&key, nonce, verification_data, 0, &mut buffer)
        .map(|a| a.to_vec())
}
