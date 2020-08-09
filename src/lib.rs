/**************************************************************************************************
 *                                                                                                *
 * This Source Code Form is subject to the terms of the Mozilla Public                            *
 * License, v. 2.0. If a copy of the MPL was not distributed with this                            *
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.                                       *
 *                                                                                                *
 **************************************************************************************************/

// ======================================== Configuration ======================================= \\

#![no_std]

// ======================================== Documentation ======================================= \\

//! # smol-timeout
//!
//! A way to poll a future until it or a `smol::Timer` completes.
//!
//! ## Example
//!
//! ```rust
//! use smol::Timer;
//! use smol_timeout::TimeoutExt;
//! use std::time::Duration;
//!
//! smol::run(async {
//!     let foo = async {
//!         Timer::new(Duration::from_millis(250)).await;
//!         24
//!     };
//!
//!     let foo = foo.timeout(Duration::from_millis(100));
//!     assert_eq!(foo.await, None);
//!
//!     let bar = async {
//!         Timer::new(Duration::from_millis(100)).await;
//!         42
//!     };
//!
//!     let bar = bar.timeout(Duration::from_millis(250));
//!     assert_eq!(bar.await, Some(42));
//! });
//! ```

// =========================================== Imports ========================================== \\

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use pin_project_lite::pin_project;
use smol::Timer;

// ============================================ Types =========================================== \\

pin_project! {
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
    ///     Timer::new(Duration::from_millis(250)).await;
    ///     24
    /// };
    ///
    /// let foo = foo.timeout(Duration::from_millis(100));
    /// assert_eq!(foo.await, None);
    ///
    /// let bar = async {
    ///     Timer::new(Duration::from_millis(100)).await;
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
}

// ========================================= Interfaces ========================================= \\

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
    ///     Timer::new(Duration::from_millis(250)).await;
    ///     24
    /// };
    ///
    /// let foo = foo.timeout(Duration::from_millis(100));
    /// assert_eq!(foo.await, None);
    ///
    /// let bar = async {
    ///     Timer::new(Duration::from_millis(100)).await;
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
            timer: Timer::new(after),
        }
    }
}

impl<Fut: Future> TimeoutExt for Fut {}

// ========================================= impl Future ======================================== \\

impl<Fut: Future> Future for Timeout<Fut> {
    type Output = Option<Fut::Output>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();

        if this.timer.poll(ctx).is_ready() {
            return Poll::Ready(None);
        }

        if let Poll::Ready(output) = this.future.poll(ctx) {
            return Poll::Ready(Some(output));
        }

        Poll::Pending
    }
}
