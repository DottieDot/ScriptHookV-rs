use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn shv_entrypoint(_metadata: TokenStream, item: TokenStream) -> TokenStream {
  let entrypoint = parse_macro_input!(item as ItemFn);

  let expanded = quote! {
    #entrypoint

    static __SCRIPTHOOKV: ::once_cell::sync::OnceCell<::std::sync::Arc<::std::sync::Mutex<scripthookv::ScriptHookV>>> = ::once_cell::sync::OnceCell::new();

    extern "C" fn __shv_script_entrypoint() {
      loop {
        {
          let mut shv = __SCRIPTHOOKV
            .get()
            .expect("ScriptHookv is not initialized")
            .lock()
            .unwrap();
          shv.update_scripts();
        }
        unsafe {
          ::scripthookv::shv_bindings::scriptWait(0);
        }
      }
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "stdcall" fn DllMain(
      instance: scripthookv::ModuleHandle,
      reason: u32,
      _reserved: *const std::ffi::c_void
    ) -> i32 {
      match reason {
        1 /* DLL_PROCESS_ATTACH */ => {
          __SCRIPTHOOKV.get_or_init(|| {
            ::std::sync::Arc::new(::std::sync::Mutex::new(entrypoint(instance)))
          });
          unsafe {
            ::scripthookv::shv_bindings::scriptRegister(instance, __shv_script_entrypoint);
          }
          1
        }
        0 /* DLL_PROCESS_DETACH */ => {
          if let Some(shv) = &__SCRIPTHOOKV.get() {
            shv.lock().unwrap().cleanup();
          }
          1
        },
        _ => 1,
      }
    }
  };

  expanded.into()
}
