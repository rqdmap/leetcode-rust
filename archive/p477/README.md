给出一个序列, 求出所有元素两两之间的海明距离之和. 

最开始没看范围, 写了个n=1e4的$O(n^2log(n))$ 算法华丽的T了...

期间遇到几个坑点:

- Rust的if语句必须要有`{}`, 哪怕只有一条语句在后面  ~~压行选手狂怒~~

- L8 不能写成`if x & 1 << bit {...}`, 因为rust中不会将int默认转为bool!

- L7 的所有权租借问题, 必须要有`&nums`(另一种写法是`for &n in nums.iter()`), 不然报错, 这可能涉及到具体的底层实现, 暂时不予深究:

```rust
error[E0382]: use of moved value: `nums`
   --> src/main.rs:7:22
    |
2   |     pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
    |                                   ---- move occurs because `nums` has type `Vec<i32>`, which does not implement the `Copy` trait
...
7   |             for x in nums {
    |                      ^^^^ `nums` moved due to this implicit call to `.into_iter()`, in previous iteration of loop
    |
note: `into_iter` takes ownership of the receiver `self`, which moves `nums`
   --> /home/rqdmap/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:262:18
    |
262 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
help: consider iterating over a slice of the `Vec<i32>`'s content to avoid moving into the `for` loop
    |
7   |             for x in &nums {
    |                      +

For more information about this error, try `rustc --explain E0382`.
```


