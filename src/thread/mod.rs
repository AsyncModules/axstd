//! Native threads.

#[cfg(feature = "multitask")]
mod multi;
#[cfg(feature = "multitask")]
pub use multi::*;

use arceos_api::task as api;

/// Current thread gives up the CPU time voluntarily, and switches to another
/// ready thread.
///
/// For single-threaded configuration (`multitask` feature is disabled), we just
/// relax the CPU and wait for incoming interrupts.
pub fn yield_now() {
    api::ax_yield_now();
}
#[cfg(feature = "async")]
pub async fn async_yield_now() {
    api::ax_async_yield_now().await;
}

/// Exits the current thread.
///
/// For single-threaded configuration (`multitask` feature is disabled),
/// it directly terminates the main thread and shutdown.
pub fn exit(exit_code: i32) -> ! {
    api::ax_exit(exit_code);
}

/// Current thread is going to sleep for the given duration.
///
/// If one of `multitask` or `irq` features is not enabled, it uses busy-wait
/// instead.
pub fn sleep(dur: core::time::Duration) {
    sleep_until(arceos_api::time::ax_current_time() + dur);
}
#[cfg(feature = "async")]
pub async fn async_sleep(dur: core::time::Duration) {
    async_sleep_until(arceos_api::time::ax_current_time() + dur).await;
}

/// Current thread is going to sleep, it will be woken up at the given deadline.
///
/// If one of `multitask` or `irq` features is not enabled, it uses busy-wait
/// instead.
pub fn sleep_until(deadline: arceos_api::time::AxTimeValue) {
    api::ax_sleep_until(deadline);
}
#[cfg(feature = "async")]
pub async fn async_sleep_until(deadline: arceos_api::time::AxTimeValue) {
    api::ax_async_sleep_until(deadline).await;
}