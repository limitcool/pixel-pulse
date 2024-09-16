# PixelPulse

PixelPulse 是一个简单的 Rust 应用程序，用于随机提供 WebP 格式的图片。

PixelPulse is a simple Rust application that serves random WebP images.

## 功能 / Features

- 随机提供 WebP 格式的图片
- Docker 支持，易于部署
- 使用 Axum 框架构建的高性能 web 服务

- Serves random WebP images
- Docker support for easy deployment
- High-performance web service built with the Axum framework

## 快速开始 / Quick Start

### 使用 Docker Compose / Using Docker Compose

1. 克隆仓库 / Clone the repository:
   ```bash
   git clone https://github.com/limitcool/pixel-pulse.git
   cd pixel-pulse
   ```

2. 构建并启动容器 / Build and start the container:
   ```bash
   docker-compose up -d
   ```

3. 访问服务 / Access the service:
   ```bash
   http://localhost:8544/random-image
   ```

### 手动运行 / Manual Run

1. 确保已安装 Rust / Ensure Rust is installed:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. 克隆仓库 / Clone the repository:
   ```bash
   git clone https://github.com/limitcool/pixel-pulse.git
   cd pixel-pulse
   ```

3. 构建并运行项目 / Build and run the project:
   ```bash
   cargo run --release
   ```

4. 访问服务 / Access the service:
   ```bash
   http://localhost:8544/random-image
   ```

## 配置 / Configuration

- 图片目录 / Image directory: `/app/images/webp`
- 端口 / Port: 8544

您可以通过修改 `src/main.rs` 文件来更改这些设置。
You can change these settings by modifying the `src/main.rs` file.

## 贡献 / Contributing

欢迎提交 Pull Requests 来改进这个项目。对于重大更改，请先开 issue 讨论您想要改变的内容。

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## 许可证 / License

[MIT](https://choosealicense.com/licenses/mit/)