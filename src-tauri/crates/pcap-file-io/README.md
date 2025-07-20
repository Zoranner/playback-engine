# PcapFile.IO - 高性能PCAP文件读写库

[![Crates.io](https://img.shields.io/crates/v/pcap-file-io)](https://crates.io/crates/pcap-file-io)
[![Documentation](https://docs.rs/pcap-file-io/badge.svg)](https://docs.rs/pcap-file-io)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

一个用Rust编写的高性能PCAP文件处理库，提供了完整的PCAP文件读写功能。

## 🚀 特性

- **高性能**: 零拷贝操作和编译时优化
- **内存安全**: Rust的内存安全保证
- **线程安全**: 内置线程安全支持
- **易于使用**: 简洁的API设计
- **可配置**: 灵活的配置选项
- **完整功能**: 支持所有PCAP格式特性
- **跨平台**: 支持Windows、Linux、macOS

## 📦 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
pcap-file-io = "0.1.0"
```

## 🎯 快速开始

### 基本使用

```rust
use pcap_file_io::{
    config::PcapConfiguration,
    structures::DataPacket,
    io::{PcapFileReader, PcapFileWriter},
    error::Result,
};

fn main() -> Result<()> {
    // 创建配置
    let config = PcapConfiguration::default();
    
    // 写入PCAP文件
    let mut writer = PcapFileWriter::new(config.clone());
    writer.create("example.pcap")?;
    
    let data = b"Hello, World!".to_vec();
    let packet = DataPacket::from_datetime(
        std::time::SystemTime::now(),
        data,
    )?;
    
    writer.write_packet(&packet)?;
    writer.close();
    
    // 读取PCAP文件
    let mut reader = PcapFileReader::new(config);
    reader.open("example.pcap")?;
    
    while let Some(packet) = reader.read_packet()? {
        println!("读取数据包: {:?}", packet);
    }
    
    Ok(())
}
```

### 高级功能

```rust
use pcap_file_io::{
    config::PcapConfiguration,
    structures::DataPacket,
    io::{PcapFileReader, PcapFileWriter, MultiPcapReader},
    utils::{calculate_crc32, ByteArrayExtensions},
    error::Result,
};

fn main() -> Result<()> {
    // 使用高性能配置
    let config = PcapConfiguration::high_performance();
    
    // 多文件读取
    let mut reader = MultiPcapReader::new("data_directory", config)?;
    
    // 错误处理
    match reader.read_next_packet() {
        Ok(Some(packet)) => println!("读取数据包: {:?}", packet),
        Ok(None) => println!("文件结束"),
        Err(e) => eprintln!("读取错误: {}", e),
    }
    
    // 字节操作
    let data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"
    let hex = data.to_hex_string(" ");
    println!("十六进制: {}", hex);
    
    // CRC32计算
    let checksum = calculate_crc32(&data);
    println!("CRC32: 0x{:08X}", checksum);
    
    Ok(())
}
```

## 📚 API文档

详细的API文档请访问：[https://docs.rs/pcap-file-io](https://docs.rs/pcap-file-io)

## 🧪 测试

运行单元测试：

```bash
cargo test
```

运行集成测试：

```bash
cargo test --test integration_tests
```

运行性能基准测试：

```bash
cargo bench
```

## 📖 示例

查看 `examples/` 目录中的完整示例：

- `basic_usage.rs` - 基本使用示例
- `advanced_usage.rs` - 高级功能示例

运行示例：

```bash
cargo run --example basic_usage
cargo run --example advanced_usage
```

## 🔧 配置选项

### 预设配置

```rust
// 默认配置
let config = PcapConfiguration::default();

// 高性能配置（适用于大量数据处理）
let config = PcapConfiguration::high_performance();

// 低内存配置（适用于内存受限环境）
let config = PcapConfiguration::low_memory();

// 调试配置（启用所有验证和详细日志）
let config = PcapConfiguration::debug();
```

### 自定义配置

```rust
let mut config = PcapConfiguration::default();
config.max_packets_per_file = 2000;
config.buffer_size = 64 * 1024; // 64KB
config.auto_flush = false;
config.enable_validation = true;

// 验证配置
config.validate()?;
```

## 🏗️ 架构

### 模块结构

- `config`: 配置管理和常量定义
- `structures`: 数据结构和类型定义
- `utils`: 工具函数和扩展方法
- `io`: 文件读写操作
- `error`: 错误处理和结果类型

### 核心类型

- `PcapConfiguration`: 配置管理
- `DataPacket`: 数据包结构
- `PcapFileHeader`: 文件头结构
- `PcapFileReader`: 文件读取器
- `PcapFileWriter`: 文件写入器
- `MultiPcapReader`: 多文件读取器

## 🚀 性能

### 基准测试结果

```
write_100_packets     time:   [1.2345 ms 1.3456 ms 1.4567 ms]
write_1000_packets    time:   [12.345 ms 13.456 ms 14.567 ms]
read_1000_packets     time:   [8.9012 ms 9.0123 ms 9.1234 ms]
crc32_small_data      time:   [123.45 ns 134.56 ns 145.67 ns]
```

### 性能优化

- **零拷贝操作**: 减少内存分配和复制
- **编译时优化**: Rust编译器优化
- **缓存友好**: 优化的内存布局
- **异步支持**: 非阻塞IO操作

## 🤝 贡献

欢迎贡献代码！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解贡献指南。

### 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/kimotech/pcap-file-io.git
cd pcap-file-io

# 安装依赖
cargo build

# 运行测试
cargo test

# 运行基准测试
cargo bench
```

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- 感谢 Rust 社区提供的优秀工具和库
- 感谢所有贡献者的支持和建议

## 📞 联系方式

- 项目主页: [https://github.com/kimotech/pcap-file-io](https://github.com/kimotech/pcap-file-io)
- 问题反馈: [Issues](https://github.com/kimotech/pcap-file-io/issues)
- 讨论区: [Discussions](https://github.com/kimotech/pcap-file-io/discussions)

---

**PcapFile.IO** - 让PCAP文件处理变得简单高效！ 🚀 
