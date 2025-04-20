# 哈希计算器

一个简单的哈希计算工具，支持多种哈希算法。

## 功能

- 支持多种哈希算法：
  - SHA-1
  - SHA-256
  - SHA-384
  - SHA-512
  - SHA3-256
  - SHA3-384
  - SHA3-512
  - SM3
- 支持文本和十六进制输入
- 实时计算哈希值
- 可复制哈希结果

## 使用方法

1. 选择输入类型（文本或十六进制）
2. 选择哈希算法
3. 在输入框中输入需要计算哈希的内容
4. 结果将自动显示
5. 点击"复制结果"按钮可以复制哈希值

## 构建与运行

```bash
# 克隆项目
git clone https://github.com/yourusername/hash_calculator.git
cd hash_calculator

# 编译并运行
cargo run

# 构建发布版本
cargo build --release
```

## 依赖

- eframe - GUI 框架
- sha1, sha2, sha3 - SHA 系列哈希算法
- libsm - 中国 SM3 哈希算法
- hex - 十六进制转换
