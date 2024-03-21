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
