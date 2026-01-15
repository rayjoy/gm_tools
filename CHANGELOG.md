# 变更日志

本文记录本项目的重要变更与开发环境调整。

## 2026-01-15
- 修复：Windows release 版本隐藏冗余控制台窗口（GUI 程序不再弹出额外窗口）。
- 修复：SM2 加密短输入（<32 字节）按 PKCS#7 规则最小补齐，避免 `libsm` 崩溃。
- 修复：SM2 解密自动兼容 `C1C2C3` 密文格式，必要时转换为 `C1C3C2` 后重试。
- 新增：示例二进制 `test_decrypt` 用于验证解密与填充逻辑。
- 打包：已更新 release 二进制与 `dist/gm_tools-windows-x86_64.zip`。

## 2026-01-14
- 初始工作区同步与项目文件
  - 新增：`Cargo.toml`、`Cargo.lock` 及 `src/` 下源文件。
  - 新增示例二进制：`src/bin/test_crash.rs`、`src/bin/test_decrypt.rs`。

- 开发环境修复
  - 为活动 toolchain (`stable-x86_64-pc-windows-msvc`) 安装了 `rust-src` 组件。
  - 永久将 `C:\Users\zhang\.cargo\bin` 加入用户 `PATH`。
  - 通过添加 VS Code 工作区配置修复 `rust-analyzer` 报错：
    - `.vscode/settings.json` 中设置 `rust-analyzer.rustupPath` 与 `rust-analyzer.rustcSource=discover`。
    - 在 `rust-analyzer` 服务器环境中优先使用 `C:\Users\zhang\.cargo\bin`（通过 `rust-analyzer.server.extraEnv.PATH` 设置）。

- 文档
  - 新增 `README.md`（包含设计说明、使用与构建说明、已知问题与开发注意事项）。
  - 新增本 `CHANGELOG.md`。

- 性能优化
  - SM2：复用 `SigCtx` 并减少密钥生成时的拷贝与分配，提升“生成新密钥对”的速度。

说明：
- SM2 功能依赖 `libsm` 库；某些版本在处理短输入时可能存在兼容性问题（例如加密输入 <32 字节可能导致 panic）。代码中包含临时兼容性处理（填充、格式转换），详细说明见 `README.md`。
