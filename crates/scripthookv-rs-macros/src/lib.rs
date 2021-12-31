use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn shv_entrypoint(_metadata: TokenStream, item: TokenStream) -> TokenStream {
  let entrypoint = parse_macro_input!(item as ItemFn);

  let expanded = quote! {
    #entrypoint

    static SCRIPTHOOKV: once_cell::sync::OnceCell<scripthookv::ScriptHookV> = once_cell::sync::OnceCell::new();

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "stdcall" fn DllMain(
      instance: scripthookv::ModuleHandle,
      reason: u32,
      _reserved: *const std::ffi::c_void
    ) -> i32 {
      match reason {
        1 /* DLL_PROCESS_ATTACH */ => {
          SCRIPTHOOKV.get_or_init(|| {
            entrypoint(instance)
          });
          1
        }
        0 /* DLL_PROCESS_DETACH */ => {
          if let Some(shv) = &SCRIPTHOOKV.get() {
            shv.cleanup();
          }
          1
        },
        _ => 1,
      }
    }
  };

  expanded.into()
}
