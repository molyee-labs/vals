use std::time::*;

lazy_static! { static ref TIMER:Instant = Instant::now(); }

#[inline]
pub fn now() -> SystemTime {
    SystemTime::now()
}

#[inline]
pub fn timestamp() -> Result<Duration, SystemTimeError> {
    now().duration_since(SystemTime::UNIX_EPOCH)
}

#[inline]
pub fn elapsed() -> Duration {
    TIMER.elapsed()
}