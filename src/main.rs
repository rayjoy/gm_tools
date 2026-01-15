#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use sm3::{Sm3, Digest};
use sm4::cipher::{NewBlockCipher, BlockEncrypt, BlockDecrypt, generic_array::GenericArray};
use sm4::Sm4;
use zuc::zuc128::zuc128_xor_inplace;
use num_traits::Num;
use num_bigint::BigUint;
use libsm::sm2::signature::{SigCtx, Signature};
use libsm::sm2::ecc::Point; // FieldElem is private

// We will rely on libsm for SM2.
// Since I need to discover the exact API, I'll start with imports that I think exist.




fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "国密算法工具箱 (GM Tools)",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(GmApp::default()))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 尝试加载 Windows 系统自带的中文字体
    // 优先尝试微软雅黑 (msyh.ttc), 然后是黑体 (simhei.ttf)
    let font_paths = [
        "c:/Windows/Fonts/msyh.ttc",
        "c:/Windows/Fonts/simhei.ttf",
    ];

    let mut font_data_bytes = None;
    for path in font_paths {
        if let Ok(data) = std::fs::read(path) {
            println!("已加载字体: {}", path);
            font_data_bytes = Some(data);
            break;
        }
    }

    if let Some(data) = font_data_bytes {
        // 安装字体
        fonts.font_data.insert(
            "microsoft_yahei".to_owned(),
            egui::FontData::from_owned(data),
        );

        // 将新字体设置为 Proportional 和 Monospace 的首选字体
        if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            family.insert(0, "microsoft_yahei".to_owned());
        }
        if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
            family.push("microsoft_yahei".to_owned());
        }

        // 应用字体配置
        ctx.set_fonts(fonts);
    } else {
        println!("警告: 未找到中文字体，可能会显示乱码。");
    }
}

struct GmApp {
    selected_tab: Tab,
    sm3_state: Sm3State,
    sm4_state: Sm4State,
    sm2_state: Sm2State,
    zuc_state: ZucState,
    sm2_ctx: SigCtx,
}

impl Default for GmApp {
    fn default() -> Self {
        Self {
            selected_tab: Tab::default(),
            sm3_state: Sm3State::default(),
            sm4_state: Sm4State::default(),
            sm2_state: Sm2State::default(),
            zuc_state: ZucState::default(),
            sm2_ctx: SigCtx::new(),
        }
    }
}

#[derive(PartialEq, Eq, Default)]
enum Tab {
    #[default]
    SM3,
    SM4,
    SM2,
    ZUC,
}


#[derive(Default)]
struct ZucState {
    key: String,
    iv: String,
    input: String,
    output: String,
}

#[derive(Default)]
struct Sm3State {
    input: String,
    output: String,
}

#[derive(Default)]
struct Sm2State {
    pri_key: String,
    pub_key: String,
    input: String,
    output: String,
    mode: Sm2Mode,
    cached_pk: Option<Point>, // Cache for Point object to avoid parsing issues if generated here
    cached_sk: Option<BigUint>,
    signature_input: String,
}



#[derive(PartialEq, Eq, Default)]
enum Sm2Mode {
    #[default]
    Sign,
    Verify,
    Encrypt,
    Decrypt,
    GenKey, 
}


struct Sm4State {
    key: String,
    iv: String,
    data: String,
    output: String,
    mode_cbc: bool,
}

impl Default for Sm4State {
    fn default() -> Self {
        Self {
            key: String::new(),
            iv: String::new(),
            data: String::new(),
            output: String::new(),
            mode_cbc: true, // Default to CBC
        }
    }
}

