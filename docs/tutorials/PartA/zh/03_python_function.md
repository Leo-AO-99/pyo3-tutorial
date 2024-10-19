---
title: 3. Python 函数
parent: Part A. Rust for Python
nav_order: 3
---

# Python 函数

（本章代码，如无特殊标注，均在 `src/signature.rs` 中）

**本章内容主要为如何定义一个python可以调用的函数，以及如何处理函数调用时发生的错误**

使用 `#[pyfunction]` 装饰的函数，可以被python调用。

函数签名就是依照编写的rust的函数签名，但是如果想更高自由度的定义函数签名，可以使用 `#[pyo3]`来进行自由定义

## #[pyo3]

### 函数名指定 name = "..."

```rust
// src/lib.rs
#[pyfunction]
#[pyo3(name = "sum_to_string")]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
```

那么在python调用时只能调用到 `sum_to_string` 函数，而不能调用到 `sum_as_string` 函数。

### 函数签名 signature = ()

绝大部分类型的变量都是直接写在函数签名中即可， 但是如果需要python的None， 则需要使用 `Option<T>` 来表示，以及要使用 `**` 的时候

#### 1. **kwds

```rust
#[pyfunction]
#[pyo3(signature = (**kwds))]
fn kwds_args(kwds: Option<&Bound<'_, PyDict>>) -> usize {
    kwds.map_or(0, |dict| dict.len())
}
```

#### 2. **args

```rust
#[pyfunction]
#[pyo3(signature = (*args))]
fn args_args(args: &Bound<'_, PyTuple>) {
    println!("args: {:?}", args);
}
```

#### 3. None

```rust
#[pyfunction]
#[pyo3(signature = (n=None))]
fn none_default(n: Option<i32>) {
    match n {
        Some(n) => println!("n: {}", n),
        None => println!("n is None"),
    }
}
```

#### 4. 自定义类型

```rust
#[pyfunction]
#[pyo3(signature = (r#struct = "foo"))]
fn function_with_keyword(r#struct: &str) {
    /* ... */
}
```

#### 测试效果

```python
import pyo3_tutorial.signature as signature

if __name__ == "__main__":
    print(signature.kwds_args())
    print(signature.kwds_args(x=1, y=2))
    signature.none_default(None)
    signature.none_default(1)
    signature.args_args(1, 2, 3)

```

```shell
0
2
n is None
n: 1
args: (1, 2, 3)
```

### 函数签名 text_signature = (...)

用以修改 `__text_signature__` 属性， 在python中调用时显示的函数签名

不是很熟悉这个参数的作用，应该是给编写文档用的，所以应该能够编写更详尽的内容

## 错误处理

本部分主要为如何在rust部分抛出异常，以及如何在python部分捕获异常

### Python Style

```rust
// src/math.rs
use pyo3::exceptions::PyZeroDivisionError;

#[pyfunction]
fn divide(a: i32, b: i32) -> PyResult<i32> {
    if b == 0 {
        return Err(PyZeroDivisionError::new_err("b cannot be zero"));
    }
    Ok(a / b)
}
```

可以看到，我们可以选择抛出python已有的异常，接着使用python代码测试一下效果

```python
if __name__ == "__main__":
    try:
        print(math.divide(1, 0))
    except ZeroDivisionError as e:
        print(e)
    # b cannot be zero
```

### Rust Style

```rust
// src/math.rs
use std::num::ParseIntError;

#[pyfunction]
fn parse_int(a: &str) -> Result<i32, ParseIntError> {
    a.parse::<i32>()
}
```

在这里，我们直接抛出了rust的异常类型，我们来看一下python会怎么处理

```python
if __name__ == "__main__":
    try:
        print(math.parse_int("1aa23"))
    except ValueError as e:
        print(e)
    # invalid digit found in string
```

可以看到，我们直接catch ValueError就成功了

文档原话为 PyO3 will automatically convert a `Result<T, E>` returned by a `#[pyfunction]` into a `PyResult<T>` as long as there is an implementation of `std::from::From<E>` for `PyErr`. Many error types in the Rust standard library have a From conversion defined in this way.

由此可知，我们也可以抛出一些我们新创建的异常类型

TODO