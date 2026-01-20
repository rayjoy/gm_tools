//! ZUC-256 New Initialization Scheme (48 rounds, 128-bit IV)
//! Based on "A New Initialization Scheme of the ZUC-256 Stream Cipher"
//! and corresponding SAGE liaison.

pub const S0: [u8; 256] = [
    0x3E, 0x72, 0x5B, 0x47, 0xCA, 0xE0, 0x00, 0x33, 0x04, 0xD1, 0x54, 0x98, 0x09, 0xB9, 0x6D, 0xCB,
    0x7B, 0x1B, 0xF9, 0x32, 0xAF, 0x9D, 0x6A, 0xA5, 0xB8, 0x2D, 0xFC, 0x1D, 0x08, 0x53, 0x03, 0x90,
    0x4D, 0x4E, 0x84, 0x99, 0xE4, 0xCE, 0xD9, 0x91, 0xDD, 0xB6, 0x85, 0x48, 0x8B, 0x29, 0x6E, 0xAC,
    0xCD, 0xC1, 0xF8, 0x1E, 0x73, 0x43, 0x69, 0xC6, 0xB5, 0xBD, 0xFD, 0x39, 0x63, 0x20, 0xD4, 0x38,
    0x76, 0x7D, 0xB2, 0xA7, 0xCF, 0xED, 0x57, 0xC5, 0xF3, 0x2C, 0xBB, 0x14, 0x21, 0x06, 0x55, 0x9B,
    0xE3, 0xEF, 0x5E, 0x31, 0x4F, 0x7F, 0x5A, 0xA4, 0x0D, 0x82, 0x51, 0x49, 0x5F, 0xBA, 0x58, 0x1C,
    0x4A, 0x16, 0xD5, 0x17, 0xA8, 0x92, 0x24, 0x1F, 0x8C, 0xFF, 0xD8, 0xAE, 0x2E, 0x01, 0xD3, 0xAD,
    0x3B, 0x4B, 0xDA, 0x46, 0xEB, 0xC9, 0xDE, 0x9A, 0x8F, 0x87, 0xD7, 0x3A, 0x80, 0x6F, 0x2F, 0xC8,
    0xB1, 0xB4, 0x37, 0xF7, 0x0A, 0x22, 0x13, 0x28, 0x7C, 0xCC, 0x3C, 0x89, 0xC7, 0xC3, 0x96, 0x56,
    0x07, 0xBF, 0x7E, 0xF0, 0x0B, 0x2B, 0x97, 0x52, 0x35, 0x41, 0x79, 0x61, 0xA6, 0x4C, 0x10, 0xFE,
    0xBC, 0x26, 0x95, 0x88, 0x8A, 0xB0, 0xA3, 0xFB, 0xC0, 0x18, 0x94, 0xF2, 0xE1, 0xE5, 0xE9, 0x5D,
    0xD0, 0xDC, 0x11, 0x66, 0x64, 0x5C, 0xEC, 0x59, 0x42, 0x75, 0x12, 0xF5, 0x74, 0x9C, 0xAA, 0x23,
    0x0E, 0x86, 0xAB, 0xBE, 0x2A, 0x02, 0xE7, 0x67, 0xE6, 0x44, 0xA2, 0x6C, 0xC2, 0x93, 0x9F, 0xF1,
    0xF6, 0xFA, 0x36, 0xD2, 0x50, 0x68, 0x9E, 0x62, 0x71, 0x15, 0x3D, 0xD6, 0x40, 0xC4, 0xE2, 0x0F,
    0x8E, 0x83, 0x77, 0x6B, 0x25, 0x05, 0x3F, 0x0C, 0x30, 0xEA, 0x70, 0xB7, 0xA1, 0xE8, 0xA9, 0x65,
    0x8D, 0x27, 0x1A, 0xDB, 0x81, 0xB3, 0xA0, 0xF4, 0x45, 0x7A, 0x19, 0xDF, 0xEE, 0x78, 0x34, 0x60,
];

