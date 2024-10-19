---
title: 环境配置
parent: Part 101
nav_order: 1
---

# 环境安装

os: macos, M3, 14.1.1

python虚拟环境: conda

当前使用的pyo3版本：0.22.5

因为pyo3还在早期版本，可能随时有大更新，尽量做到定期更新

## 在此之前

如果使用mac环境，要修改`~/.cargo/config.toml`，添加

```toml
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]

[target.aarch64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```

## maturin

构建和发布带有 pyo3、cffi 和 uniffi 绑定的 crates，以及将 Rust 二进制文件作为 Python 包发布

```bash
brew install maturin
```

使用conda，安装失败，而且maturin只是一个命令行工具，所以使用了homebrew安装

## 构建环境

```bash
mkdir <target_dir>
cd <target_dir>
maturin init
```

这步会直接生成Github Actions的CI文件，`.github/workflows/CI.yml`，当把代码push到github后，会自动执行CI，生成轮子

调用如下命令行就可以直接安装轮子

```bash
pip install git+https://github.com/Leo-AO-99/pyo3-tutorial@main
```

完成后

```file
target_dir/
│
├── src/
│   └── lib.rs
├── Cargo.toml
└── pyproject.toml
```

lib.rs 内容如下

```rust
use pyo3::prelude::*;

#[pymodule]
fn pyo3_tutorial(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
```

具体解释会在后续章节中讲解，但是可以很容易看出sum_as_string的作用，将两个数字相加，然后返回字符串

接着执行，以及每次修改后都要执行该命令，会把编译后的库安装在当前的python环境中，后续不再赘述

```bash
maturin develop
```

(当然，cargo会生成二进制文件，各位可以尝试自己编写安装脚本)

测试下效果

```bash
$ python                     
Python 3.10.15 (main, Oct  3 2024, 02:24:49) [Clang 14.0.6 ] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> import pyo3_tutorial
>>> pyo3_tutorial.sum_as_string(1,2)
'3'
```

好了，你已经学会pyo3，接下来去成为Rust编程大师吧