use blake3::Hasher;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/*对加密的内容的一个加密工具*/
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Blake3Config {
    /*用于加盐*/
    pub salt: Option<String>,
}

impl Blake3Config {
    pub fn new(blake: Blake3Config) -> Self {
        let salt_config = match blake.salt {
            None => "brave".to_string(),
            Some(data) => data.to_string(),
        };

        Self {
            salt: Some(salt_config),
        }
    }

    /*带加盐的*/
    pub fn generate_with_salt(&self, str: String) -> String {
        let mut hasher = Hasher::new_derive_key(&self.salt.clone().unwrap());
        hasher.update(str.as_ref());
        let hash2 = hasher.finalize();
        hash2.to_string()
    }

    /*不加盐的*/
    pub fn generate(&self, str: String) -> String {
        let mut hasher = Hasher::new();
        hasher.update(str.as_ref());
        let hash2 = hasher.finalize();
        hash2.to_string()
    }
}

#[cfg(test)]
mod blake3tests {
    use super::*;
    use ring::rand::generate;

    #[test]
    fn blake3_test() {
        // let hash1 = blake3::hash(b"foobar");
        //
        // // Hash an input incrementally.
        // let mut hasher = blake3::Hasher::new();
        // hasher.update(b"foo");
        // hasher.update(b"bar");
        // let hash2 = hasher.finalize();
        // assert_eq!(hash1, hash2);
        //
        // // Extended output. OutputReader also implements Read and Seek.
        // let mut output = [0; 1000];
        // let mut output_reader = hasher.finalize_xof();
        // output_reader.fill(&mut output);
        // assert_eq!(hash1, output[..32]);
        //
        // // Print a hash as hex.
        // println!("{}", hash1);

        let blake = Blake3Config::new(Blake3Config {
            salt: Some("qwe123".to_string()),
        });
        let str = blake.generate("123".to_string());

        println!("{}", str);
    }
}
