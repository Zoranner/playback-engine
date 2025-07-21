//! 索引功能测试
//!
//! 测试有索引写入索引内容的正确性，验证PIDX索引系统

use pcap_io::{Configuration, DataPacket, Info, Read, Reader, Result, Write, Writer};
use pcap_io::{PidxIndex, PidxReader, PidxWriter};
use std::path::Path;
use std::time::SystemTime;
use tempfile::TempDir;

/// 创建测试数据包
fn create_test_packet(sequence: usize, size: usize) -> Result<DataPacket> {
    let mut data = vec![0u8; size];

    // 填充可预测的测试数据
    for i in 0..size {
        data[i] = ((sequence + i) % 256) as u8;
    }

    let capture_time = SystemTime::now();
    Ok(DataPacket::from_datetime(capture_time, data)?)
}

#[test]
fn test_index_generation_and_loading() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let base_path = temp_dir.path();
    let project_name = "index_test_project";

    const PACKET_COUNT: usize = 5000;
    const PACKET_SIZE: usize = 1024;

    // 步骤1: 写入数据包
    let mut config = Configuration::default();
    config.enable_index_cache = true; // 启用索引缓存

    let mut writer = Writer::new(base_path, project_name, config.clone()).expect("创建Writer失败");

    let mut written_timestamps = Vec::new();

    for i in 0..PACKET_COUNT {
        let packet = create_test_packet(i, PACKET_SIZE).expect("创建数据包失败");
        written_timestamps.push(packet.get_timestamp_ns());
        writer.write_packet(&packet).expect("写入数据包失败");
    }

    writer.finalize().expect("完成写入失败");

    // 步骤2: 验证自动生成的索引
    let dataset_path = base_path.join(project_name);

    // 查找PIDX文件
    let pidx_path = PidxReader::find_pidx_file(&dataset_path)
        .expect("查找PIDX文件失败")
        .expect("PIDX文件不存在");

    // 加载索引
    let index = PidxReader::load_index(&pidx_path).expect("加载索引失败");

    // 验证索引内容
    assert_eq!(
        index.total_packets, PACKET_COUNT as u64,
        "索引中的总数据包数不正确"
    );
    assert!(index.data_files.files.len() > 0, "索引中没有文件信息");

    // 验证时间戳范围
    assert!(
        index.start_timestamp <= index.end_timestamp,
        "时间戳范围不正确"
    );

    // 验证文件索引完整性
    let mut total_packets_in_index = 0;
    for file_index in &index.data_files.files {
        assert!(file_index.packet_count > 0, "文件索引中数据包数量为0");
        total_packets_in_index += file_index.packet_count;
    }
    assert_eq!(
        total_packets_in_index, PACKET_COUNT as u64,
        "索引中文件的数据包总数不匹配"
    );

    println!(
        "✅ 索引生成和加载测试通过：{} 个数据包，{} 个文件",
        index.total_packets,
        index.data_files.files.len()
    );
}

#[test]
fn test_manual_index_generation() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let base_path = temp_dir.path();
    let project_name = "manual_index_test";

    const PACKET_COUNT: usize = 3000;
    const PACKET_SIZE: usize = 512;

    // 步骤1: 写入数据包（禁用自动索引）
    let mut config = Configuration::default();
    config.enable_index_cache = false;

    let mut writer = Writer::new(base_path, project_name, config).expect("创建Writer失败");

    for i in 0..PACKET_COUNT {
        let packet = create_test_packet(i, PACKET_SIZE).expect("创建数据包失败");
        writer.write_packet(&packet).expect("写入数据包失败");
    }

    writer.finalize().expect("完成写入失败");

    // 步骤2: 手动生成索引
    let dataset_path = base_path.join(project_name);
    let mut pidx_writer = PidxWriter::new(&dataset_path).expect("创建PidxWriter失败");
    let index_path = pidx_writer.generate_index().expect("生成索引失败");

    // 步骤3: 验证手动生成的索引
    let index = PidxReader::load_index(&index_path).expect("加载手动生成的索引失败");

    assert_eq!(
        index.total_packets, PACKET_COUNT as u64,
        "手动生成的索引总数据包数不正确"
    );
    assert!(
        index.data_files.files.len() > 0,
        "手动生成的索引没有文件信息"
    );

    // 验证索引与实际数据的一致性
    let config = Configuration::default();
    let reader = Reader::new(dataset_path, config).expect("创建Reader失败");
    let dataset_info = reader.dataset_info();

    assert_eq!(
        index.total_packets, dataset_info.total_packets,
        "索引与数据集信息不匹配"
    );

    println!("✅ 手动索引生成测试通过：{} 个数据包", index.total_packets);
}

