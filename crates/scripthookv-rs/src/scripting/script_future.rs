use smol::future::Future;

pub struct ScriptFuture<T, Fun>
where
  Fun: FnMut() -> std::task::Poll<T> + Send
{
  poll: Box<Fun>
}

impl<T, Fun> ScriptFuture<T, Fun>
where
  Fun: FnMut() -> std::task::Poll<T> + Send
{
  pub fn new(fun: Fun) -> Self {
    Self {
      poll: Box::new(fun)
    }
  }
}

impl<T, Fun> Future for ScriptFuture<T, Fun>
where
  Fun: FnMut() -> std::task::Poll<T> + Send
{
  type Output = T;

  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>
  ) -> std::task::Poll<Self::Output> {
    cx.waker().wake_by_ref();
    (self.get_mut().poll)()
  }
}
