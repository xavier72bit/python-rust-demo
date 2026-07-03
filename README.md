# Python Rust Demo

这里我们讨论的是[Using Rust from Python](https://pyo3.rs/v0.29.0/rust-from-python.html).

* Python语法简洁，开发难度低，但是运行速度慢
* Rust语法复杂，开发难度高，但是运行速度快

将这二者结合，当Python遇到性能瓶颈时，让Rust接管负责。

# 概述

## Rust

在Rust侧，使用[pyo3](https://github.com/PyO3/pyo3)编写Python软件包/模块，然后使用[maturin](https://github.com/PyO3/maturin)工具进行打包。

## Python

从Python模块的维度来说，打包后的Python库，应该包含两种文件：
* `*.pyd`: 一个二进制文件，是rust编译链接后的产物，类比于Windows里的`.dll`、Linux里的`.so`、macOS里的`.dylib`。
* `*.pyi`: 一个文本文件，被称为Python存根文件，用于对外部实现的方法、模块进行Python层面的定义，并提供类型标注与注释。

在Python侧，根据pyi文件，直接调用Rust二进制库文件。

# 目录结构

```bash
.
├── README.md
├── example_py_module  # 一个rust实现的python模块
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── example_py_module.pyi
│   ├── pyproject.toml
│   ├── src  # rust 源码
│   └── target
└── main.py
```

# 初始化

1. 创建并激活一个Python虚拟环境
```shell
virtualenv .venv
source .venv/bin/activate
```

2. 安装maturin工具
```shell
pip install maturin # or `pip install --upgrade maturin`
```

3. 初始化maturin模块
```shell
mkdir example_py_module && cd example_py_module
maturin init # 随后会出现一个交互式命令，选择pyo3
```

# 开发流程

1. 写Rust代码，开发功能

2. 运行maturin构建命令
```shell
maturin develop
```

3. 生成Python Stub
```shell
maturin generate-stubs --out .
```

4. 使用python脚本进行验证
```shell
python main.py
```

# 打包发布流程

TODO
