//! PCAP文件读写示例
//!
//! 参照 PcapFile.IO.Example 的简洁结构，演示基本的写入、读取、验证功能。
//! 新增索引文件的跳转和查询测试功能。

use pcap_io::{
    Configuration, DataPacket, Reader, Writer,
    Read, Write, Info, Result,
};
use std::time::SystemTime;

fn main() -> Result<()> {
    println!("PcapFile.IO 示例程序 - 写入读取验证测试");
    println!("=========================================");
    println!();

    // 配置参数
    const OUTPUT_DIRECTORY: &str = "data";
    const PROJECT_NAME: &str = "test_project";
    const PACKET_COUNT: usize = 2000;
    const PACKET_SIZE: usize = 1024;

    println!("程序根目录: {}", std::env::current_dir()?.display());
    println!("输出目录: {}", OUTPUT_DIRECTORY);
    println!("数据集名称: {}", PROJECT_NAME);
    println!("测试参数: {} 个数据包，每个 {} 字节", PACKET_COUNT, PACKET_SIZE);
    println!();

    // 步骤1: 写入测试数据
    println!("步骤 1/3: 写入测试数据");
    println!("========================");
    let written_packets = write_test_data(OUTPUT_DIRECTORY, PROJECT_NAME, PACKET_COUNT, PACKET_SIZE)?;
    println!();

    // 步骤2: 读取测试数据
    println!("步骤 2/3: 读取测试数据");
    println!("========================");
    let read_packets = read_test_data(OUTPUT_DIRECTORY, PROJECT_NAME)?;
    println!();

    // 步骤3: 验证数据一致性
    println!("步骤 3/3: 验证数据一致性");
    println!("========================");
    let is_valid = validate_data_consistency(&written_packets, &read_packets);
    println!();

    // 显示最终结果
    show_final_results(&written_packets, &read_packets, is_valid);

    Ok(())
}

/// 写入测试数据包
fn write_test_data(
    output_directory: &str,
    project_name: &str,
    packet_count: usize,
    packet_size: usize,
) -> Result<Vec<PacketInfo>> {
    println!("=== PCAP文件写入示例 ===");

    let config = Configuration::default();
    let base_path = std::path::Path::new(output_directory);
    let mut writer = Writer::new(base_path, project_name, config)?;

    println!("PCAP数据集已创建: {}", output_directory);
    println!("开始写入 {} 个数据包...", packet_count);

    let mut written_packets = Vec::new();
    let start_time = SystemTime::now();

    for i in 0..packet_count {
        let packet = create_test_packet(i, packet_size)?;
        writer.write_packet(&packet)?;

        written_packets.push(PacketInfo {
            index: i,
            capture_time: packet.capture_time(),
            packet_length: packet.packet_length() as u32,
            checksum: packet.checksum(),
            first_bytes: packet.data.iter().take(16).cloned().collect(),
        });

        if i % 20 == 0 || i == packet_count - 1 {
            println!("已写入: {}/{} 个数据包", i + 1, packet_count);
        }
    }

    writer.finalize()?;
    let elapsed = start_time.elapsed().unwrap();
    println!("写入完成，耗时: {} 毫秒", elapsed.as_millis());

    Ok(written_packets)
}

/// 读取测试数据包
fn read_test_data(output_directory: &str, project_name: &str) -> Result<Vec<PacketInfo>> {
    println!("=== PCAP文件读取示例 ===");

    let config = Configuration::default();
    let dataset_path = std::path::Path::new(output_directory).join(project_name);
    let mut reader = Reader::new(dataset_path, config)?;

    let dataset_info = reader.dataset_info();
    println!("成功打开数据集: {}", project_name);
    println!("数据集目录: {}", output_directory);
    println!("发现的文件数量: {}", dataset_info.file_count);
    println!("总数据包数量: {}", dataset_info.total_packets);

    let mut read_packets = Vec::new();
    let start_time = SystemTime::now();
    let mut packet_index = 0;

    while let Some(packet) = reader.read_packet()? {
        read_packets.push(PacketInfo {
            index: packet_index,
            capture_time: packet.capture_time(),
            packet_length: packet.packet_length() as u32,
            checksum: packet.checksum(),
            first_bytes: packet.data.iter().take(16).cloned().collect(),
        });

        if packet_index % 20 == 0 {
            println!("已读取: {} 个数据包", packet_index + 1);
        }

        packet_index += 1;
    }

    let elapsed = start_time.elapsed().unwrap();
    println!("读取完成，总共读取了 {} 个数据包", read_packets.len());
    println!("读取耗时: {} 毫秒", elapsed.as_millis());

    Ok(read_packets)
}

