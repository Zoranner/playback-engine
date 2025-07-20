//! 高级使用示例
//!
//! 演示如何使用 pcap-file-io 库的高级功能。

use pcap_file_io::{
    config::PcapConfiguration,
    structures::DataPacket,
    io::{PcapFileReader, PcapFileWriter, MultiPcapReader},
    utils::{calculate_crc32, ByteArrayExtensions, DateTimeExtensions},
    error::Result,
};
use chrono::{DateTime, Utc};

fn main() -> Result<()> {
    println!("=== PcapFile.IO 高级使用示例 ===");

    // 1. 使用高性能配置
    let config = PcapConfiguration::high_performance();
    println!("使用高性能配置");
    config.validate()?;
    println!("配置验证通过");

    // 2. 创建自定义数据包
    let custom_data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
    let packet = DataPacket::from_timestamp(
        1234567890, // 时间戳（秒）
        123456789,  // 时间戳（纳秒）
        custom_data,
    )?;

    // 3. 验证数据包
    if packet.is_valid() {
        println!("数据包验证通过");
    } else {
        println!("数据包验证失败");
    }

    // 4. 使用字节数组扩展方法
    let data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"
    let hex_string = data.to_hex_string(" ");
    println!("十六进制表示: {}", hex_string);

    let utf8_string = data.to_utf8_string()?;
    println!("UTF8字符串: {}", utf8_string);

    let base64_string = data.to_base64_string();
    println!("Base64编码: {}", base64_string);

    // 5. 计算CRC32校验和
    let checksum = calculate_crc32(&data);
    println!("CRC32校验和: 0x{:08X}", checksum);

    // 6. 使用DateTime扩展方法
    let now = Utc::now();
    let milliseconds = now.to_unix_time_milliseconds();
    println!("当前时间戳(毫秒): {}", milliseconds);

    let nanoseconds = now.get_nanoseconds();
    println!("纳秒部分: {}", nanoseconds);

    // 7. 错误处理示例
    println!("\n=== 错误处理示例 ===");

    // 尝试打开不存在的文件
    let mut reader = PcapFileReader::new(config.clone());
    match reader.open("nonexistent.pcap") {
        Ok(_) => println!("意外成功"),
        Err(e) => println!("预期的错误: {}", e),
    }

    // 尝试创建无效的数据包
    match DataPacket::from_timestamp(0, 0, vec![]) {
        Ok(_) => println!("意外成功"),
        Err(e) => println!("预期的错误: {}", e),
    }

    // 尝试使用无效配置
    let mut invalid_config = PcapConfiguration::default();
    invalid_config.max_packet_size = 0;

    match invalid_config.validate() {
        Ok(_) => println!("意外成功"),
        Err(e) => println!("预期的错误: {}", e),
    }

    // 8. 多文件读取示例
    println!("\n=== 多文件读取示例 ===");

    // 创建多个测试文件
    for i in 0..3 {
        let mut writer = PcapFileWriter::new(config.clone());
        let filename = format!("test_file_{}.pcap", i);
        writer.create(&filename)?;

        let data = format!("测试数据包 #{}", i).into_bytes();
        let packet = DataPacket::from_datetime(
            std::time::SystemTime::now(),
            data,
        )?;

        writer.write_packet(&packet)?;
        writer.close();
        println!("创建测试文件: {}", filename);
    }

    // 使用多文件读取器
    let mut multi_reader = MultiPcapReader::new(".", config)?;
    println!("多文件读取器初始化成功");

    let mut total_packets = 0;
    while let Some(packet) = multi_reader.read_next_packet()? {
        println!("从多文件读取器读取数据包: {:?}", packet);
        total_packets += 1;
    }

    println!("总共从多文件读取了 {} 个数据包", total_packets);

    // 9. 清理测试文件
    for i in 0..3 {
        let filename = format!("test_file_{}.pcap", i);
        if std::path::Path::new(&filename).exists() {
            std::fs::remove_file(&filename)?;
            println!("清理测试文件: {}", filename);
        }
    }

    println!("=== 高级示例完成 ===");
    Ok(())
}
