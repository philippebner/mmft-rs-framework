#[macro_export]
/// Generates a wasm binding with a JsValue interface. Inputs and outputs must be serde compatible!
///
/// # Arguments
///
/// * `function_name` - the call name of the function
/// * `call_function` - the function to be bound
///
/// # Examples
///
/// ```
/// mmft_framework::wasm_interface_function!(
///     create_meander,
///     meander_designer::meander_designer::create_meander
/// );
/// ```
macro_rules! wasm_interface_function {
    ($function_name: ident, $call_function: ty) => {
        paste::item! {
            #[wasm_bindgen]
            pub fn [<$function_name>](input: wasm_bindgen::prelude::JsValue) -> JsValue {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                let output = $call_function(serde_wasm_bindgen::from_value(input).unwrap());
                serde_wasm_bindgen::to_value(&output).unwrap()
            }
        }
    };
}
