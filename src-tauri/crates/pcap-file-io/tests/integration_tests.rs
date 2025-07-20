//! 集成测试
//!
//! 测试 pcap-file-io 库的完整功能。

use pcap_file_io::{
    config::PcapConfiguration,
    structures::{DataPacket, PcapFileHeader},
    io::{PcapFileReader, PcapFileWriter},
    utils::{calculate_crc32, ByteArrayExtensions},
    error::Result,
};
use std::time::SystemTime;
use tempfile::NamedTempFile;

#[test]
fn test_full_write_read_cycle() -> Result<()> {
    let config = PcapConfiguration::default();
    let temp_file = NamedTempFile::new()?;
    let file_path = temp_file.path();

    // 写入测试
    {
        let mut writer = PcapFileWriter::new(config.clone());
        writer.create(file_path)?;

        let test_data = vec![
            b"First packet".to_vec(),
            b"Second packet".to_vec(),
            b"Third packet".to_vec(),
        ];

        for (i, data) in test_data.iter().enumerate() {
            let packet = DataPacket::from_datetime(SystemTime::now(), data.clone())?;
            writer.write_packet(&packet)?;
            println!("写入数据包 #{}", i);
        }

        writer.close();
    }

    // 读取测试
    {
        let mut reader = PcapFileReader::new(config);
        reader.open(file_path)?;

        let mut packet_count = 0;
        while let Some(packet) = reader.read_packet()? {
            println!("读取数据包 #{}: {:?}", packet_count, packet);
            assert!(packet.is_valid());
            packet_count += 1;
        }

        assert_eq!(packet_count, 3);
    }

    Ok(())
}

#[test]
fn test_large_file_handling() -> Result<()> {
    let config = PcapConfiguration::high_performance();
    let temp_file = NamedTempFile::new()?;
    let file_path = temp_file.path();

    // 写入大量数据包
    {
        let mut writer = PcapFileWriter::new(config.clone());
        writer.create(file_path)?;

        for i in 0..1000 {
            let data = format!("Large file packet #{} with some additional data to make it larger", i).into_bytes();
            let packet = DataPacket::from_datetime(SystemTime::now(), data)?;
            writer.write_packet(&packet)?;
        }

        writer.close();
    }

    // 读取并验证
    {
        let mut reader = PcapFileReader::new(config);
        reader.open(file_path)?;

        let mut packet_count = 0;
        while let Some(packet) = reader.read_packet()? {
            assert!(packet.is_valid());
            packet_count += 1;
        }

        assert_eq!(packet_count, 1000);
    }

    Ok(())
}

#[test]
fn test_error_handling() {
    let config = PcapConfiguration::default();
    let mut reader = PcapFileReader::new(config);

    // 测试打开不存在的文件
    let result = reader.open("nonexistent_file.pcap");
    assert!(result.is_err());

    // 测试创建无效数据包
    let result = DataPacket::from_timestamp(0, 0, vec![]);
    assert!(result.is_err());
}

#[test]
fn test_byte_operations() {
    let data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"

    // 测试十六进制转换
    let hex = data.to_hex_string(" ");
    assert_eq!(hex, "48 65 6C 6C 6F");

    // 测试UTF8转换
    let utf8 = data.to_utf8_string();
    assert!(utf8.is_ok());
    assert_eq!(utf8.unwrap(), "Hello");

    // 测试Base64转换
    let base64 = data.to_base64_string();
    assert!(!base64.is_empty());

    // 测试子数组
    let sub = data.sub_array(1, 3);
    assert!(sub.is_ok());
    assert_eq!(sub.unwrap(), vec![0x65, 0x6C, 0x6C]);
}

#[test]
fn test_crc32_calculation() {
    let data1 = b"Hello, World!";
    let data2 = b"Hello, World!";
    let data3 = b"Different data";

    let checksum1 = calculate_crc32(data1);
    let checksum2 = calculate_crc32(data2);
    let checksum3 = calculate_crc32(data3);

    // 相同数据应该产生相同的校验和
    assert_eq!(checksum1, checksum2);

    // 不同数据应该产生不同的校验和
    assert_ne!(checksum1, checksum3);

    // 校验和不应该为0
    assert_ne!(checksum1, 0);
}

#[test]
fn test_file_header_operations() {
    let header = PcapFileHeader::new(0);
    assert!(header.is_valid());

    let bytes = header.to_bytes();
    assert_eq!(bytes.len(), PcapFileHeader::HEADER_SIZE);

    let header2 = PcapFileHeader::from_bytes(&bytes);
    assert!(header2.is_ok());
    assert!(header2.unwrap().is_valid());
}

#[test]
fn test_configuration_validation() {
    let config = PcapConfiguration::default();
    assert!(config.validate().is_ok());

    let high_perf = PcapConfiguration::high_performance();
    assert!(high_perf.validate().is_ok());

    let low_mem = PcapConfiguration::low_memory();
    assert!(low_mem.validate().is_ok());

    // 测试无效配置
    let mut invalid_config = PcapConfiguration::default();
    invalid_config.max_packet_size = 0;
    assert!(invalid_config.validate().is_err());
}
