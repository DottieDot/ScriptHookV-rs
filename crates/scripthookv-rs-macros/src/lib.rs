use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn shv_entrypoint(_metadata: TokenStream, item: TokenStream) -> TokenStream {
  let entrypoint = parse_macro_input!(item as ItemFn);

  let expanded = quote! {
    #entrypoint

    static __SCRIPTHOOKV: ::once_cell::sync::OnceCell<::std::sync::Arc<::std::sync::Mutex<scripthookv::ScriptHookV>>> = ::once_cell::sync::OnceCell::new();

    thread_local! {
      static __SCRIPT_MANAGER: ::once_cell::unsync::OnceCell<::std::cell::RefCell<::scripthookv::scripting::ScriptManager<'static>>> = ::once_cell::unsync::OnceCell::new();
    }

    extern "C" fn __shv_script_entrypoint() {
      __SCRIPT_MANAGER.with(|shvm| {
        let mut script_manager = shvm.get_or_init(|| {
          __SCRIPTHOOKV
            .get()
            .expect("ScriptHookv is not initialized")
            .lock()
            .unwrap()
            .new_script_manager_for_thread()
            .into()
        }).borrow_mut();
        loop {
          script_manager.tick();
          unsafe {
            ::scripthookv::BACKEND.get().expect("runtime not set").script_wait(0);
          }
        }
      });
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "stdcall" fn DllMain(
      instance: ::scripthookv::ModuleHandle,
      reason: u32,
      _reserved: *const ::std::ffi::c_void
    ) -> i32 {
      match reason {
        1 /* DLL_PROCESS_ATTACH */ => {
          __SCRIPTHOOKV.get_or_init(|| {
            ::std::sync::Arc::new(::std::sync::Mutex::new(entrypoint(instance)))
          });
          unsafe {
            ::scripthookv::BACKEND.get().expect("runtime not set").script_register(instance, __shv_script_entrypoint);
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
