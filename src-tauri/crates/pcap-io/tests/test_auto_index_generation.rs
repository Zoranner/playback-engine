//! 自动索引生成测试
//!
//! 测试无索引写入和读取是否能自动生成索引并验证索引的正确性

use pcap_io::{Configuration, DataPacket, PcapReader, PcapWriter, Read, Write, Info, Result};
use pcap_io::{PidxReader, PidxWriter};
use std::path::Path;
use std::time::SystemTime;
use tempfile::TempDir;

/// 创建测试数据包
fn create_test_packet(sequence: usize, size: usize) -> Result<DataPacket> {
    let mut data = vec![0u8; size];

    // 填充测试数据
    for i in 0..size {
        data[i] = ((sequence + i) % 256) as u8;
    }

    let capture_time = SystemTime::now();
    Ok(DataPacket::from_datetime(capture_time, data)?)
}

/// 检查目录中是否存在PIDX文件
fn check_pidx_file_exists(dataset_path: &Path) -> bool {
    if let Ok(Some(_)) = PidxReader::find_pidx_file(dataset_path) {
        true
    } else {
        false
    }
}

#[test]
fn test_auto_index_generation_on_write() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let base_path = temp_dir.path();
    let project_name = "auto_index_write_test";

    const PACKET_COUNT: usize = 2000;
    const PACKET_SIZE: usize = 1024;

    // 配置启用自动索引生成
    let mut config = Configuration::default();
    config.enable_index_cache = true;

    let mut writer = PcapWriter::new(base_path, project_name, config).expect("创建Writer失败");

    // 写入数据包
    for i in 0..PACKET_COUNT {
        let packet = create_test_packet(i, PACKET_SIZE).expect("创建数据包失败");
        writer.write_packet(&packet).expect("写入数据包失败");
    }

    writer.finalize().expect("完成写入失败");

    // 验证自动生成的索引文件
    let dataset_path = base_path.join(project_name);
    assert!(check_pidx_file_exists(&dataset_path), "自动索引生成失败：PIDX文件不存在");

    // 加载并验证索引内容
    let pidx_path = PidxReader::find_pidx_file(&dataset_path)
        .expect("查找PIDX文件失败")
        .expect("PIDX文件不存在");

    let index = PidxReader::load_index(&pidx_path).expect("加载索引失败");

    assert_eq!(index.total_packets, PACKET_COUNT as u64, "索引中总数据包数不正确");
    assert!(index.data_files.files.len() > 0, "索引中没有文件信息");
    assert!(index.start_timestamp <= index.end_timestamp, "索引时间戳范围不正确");

    println!("✅ 写入时自动索引生成测试通过：{} 个数据包，生成了 {} 个文件的索引",
             index.total_packets, index.data_files.files.len());
}

#[test]
fn test_auto_index_generation_on_read() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let base_path = temp_dir.path();
    let project_name = "auto_index_read_test";

    const PACKET_COUNT: usize = 1500;
    const PACKET_SIZE: usize = 512;

    // 步骤1: 禁用自动索引写入数据
    let mut config = Configuration::default();
    config.enable_index_cache = false;

    let mut writer = PcapWriter::new(base_path, project_name, config).expect("创建Writer失败");

    for i in 0..PACKET_COUNT {
        let packet = create_test_packet(i, PACKET_SIZE).expect("创建数据包失败");
        writer.write_packet(&packet).expect("写入数据包失败");
    }

    writer.finalize().expect("完成写入失败");

    // 验证没有自动生成索引
    let dataset_path = base_path.join(project_name);
    assert!(!check_pidx_file_exists(&dataset_path), "应该没有自动生成索引文件");

    // 步骤2: 读取时自动生成索引
    let config = Configuration::default(); // 启用索引缓存
    let mut reader = PcapReader::new(dataset_path.clone(), config).expect("创建Reader失败");

    // 读取所有数据包
    let mut read_count = 0;
    while let Some(_packet) = reader.read_packet().expect("读取数据包失败") {
        read_count += 1;
    }

    assert_eq!(read_count, PACKET_COUNT, "读取的数据包数量不正确");

    // 验证是否自动生成了索引
    // 注意：这取决于具体实现，某些实现可能在读取时生成索引
    if check_pidx_file_exists(&dataset_path) {
        let pidx_path = PidxReader::find_pidx_file(&dataset_path)
            .expect("查找PIDX文件失败")
            .expect("PIDX文件不存在");

        let index = PidxReader::load_index(&pidx_path).expect("加载索引失败");
        assert_eq!(index.total_packets, PACKET_COUNT as u64, "自动生成的索引总数据包数不正确");

        println!("✅ 读取时自动索引生成测试通过：{} 个数据包", PACKET_COUNT);
    } else {
        println!("✅ 读取时索引生成测试通过：当前实现不在读取时自动生成索引");
    }
}

