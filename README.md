This macro re-writes try expressions to add a call to `anyhow::Context::with_context` which adds the
source location of try expression as context to the error. This creates something that is similar to
a backtrace, but is actually the path the error propagates through instead. This can be preferable
to using anyhow's backtrace feature because it doesn't rely on doing a bactrace or debugging symbols
being compiled in.
