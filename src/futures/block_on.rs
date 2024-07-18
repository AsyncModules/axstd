use core::future::Future;
use arceos_api::task as api;

/// Run a future to completion.
///
/// This calls `.poll()` on the future in a busy loop, which blocks
/// the current thread at 100% cpu usage until the future is done. The
/// future's `Waker` mechanism is not used.
///
/// You can use this to run multiple futures concurrently with [`join`][crate::join].
///
/// It's suitable for systems with no or limited concurrency and without
/// strict requirements around power consumption. For more complex use
/// cases, prefer using a "real" executor like `embassy-executor`, which
/// supports multiple tasks, and putting the core to sleep when no task
/// needs to do work.
pub fn block_on<F, T>(f: F) -> T::Output 
where 
    F: FnOnce() -> T,
    T: Future<Output = i32> + Send + 'static,
{
    api::block_on(f(), "main_coroutine".into())
}
