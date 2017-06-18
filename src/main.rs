extern crate llvm;

use llvm::{Context, Module, Builder, Type};

fn main() {
    let context = Context::new();
    let module = Module::new("playground", &context);

    let function = module.add_function("add", Type::get::<fn(f64, f64) -> f64>(&context));
}
