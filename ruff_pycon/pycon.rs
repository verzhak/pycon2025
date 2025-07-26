
use crate::checkers::ast::Checker;
use ruff_diagnostics::{ Diagnostic, Violation };
use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_ast::{self as ast, ExceptHandler, Expr, Alias};
use ruff_python_ast::name::UnqualifiedName;
use ruff_text_size::Ranged;
use ruff_python_ast::{ Stmt, Parameters, Identifier };

// ##################################################################
// PyConAttributeError

#[derive(ViolationMetadata)]
pub(crate) struct PyConAttributeError;

impl Violation for PyConAttributeError {

    #[derive_message_formats]
    fn message(&self) -> String {

        "AttributeError is a deprecated replacement for cached_property".to_string()

    }
}

fn check(checker : &Checker, elem : &Expr) {
    if let Some(name) = UnqualifiedName::from_expr(elem) {
        if name.segments().join(".") == "AttributeError" {
            checker.report_diagnostic(Diagnostic::new(PyConAttributeError {}, elem.range()));
        }
    }
}

pub(crate) fn pycon_attribute_error(checker: &Checker, handlers: &[ExceptHandler]) {
    for handler in handlers {
        let ExceptHandler::ExceptHandler(ast::ExceptHandlerExceptHandler {
            type_: Some(type_),
            ..
        }) = handler
        else {
            continue;
        };

        match type_.as_ref() {
            Expr::Attribute(_) | Expr::Name(_) => {
                check(checker, &type_);
            }
            Expr::Tuple(ast::ExprTuple { elts, .. }) => {
                for elem in elts {
                    check(checker, &elem);
                }
            }
            _ => {}
        }
    }
}

// ##################################################################
// PyConDeprecatedImport

#[derive(ViolationMetadata)]
pub(crate) struct PyConDeprecatedImport;

impl Violation for PyConDeprecatedImport {

    #[derive_message_formats]
    fn message(&self) -> String {

        "Deprecated import".to_string()

    }
}

pub(crate) fn pycon_deprecated_import(checker: &Checker, stmt: &Stmt, module : Option<&str>, names : &Vec<Alias>) {

    for name in names {

        if module == Some("project.various") && name.name.id == "DeprecatedClass" {

            checker.report_diagnostic(Diagnostic::new(PyConDeprecatedImport {}, stmt.range()));

        }

    }

}

// ##################################################################
// PyConDeprecatedTypingImport

#[derive(ViolationMetadata)]
pub(crate) struct PyConDeprecatedTypingImport {

    class_name : String,

}

impl Violation for PyConDeprecatedTypingImport {

    #[derive_message_formats]
    fn message(&self) -> String {

        format!("Deprecated import - class {} is deprecated for import from typing module", self.class_name)

    }

}

const TYPING_CLASSES : [ &str; 10 ] = [ "List", "Tuple", "Dict", "Set", "Union", "Iterator", "Iterable", "Collection", "Optional", "Generic" ];

pub(crate) fn pycon_deprecated_typing_import(checker: &Checker, stmt: &Stmt, module : Option<&str>, names : &Vec<Alias>) {

    for name in names {

        if module == Some("typing") && TYPING_CLASSES.contains(&name.name.id.as_str()) {

            checker.report_diagnostic(Diagnostic::new(PyConDeprecatedTypingImport { class_name : name.name.id.as_str().into() }, stmt.range()));

        }

    }

}

// ##################################################################
// PyConArgsWithoutType

#[derive(ViolationMetadata)]
pub(crate) struct PyConArgsWithoutType;

impl Violation for PyConArgsWithoutType {

    #[derive_message_formats]
    fn message(&self) -> String {

        "Args without type".to_string()

    }
}

pub(crate) fn pycon_args_without_type(checker: &Checker, parameters : &Box<Parameters>) {

    for arg in &parameters.args {

        let id = &arg.parameter.name.id;

        if id != "cls" && id != "self" && arg.parameter.annotation.is_none() {

            checker.report_diagnostic(Diagnostic::new(PyConArgsWithoutType {}, arg.parameter.range));

        }

    }

}

// ##################################################################
// PyConFunctionWithoutReturnDefinition

#[derive(ViolationMetadata)]
pub(crate) struct PyConFunctionWithoutReturnDefinition;

impl Violation for PyConFunctionWithoutReturnDefinition {

    #[derive_message_formats]
    fn message(&self) -> String {

        "Function without return definition".to_string()

    }

}

pub(crate) fn pycon_function_without_return_definition(checker: &Checker, name : &Identifier, returns : &Option<Box<Expr>>) {

    if name != "__init__" && name != "__del__" && returns.is_none() {

        checker.report_diagnostic(Diagnostic::new(PyConFunctionWithoutReturnDefinition {}, name.range()));

    }

}

