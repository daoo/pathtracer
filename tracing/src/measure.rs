use std::time;

pub fn measure<F, O>(f: F) -> (time::Duration, O)
where
    F: FnOnce() -> O,
{
    let t1 = time::Instant::now();
    let output = f();
    let t2 = time::Instant::now();
    (t2 - t1, output)
}
