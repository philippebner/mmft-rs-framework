#[macro_export]
/// Generates a python binding with a JSON-like/Dict interface. Inputs and outputs must be serde compatible!
///
/// # Arguments
///
/// * `module` - the python module parameter (see pyo3)
/// * `function_name` - the call name of the function
/// * `call_function` - the function to be bound
/// * `input_type` - optional; Struct of the input type
/// * `output_type` - optional; Struct of the output type
///
/// # Examples
///
/// ```
/// mmft_framework::py_interface_function!(
///     module,
///     create_meander,
///     meander_designer_lib::meander_designer::create_meander
/// );
/// ```
macro_rules! py_interface_function {
    ($module: ident, $function_name: ident, $call_function: ty) => {
        let data_struct_name = stringify!($function_name).to_case(convert_case::Case::Pascal);
        let input_type = format!("{data_struct_name}Input");
        let output_type = format!("{data_struct_name}Output");
        $crate::py_interface_function!(
            $module,
            $function_name,
            $call_function,
            input_type,
            output_type
        );
    };

    ($module: ident, $function_name: ident, $call_function: ty, $input_type: ident, $output_type: ident) => {
        paste::item! {
            #[pyfunction]
            fn [<$function_name>](py: Python, input: PyObject) -> PyResult<Py<PyAny>> {
                let parameters = pythonize::depythonize(input.as_ref(py)).unwrap();
                let result = $call_function(parameters);
                Ok(pythonize::pythonize(py, &result).unwrap())
            }

            $module.add_function(wrap_pyfunction!($function_name, $module)?)?;
        }
    };
}
