use std::time::{Duration, Instant};

pub fn set_interval<F: FnMut()>(interval: Duration, mut f: F) {
    let mut last_time = Instant::now();

    loop {
        f();

        while last_time.elapsed() < interval {
            // TODO: Maybe use
            // std::hint::spin_loop();
            // or
            // thread::yield_now();
        }
        last_time += interval;
    }
}
