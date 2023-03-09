## rust-by-example

[通过例子学 Rust 中文版](https://rustwiki.org/zh-CN/rust-by-example/index.html)

### 实现RGB的Display特征

主要看到`{:02X}`是用大写的16进制表示, 同时用0补全至2位.

```rust
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{}{}{}", self.red, self.green, self.blue, format!("{:02X}",
                self.red), format!("{:02X}", self.green), format!("{:02X}", self.blue))
    }
}
```

### 矩阵翻转

为了在翻转后原矩阵仍然可用, 需要通过`&`借用一下所有权, 而不直接把所有权move过去.

```rust
use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn trans(m: &Matrix) -> Matrix {
    let new_matrix= Matrix{
        0: m.0,
        1: m.2,
        2: m.1,
        3: m.3
    };
    new_matrix
}

fn main(){
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Before transpose:\n{}", matrix);
    println!("Transpose matrix:\n{}", trans(&matrix));
    println!("After transpose:\n{}", matrix);
}
```

### 结构体解构

包括L16对嵌套结构体的解构(还有更优美的办法吗??)以及`rect_area`和`square`两个函数的实现.


```rust
// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[allow(dead_code)]
struct Rectangle {
    // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32{
    let Rectangle {top_left: Point{x: x1, y: y1}, bottom_right: Point{x: x2, y: y2}} = rect;
    let res: f32 = (x1 - x2).abs() * (y1 - y2).abs();
    res
}

fn square(p: Point, a: f32) -> Rectangle {
    let res = Rectangle{
        bottom_right: Point{
            x: p.x + a,
            y: p.y - a
        },
        top_left: p
    };

    res
}

fn main() {
    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right
    };

    let a: f32 = 10.0;

    println!("Square with width {} has area: {:.10}", a, rect_area(&square(point, a)))
}
```

不过发现, 当使用`f32`时...

```
Square with width 10 has area: 99.9999923706
```

换成`f64`才显示为100...

```
Square with width 10 has area: 100.0000000000
```

float这么垃圾嘛...



