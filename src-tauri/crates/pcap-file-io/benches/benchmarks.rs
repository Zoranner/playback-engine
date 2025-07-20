//! 性能基准测试
//!
//! 测试 pcap-file-io 库的性能表现。

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pcap_file_io::{
    config::PcapConfiguration,
    structures::DataPacket,
    io::{PcapFileReader, PcapFileWriter},
    utils::calculate_crc32,
    error::Result,
};
use std::time::SystemTime;
use tempfile::NamedTempFile;

fn bench_write_packets(c: &mut Criterion) {
    let mut group = c.benchmark_group("write_packets");

    group.bench_function("write_100_packets", |b| {
        b.iter(|| {
            let config = PcapConfiguration::high_performance();
            let temp_file = NamedTempFile::new().unwrap();
            let file_path = temp_file.path();

            let mut writer = PcapFileWriter::new(config);
            writer.create(file_path).unwrap();

            for i in 0..100 {
                let data = format!("Benchmark packet #{}", i).into_bytes();
                let packet = DataPacket::from_datetime(SystemTime::now(), data).unwrap();
                writer.write_packet(&packet).unwrap();
            }

            writer.close();
        });
    });

    group.bench_function("write_1000_packets", |b| {
        b.iter(|| {
            let config = PcapConfiguration::high_performance();
            let temp_file = NamedTempFile::new().unwrap();
            let file_path = temp_file.path();

            let mut writer = PcapFileWriter::new(config);
            writer.create(file_path).unwrap();

            for i in 0..1000 {
                let data = format!("Benchmark packet #{}", i).into_bytes();
                let packet = DataPacket::from_datetime(SystemTime::now(), data).unwrap();
                writer.write_packet(&packet).unwrap();
            }

            writer.close();
        });
    });

    group.finish();
}

fn bench_read_packets(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_packets");

    // 准备测试文件
    let config = PcapConfiguration::high_performance();
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    {
        let mut writer = PcapFileWriter::new(config.clone());
        writer.create(file_path).unwrap();

        for i in 0..1000 {
            let data = format!("Benchmark packet #{}", i).into_bytes();
            let packet = DataPacket::from_datetime(SystemTime::now(), data).unwrap();
            writer.write_packet(&packet).unwrap();
        }

        writer.close();
    }

    group.bench_function("read_1000_packets", |b| {
        b.iter(|| {
            let mut reader = PcapFileReader::new(config.clone());
            reader.open(file_path).unwrap();

            let mut packet_count = 0;
            while let Some(packet) = reader.read_packet().unwrap() {
                black_box(packet);
                packet_count += 1;
            }

            assert_eq!(packet_count, 1000);
        });
    });

    group.finish();
}

fn bench_crc32_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("crc32_calculation");

    let data_small = b"Hello, World!";
    let data_medium = vec![0u8; 1024];
    let data_large = vec![0u8; 1024 * 1024]; // 1MB

    group.bench_function("small_data", |b| {
        b.iter(|| {
            black_box(calculate_crc32(data_small));
        });
    });

    group.bench_function("medium_data", |b| {
        b.iter(|| {
            black_box(calculate_crc32(&data_medium));
        });
    });

    group.bench_function("large_data", |b| {
        b.iter(|| {
            black_box(calculate_crc32(&data_large));
        });
    });

    group.finish();
}

fn bench_packet_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("packet_creation");

    let data = vec![0u8; 1024];

    group.bench_function("from_datetime", |b| {
        b.iter(|| {
            black_box(DataPacket::from_datetime(SystemTime::now(), data.clone()).unwrap());
        });
    });

    group.bench_function("from_timestamp", |b| {
        b.iter(|| {
            black_box(DataPacket::from_timestamp(1234567890, 123456789, data.clone()).unwrap());
        });
    });

    group.finish();
}

fn bench_configuration_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("configuration_validation");

    let config = PcapConfiguration::default();

    group.bench_function("validate_default", |b| {
        b.iter(|| {
            black_box(config.validate().unwrap());
        });
    });

    group.bench_function("validate_high_performance", |b| {
        b.iter(|| {
            black_box(PcapConfiguration::high_performance().validate().unwrap());
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_write_packets,
    bench_read_packets,
    bench_crc32_calculation,
    bench_packet_creation,
    bench_configuration_validation
);
criterion_main!(benches);
