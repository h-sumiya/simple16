```rust
mod bits;
mod dump;
mod load;

fn main() {
    let data = vec![1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 1000];
    let mut buf = Vec::new();
    dump::dump(&data, &mut buf);
    println!("dumped: {:?}", buf);

    let mut data2 = Vec::new();
    load::load(&buf, &mut data2).unwrap();

    assert_eq!(data, data2)
}
```