pub const S1: [u8; 256] = [
    0x55, 0xC2, 0x63, 0x71, 0x3B, 0xC8, 0x47, 0x86, 0x9F, 0x3C, 0xDA, 0x5B, 0x29, 0xAA, 0xFD, 0x77,
    0x8C, 0xC5, 0x94, 0x0C, 0xA6, 0x1A, 0x13, 0x00, 0xE3, 0xA8, 0x16, 0x72, 0x40, 0xF9, 0xF8, 0x42,
    0x44, 0x26, 0x68, 0x96, 0x81, 0xD9, 0x45, 0x3E, 0x10, 0x76, 0xC6, 0xA7, 0x8B, 0x39, 0x43, 0xE1,
    0x3A, 0xB5, 0x56, 0x2A, 0xC0, 0x6D, 0xB3, 0x05, 0x22, 0x66, 0xBF, 0xDC, 0x0B, 0xFA, 0x62, 0x48,
    0xDD, 0x20, 0x11, 0x06, 0x36, 0xC9, 0xC1, 0xCF, 0xF6, 0x27, 0x52, 0xBB, 0x69, 0xF5, 0xD4, 0x87,
    0x7F, 0x84, 0x4C, 0xD2, 0x9C, 0x57, 0xA4, 0xBC, 0x4F, 0x9A, 0xDF, 0xFE, 0xD6, 0x8D, 0x7A, 0xEB,
    0x2B, 0x53, 0xD8, 0x5C, 0xA1, 0x14, 0x17, 0xFB, 0x23, 0xD5, 0x7D, 0x30, 0x67, 0x73, 0x08, 0x09,
    0xEE, 0xB7, 0x70, 0x3F, 0x61, 0xB2, 0x19, 0x8E, 0x4E, 0xE5, 0x4B, 0x93, 0x8F, 0x5D, 0xDB, 0xA9,
    0xAD, 0xF1, 0xAE, 0x2E, 0xCB, 0x0D, 0xFC, 0xF4, 0x2D, 0x46, 0x6E, 0x1D, 0x97, 0xE8, 0xD1, 0xE9,
    0x4D, 0x37, 0xA5, 0x75, 0x5E, 0x83, 0x9E, 0xAB, 0x82, 0x9D, 0xB9, 0x1C, 0xE0, 0xCD, 0x49, 0x89,
    0x01, 0xB6, 0xBD, 0x58, 0x24, 0xA2, 0x5F, 0x38, 0x78, 0x99, 0x15, 0x90, 0x50, 0xB8, 0x95, 0xE4,
    0xD0, 0x91, 0xC7, 0xCE, 0xED, 0x0F, 0xB4, 0x6F, 0xA0, 0xCC, 0xF0, 0x02, 0x4A, 0x79, 0xC3, 0xDE,
    0xA3, 0xEF, 0xEA, 0x51, 0xE6, 0x6B, 0x18, 0xEC, 0x1B, 0x2C, 0x80, 0xF7, 0x74, 0xE7, 0xFF, 0x21,
    0x5A, 0x6A, 0x54, 0x1E, 0x41, 0x31, 0x92, 0x35, 0xC4, 0x33, 0x07, 0x0A, 0xBA, 0x7E, 0x0E, 0x34,
    0x88, 0xB1, 0x98, 0x7C, 0xF3, 0x3D, 0x60, 0x6C, 0x7B, 0xCA, 0xD3, 0x1F, 0x32, 0x65, 0x04, 0x28,
    0x64, 0xBE, 0x85, 0x9B, 0x2F, 0x59, 0x8A, 0xD7, 0xB0, 0x25, 0xAC, 0xAF, 0x12, 0x03, 0xE2, 0xF2,
];

// Constants d0..d15 for Key/IV loading (7-bits each)
const D_ENC: [u16; 16] = [
    0x64, 0x43, 0x7B, 0x2A, 0x11, 0x05, 0x51, 0x42,
    0x1A, 0x31, 0x18, 0x66, 0x14, 0x2E, 0x01, 0x5C,
];

const D_MAC_32: [u16; 16] = [
    0x64, 0x43, 0x7A, 0x2A, 0x11, 0x05, 0x51, 0x42, // d2=7A
    0x1A, 0x31, 0x18, 0x66, 0x14, 0x2E, 0x01, 0x5C,
];

const D_MAC_64: [u16; 16] = [
    0x65, 0x43, 0x7B, 0x2A, 0x11, 0x05, 0x51, 0x42, // d0=65
    0x1A, 0x31, 0x18, 0x66, 0x14, 0x2E, 0x01, 0x5C,
];

