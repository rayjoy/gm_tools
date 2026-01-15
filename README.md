# GM Tools — 设计与使用说明

## 概要
- 名称：GM Tools（国密算法工具箱）
- 语言：Rust
- 目标：提供图形化桌面工具以便本地演示与调试国密（SM2/SM3/SM4）相关算法的常见操作（摘要、对称加解密、非对称签名/验签/加密/解密）。

## 功能一览
- SM3：对任意 UTF-8 字符串计算 SM3 摘要，输出 Hex 编码。
- SM4：支持 ECB/CBC/GCM 三种模式的加密/解密。
    - ECB/CBC 模式使用 PKCS#7 填充。
    - GCM 模式支持认证加密，需提供 Nonce 和 AAD。
- SM2：支持密钥生成、签名、验签、加密、解密。对某些第三方库（libsm）短输入存在兼容处理与安全提示。
- ZUC：支持祖冲之序列密码算法。
    - **ZUC-128**：128位密钥，128位初始向量。
    - **ZUC-256**：256位密钥，184位(23字节)初始向量。

## 项目结构
- `Cargo.toml`：依赖与元信息。
- `src/main.rs`：主程序，基于 `eframe/egui` 实现 GUI，包含四大功能模块（SM3/SM4/SM2/ZUC）和 UI 逻辑。
- `src/check_libsm.rs`：用于快速检测 `libsm` 能否成功初始化的最小程序（测试用途）。
- `src/bin/*`：包含两个示例二进制 `test_crash.rs`、`test_decrypt.rs`（作为附加测试/示例）。
- `.vscode/settings.json`：本次为方便开发/IDE 调试创建的工作区配置（rust-analyzer 相关）。

（已提交的改动：添加或更新了 `.vscode/settings.json`，并在仓库中提交了所有未跟踪文件。）

## 设计说明（模块/流程）

1) GUI 层（`src/main.rs`）
- 使用 `eframe::egui` 创建单窗口应用，分为四张 Tab：`SM3` / `SM4` / `SM2` / `ZUC`。
- 每张 Tab 管理独立的状态结构体（`Sm3State`、`Sm4State`、`Sm2State`、`ZucState`），保存输入、输出与模式选择。
- 所有交互（按钮点击）在 UI 层触发对应 `process_*` 方法完成具体计算并把结果写回状态，UI 即时展示结果。

2) SM3 子系统
- 直接使用 `sm3` crate 的 `Sm3::new()`、`update()`、`finalize()` 提供摘要。
- 输入以 UTF-8 字符串读取，输出使用 Hex 编码显示。

3) SM4 子系统
- 使用 `sm4` crate，提供 ECB 与 CBC 模式。
- 加密：对原文做 PKCS#7 填充（块大小 16），按选定模式逐块加密。
- 解密：按模式逐块解密并校验 PKCS#7 填充，若填充无效返回错误信息。

4) SM2 子系统（基于 `libsm`）
- 密钥生成：调用 `libsm::sm2::signature::SigCtx::new()` 与 `new_keypair()`，把私钥/公钥序列化为 Hex。
- 签名/验签：使用 `SigCtx` 的 `sign` / `verify` 接口，签名结果以 DER 格式 Hex 输出/输入。
- 加密/解密：使用 `libsm::sm2::encrypt::EncryptCtx` / `DecryptCtx`。实现中包含对常见不兼容输入格式的自动兼容：
  - 对于加密，若输入小于 32 字节会做 PKCS#7-like 填充以避免 libsm 某版本 panic（并在输出中提示）。
  - 对于解密，先尝试常规 C1C3C2 格式，若失败会尝试把 C1C2C3 格式转换为 C1C3C2 再解密，并做 PKCS#7 去填充尝试。

5) ZUC 子系统
- 使用 `zuc` crate (v0.4.1+)。
- 支持 **ZUC-128** 与 **ZUC-256** 两种模式。
- 作为序列密码，加密与解密运算逻辑相同（异或密钥流）。UI 上提供了独立按钮以便于理解。

