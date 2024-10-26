use std::time::Instant;
use time::Duration;

#[inline]
pub fn measure<F, O>(f: F) -> (Duration, O)
where
    F: FnOnce() -> O,
{
    let t1 = Instant::now();
    let output = f();
    let t2 = Instant::now();
    (time::Duration::try_from(t2 - t1).unwrap(), output)
}