#[test]
fn test_index_content_verification() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let base_path = temp_dir.path();
    let project_name = "index_content_test";

    const PACKET_COUNT: usize = 2000;
    const PACKET_SIZE: usize = 256;

    // 创建具有已知时间戳的数据包
    let config = Configuration::default();
    let mut writer = Writer::new(base_path, project_name, config.clone()).expect("创建Writer失败");

    let mut expected_timestamps = Vec::new();

    for i in 0..PACKET_COUNT {
        let packet = create_test_packet(i, PACKET_SIZE).expect("创建数据包失败");
        expected_timestamps.push(packet.get_timestamp_ns());
        writer.write_packet(&packet).expect("写入数据包失败");
    }

    writer.finalize().expect("完成写入失败");

    // 读取并验证索引内容
    let dataset_path = base_path.join(project_name);
    let pidx_path = PidxReader::find_pidx_file(&dataset_path)
        .expect("查找PIDX文件失败")
        .expect("PIDX文件不存在");

    let index = PidxReader::load_index(&pidx_path).expect("加载索引失败");

    // 验证索引中的时间戳范围
    let min_expected = expected_timestamps.iter().min().unwrap();
    let max_expected = expected_timestamps.iter().max().unwrap();

    assert!(
        index.start_timestamp <= *min_expected,
        "索引开始时间戳不正确"
    );
    assert!(index.end_timestamp >= *max_expected, "索引结束时间戳不正确");

    // 验证每个文件的索引信息
    for file_index in &index.data_files.files {
        assert!(!file_index.file_name.is_empty(), "文件名为空");
        assert!(file_index.packet_count > 0, "文件数据包数量为0");
        assert!(file_index.file_size > 0, "文件大小为0");
        assert!(
            file_index.start_timestamp <= file_index.end_timestamp,
            "文件时间戳范围不正确"
        );
    }

    println!(
        "✅ 索引内容验证测试通过：验证了 {} 个文件的索引信息",
        index.data_files.files.len()
    );
}

#[test]
fn test_index_query_functionality() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let base_path = temp_dir.path();
    let project_name = "index_query_test";

    const PACKET_COUNT: usize = 1500;
    const PACKET_SIZE: usize = 512;

    // 写入数据包
    let config = Configuration::default();
    let mut writer = Writer::new(base_path, project_name, config.clone()).expect("创建Writer失败");

    for i in 0..PACKET_COUNT {
        let packet = create_test_packet(i, PACKET_SIZE).expect("创建数据包失败");
        writer.write_packet(&packet).expect("写入数据包失败");
    }

    writer.finalize().expect("完成写入失败");

    // 加载索引
    let dataset_path = base_path.join(project_name);
    let pidx_path = PidxReader::find_pidx_file(&dataset_path)
        .expect("查找PIDX文件失败")
        .expect("PIDX文件不存在");

    let index = PidxReader::load_index(&pidx_path).expect("加载索引失败");

    // 测试索引查询功能（如果提供的话）
    assert_eq!(
        index.total_packets, PACKET_COUNT as u64,
        "索引查询：总数据包数不正确"
    );

    // 验证文件信息查询
    for (i, file_index) in index.data_files.files.iter().enumerate() {
        assert!(!file_index.file_name.is_empty(), "文件 {} 名称为空", i);
        assert!(file_index.packet_count > 0, "文件 {} 数据包数量为0", i);
    }

    println!(
        "✅ 索引查询功能测试通过：查询了 {} 个文件的信息",
        index.data_files.files.len()
    );
}
