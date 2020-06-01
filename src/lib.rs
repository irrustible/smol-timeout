#![no_std]

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use pin_project::{pin_project, project};
use smol::Timer;

#[pin_project]
/// A future polling both another future and a [`Timer`] that will complete after a specified
/// timeout, and returning the future's output or [`None`] if the timer completes first.
///
/// ## Example
///
/// ```rust
/// # use smol::Timer;
/// use smol_timeout::TimeoutExt;
/// use std::time::Duration;
///
/// # smol::run(async {
/// let foo = async {
///     Timer::after(Duration::from_millis(250)).await;
///     24
/// };
///
/// let foo = foo.timeout(Duration::from_millis(100));
/// assert_eq!(foo.await, None);
///
/// let bar = async {
///     Timer::after(Duration::from_millis(100)).await;
///     42
/// };
///
/// let bar = bar.timeout(Duration::from_millis(250));
/// assert_eq!(bar.await, Some(42));
/// # })
/// ```
pub struct Timeout<Fut: Future> {
    #[pin]
    future: Fut,
    #[pin]
    timer: Timer,
}

/// An extension trait for [`Future`]s that provides a way to create [`Timeout`]s.
pub trait TimeoutExt: Future {
    /// Given a [`Duration`], creates and returns a new [`Timeout`] that will poll both the future
    /// and a [`Timer`] that will complete after the provided duration, and return the future's
    /// output or [`None`] if the timer completes first.
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use smol::Timer;
    /// use smol_timeout::TimeoutExt;
    /// use std::time::Duration;
    ///
    /// # smol::run(async {
    /// let foo = async {
    ///     Timer::after(Duration::from_millis(250)).await;
    ///     24
    /// };
    ///
    /// let foo = foo.timeout(Duration::from_millis(100));
    /// assert_eq!(foo.await, None);
    ///
    /// let bar = async {
    ///     Timer::after(Duration::from_millis(100)).await;
    ///     42
    /// };
    ///
    /// let bar = bar.timeout(Duration::from_millis(250));
    /// assert_eq!(bar.await, Some(42));
    /// # })
    /// ```
    fn timeout(self, after: Duration) -> Timeout<Self>
    where
        Self: Sized,
    {
        Timeout {
            future: self,
            timer: Timer::after(after),
        }
    }
}

impl<Fut: Future> TimeoutExt for Fut {}

impl<Fut: Future> Future for Timeout<Fut> {
    type Output = Option<Fut::Output>;

    #[project]
    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        #[project]
        let Timeout { future, timer } = self.project();

        if timer.poll(ctx).is_ready() {
            return Poll::Ready(None);
        }

        if let Poll::Ready(output) = future.poll(ctx) {
            return Poll::Ready(Some(output));
        }

        Poll::Pending
    }
}
