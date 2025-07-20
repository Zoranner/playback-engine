//! 基本使用示例
//!
//! 演示如何使用 pcap-file-io 库进行基本的PCAP文件读写操作。

use pcap_file_io::{
    config::PcapConfiguration,
    structures::DataPacket,
    io::{PcapFileReader, PcapFileWriter},
    error::Result,
};

fn main() -> Result<()> {
    println!("=== PcapFile.IO 基本使用示例 ===");

    // 1. 创建配置
    let config = PcapConfiguration::default();
    println!("使用默认配置: {:?}", config);

    // 2. 创建PCAP文件写入器
    let mut writer = PcapFileWriter::new(config.clone());
    writer.create("example.pcap")?;
    println!("创建PCAP文件: example.pcap");

    // 3. 写入一些示例数据包
    for i in 0..5 {
        let data = format!("示例数据包 #{}", i).into_bytes();
        let packet = DataPacket::from_datetime(
            std::time::SystemTime::now(),
            data,
        )?;

        let offset = writer.write_packet(&packet)?;
        println!("写入数据包 #{} 到偏移量 {}", i, offset);
    }

    // 4. 关闭写入器
    writer.close();
    println!("关闭写入器");

    // 5. 创建PCAP文件读取器
    let mut reader = PcapFileReader::new(config);
    reader.open("example.pcap")?;
    println!("打开PCAP文件进行读取");

    // 6. 读取所有数据包
    let mut packet_count = 0;
    while let Some(packet) = reader.read_packet()? {
        println!("读取数据包 #{}: {:?}", packet_count, packet);
        packet_count += 1;
    }

    println!("总共读取了 {} 个数据包", packet_count);

    // 7. 清理
    std::fs::remove_file("example.pcap")?;
    println!("清理临时文件");

    println!("=== 示例完成 ===");
    Ok(())
}