## 依赖（关键）
- `eframe` / `egui`：GUI。
- `sm3`、`sm4`、`zuc`：国密算法（摘要/对称/序列）实现。
- `libsm`：SM2（签名/加解密）的实现。
- `libsm` 的具体版本可能影响对短消息或某些格式的处理（见已知问题）。

## 构建与运行
开发机器建议使用 Windows（MSVC）toolchain，已在开发环境使用 `stable-x86_64-pc-windows-msvc` 测试。

在项目根目录执行：
```powershell
cargo run --bin gm_tools --release
```

或调试运行（开发模式）：
```powershell
cargo run --bin gm_tools
```

如果想仅构建二进制：
```powershell
cargo build --release
```

## 变更日志
详见根目录 `CHANGELOG.md`，本次修复与打包记录已更新。

## 发布与打包
- release 二进制位于 `target/release/gm_tools.exe`。
- 可分发包位于 `dist/gm_tools-windows-x86_64.zip`（包含 `gm_tools.exe` 与 `README.md`）。

## 开发注意与已知问题
- rust-analyzer：需要安装 `rust-src` 组件并确保 `rustup` 在 PATH 中。为方便开发，本仓库新增了工作区设置（`.vscode/settings.json`），指定 `rust-analyzer.rustupPath` 并在服务器环境中优先使用 `C:\\Users\\zhang\\.cargo\\bin`。如果你复制仓库到其他机器，请把 `rustupPath` 调整为你的路径或在 Settings 中使用自动发现。
- `libsm` 兼容性：某些版本的 `libsm` 在处理短输入（例如 SM2 加密长度 < 32 字节）时存在 panic 或异常行为。当前实现对加密端做了最小填充以避免崩溃，但这只是工具层面的兼容性处理——生产环境请改用稳定库或在上层保证输入长度与格式。
- 私钥/公钥的导入：当前 UI 实现更依赖于内部生成的键对（`cached_pk` / `cached_sk`），对外部导入公钥对象支持有限（仅接受 Hex 公钥字符串作为 `04||X||Y`），导入时请确保格式正确。

## 性能说明
- SM2 密钥生成：已做一次轻量优化，复用 `SigCtx` 并减少生成过程中的不必要拷贝，以降低每次点击“生成新密钥对”的额外开销。
- 如果你后续希望进一步提升“体感速度”（避免 UI 卡顿），建议将密钥生成放到后台线程并在 UI 上显示“生成中…”。

## 测试
- 可以运行 `cargo run --bin test_crash` / `cargo run --bin test_decrypt` 来执行仓库中提供的示例二进制（用于复现/测试某些 edge-case）。

## 扩展计划（可选）
- 支持从 PEM/DER 格式导入/导出 SM2 密钥。
- 改进异常处理，避免 catch_unwind 作为常用控制流。
- 添加更完善的单元测试与集成测试，覆盖边界条件（短输入、不同密钥格式等）。

## 贡献与许可证
- 欢迎提交 Issue/PR。请在 PR 中说明变更目的并包含可复现步骤。
- 本仓库当前未在 `Cargo.toml` 中强制指定许可证，请在发布前补充合适的 License 字段。

---

附：本次已做主要修改清单（工作区/源码）

- 新增：`.vscode/settings.json` — 指定 `rustupPath`、`rustcSource=discover`、并为 rust-analyzer 服务器进程设置 `PATH` 环境以优先加载 `~/.cargo/bin`。
- 新增：`README.md`（本文件）。
- 已加入并提交到仓库：`.gitignore`, `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/check_libsm.rs`, `src/bin/test_crash.rs`, `src/bin/test_decrypt.rs`。

如果需要我把 README 按你的格式再精简或翻译为英文，告诉我需要的风格（简洁/详细/面向用户/面向开发者）。
