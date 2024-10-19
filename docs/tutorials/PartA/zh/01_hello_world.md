---
title: 1. 你好，世界
parent: Part A. Rust for Python
nav_order: 1
---

# Hello World

相信聪明的各位已经知道怎么写第一个function了，但是为了水字数，还是写一下吧。

`src/lib.rs` 中添加

```rust
#[pyfunction]
fn hello_world() {
    println!("Hello from Rust!");
}
```

目前我们先都在lib.rs下面写，等到后续讲述modules的时候，我们会把函数写到modules中。

在lib.rs中添加函数，就类似于在一个python库的根目录下的`__init__.py`中添加函数。

```python
from pyo3_tutorial import hello_world

hello_world()
# Hello from Rust!
```

