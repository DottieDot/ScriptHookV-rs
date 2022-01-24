use thiserror::Error;

#[derive(Error, Debug)]
#[error("{message}")]
pub struct WinApiError {
  pub message: String
}

#[macro_export]
macro_rules! try_winapi {
  ($call:expr) => {
    let result = $call;
    if result == 0 {
      let message = format!(
        "{} failed with error code {}",
        stringify!($call),
        std::io::Error::last_os_error()
      );
      return Err($crate::winapi::WinApiError { message });
    }
  };
}