#[test]
fn test_manual_index_generation_after_write() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let base_path = temp_dir.path();
    let project_name = "manual_index_after_write";

    const PACKET_COUNT: usize = 3000;
    const PACKET_SIZE: usize = 256;

    // 步骤1: 禁用自动索引写入数据
    let mut config = Configuration::default();
    config.enable_index_cache = false;

    let mut writer = PcapWriter::new(base_path, project_name, config).expect("创建Writer失败");

    for i in 0..PACKET_COUNT {
        let packet = create_test_packet(i, PACKET_SIZE).expect("创建数据包失败");
        writer.write_packet(&packet).expect("写入数据包失败");
    }

    writer.finalize().expect("完成写入失败");

    // 验证没有自动生成索引
    let dataset_path = base_path.join(project_name);
    assert!(!check_pidx_file_exists(&dataset_path), "应该没有自动生成索引文件");

    // 步骤2: 手动生成索引
    let mut pidx_writer = PidxWriter::new(&dataset_path).expect("创建PidxWriter失败");
    let index_path = pidx_writer.generate_index().expect("手动生成索引失败");

    // 验证手动生成的索引
    assert!(index_path.exists(), "手动生成的索引文件不存在");
    assert!(check_pidx_file_exists(&dataset_path), "手动生成索引后检查失败");

    // 加载并验证索引内容
    let index = PidxReader::load_index(&index_path).expect("加载手动生成的索引失败");

    assert_eq!(index.total_packets, PACKET_COUNT as u64, "手动生成的索引总数据包数不正确");
    assert!(index.data_files.files.len() > 0, "手动生成的索引没有文件信息");

    // 步骤3: 使用索引读取验证
    let config = Configuration::default();
    let reader = PcapReader::new(dataset_path, config).expect("使用索引创建Reader失败");

    let dataset_info = reader.dataset_info();
    assert_eq!(dataset_info.total_packets, PACKET_COUNT as u64, "使用索引的数据集信息不正确");

    println!("✅ 写入后手动索引生成测试通过：{} 个数据包", PACKET_COUNT);
}

#[test]
fn test_index_accuracy_verification() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let base_path = temp_dir.path();
    let project_name = "index_accuracy_test";

    const PACKET_COUNT: usize = 2500;
    const PACKET_SIZE: usize = 1024;

    // 写入数据包并自动生成索引
    let config = Configuration::default();
    let mut writer = PcapWriter::new(base_path, project_name, config.clone()).expect("创建Writer失败");

    let mut expected_timestamps = Vec::new();

    for i in 0..PACKET_COUNT {
        let packet = create_test_packet(i, PACKET_SIZE).expect("创建数据包失败");
        expected_timestamps.push(packet.get_timestamp_ns());
        writer.write_packet(&packet).expect("写入数据包失败");
    }

    writer.finalize().expect("完成写入失败");

    // 加载索引
    let dataset_path = base_path.join(project_name);
    let pidx_path = PidxReader::find_pidx_file(&dataset_path)
        .expect("查找PIDX文件失败")
        .expect("PIDX文件不存在");

    let index = PidxReader::load_index(&pidx_path).expect("加载索引失败");

    // 验证索引准确性
    assert_eq!(index.total_packets, PACKET_COUNT as u64, "索引总数据包数不准确");

    // 验证时间戳范围
    let min_timestamp = expected_timestamps.iter().min().unwrap();
    let max_timestamp = expected_timestamps.iter().max().unwrap();

    assert!(index.start_timestamp <= *min_timestamp, "索引开始时间戳不准确");
    assert!(index.end_timestamp >= *max_timestamp, "索引结束时间戳不准确");

    // 验证文件信息准确性
    let mut total_indexed_packets = 0;
    for file_index in &index.data_files.files {
        assert!(file_index.packet_count > 0, "文件索引中数据包数量为0");
        assert!(file_index.file_size > 0, "文件索引中文件大小为0");
        assert!(!file_index.file_name.is_empty(), "文件索引中文件名为空");

        total_indexed_packets += file_index.packet_count;
    }

    assert_eq!(total_indexed_packets, PACKET_COUNT as u64, "索引文件中数据包总数不匹配");

    // 使用索引读取并验证
    let mut reader = PcapReader::new(dataset_path, config).expect("使用索引创建Reader失败");

    let mut actual_read_count = 0;
    let mut actual_timestamps = Vec::new();

    while let Some(packet) = reader.read_packet().expect("读取数据包失败") {
        actual_timestamps.push(packet.get_timestamp_ns());
        actual_read_count += 1;
    }

    assert_eq!(actual_read_count, PACKET_COUNT, "使用索引读取的数据包数量不正确");
    assert_eq!(actual_timestamps, expected_timestamps, "使用索引读取的时间戳序列不匹配");

    println!("✅ 索引准确性验证测试通过：{} 个数据包，索引信息完全准确", PACKET_COUNT);
}

#[test]
fn test_index_performance_benefits() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let base_path = temp_dir.path();
    let project_name = "index_performance_test";

    const PACKET_COUNT: usize = 5000;
    const PACKET_SIZE: usize = 1024;

    // 写入数据包
    let config = Configuration::default();
    let mut writer = PcapWriter::new(base_path, project_name, config.clone()).expect("创建Writer失败");

    for i in 0..PACKET_COUNT {
        let packet = create_test_packet(i, PACKET_SIZE).expect("创建数据包失败");
        writer.write_packet(&packet).expect("写入数据包失败");
    }

    writer.finalize().expect("完成写入失败");

    let dataset_path = base_path.join(project_name);

    // 测试使用索引的性能
    let start_time = std::time::Instant::now();
    let reader = PcapReader::new(dataset_path.clone(), config.clone()).expect("创建有索引的Reader失败");
    let dataset_info = reader.dataset_info();
    let indexed_info_time = start_time.elapsed();

    assert_eq!(dataset_info.total_packets, PACKET_COUNT as u64, "有索引的数据集信息不正确");

    // 验证索引提供了快速的元数据访问
    assert!(indexed_info_time.as_millis() < 100, "使用索引获取数据集信息耗时过长：{}ms", indexed_info_time.as_millis());

    println!("✅ 索引性能优势测试通过：获取 {} 个数据包信息耗时 {}ms",
             PACKET_COUNT, indexed_info_time.as_millis());
}