const D_MAC_128: [u16; 16] = [
    0x65, 0x43, 0x7A, 0x2A, 0x11, 0x05, 0x51, 0x42, // d0=65, d2=7A
    0x1A, 0x31, 0x18, 0x66, 0x14, 0x2E, 0x01, 0x5C,
];

#[derive(Clone, Copy, PartialEq)]
pub enum Zuc256Mode {
    Encrypt,
    Mac32,
    Mac64,
    Mac128,
}

pub struct Zuc256NewStreamCipher {
    lfsr: [u32; 16],
    r1: u32,
    r2: u32,
}

impl Zuc256NewStreamCipher {
    /// Create a new ZUC-256 instance with the NEW initialization scheme.
    /// Key must be 32 bytes (256 bits).
    /// IV must be 16 bytes (128 bits).
    pub fn new(key: &[u8], iv: &[u8], mode: Zuc256Mode) -> Self {
        assert_eq!(key.len(), 32, "ZUC-256 requires 32-byte key");
        assert_eq!(iv.len(), 16, "ZUC-256 New Scheme requires 16-byte IV");

        let mut z = Self {
            lfsr: [0; 16],
            r1: 0,
            r2: 0,
        };
        z.init(key, iv, mode);
        z
    }

    fn init(&mut self, key: &[u8], iv: &[u8], mode: Zuc256Mode) {
        let d_const = match mode {
            Zuc256Mode::Encrypt => &D_ENC,
            Zuc256Mode::Mac32 => &D_MAC_32,
            Zuc256Mode::Mac64 => &D_MAC_64,
            Zuc256Mode::Mac128 => &D_MAC_128,
        };

        for i in 0..16 {
            let k_byte1 = key[i];     // K_i
            let d_val = d_const[i];   // d_i
            
            let byte3;
            let byte4;

            if i < 7 {
                byte3 = key[16 + i];
                byte4 = key[24 + i];
            } else if i < 15 {
                byte3 = iv[i - 7];
                byte4 = iv[i - 7 + 8];
            } else {
                byte3 = key[23];
                byte4 = key[31];
            }

            let val = ((k_byte1 as u32) << 23) |
                      ((d_val as u32 & 0x7F) << 16) |
                      ((byte3 as u32) << 8) |
                      (byte4 as u32);
            
            self.lfsr[i] = val;
        }

        self.r1 = 0;
        self.r2 = 0;

        // 48 rounds
        for _ in 0..48 {
            let w = self.f();
            let u = w >> 1;
            self.lfsr_with_initialization_mode(u);
        }

        // Discard the first word after initialization
        self.gen_word();
    }

    pub fn apply_keystream(&mut self, data: &mut [u8]) {
        let len = data.len();
        let chunks = len / 4;
        let remainder = len % 4;

        for i in 0..chunks {
            let z = self.gen_word();
            let offset = i * 4;
            
            data[offset] ^= (z >> 24) as u8;
            data[offset + 1] ^= (z >> 16) as u8;
            data[offset + 2] ^= (z >> 8) as u8;
            data[offset + 3] ^= z as u8;
        }

        if remainder > 0 {
            let z = self.gen_word();
            let offset = chunks * 4;
            for j in 0..remainder {
                let b = (z >> (24 - 8*j)) as u8;
                data[offset + j] ^= b;
            }
        }
    }

