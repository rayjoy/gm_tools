use zuc::zuc256::Zuc256StreamCipher;
use zuc::cipher::{KeyIvInit, StreamCipher, KeySizeUser, IvSizeUser};

fn main() {
    println!("ZUC-256 Key Size: {}", <Zuc256StreamCipher as KeySizeUser>::key_size());
    println!("ZUC-256 IV Size: {}", <Zuc256StreamCipher as IvSizeUser>::iv_size());
}
