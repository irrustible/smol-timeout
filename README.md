# `smol-timeout`

A way to poll a future until it or a `smol::Timer` completes.

## Example

```rust
use smol::Timer;
use smol_timeout::TimeoutExt;
use std::time::Duration;

smol::run(async {
    let foo = async {
        Timer::new(Duration::from_millis(250)).await;
        24
    };

    let foo = foo.timeout(Duration::from_millis(100));
    assert_eq!(foo.await, None);

    let bar = async {
        Timer::new(Duration::from_millis(100)).await;
        42
    };

    let bar = bar.timeout(Duration::from_millis(250));
    assert_eq!(bar.await, Some(42));
});
```

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
