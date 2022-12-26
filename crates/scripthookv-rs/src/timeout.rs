use std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
  time::{Duration, Instant}
};

use smol::future::FutureExt;
use thiserror::Error;

pub struct Timeout<Fut: Future> {
  cancel_at: Instant,
  fut:       Pin<Box<Fut>>
}

impl<Fut: Future> Timeout<Fut> {
  pub fn new(future: Fut, duration: Duration) -> Self {
    Self {
      cancel_at: Instant::now() + duration,
      fut:       Box::pin(future)
    }
  }
}

impl<Fut: Future> Future for Timeout<Fut> {
  type Output = Result<Fut::Output, TimedOutError>;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    if Instant::now() >= self.cancel_at {
      Poll::Ready(Err(TimedOutError))
    } else if let Poll::Ready(r) = self.fut.poll(cx) {
      Poll::Ready(Ok(r))
    } else {
      cx.waker().wake_by_ref();
      Poll::Pending
    }
  }
}

#[derive(Error, Debug)]
#[error("the future timed out")]
pub struct TimedOutError;

pub trait TimeoutExt {
  fn timeout(self, duration: Duration) -> Timeout<Self>
  where
    Self: Future + Sized
  {
    Timeout::new(self, duration)
  }
}

impl<Output, T: Future<Output = Output>> TimeoutExt for T {}
