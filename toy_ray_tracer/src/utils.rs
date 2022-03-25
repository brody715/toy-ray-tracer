use std::time::Instant;

macro_rules! integer {
    ($t:tt) => {
        ///
        /// Panics if the range is empty.
        #[inline(always)]
        pub fn $t(range: impl RangeBounds<$t>) -> $t {
            fastrand::$t(range)
        }
    };
}

pub(crate) struct ExecutionTimer<F: Fn(&Instant) -> ()> {
    start: Instant,
    handler: F,
}

impl<F: Fn(&Instant) -> ()> ExecutionTimer<F> {
    #[must_use]
    pub(crate) fn new(handler: F) -> Self {
        Self {
            start: Instant::now(),
            handler,
        }
    }

    fn when_stop(&self) {
        (self.handler)(&self.start);
    }
}

impl<F: Fn(&Instant) -> ()> Drop for ExecutionTimer<F> {
    fn drop(&mut self) {
        self.when_stop()
    }
}

pub mod random {
    use fastrand;

    pub fn new_rng() -> fastrand::Rng {
        return fastrand::Rng::new();
    }

    #[allow(unused_imports)]
    use std::ops::{Range, RangeBounds};

    #[inline(always)]
    pub fn f32() -> f32 {
        return fastrand::f32();
    }

    integer!(usize);
}