    /// Generate MAC tag.
    /// `msg_bits`: message length in bits.
    /// `tag_len_bits`: 32, 64, or 128.
    /// `msg`: message data.
    pub fn generate_mac(&mut self, msg: &[u8], msg_bits: usize, tag_len_bits: usize) -> Vec<u8> {
        // L = ceil(l + 2*t)/32
        // words
        let t = tag_len_bits;
        let l = msg_bits;
        let l_plus_2t = l + 2 * t;
        let word_count = (l_plus_2t + 31) / 32;

        let mut keystream = Vec::with_capacity(word_count * 4);
        for _ in 0..word_count {
             let z = self.gen_word();
             keystream.extend_from_slice(&z.to_be_bytes());
        }

        // Bit extraction helper
        // Get t bits starting from bit_index
        let get_w = |start_bit: usize, len: usize| -> Vec<u8> {
            let mut res = vec![0u8; (len + 7) / 8];
            for i in 0..len {
                 let bit_idx = start_bit + i;
                 let byte_idx = bit_idx / 8;
                 let bit_in_byte = 7 - (bit_idx % 8);
                 if byte_idx < keystream.len() {
                    let bit = (keystream[byte_idx] >> bit_in_byte) & 1;
                    let res_byte_idx = i / 8;
                    let res_bit_in_byte = 7 - (i % 8);
                    if bit == 1 {
                        res[res_byte_idx] |= 1 << res_bit_in_byte;
                    }
                 }
            }
            res
        };

        let mut tag = get_w(0, t);
        
        for i in 0..l {
            // Check m_i
            let byte_idx = i / 8;
            let bit_in_byte = 7 - (i % 8);
            if byte_idx < msg.len() {
                if (msg[byte_idx] >> bit_in_byte) & 1 == 1 {
                     let w_i = get_w(t + i, t);
                     // Tag ^= W_i
                     for (tb, wb) in tag.iter_mut().zip(w_i.iter()) {
                         *tb ^= *wb;
                     }
                }
            }
        }
        
        // Final XOR
        let w_l = get_w(l + t, t);
        for (tb, wb) in tag.iter_mut().zip(w_l.iter()) {
             *tb ^= *wb;
        }

        tag
    }

    fn gen_word(&mut self) -> u32 {
        let s2 = self.lfsr[2];
        let s0 = self.lfsr[0];
        let x3 = ((s2 & 0xFFFF) << 16) | (s0 >> 15);

        let w = self.f();
        let z = w ^ x3;
        
        self.lfsr_with_work_mode();
        z
    }

    fn f(&mut self) -> u32 {
        let s15 = self.lfsr[15];
        let s14 = self.lfsr[14];
        let x0 = ((s15 >> 15) << 16) | (s14 & 0xFFFF);

        let s11 = self.lfsr[11];
        let s9 = self.lfsr[9];
        let x1 = ((s11 & 0xFFFF) << 16) | (s9 >> 15);

        let s7 = self.lfsr[7];
        let s5 = self.lfsr[5];
        let x2 = ((s7 & 0xFFFF) << 16) | (s5 >> 15);

        let w = (x0 ^ self.r1).wrapping_add(self.r2);
        let w1 = self.r1.wrapping_add(x1);
        let w2 = self.r2 ^ x2;

        let w1l = w1 & 0xFFFF;
        let w2h = w2 >> 16;
        self.r1 = s_transform(l1((w1l << 16) | w2h));

        let w2l = w2 & 0xFFFF;
        let w1h = w1 >> 16;
        self.r2 = s_transform(l2((w2l << 16) | w1h));

        w
    }

    fn lfsr_with_initialization_mode(&mut self, u: u32) {
        let v = self.calc_v();
        let s16 = self.add_mod31(v, u);
        self.update_lfsr(s16);
    }
    
    fn lfsr_with_work_mode(&mut self) {
        let v = self.calc_v();
        self.update_lfsr(v);
    }
    
    fn calc_v(&self) -> u32 {
        let s15 = self.lfsr[15];
        let s13 = self.lfsr[13];
        let s10 = self.lfsr[10];
        let s4 = self.lfsr[4];
        let s0 = self.lfsr[0];

        let v1 = mul31(s15, 15);
        let v2 = mul31(s13, 17);
        let v3 = mul31(s10, 21);
        let v4 = mul31(s4, 20);
        let v5 = s0; 
        let v6 = mul31(s0, 8);

        let sum = self.add_mod31(v1, v2);
        let sum = self.add_mod31(sum, v3);
        let sum = self.add_mod31(sum, v4);
        let sum = self.add_mod31(sum, v5);
        self.add_mod31(sum, v6)
    }

    fn add_mod31(&self, a: u32, b: u32) -> u32 {
        let v = a.wrapping_add(b);
        let v = (v & 0x7FFFFFFF) + (v >> 31);
        let v = (v & 0x7FFFFFFF) + (v >> 31);
        v
    }

