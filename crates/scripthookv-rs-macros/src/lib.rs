use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn shv_entrypoint(_metadata: TokenStream, item: TokenStream) -> TokenStream {
  let entrypoint = parse_macro_input!(item as ItemFn);

  let expanded = quote! {
    #entrypoint

    static __SCRIPTHOOKV: ::once_cell::sync::OnceCell<::std::sync::Arc<::std::sync::Mutex<scripthookv::ScriptHookV>>> = ::once_cell::sync::OnceCell::new();
    static mut __SCRIPT_MANAGER: ::std::option::Option<::scripthookv::scripting::ScriptManager> = None;

    extern "C" fn __shv_script_entrypoint() {
      let mut script_manager = {
        unsafe {
          if __SCRIPT_MANAGER.is_none() {
            __SCRIPT_MANAGER = Some(
              __SCRIPTHOOKV
                .get()
                .expect("ScriptHookv is not initialized")
                .lock()
                .unwrap()
                .new_script_manager_for_thread()
            );
          }
          __SCRIPT_MANAGER.as_mut().unwrap()
        }
      };
      loop {
        script_manager.tick();
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
