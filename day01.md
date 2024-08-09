# 第一步 安装

已经安装过了，略过，没有安装的这样安装

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# 第二步 卸载

```bash
rustup self uninstall
```

# 包管理器

在rust中使用`cargo`作为包管理器，类似web中的node,py中pipe等等

## 创建项目

```bash
cargo new [project_name]
```

## 构建命令
```bash
cargo build
```

## 运行

```bash
cargo run
```

## 检查

```bash
cargo check
```

更多的命令可以通过查看，其中包括`release`,`login`,`publish`,`clean`,`remove`等等，这些后面遇到再说

```bash
cargo --list
```

# 可变变量和不可变

```rust
let x = 5; // 这是不可变的

let mut x = 5; // 这是可变的

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 这是常量

```

这里也有作用域的概念全局作用于和块作用域，例如。

```rust
let x = 5;

{
    let x = x * 8
    println!("the value of x is {x}"); // 40
}

 println!("the value of x is {x}"); // 5

```

其中let声明的变量可以多次声明例如，这样也是可以的，后一个会覆盖前面的；

```rust
let x = 5;

let x = 6;
```

注意：rust是强类型的语言，不能把不同的类型赋值给预先定义的类型的变量例如：

```rust
let x = "string";

const space = "   ";

x = space.len(); // 这样是不行的，因为len是3 是数字
```