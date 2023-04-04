use blake3::Hasher;
use serde::{Deserialize, Serialize};

/*对加密的内容的一个加密工具*/
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Blake3Config {
    /*用于加盐*/
    pub salt: Option<String>,
}

impl Blake3Config {
    /*带加盐的*/
    pub fn generate_with_salt(&self, str: &str) -> String {
        //用于默认值
        let salt_config = match &self.salt {
            None => "brave".to_string(),
            Some(data) => data.to_string(),
        };

        let mut hasher = Hasher::new_derive_key(&salt_config.to_string());
        hasher.update(str.as_ref());
        let hash2 = hasher.finalize();
        hash2.to_string()
    }

    /*不加盐的*/
    pub fn generate(&self, str: &str) -> String {
        let mut hasher = Hasher::new();
        hasher.update(str.as_ref());
        let hash2 = hasher.finalize();
        hash2.to_string()
    }
}

#[cfg(test)]
mod blake3tests {
    use super::*;

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

        let blake = Blake3Config {
            salt: Some("brave".to_string()),
        };
        let str = blake.generate_with_salt("123456");

        println!("{}", str);
    }
}
