
/// Used for calling natives.
/// 
/// Parameters are as follows:
/// 1. return type
/// 2. native hash
/// 3. native params
/// 
/// ```
/// unsafe fn wait(ms: i32) -> Void { 
///   call_native!(Void, 0x4EDE34FBADD967A6u64, ms)
/// }
/// ```
#[macro_export]
macro_rules! call_native {
  ($type:ty, $hash:literal $(, $args:expr)*) => {{
    $crate::natives::native_init($hash);
    $(
      $crate::natives::native_push(&$args);
    )*
    $crate::natives::native_call::<$type>()
  }}
}