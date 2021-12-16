
#[macro_export]
macro_rules! call_native {
  ($type:ident, $hash:literal $(, $args:expr)*) => {{
    $crate::natives::native_init($hash);
    $(
      $crate::natives::native_push(&$args);
    )*
    $crate::natives::native_call::<$type>()
  }}
}