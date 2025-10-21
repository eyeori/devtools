# DevTools

DevTools 是一个基于 Rust 编写的MCP工具，旨在为开发者提供一些常用的MCP工具。

## 功能特性

DevTools 提供了多种常用地开发工具，包括：

### 编码/解码工具

- MD5 哈希计算
- Base64 编码/解码
- URL 编码/解码
- Hex 到字符串转换

### 时间处理工具

- 获取当前日期时间及时间戳
- 时间戳转日期时间
- 日期时间转时间戳

### 环境配置工具

- 列出所有本地配置键名

## 安装

确保已安装 Rust 和 Cargo，然后克隆项目并构建：

```bash
git clone <repository-url>
cd devtools
cargo build --release
```

构建后的二进制文件位于 `target/release/devtools`。

## 使用方法

### 命令行使用

DevTools 可以作为命令行工具使用：

```bash
# 基本使用
./devtools

# 指定配置文件
./devtools -c config.yaml

# 指定日志配置文件
./devtools -l log.yaml

# 指定日志目标
./devtools --logger my_logger
```

### 工具调用

DevTools 通过标准MCP协议调用具体工具

可用工具列表：

- `encode::md5_encode` - MD5 哈希计算
- `encode::base64_encode` - Base64 编码
- `encode::base64_decode` - Base64 解码
- `encode::url_encode` - URL 编码
- `encode::url_decode` - URL 解码
- `encode::hex_to_string` - Hex 到字符串转换
- `time::datetime_now` - 获取当前日期时间及时间戳
- `time::timestamp_to_datetime` - 时间戳转日期时间
- `time::datetime_to_timestamp` - 日期时间转时间戳
- `env::list_config_keys` - 列出所有本地配置键名

## 配置

DevTools 支持通过 YAML 文件进行配置。默认配置文件为 `config.yaml`，日志配置文件为 `log.yaml`。

## 技术栈

- Rust 2024 edition
- [clap](https://crates.io/crates/clap) - 命令行参数解析
- [tokio](https://crates.io/crates/tokio) - 异步运行时
- [serde](https://crates.io/crates/serde) - 数据序列化
- [log](https://crates.io/crates/log) + [log4rs](https://crates.io/crates/log4rs) - 日志系统
- [base64](https://crates.io/crates/base64) - Base64 编码/解码
- [chrono](https://crates.io/crates/chrono) - 时间处理
- [md-5](https://crates.io/crates/md-5) - MD5 哈希计算
- [rmcp](https://crates.io/crates/rmcp) - 远程控制协议

## 许可证

本项目采用 [MIT 许可证](LICENSE)。
