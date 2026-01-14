use libsm::sm2::signature::{SigCtx, Signature};
use num_bigint::BigUint;
use num_traits::Num;

fn main() {
    // Standard test key (though code uses generated ones usually)
    let sk_hex = "128B2FA8BD433C6C068C8D803DFF79792A519A55171B1B650C23661D15897263";
    let pk_hex = "0404EBFC718E8D1798620432268E77FEB6415E2EDE0E073C0F4F640ECD2E149A73E858F9D81E5430A57B36DAAB8F950A3C64E6EE6A63094C992833076DB6855FD6";
    
    let sk = BigUint::from_str_radix(sk_hex, 16).unwrap();
    // Reconstruct pk from hex just for test context, assuming we have a way or just use context
    // Actually SigCtx::new().sign wants `pk` as `Point`.
    // In main.rs, user generates keypair, so we have a valid Point.
    
    // We can't easily construct a Point without using libsm internals or generator.
    // But we can check if `Sign` crashes on short data.
    
    println!("Testing Sign with empty data...");
    // Mocking Context
    let ctx = SigCtx::new();
    let (pk_point, sk_int) = ctx.new_keypair().unwrap(); // Generate valid pair
    
    let data = b""; // Empty
    let res = ctx.sign(data, &sk_int, &pk_point);
    println!("Sign empty Result: {:?}", res.is_ok());

    let data = b"1"; // Short
    let res = ctx.sign(data, &sk_int, &pk_point);
    println!("Sign short Result: {:?}", res.is_ok());
    
    println!("Testing Verify with short signature...");
    // Let's make a garbage signature
    let garbage_sig_der = vec![0x30, 0x01, 0x02]; // Malformed DER
    let sig_res = Signature::der_decode(&garbage_sig_der);
    println!("Decode garbage Result: {:?}", sig_res.is_ok());
    
    let empty_sig_der: Vec<u8> = vec![];
    let sig_res_empty = Signature::der_decode(&empty_sig_der);
    println!("Decode empty Result: {:?}", sig_res_empty.is_ok());

    println!("Testing Sign with ZERO Private Key...");
    let zero_sk = BigUint::default(); // 0
    // We use the previously generated valid pk, so pk matches a DIFFERENT sk. 
    // This is 'mismatched sk/pk' scenario + 'invalid sk' scenario.
    let res_zero = ctx.sign(b"test", &zero_sk, &pk_point);
    println!("Sign zero_sk Result: {:?}", res_zero.is_ok());
    
    println!("Testing DER decode with fuzzy bad inputs...");
    let bad_inputs = vec![
        vec![0x30], 
        vec![0x30, 0x00],
        vec![0x30, 0x01],
        vec![0x30, 0x80], // Indefinite length, often dangerous
        vec![0x02, 0x01, 0x00], // Integers only
        vec![0x30, 0x03, 0x02, 0x01], // Sequence truncated
    ];
    
    use libsm::sm2::encrypt::EncryptCtx;
    use libsm::sm2::encrypt::DecryptCtx; // Import if needed for completeness
    
    println!("Testing Encrypt/Decrypt...");
    let enc_ctx = EncryptCtx::new(32, pk_point.clone());
    
    // We already know empty panics, skipping empty test to check short string
    // let res_enc_empty = enc_ctx.encrypt(b"");
    // println!("Encrypt empty Result: {:?}", res_enc_empty.is_ok());

    println!("Finding safe length for Encrypt...");
    for len in 0..100 {
        let input = vec![0u8; len];
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
             let enc_ctx = EncryptCtx::new(32, pk_point.clone());
             enc_ctx.encrypt(&input)
        }));
        if res.is_err() {
            // println!("Len {} -> PANIC", len); // Reduce noise
        } else {
            let inner_res = res.unwrap();
            println!("Len {} -> Result {:?}", len, inner_res.is_ok());
            break; // Found one!
        }
    }
}
