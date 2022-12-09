# 简介

# Cargo
是 Rust 的构建系统和包管理器，除了创建工程以外还具备构建（build）工程、运行（run）工程等一系列功能
## 常见指令
```shell
# 创建新项目
$ cargo new your-project

# 代码格式化
cargo fmt

# 代码优化
cargo clippy

# 查看第三方库的版本和依赖关系
cargo tree

# 检查项目中未使用的依赖
cargo udeps

# 编译 , 发布构建--release
$ cargo build

# 运行：编译 + 执行
$ cargo run 

# 检查代码，比 cargo
```

cargo build/run --release  使用 release 编译会比默认的 debug 编译性能提升 10 倍以上，但是 release 缺点是编译速度较慢，而且不会显示 panic backtrace 的具体行号