#[cfg(test)]
mod tests {
    use playback_engine_lib::{PprojConfig, DatasetConfig, NetworkConfig, NetworkType};
    use playback_engine_lib::{PidxIndex, PacketIndexEntry, PcapFileIndex};

    #[test]
    fn test_pproj_xml_serialization() {
        // 创建测试数据
        let network_config = NetworkConfig::multicast("239.255.255.250", 5000);
        let dataset_config = DatasetConfig::new("test_dataset".to_string(), "/path/to/data")
            .with_network_config(network_config)
            .with_description("测试数据集".to_string())
            .add_tag("test".to_string());

        let pproj_config = PprojConfig::new("test_project".to_string())
            .with_description("测试工程".to_string())
            .with_author("测试作者".to_string())
            .add_dataset(dataset_config)
            .add_tag("test".to_string());

        // 序列化为XML
        let xml_result = serde_xml_rs::to_string(&pproj_config);
        assert!(xml_result.is_ok(), "XML序列化失败: {:?}", xml_result.err());

        let xml_string = xml_result.unwrap();
        println!("PPROJ XML序列化结果:");
        println!("{}", xml_string);

        // 反序列化
        let deserialized_result: Result<PprojConfig, _> = serde_xml_rs::from_str(&xml_string);
        assert!(deserialized_result.is_ok(), "XML反序列化失败: {:?}", deserialized_result.err());

        let deserialized_config = deserialized_result.unwrap();
        assert_eq!(deserialized_config.project_name, pproj_config.project_name);
        assert_eq!(deserialized_config.datasets.len(), 1);
        assert_eq!(deserialized_config.datasets[0].name, "test_dataset");
    }

    #[test]
    fn test_pidx_xml_serialization() {
        // 创建测试数据包索引条目
        let packet_entry = PacketIndexEntry {
            timestamp_ns: 1000000000,
            file_name: "test.pcap".to_string(),
            byte_offset: 16,
            packet_size: 64,
        };

        let file_index = PcapFileIndex {
            file_name: "test.pcap".to_string(),
            file_hash: "abc123".to_string(),
            file_size: 1024,
            packet_count: 1,
            start_timestamp: 1000000000,
            end_timestamp: 1000000100,
            packets: vec![packet_entry],
        };

        let pidx_index = PidxIndex::new("test_dataset".to_string(), "/path/to/dataset".to_string());
        let mut pidx_index = pidx_index;
        pidx_index.files.push(file_index);
        pidx_index.total_packets = 1;
        pidx_index.total_duration = 100;
        pidx_index.start_timestamp = 1000000000;
        pidx_index.end_timestamp = 1000000100;

        // 序列化为XML
        let xml_result = serde_xml_rs::to_string(&pidx_index);
        assert!(xml_result.is_ok(), "XML序列化失败: {:?}", xml_result.err());

        let xml_string = xml_result.unwrap();
        println!("PIDX XML序列化结果:");
        println!("{}", xml_string);

        // 反序列化
        let deserialized_result: Result<PidxIndex, _> = serde_xml_rs::from_str(&xml_string);
        assert!(deserialized_result.is_ok(), "XML反序列化失败: {:?}", deserialized_result.err());

        let deserialized_index = deserialized_result.unwrap();
        assert_eq!(deserialized_index.dataset_name, pidx_index.dataset_name);
        assert_eq!(deserialized_index.files.len(), 1);
        assert_eq!(deserialized_index.files[0].file_name, "test.pcap");
    }

    #[test]
    fn test_network_types_serialization() {
        let unicast = NetworkConfig::unicast("192.168.1.100", 8080);
        let multicast = NetworkConfig::multicast("239.1.1.1", 9090);
        let broadcast = NetworkConfig::broadcast(7070);

        // 测试各种网络类型的序列化
        for config in &[unicast, multicast, broadcast] {
            let xml_result = serde_xml_rs::to_string(config);
            assert!(xml_result.is_ok(), "网络配置序列化失败: {:?}", xml_result.err());

            let xml_string = xml_result.unwrap();
            println!("网络配置 {:?} XML:", config.network_type);
            println!("{}", xml_string);

            // 反序列化测试
            let deserialized: Result<NetworkConfig, _> = serde_xml_rs::from_str(&xml_string);
            assert!(deserialized.is_ok(), "网络配置反序列化失败: {:?}", deserialized.err());

            let deserialized_config = deserialized.unwrap();
            assert_eq!(deserialized_config.network_type, config.network_type);
            assert_eq!(deserialized_config.ip_address, config.ip_address);
            assert_eq!(deserialized_config.port, config.port);
        }
    }
}
