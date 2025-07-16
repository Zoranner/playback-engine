#[cfg(test)]
mod tests {
    use playback_engine_lib::{NetworkType, NetworkConfig};

    #[test]
    fn test_simple_network_type_serialization() {
        let unicast = NetworkType::Unicast;
        let multicast = NetworkType::Multicast;
        let broadcast = NetworkType::Broadcast;

        // 测试枚举序列化
        let unicast_xml = serde_xml_rs::to_string(&unicast).unwrap();
        println!("Unicast XML: {}", unicast_xml);

        let multicast_xml = serde_xml_rs::to_string(&multicast).unwrap();
        println!("Multicast XML: {}", multicast_xml);

        let broadcast_xml = serde_xml_rs::to_string(&broadcast).unwrap();
        println!("Broadcast XML: {}", broadcast_xml);

        // 测试反序列化
        let deserialized_unicast: NetworkType = serde_xml_rs::from_str(&unicast_xml).unwrap();
        assert_eq!(deserialized_unicast, NetworkType::Unicast);
    }

    #[test]
    fn test_simple_network_config() {
        let config = NetworkConfig {
            network_type: NetworkType::Unicast,
            ip_address: "192.168.1.1".to_string(),
            port: 8080,
            interface: None,
            enabled: true,
        };

        let xml = serde_xml_rs::to_string(&config).unwrap();
        println!("NetworkConfig XML: {}", xml);

        let deserialized: NetworkConfig = serde_xml_rs::from_str(&xml).unwrap();
        assert_eq!(deserialized.network_type, config.network_type);
        assert_eq!(deserialized.ip_address, config.ip_address);
        assert_eq!(deserialized.port, config.port);
    }
}