    fn update_lfsr(&mut self, s16: u32) {
        // Shift
        for i in 0..15 {
            self.lfsr[i] = self.lfsr[i+1];
        }
        self.lfsr[15] = s16;
        if self.lfsr[15] == 0 {
            self.lfsr[15] = 0x7FFFFFFF;
        }
    }
}

// Helpers
fn rot(a: u32, k: u32) -> u32 { (a << k) | (a >> (32 - k)) }
fn l1(x: u32) -> u32 { x ^ rot(x, 2) ^ rot(x, 10) ^ rot(x, 18) ^ rot(x, 24) }
fn l2(x: u32) -> u32 { x ^ rot(x, 8) ^ rot(x, 14) ^ rot(x, 22) ^ rot(x, 30) }

fn mul31(s: u32, k: u32) -> u32 {
    let s = s & 0x7FFFFFFF;
    ((s << k) | (s >> (31 - k))) & 0x7FFFFFFF
}

fn s_transform(a: u32) -> u32 {
    let b0 = S0[((a >> 24) & 0xFF) as usize];
    let b1 = S1[((a >> 16) & 0xFF) as usize];
    let b2 = S0[((a >> 8) & 0xFF) as usize];
    let b3 = S1[(a & 0xFF) as usize];
    u32::from_be_bytes([b0, b1, b2, b3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_1() {
        // TV1: Key=0, IV=0
        let key = [0u8; 32];
        let iv = [0u8; 16];
        let mut cipher = Zuc256NewStreamCipher::new(&key, &iv, Zuc256Mode::Encrypt);
        
        // Generate first 4 words of keystream
        let z0 = cipher.gen_word();
        let z1 = cipher.gen_word();
        let z2 = cipher.gen_word();
        let z3 = cipher.gen_word();

        println!("TV1 Output: {:08x}, {:08x}, {:08x}, {:08x}", z0, z1, z2, z3);
        
        // Assert first 4 words exactly match PDF test vectors
        assert_eq!(z0, 0x0234e932);
        assert_eq!(z1, 0xf0c22292);
        assert_eq!(z2, 0x38853662);
        assert_eq!(z3, 0xaa624def);
    }

    #[test]
    fn test_vector_2() {
        // TV2: Key=All 1s, IV=All 1s
        let key = [0xFFu8; 32];
        let iv = [0xFFu8; 16];
        let mut cipher = Zuc256NewStreamCipher::new(&key, &iv, Zuc256Mode::Encrypt);
        
        // Generate first 4 words of keystream
        let z0 = cipher.gen_word();
        let z1 = cipher.gen_word();
        let z2 = cipher.gen_word();
        let z3 = cipher.gen_word();

        println!("TV2 Output: {:08x}, {:08x}, {:08x}, {:08x}", z0, z1, z2, z3);

        assert_eq!(z0, 0x3985e2af);
        assert_eq!(z1, 0x3533d429);
        assert_eq!(z2, 0x338580f0);
        assert_eq!(z3, 0xe0d80ce9);
    }

    #[test]
    fn test_mac_tv1() {
        // Mac TV1 (from PDF text)
        // Key=0, IV=0, M=0 (All zero msg, 400 bits)
        // 32-bit tag: d51f12fc
        let key = [0u8; 32];
        let iv = [0u8; 16];
        let msg = [0u8; 50]; // 400 bits = 50 bytes

        let mut cipher = Zuc256NewStreamCipher::new(&key, &iv, Zuc256Mode::Mac32);
        let tag = cipher.generate_mac(&msg, 400, 32);
        assert_eq!(hex::encode(&tag), "d51f12fc");

        // 64-bit tag: 3f4aaa58 99158f4a
        let mut cipher = Zuc256NewStreamCipher::new(&key, &iv, Zuc256Mode::Mac64);
        let tag = cipher.generate_mac(&msg, 400, 64);
        assert_eq!(hex::encode(&tag), "3f4aaa5899158f4a");

        // 128-bit tag: cf4bc324 7d0f6ae5 ce498d54 4556c247
        let mut cipher = Zuc256NewStreamCipher::new(&key, &iv, Zuc256Mode::Mac128);
        let tag = cipher.generate_mac(&msg, 400, 128);
        assert_eq!(hex::encode(&tag), "cf4bc3247d0f6ae5ce498d544556c247");
    }
}
