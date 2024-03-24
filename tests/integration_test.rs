use anyhow_trace::anyhow_trace;
use anyhow::{bail, Result};

fn foo_err() -> Result<()> {
    bail!("this is foo error")
}

struct FooBar;

#[anyhow_trace]
impl FooBar {
    fn thing() -> Result<bool> {
        fn im_inside() -> Result<()> {
            foo_err()?;
            Ok(())
        }
        im_inside()?;
        Ok(true)
    }
}

#[anyhow_trace]
fn inner() -> Result<()> {
    if FooBar::thing()? {
        Ok(())
    } else {
        Ok(())
    }
}

#[test]
fn simple() {
    let err = inner().unwrap_err();
    assert_eq!(format!("{err:?}"), "\
        inner at tests/integration_test.rs:24:8\n\n\
        Caused by:\n    \
            0: FooBar::thing at tests/integration_test.rs:17:9\n    \
            1: FooBar::thing::im_inside at tests/integration_test.rs:14:13\n    \
            2: this is foo error\
    ");
}
