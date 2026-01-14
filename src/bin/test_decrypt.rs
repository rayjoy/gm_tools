use libsm::sm2::signature::SigCtx;
use libsm::sm2::encrypt::{EncryptCtx, DecryptCtx};
use num_bigint::BigUint;

fn main() {
    // 1. Setup Keys
    let ctx = SigCtx::new();
    let (pk, sk) = ctx.new_keypair().unwrap();


    // 2. Prepare Data (Mocking the UI logic)
    let input = "abc";
    let data = input.as_bytes();
    
    // Simulate the padding logic in main.rs
    let mut data_vec = data.to_vec();
    if data_vec.len() < 32 {
        let pad_len = 32 - data_vec.len();
        data_vec.extend(std::iter::repeat(pad_len as u8).take(pad_len));
        println!("Padded data (hex): {}", hex::encode(&data_vec));
    }

    // 3. Encrypt
    let enc_ctx = EncryptCtx::new(32, pk.clone());
    let ciphertext = enc_ctx.encrypt(&data_vec).expect("Encryption failed");
    let ciphertext_hex = hex::encode(&ciphertext);
    println!("Ciphertext (hex): {}", ciphertext_hex);

    // 4. Decrypt
    // Simulate UI input: hex decode
    let dec_data = hex::decode(&ciphertext_hex).expect("Hex decode failed");
    
    let dec_ctx = DecryptCtx::new(32, sk.clone());
    match dec_ctx.decrypt(&dec_data) {
        Ok(plaintext) => {
            println!("Decryption success!");
            println!("Plaintext (raw hex): {}", hex::encode(&plaintext));
            // Unpad logic
            let mut display_text = plaintext.clone();
             if let Some(&pad_byte) = display_text.last() {
                 let pad_len = pad_byte as usize;
                 if pad_len > 0 && pad_len <= display_text.len() && pad_len <= 32 {
                     let start = display_text.len() - pad_len;
                     if display_text[start..].iter().all(|&b| b == pad_byte) {
                          display_text.truncate(start);
                          println!("Unpadded successfully.");
                     }
                 }
             }
             println!("Result: {:?}", String::from_utf8(display_text));
        },
        Err(e) => {
            println!("Decryption FAILED: {:?}", e);
        }
    }
}