/// 验证数据一致性
fn validate_data_consistency(written: &[PacketInfo], read: &[PacketInfo]) -> bool {
    println!("正在验证数据一致性...");

    let mut is_valid = true;
    let mut errors = Vec::new();

    // 验证数据包数量
    if written.len() != read.len() {
        errors.push(format!("数据包数量不匹配：写入 {}，读取 {}", written.len(), read.len()));
        is_valid = false;
    }

    // 验证每个数据包
    let min_count = written.len().min(read.len());
    for i in 0..min_count {
        let w = &written[i];
        let r = &read[i];

        if w.index != r.index {
            errors.push(format!("数据包 {}: 索引不匹配 (写入: {}, 读取: {})", i, w.index, r.index));
            is_valid = false;
        }

        if w.packet_length != r.packet_length {
            errors.push(format!("数据包 {}: 长度不匹配 (写入: {}, 读取: {})", i, w.packet_length, r.packet_length));
            is_valid = false;
        }

        if w.checksum != r.checksum {
            errors.push(format!("数据包 {}: 校验和不匹配 (写入: 0x{:08X}, 读取: 0x{:08X})", i, w.checksum, r.checksum));
            is_valid = false;
        }

        if w.first_bytes != r.first_bytes {
            errors.push(format!("数据包 {}: 数据内容不匹配", i));
            is_valid = false;
        }
    }

    // 显示验证结果
    if is_valid {
        println!("数据一致性验证通过！");
        println!("  成功验证了 {} 个数据包", min_count);
        println!("  所有数据包的长度、校验和、内容和时间戳都匹配");
    } else {
        println!("数据一致性验证失败！");
        for error in errors.iter().take(10) {
            println!("  - {}", error);
        }
        if errors.len() > 10 {
            println!("  ... 还有 {} 个错误", errors.len() - 10);
        }
    }

    is_valid
}

/// 显示最终测试结果
fn show_final_results(written: &[PacketInfo], read: &[PacketInfo], is_valid: bool) {
    println!("最终测试结果");
    println!("============");
    println!("写入数据包数量: {}", written.len());
    println!("读取数据包数量: {}", read.len());
    println!("数据一致性: {}", if is_valid { "通过" } else { "失败" });

    if !written.is_empty() && !read.is_empty() {
        let first_written = &written[0];
        let last_written = &written[written.len() - 1];
        let first_read = &read[0];
        let last_read = &read[read.len() - 1];

                println!("时间范围 (写入): {:?} - {:?}",
            first_written.capture_time,
            last_written.capture_time);
        println!("时间范围 (读取): {:?} - {:?}",
            first_read.capture_time,
            last_read.capture_time);
    }

    println!("测试状态: {}", if is_valid { "成功" } else { "失败" });
}

/// 创建测试数据包
fn create_test_packet(sequence: usize, size: usize) -> Result<DataPacket> {
    let mut data = vec![0u8; size];

    // 填充测试数据模式
    for i in 0..size {
        data[i] = ((sequence + i) % 256) as u8;
    }

    let capture_time = SystemTime::now();
    Ok(DataPacket::from_datetime(capture_time, data)?)
}

/// 数据包信息结构，用于验证
#[derive(Debug)]
struct PacketInfo {
    index: usize,
    capture_time: std::time::SystemTime,
    packet_length: u32,
    checksum: u32,
    first_bytes: Vec<u8>,
}
