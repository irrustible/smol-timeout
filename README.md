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

> This Source Code Form is subject to the terms of the Mozilla Public
> License, v. 2.0. If a copy of the MPL was not distributed with this
> file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
