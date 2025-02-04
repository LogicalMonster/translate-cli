# Translate CLI

## 概述

Translate CLI 是一款命令行工具，允许用户使用 Google Cloud Translation API 翻译文本。用户可以通过标准输入输入文本，工具会返回翻译后的目标语言文本。

## 特性

- 支持多种目标语言（默认：简体中文 `zh-CN`）。
- 从标准输入读取文本。
- 使用 Google Cloud Translation API 获取翻译。
- 从配置文件加载 API 密钥。
- 支持 `.env` 文件加载环境变量。

## 前提条件

- Rust（包含 `cargo` 包管理器）
- Google Cloud API 密钥（具有 Translation API 访问权限）
- `.env` 文件或配置文件，用于指定 API 密钥

## 安装

1. 克隆代码库：
   ```sh
   git clone https://github.com/LogicalMonster/translate-cli.git
   cd translate-cli
   ```

2. 安装依赖：
   ```sh
   cargo build --release
   ```

3. （可选）将二进制文件移动到 `$PATH` 中的目录：
   ```sh
   mv target/release/translate-cli /usr/local/bin/
   ```

## 配置

### API 密钥设置

你需要提供一个 Google Cloud API 密钥，可以通过以下两种方式之一进行配置：

1. **使用配置文件**  
   创建一个配置文件（默认路径：`~/.translate_cli_config`），内容如下：
   ```toml
   google_cloud_api_key = "your-api-key-here"
   ```
   你也可以通过设置 `TRANSLATE_CLI_CONFIG` 环境变量来指定自定义配置文件路径，见 `.env` 文件。

2. **使用 `.env` 文件或直接配置环境变量**  
   在项目根目录创建一个 `.env` 文件，内容如下：
   ```sh
   GOOGLE_CLOUD_API_KEY=your-api-key-here
   ```

## 使用

1. 使用命令行执行程序，并传入可选的语言代码：
   ```sh
   echo "Hello, world!" | translate-cli --target=es
   ```
   输出：
   ```sh
   "Hola, mundo!"
   ```

2. 如果没有传入目标语言，默认翻译为简体中文（`zh-CN`）：
   ```sh
   echo "Hello, world!" | translate-cli
   ```
   输出：
   ```sh
   "你好，世界！"
   ```

## 命令行参数

| 参数            | 描述                                  | 默认值    |
|-----------------|--------------------------------------|----------|
| `-t, --target`  | 目标语言代码（例如：`es`，`ja`）     | `zh-CN`  |

## 依赖项

- `reqwest`（HTTP 客户端）
- `serde` 和 `serde_json`（JSON 解析）
- `tokio`（异步运行时）
- `config`（配置管理）
- `clap`（命令行参数解析）
- `dotenv`（环境变量管理）
- `shellexpand`（路径中扩展 `~`）

## 许可证

本项目使用 GPL-3.0 许可证。

## 贡献

欢迎提交问题或 Pull Request！

## 作者

- [GitHub](https://github.com/LogicalMonster)
