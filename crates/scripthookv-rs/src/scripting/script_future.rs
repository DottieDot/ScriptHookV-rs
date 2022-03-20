use smol::future::Future;

pub struct ScriptFuture<T, Fun>
where
  Fun: FnMut() -> std::task::Poll<T>
{
  poll: Box<Fun>
}

impl<T, Fun> ScriptFuture<T, Fun>
where
  Fun: FnMut() -> std::task::Poll<T>
{
  pub fn new(fun: Fun) -> Self {
    Self {
      poll: Box::new(fun)
    }
  }
}

impl<T, Fun> Future for ScriptFuture<T, Fun>
where
  Fun: FnMut() -> std::task::Poll<T>
{
  type Output = T;

  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>
  ) -> std::task::Poll<Self::Output> {
    cx.waker().clone().wake();
    (self.get_mut().poll)()
  }
}

unsafe impl<T, Fun> Send for ScriptFuture<T, Fun> where Fun: FnMut() -> std::task::Poll<T> {}
