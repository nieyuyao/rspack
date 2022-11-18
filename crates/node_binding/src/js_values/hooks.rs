use napi::bindgen_prelude::*;

#[napi(object)]
pub struct JsHooks {
  pub done: JsFunction,
  pub process_assets: JsFunction,
  pub compilation: JsFunction,
  pub this_compilation: JsFunction,
}
