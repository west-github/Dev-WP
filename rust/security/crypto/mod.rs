#![allow(dead_code, unused_variables)]

trait Encoder {
    fn encode(password: &[u8]) -> String {
        String::from_utf8_lossy(password).to_string()
    }

    fn matches(password: &[u8], encode_password: String) -> bool {
        String::from_utf8_lossy(password).to_string() == encode_password
    }
    fn upgrade_encoding(_pwd: String) -> bool {
        false
    }
}

pub struct Argon;
impl Encoder for Argon {}

pub struct Bcrypt;
impl Encoder for Bcrypt {}

pub struct Scrypt;
impl Encoder for Scrypt {}

mod key_generator {

    pub trait Key {
        type Output;

        fn get_key(&self) -> Self::Output;
    }

    struct SecureRandom {
        key: String,
    }

    impl Key for SecureRandom {
        type Output = Vec<u8>;

        fn get_key(&self) -> Self::Output {
            self.key.as_bytes().to_owned()
        }
    }

    struct Hex<T: Key> {
        bytes_gen: T,
    }

    impl<T: Key<Output = Vec<u8>>> Key for Hex<T> {
        type Output = String;

        fn get_key(&self) -> Self::Output {
            String::from_utf8(self.bytes_gen.get_key()).unwrap()
        }
    }

    struct Base64<T: Key> {
        bytes_gen: T,
    }

    impl<T: Key<Output = Vec<u8>>> Key for Base64<T> {
        type Output = String;

        fn get_key(&self) -> Self::Output {
            String::from_utf8(self.bytes_gen.get_key()).unwrap()
        }
    }
}
