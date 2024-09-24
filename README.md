# Python Rust Demo

* Python语法简洁，开发难度低，但是运行速度慢
* Rust语法复杂，开发难度高，但是运行速度快

我们可以将这二者结合，让Python在顶层调用，当Python遇到性能瓶颈时，Rust专门负责编写吃性能的模块。

# 概述

* 在Rust侧，使用[pyo3](https://github.com/PyO3/pyo3)编写Python软件包/模块，然后使用[maturin](https://github.com/PyO3/maturin)打包。
    * 从Python模块的维度来说，打包后的Python库，应该包含两种文件：
        * `*.pyd`: 一个二进制文件，是rust编译链接后的产物，类比于Windows里的`.dll`、Linux里的`.so`、macOS里的`.dylib`。
        * `*.pyi`: 一个文本文件，被称为Python存根文件，用于对外部实现的方法、模块进行Python层面的定义，并提供类型标注与注释。
* 在Python侧，根据pyi文件，直接调用Rust二进制库文件。

# 目录结构

# 开发环境搭建

这里是一个maturin项目的开发示例，如果你想从零开始构建一个maturin项目，可以按照如下步骤：

1. 创建并激活一个Python虚拟环境
```shell
virtualenv .venv
source .venv/bin/activate
```

2. 安装maturin
```shell
pip install maturin
```

3. 初始化maturin项目
```shell
maturin init
# 随后会出现一个交互式命令，选择pyo3即可
```

4. 创建python存根文件与测试脚本
```shell
# 这里的YOUR_PY_MODULE_NAME与pyproject.toml中的project.name是一致的
touch YOUR_PY_MODULE_NAME.pyi
touch main.py
```

# 日常开发流程

1. 在src目录下写Rust代码，开发功能

2. 修订存根文件

3. 运行maturin构建命令
```shell
maturin develop
```

4. 在python脚本，用来测试构建好的Python库

# 打包发布流程

TODO