impl eframe::App for GmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("国密算法小工具 (GM Tools)");
            ui.separator();

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, Tab::SM3, "SM3 摘要");
                ui.selectable_value(&mut self.selected_tab, Tab::SM4, "SM4 加解密");
                ui.selectable_value(&mut self.selected_tab, Tab::SM2, "SM2 非对称");
                ui.selectable_value(&mut self.selected_tab, Tab::ZUC, "ZUC 序列密码");
            });
            ui.separator();

            match self.selected_tab {
                Tab::SM3 => self.show_sm3(ui),
                Tab::SM4 => self.show_sm4(ui),
                Tab::SM2 => self.show_sm2(ui),
                Tab::ZUC => self.show_zuc(ui),
            }
        });
    }
}

impl GmApp {
    fn show_sm2(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("功能:");
            ui.radio_value(&mut self.sm2_state.mode, Sm2Mode::GenKey, "生成密钥");
            ui.radio_value(&mut self.sm2_state.mode, Sm2Mode::Sign, "签名");
            ui.radio_value(&mut self.sm2_state.mode, Sm2Mode::Verify, "验签");
            ui.radio_value(&mut self.sm2_state.mode, Sm2Mode::Encrypt, "加密");
            ui.radio_value(&mut self.sm2_state.mode, Sm2Mode::Decrypt, "解密");
        });
        ui.separator();

        if self.sm2_state.mode == Sm2Mode::GenKey {
            if ui.button("生成新密钥对").clicked() {
                self.process_sm2_genkey();
            }
        }

        ui.label("私钥 Private Key (Hex):");
        ui.text_edit_multiline(&mut self.sm2_state.pri_key);
        
        ui.label("公钥 Public Key (Hex 04...):");
        ui.text_edit_multiline(&mut self.sm2_state.pub_key);

        if self.sm2_state.mode != Sm2Mode::GenKey {
            ui.separator();
            
            let label_text = if self.sm2_state.mode == Sm2Mode::Verify {
                "原始数据 (String):"
            } else {
                "输入数据 (String/Hex):"
            };
            ui.label(label_text);
            ui.text_edit_multiline(&mut self.sm2_state.input);
            
            if self.sm2_state.mode == Sm2Mode::Verify {
                 ui.label("签名值 (Hex DER):");
                 ui.text_edit_multiline(&mut self.sm2_state.signature_input);
            }

            let btn_text = match self.sm2_state.mode {
                Sm2Mode::Sign => "签名",
                Sm2Mode::Verify => "验证",
                Sm2Mode::Encrypt => "加密",
                Sm2Mode::Decrypt => "解密",
                _ => "",
            };

            if ui.button(btn_text).clicked() {
                self.process_sm2_action();
            }

            ui.label("输出结果:");
            ui.text_edit_multiline(&mut self.sm2_state.output);
        }
    }


    fn process_sm2_genkey(&mut self) {
        let ctx = &self.sm2_ctx;
        match ctx.new_keypair() {
            Ok((pk, sk)) => {
                self.sm2_state.pri_key = sk.to_str_radix(16);
                let x = pk.x.to_str(16);
                let y = pk.y.to_str(16);
                
                let x_pad = format!("{:0>64}", x);
                let y_pad = format!("{:0>64}", y);
                self.sm2_state.pub_key = format!("04{}{}", x_pad, y_pad);

                // Cache keys (move, avoid clone)
                self.sm2_state.cached_sk = Some(sk);
                self.sm2_state.cached_pk = Some(pk);
                self.sm2_state.output = "密钥生成成功".to_string();
            }

            Err(e) => {
                self.sm2_state.output = format!("生成失败: {:?}", e);
            }
        }
    }


    fn process_sm2_action(&mut self) {
        let ctx = &self.sm2_ctx;
        match self.sm2_state.mode {
            Sm2Mode::GenKey => {},
            Sm2Mode::Sign => {
                 let sk_str = &self.sm2_state.pri_key;
                 let sk = match BigUint::from_str_radix(sk_str, 16) {
                     Ok(v) => v,
                     Err(_) => { self.sm2_state.output = "Private key 格式错误".into(); return; }
                 };
                 
                 // Use cached PK if available, else warn
                 let pk = if let Some(ref p) = self.sm2_state.cached_pk {
                     p
                 } else {
                     self.sm2_state.output = "请先生成密钥 (暂不支持导入公钥对象)".into();
                     return;
                 };
                 
                 let data = self.sm2_state.input.as_bytes();
                 match ctx.sign(data, &sk, pk) {
                     Ok(signature) => {
                         let der = signature.der_encode();
                         self.sm2_state.output = hex::encode(der);
                     }
                     Err(e) => self.sm2_state.output = format!("签名失败: {:?}", e),
                 }
            },
            Sm2Mode::Verify => {
                let pk = if let Some(ref p) = self.sm2_state.cached_pk {
                     p
                 } else {
                     self.sm2_state.output = "请先生成密钥 (暂不支持导入公钥对象)".into();
                     return;
                 };
                 
                 let data = self.sm2_state.input.as_bytes();
                 let sig_hex = &self.sm2_state.signature_input;
                 let sig_der = match hex::decode(sig_hex) {
                     Ok(v) => v,
                     Err(_) => { self.sm2_state.output = "签名值必须是 Hex".into(); return; }
                 };
                 
                 match Signature::der_decode(&sig_der) {
                     Ok(signature) => {
                         match ctx.verify(data, pk, &signature) {
                             Ok(true) => self.sm2_state.output = "验签结果: 通过 (Valid)".to_string(),
                             Ok(false) => self.sm2_state.output = "验签结果: 失败 (Invalid)".to_string(),
                             Err(e) => self.sm2_state.output = format!("验签过程出错: {:?}", e),
                         }
                     },
                     Err(e) => {
                         self.sm2_state.output = format!("签名解析失败: {:?}", e);
                     }
                 }

            },
            Sm2Mode::Encrypt => {
                 let pk = if let Some(ref p) = self.sm2_state.cached_pk {
                     p
                 } else {
                     self.sm2_state.output = "请先生成密钥 (暂不支持导入公钥对象)".into();
                     return;
                 };
                 let data = self.sm2_state.input.as_bytes();
                 
                 if data.is_empty() {
                     self.sm2_state.output = "加密失败: 输入数据不能为空".to_string();
                     return;
                 }

                 // Workaround for libsm 0.6.0 panic on inputs < 32 bytes
                 let mut data_vec = data.to_vec();
                 let mut _padding_info = String::new();
                 if data_vec.len() < 32 {
                     let pad_len = 32 - data_vec.len();
                     // PKCS#7-like padding: pad with bytes of value `pad_len`
                     data_vec.extend(std::iter::repeat(pad_len as u8).take(pad_len));
                     _padding_info = format!("\n(注意: 原数据不足32字节，已按 PKCS#7 规则补齐至32字节)");
                 }

                 let pk_clone = pk.clone();
                 
                 let result = std::panic::catch_unwind(move || {
                     let ctx = libsm::sm2::encrypt::EncryptCtx::new(32, pk_clone);
                     ctx.encrypt(&data_vec)
                 });

                 match result {
                     Ok(encrypt_res) => {
                         match encrypt_res {
                             Ok(ciphertext) => {
                                 self.sm2_state.output = hex::encode(ciphertext) + &_padding_info;
                             }
                             Err(e) => self.sm2_state.output = format!("加密失败: {:?}", e),
                         }
                     }
                     Err(_) => {
                         self.sm2_state.output = "加密崩溃: libsm 库在处理短数据(可能<32字节)时发生 panic。请尝试更长的数据。".to_string();
                     }
                 }
            },
            Sm2Mode::Decrypt => {
                 let sk_str = &self.sm2_state.pri_key;
                 let sk = match BigUint::from_str_radix(sk_str, 16) {
                     Ok(v) => v,
                     Err(_) => { self.sm2_state.output = "Private key 格式错误".into(); return; }
                 };
                 let data = match hex::decode(&self.sm2_state.input) {
                     Ok(d) => d,
                     Err(_) => { self.sm2_state.output = "输入数据必须是 Hex".into(); return; }
                 };
                 
                 // Libsm manual check to avoid panic
                 if data.len() < 97 {
                     self.sm2_state.output = format!("解密失败: 数据长度不足 ({} bytes)。SM2 密文至少需要 97 字节 (C1[65] + C3[32])。", data.len());
                     return;
                 }
                 
                 let dec_ctx = libsm::sm2::encrypt::DecryptCtx::new(32, sk);
                 
                 // Defining a helper closure for processing plaintext (Unpadding + Display)
                 // Needs to capture self (for output) but self is borrowed as mut.
                 // So we process result outside.
                 
                 let attempt_decrypt = |input_data: &[u8]| -> Option<(Vec<u8>, String)> {
                     match dec_ctx.decrypt(input_data) {
                         Ok(pt) => Some((pt, String::new())),
                         Err(_) => None
                     }
                 };

                 // 1. Try Standard Decryption (Assuming Input is C1C3C2)
                 let mut success_result = attempt_decrypt(&data);
                 let mut mode_msg = String::new();

                 // 2. If failed, Try C1C2C3 -> C1C3C2 conversion
                 if success_result.is_none() && data.len() >= 97 {
                      let c1_len = 65;
                      let c3_len = 32;
                      let c2_len = data.len() - 97;
                      
                      // Assuming standard uncompressed C1 (04...)
                      // Input C1C2C3: C1 [0..65], C2 [65..65+C2Len], C3 [65+C2Len..end]
                      let c1 = &data[0..c1_len];
                      let c2 = &data[c1_len .. c1_len + c2_len];
                      let c3 = &data[c1_len + c2_len .. ];
                      
                      let mut swapped = Vec::with_capacity(data.len());
                      swapped.extend_from_slice(c1);
                      swapped.extend_from_slice(c3);
                      swapped.extend_from_slice(c2);
                      
                      if let Some((pt, _)) = attempt_decrypt(&swapped) {
                          success_result = Some((pt, String::new()));
                          mode_msg = " [检测到 C1C2C3 格式，已自动兼容]".to_string();
                      }
                 }

                 match success_result {
                     Some((plaintext, _)) => {
                         let mut display_text = plaintext.clone();
                         let mut unpad_msg = String::new();
                         
                         // PKCS#7 Unpadding
                         if let Some(&pad_byte) = display_text.last() {
                             let pad_len = pad_byte as usize;
                             if pad_len > 0 && pad_len <= display_text.len() && pad_len <= 32 {
                                 let start = display_text.len() - pad_len;
                                 if display_text[start..].iter().all(|&b| b == pad_byte) {
                                      display_text.truncate(start);
                                      unpad_msg = format!(" (已自动去除 PKCS#7 填充: {} 字节)", pad_len);
                                 }
                             }
                         }

                         let final_msg = format!("{}{}", mode_msg, unpad_msg);

                         match String::from_utf8(display_text) {
                             Ok(s) => self.sm2_state.output = s + &final_msg,
                             Err(_) => self.sm2_state.output = "解密成功 (Hex): ".to_string() + &hex::encode(plaintext) + &final_msg,
                         }
                     },
                     None => {
                        // If everything failed, running original logic to get the specific error message
                        match dec_ctx.decrypt(&data) {
                            Ok(_) => {}, // Should not happen
                            Err(e) => self.sm2_state.output = format!("解密失败: {:?}", e),
                        }
                     }
                 }
            },

        }
    }



    fn show_sm3(&mut self, ui: &mut egui::Ui) {

        ui.label("输入内容 (UTF-8 字符串):");
        ui.text_edit_multiline(&mut self.sm3_state.input);

        if ui.button("计算 Hash").clicked() {
            let mut hasher = Sm3::new();
            hasher.update(self.sm3_state.input.as_bytes());
            let result = hasher.finalize();
            self.sm3_state.output = hex::encode(result);
        }

        ui.label("输出结果 (Hex):");
        ui.text_edit_multiline(&mut self.sm3_state.output);
    }

    fn show_sm4(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("模式:");
            ui.radio_value(&mut self.sm4_state.mode_cbc, false, "ECB");
            ui.radio_value(&mut self.sm4_state.mode_cbc, true, "CBC");
        });

        ui.label("密钥 Key (16 bytes, Hex encoded):");
        ui.text_edit_singleline(&mut self.sm4_state.key);

        if self.sm4_state.mode_cbc {
            ui.label("向量 IV (16 bytes, Hex encoded):");
            ui.text_edit_singleline(&mut self.sm4_state.iv);
        }

        ui.label("数据 (Hex encoded):");
        ui.text_edit_multiline(&mut self.sm4_state.data);

        ui.horizontal(|ui| {
            if ui.button("加密").clicked() {
                self.process_sm4_action(true);
            }
            if ui.button("解密").clicked() {
                self.process_sm4_action(false);
            }
        });

        ui.label("输出结果 (Hex):");
        ui.text_edit_multiline(&mut self.sm4_state.output);
    }

    fn process_sm4_action(&mut self, encrypt: bool) {
        let key_bytes = match hex::decode(&self.sm4_state.key) {
            Ok(k) if k.len() == 16 => k,
            _ => {
                self.sm4_state.output = "错误: Key 必须是 16 字节 (32 hex characters)".to_string();
                return;
            }
        };

        let iv_bytes = if self.sm4_state.mode_cbc {
            match hex::decode(&self.sm4_state.iv) {
                Ok(v) if v.len() == 16 => Some(v),
                _ => {
                    self.sm4_state.output = "错误: IV 必须是 16 字节 (32 hex characters)".to_string();
                    return;
                }
            }
        } else {
            None
        };

        let data_bytes = match hex::decode(&self.sm4_state.data) {
            Ok(d) => d,
            Err(_) => {
                self.sm4_state.output = "错误: 数据必须是合法的 Hex 字符串".to_string();
                return;
            }
        };

        match self.sm4_crypt(&key_bytes, iv_bytes.as_deref(), &data_bytes, encrypt) {
            Ok(res) => self.sm4_state.output = hex::encode(res),
            Err(e) => self.sm4_state.output = format!("操作失败: {}", e),
        }
    }

    fn sm4_crypt(&self, key: &[u8], iv: Option<&[u8]>, data: &[u8], encrypt: bool) -> Result<Vec<u8>, String> {
        let key_arr = GenericArray::clone_from_slice(key);
        let cipher = Sm4::new(&key_arr);

        if encrypt {
            // Padding (PKCS7)
            let block_size = 16;
            let padding_len = block_size - (data.len() % block_size);
            let mut padded_data = data.to_vec();
            padded_data.extend(std::iter::repeat(padding_len as u8).take(padding_len));

            let mut output = Vec::with_capacity(padded_data.len());
            let blocks = padded_data.chunks_exact(block_size);

            if let Some(mut current_iv) = iv.map(|v| GenericArray::clone_from_slice(v)) {
                // CBC Encrypt
                for block in blocks {
                    let mut block_arr = GenericArray::clone_from_slice(block);
                    // XOR with IV
                    for (b, iv_b) in block_arr.iter_mut().zip(current_iv.iter()) {
                        *b ^= *iv_b;
                    }
                    cipher.encrypt_block(&mut block_arr);
                    output.extend_from_slice(&block_arr);
                    current_iv = block_arr; // Update IV
                }
            } else {
                // ECB Encrypt
                for block in blocks {
                    let mut block_arr = GenericArray::clone_from_slice(block);
                    cipher.encrypt_block(&mut block_arr);
                    output.extend_from_slice(&block_arr);
                }
            }
            Ok(output)
        } else {
            // Decrypt
            if data.len() % 16 != 0 {
                return Err("解密数据长度必须是 16 的倍数".to_string());
            }

            let mut output = Vec::with_capacity(data.len());
            let blocks = data.chunks_exact(16);

            if let Some(initial_iv) = iv.map(|v| GenericArray::clone_from_slice(v)) {
                // CBC Decrypt
                let mut prev_ciphertext = initial_iv;
                for block in blocks {
                    let block_arr = GenericArray::clone_from_slice(block);
                    let mut decrypted_block = block_arr.clone();
                    cipher.decrypt_block(&mut decrypted_block);
                    
                    // XOR with prev ciphertext
                    for (b, iv_b) in decrypted_block.iter_mut().zip(prev_ciphertext.iter()) {
                        *b ^= *iv_b;
                    }
                    output.extend_from_slice(&decrypted_block);
                    prev_ciphertext = block_arr;
                }
            } else {
                // ECB Decrypt
                for block in blocks {
                    let mut block_arr = GenericArray::clone_from_slice(block);
                    cipher.decrypt_block(&mut block_arr);
                    output.extend_from_slice(&block_arr);
                }
            }

            // Unpad (PKCS7)
            if let Some(&pad) = output.last() {
                let pad_len = pad as usize;
                if pad_len > 0 && pad_len <= 16 && output.len() >= pad_len {
                    // Start of padding
                    let pad_start = output.len() - pad_len;
                    // Check if all padding bytes are correct
                    let is_padding_valid = output[pad_start..].iter().all(|&b| b == pad);
                    if is_padding_valid {
                        output.truncate(pad_start);
                        Ok(output)
                    } else {
                        // Keep raw if padding invalid? Or Error?
                        // For a tool, usually we might show raw or error.
                        // Let's return error to be safe.
                        Err("Padding 校验失败 (PKCS7)".to_string())
                    }
                } else {
                    Err("无效的 Padding 长度".to_string())
                }
            } else {
                Ok(output) // empty
            }
        }
    }

    fn show_zuc(&mut self, ui: &mut egui::Ui) {
        ui.heading("ZUC 祖冲之序列密码 (128-bit)");
        ui.separator();

        ui.label("Key (16 bytes, Hex):");
        ui.text_edit_singleline(&mut self.zuc_state.key);

        ui.label("IV (16 bytes, Hex):");
        ui.text_edit_singleline(&mut self.zuc_state.iv);

        ui.label("输入数据 (Hex):");
        ui.text_edit_multiline(&mut self.zuc_state.input);

        ui.horizontal(|ui| {
            if ui.button("加密 / 解密").clicked() {
                self.process_zuc();
            }
        });

        ui.label("输出结果 (Hex):");
        ui.text_edit_multiline(&mut self.zuc_state.output);
    }

    fn process_zuc(&mut self) {
        let key_bytes = match hex::decode(&self.zuc_state.key) {
            Ok(k) if k.len() == 16 => k,
            _ => {
                self.zuc_state.output = "错误: Key 必须是 16 字节 (32 hex characters)".to_string();
                return;
            }
        };

        let iv_bytes = match hex::decode(&self.zuc_state.iv) {
            Ok(v) if v.len() == 16 => v,
            _ => {
                self.zuc_state.output = "错误: IV 必须是 16 字节 (32 hex characters)".to_string();
                return;
            }
        };

        let mut data_bytes = match hex::decode(&self.zuc_state.input) {
            Ok(d) => d,
            Err(_) => {
                self.zuc_state.output = "错误: 数据必须是合法的 Hex 字符串".to_string();
                return;
            }
        };

        // ZUC implementation usage
        let mut key_arr = [0u8; 16];
        key_arr.copy_from_slice(&key_bytes);
        
        let mut iv_arr = [0u8; 16];
        iv_arr.copy_from_slice(&iv_bytes);

        // bitlen is usually bytes * 8 for full byte streams
        let bitlen = data_bytes.len() * 8;

        zuc128_xor_inplace(&key_arr, &iv_arr, &mut data_bytes, bitlen);

        self.zuc_state.output = hex::encode(data_bytes);
    }
}


