This macro re-writes try expressions to add a call to `anyhow::Context::with_context` which adds the
source location of try expression as context to the error. This creates something that is similar to
a backtrace, but is actually the path the error propagates through instead. This can be preferable
to using anyhow's backtrace feature because it doesn't rely on doing a bactrace or debugging symbols
being compiled in.

Here is an example
```rust
use anyhow_trace::anyhow_trace;
use anyhow::{bail, Result};

fn foo_err() -> Result<()> {
    bail!("this is foo error")
}

struct FooBar;

#[anyhow_trace]
impl FooBar {
    fn thing() -> Result<()> {
        fn im_inside() -> Result<()> {
            foo_err()?;
            Ok(())
        }
        im_inside()?;
        Ok(())
    }
}

#[anyhow_trace]
fn inner() -> Result<()> {
    FooBar::thing()?;
    Ok(())
}

#[anyhow_trace]
fn main() -> Result<()> {
    inner()?;
    Ok(())
}
```

This prints out the following:
```
Error: main at examples/example.rs:30:5

Caused by:
    0: inner at examples/example.rs:24:5
    1: FooBar::thing at examples/example.rs:17:9
    2: FooBar::thing::im_inside at examples/example.rs:14:13
    3: this is foo error
```
