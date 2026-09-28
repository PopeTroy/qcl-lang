use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::StructType;
use std::collections::HashMap;

pub struct LlvmCodegen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub dim_type_cache: HashMap<String, StructType<'ctx>>,
    pub frame_type_cache: HashMap<String, StructType<'ctx>>,
}

impl<'ctx> LlvmCodegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        Self {
            context,
            module,
            dim_type_cache: HashMap::new(),
            frame_type_cache: HashMap::new(),
        }
    }

    // Represents Uncertain<Vector3<ICRF, Length>> as:
    // { double[3] value, double[3][3] cov, i8 dist_type }
    pub fn get_uncertain_tensor_type(&mut self) -> StructType<'ctx> {
        let double_type = self.context.f64_type();
        let val_arr_type = double_type.array_type(3);
        let cov_row_type = double_type.array_type(3);
        let cov_arr_type = cov_row_type.array_type(3);
        let dist_type = self.context.i8_type();

        self.context.struct_type(
            &[
                val_arr_type.into(),
                cov_arr_type.into(),
                dist_type.into(),
            ],
            false,
        )
    }
}
