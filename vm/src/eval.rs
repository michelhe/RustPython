use crate::compile;
use crate::frame::Scope;
use crate::pyobject::PyResult;
use crate::vm::VirtualMachine;

pub fn eval(
    vm: &VirtualMachine,
    source: &str,
    scope: Scope,
    source_path: Option<String>,
) -> PyResult {
    match vm.compile(source, &compile::Mode::Eval, source_path) {
        Ok(bytecode) => {
            debug!("Code object: {:?}", bytecode);
            vm.run_code_obj(bytecode, scope)
        }
        Err(err) => Err(vm.new_syntax_error(&err)),
    }
}

#[cfg(test)]
mod tests {
    use super::eval;
    use super::VirtualMachine;

    #[test]
    fn test_print_42() {
        let source = String::from("print('Hello world')\n");
        let mut vm = VirtualMachine::new();
        let vars = vm.new_scope_with_builtins();
        let _result = eval(&mut vm, &source, vars, Some("<unittest>".to_string()));

        // TODO: check result?
        //assert_eq!(
        //    parse_ast,
        // );
    }
}
