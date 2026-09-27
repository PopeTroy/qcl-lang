use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, StructType};

pub struct LlvmCodegen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
}

impl<'ctx> LlvmCodegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        Self { context, module }
    }

    /// Emits runtime memory structure for:
    /// Uncertain<Vector3<Frame, Dim>> = { double[3] value, double[3][3] cov, i8 dist_type }
    pub fn get_uncertain_tensor_type(&self) -> StructType<'ctx> {
        let f64_type = self.context.f64_type();
        let i8_type = self.context.i8_type();

        let val_arr_type = f64_type.array_type(3);
        let cov_row_type = f64_type.array_type(3);
        let cov_arr_type = cov_row_type.array_type(3);

        let fields: [BasicTypeEnum<'ctx>; 3] = [
            val_arr_type.into(),
            cov_arr_type.into(),
            i8_type.into(),
        ];

        self.context.struct_type(&fields, false)
    }
}
