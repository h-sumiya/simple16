```toml
[dependencies]
simple16 = { git = "https://github.com/h-sumiya/simple16" }
```

```rust
use simple16::{load,dump};

fn main() {
    let data = vec![1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 1000];
    let mut buf = Vec::new();
    dump(&data, &mut buf);
    println!("dumped: {:?}", buf);

    let mut data2 = Vec::new();
    load(&buf, &mut data2).unwrap();

    assert_eq!(data, data2)
}
```