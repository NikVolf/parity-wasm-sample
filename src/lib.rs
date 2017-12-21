
extern crate parity_wasm;

use std::sync::Arc;

use parity_wasm::interpreter;

/// Shortcuts when you don't have custom errors in runtime needed to be handled lately
pub type InterpreterError = interpreter::Error<interpreter::DummyUserError>;
pub type InterpreterMemoryInstance = interpreter::MemoryInstance<interpreter::DummyUserError>;
pub type InterpreterProgramInstance = interpreter::ProgramInstance<interpreter::DummyUserError>;
pub type InterpreterCallerContext<'a> = interpreter::CallerContext<'a, interpreter::DummyUserError>;

struct Ext;

struct Runtime<'a, 'b> {
    ext: &'a mut Ext,
    program: &'b InterpreterProgramInstance,
    memory: Arc<InterpreterMemoryInstance>,
}

impl<'a, 'b> Runtime<'a, 'b> {
	pub fn execution_params(&mut self) -> interpreter::ExecutionParams<interpreter::DummyUserError> {
		let env_instance = self.program.module("env")
			.expect("Env module always exists; qed");

		interpreter::ExecutionParams::with_external(
			"env".into(),
			Arc::new(
				interpreter::env_native_module(
                    env_instance,
                    interpreter::UserDefinedElements {

                        /*
                            Runtime implements UserFunctionExecutor, so is used `self` here
                        */
                        executor: Some(self),
                        globals: ::std::collections::HashMap::new(),

                        /*
                            This is your external functions signatures
                        */
                        functions: ::std::borrow::Cow::from(&[][..]),
                    },
                ).expect("Env module always exists; qed")
			)
		)
	}
}

impl<'a, 'b> interpreter::UserFunctionExecutor<interpreter::DummyUserError> for Runtime<'a, 'b> {
	fn execute(&mut self, name: &str, context: InterpreterCallerContext)
		-> Result<Option<interpreter::RuntimeValue>, InterpreterError>
	{
		match name {
			"_your_cool_method" => {
				panic!();
			},
			_ => {
				//trace!(target: "wasm", "Trapped due to unhandled function: '{}'", name);
				panic!();
			},
		}
	}
}

#[cfg(test)]
mod tests {

    use super::*;

    use parity_wasm::ModuleInstanceInterface;

    fn run(ext: &mut Ext) {
        let program = interpreter::ProgramInstance::new().expect("program instance to create");
        let code_buffer = vec![];

        // env module extracted from wasm program (it has one by default)
		let env_instance = program.module("env")
            .expect("Wasm program to contain env module");

        // linear memory extracted from env module (it has one by default)
		let env_memory = env_instance.memory(interpreter::ItemIndex::Internal(0))
            .expect("Linear memory to exist in wasm runtime");

        // creating runtime
		let mut runtime = Runtime {
            ext: ext,
            program: &program,
            memory: env_memory.clone(),
        };

		{
            let module = parity_wasm::deserialize_buffer(code_buffer).expect("a valid module");

			let execution_params = runtime.execution_params()
				.add_argument(interpreter::RuntimeValue::I32(0));

			let module_instance = program.add_module("module", module, Some(&execution_params.externals))
                .expect("should be added without errors");

			module_instance.execute_export("_your_cool_entry_point", execution_params).expect("should run without errors");
		}
    }
}
