# PCAP-IO 集成测试套件

本目录包含了PCAP-IO库的完整集成测试套件，按功能模块分类，可以系统性地验证库的各项功能。

## 📋 测试概览

### 测试统计
- **测试文件**: 5个
- **测试用例**: 19个  
- **覆盖范围**: 小规模数据、大规模数据、索引功能、数据一致性、自动索引生成

## 🧪 测试模块详情

### 1. 小规模数据集测试 (`test_small_dataset.rs`)
**目标**: 验证基本的读写功能和多文件处理

**测试用例**:
- `test_small_dataset_basic_functionality` - 基本功能测试 (2000个数据包)
- `test_small_dataset_multiple_files` - 多文件分割测试 (350个数据包 → 4个文件)
- `test_small_dataset_empty_and_edge_cases` - 空数据集和边界情况测试

**验证点**:
- ✅ 基本读写功能
- ✅ 文件自动分割
- ✅ 空数据集处理
- ✅ 边界情况处理

### 2. 大规模数据集测试 (`test_large_dataset.rs`)
**目标**: 验证大数据量下的性能和稳定性

**测试用例**:
- `test_large_dataset_basic_functionality` - 大规模基本功能 (10万个数据包)
- `test_large_dataset_memory_usage` - 内存使用测试 (5万个数据包)
- `test_large_dataset_file_segmentation` - 大规模文件分割 (2.5万个数据包)

**性能基准**:
- ✅ 写入吞吐量: > 1,000 包/秒 (实际: ~3,839 包/秒)
- ✅ 读取吞吐量: > 5,000 包/秒 (实际: ~15,574 包/秒)
- ✅ 内存使用: 低内存模式正常工作
- ✅ 文件分割: 自动按配置分割

### 3. 索引功能测试 (`test_index_functionality.rs`)
**目标**: 验证PIDX索引系统的生成、读取和正确性

**测试用例**:
- `test_index_generation_and_loading` - 索引生成和加载测试
- `test_manual_index_generation` - 手动索引生成测试
- `test_index_content_verification` - 索引内容验证测试
- `test_index_query_functionality` - 索引查询功能测试

**验证点**:
- ✅ 自动索引生成
- ✅ 手动索引生成
- ✅ 索引内容正确性
- ✅ 时间戳范围准确
- ✅ 文件信息完整

### 4. 数据一致性测试 (`test_data_consistency.rs`)
**目标**: 确保写入和读取的数据完全一致

**测试用例**:
- `test_basic_data_consistency` - 基本数据一致性 (1000个数据包)
- `test_large_packet_consistency` - 大数据包一致性 (100个64KB数据包)
- `test_mixed_size_packet_consistency` - 混合大小数据包一致性
- `test_timestamp_consistency` - 时间戳一致性测试

**验证维度**:
- ✅ 数据包数量
- ✅ 数据内容哈希
- ✅ 时间戳精确度
- ✅ 校验和匹配
- ✅ 数据包长度
- ✅ 首/尾字节验证

### 5. 自动索引生成测试 (`test_auto_index_generation.rs`)
**目标**: 验证无索引情况下的自动索引生成和功能

**测试用例**:
- `test_auto_index_generation_on_write` - 写入时自动索引生成
- `test_auto_index_generation_on_read` - 读取时自动索引生成
- `test_manual_index_generation_after_write` - 写入后手动索引生成
- `test_index_accuracy_verification` - 索引准确性验证
- `test_index_performance_benefits` - 索引性能优势测试

**验证点**:
- ✅ 写入时自动生成索引
- ✅ 后期手动生成索引
- ✅ 索引准确性验证
- ✅ 性能优势验证
- ✅ 快速元数据访问 (< 100ms)

## 🚀 运行测试

### 运行所有测试
```bash
cargo test --tests
```

### 运行特定测试模块
```bash
# 小规模数据集测试
cargo test --test test_small_dataset

# 大规模数据集测试 (较耗时，约35秒)
cargo test --test test_large_dataset

# 索引功能测试
cargo test --test test_index_functionality

# 数据一致性测试
cargo test --test test_data_consistency

# 自动索引生成测试
cargo test --test test_auto_index_generation
```

### 运行特定测试用例
```bash
# 运行基本功能测试
cargo test test_small_dataset_basic_functionality

# 运行大规模测试并显示输出
cargo test test_large_dataset_basic_functionality -- --nocapture
```

## 📊 测试结果示例

```
测试结果摘要:
==================
✅ 小规模数据集测试: 3/3 通过
✅ 大规模数据集测试: 3/3 通过  
✅ 索引功能测试: 4/4 通过
✅ 数据一致性测试: 4/4 通过
✅ 自动索引生成测试: 5/5 通过

总计: 19/19 测试通过 (100%)

性能指标:
- 写入性能: ~3,839 包/秒 (10万包测试)
- 读取性能: ~15,574 包/秒 (10万包测试)
- 索引查询: < 100ms (5000包数据集)
- 数据一致性: 100% (全部测试通过)
```

## 🔧 测试依赖

测试需要以下依赖项 (已配置在 `Cargo.toml` 中):
```toml
[dev-dependencies]
tempfile = "3.8"  # 用于创建临时测试目录
```

## 📝 测试说明

1. **临时文件管理**: 所有测试使用 `tempfile::TempDir` 创建临时目录，测试完成后自动清理

2. **性能基准**: 大规模测试包含性能基准验证，确保库性能符合预期

3. **数据完整性**: 通过多维度验证确保数据的完整性和一致性

4. **边界情况**: 包含空数据集、大数据包等边界情况的测试

5. **并发安全**: 测试设计避免了并发执行时的冲突

## 🚨 注意事项

- 大规模数据集测试 (`test_large_dataset`) 需要较长时间 (~35秒)
- 测试会创建临时文件，需要足够的磁盘空间
- 某些测试会验证性能基准，在低性能环境下可能需要调整阈值 
