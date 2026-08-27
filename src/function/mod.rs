//! Function definition machinery: the analogue of go-cty's `cty/function`
//! package.

pub mod stdlib;

use crate::error::Error;
use crate::refinement::RefinementBuilder;
use crate::types::Type;
use crate::value::Value;

/// Decides a function's return type from its (possibly unknown) argument
/// values (go-cty: `function.TypeFunc`).
pub type TypeFunc = Box<dyn Fn(&[Value]) -> Result<Type, Error>>;

/// Implements a function's behavior given argument values and the return type
/// already decided by the [`TypeFunc`] (go-cty: `function.ImplFunc`).
pub type ImplFunc = Box<dyn Fn(&[Value], &Type) -> Result<Value, Error>>;

/// Describes additional refinements that hold for all results of a function
/// (go-cty: `Spec.RefineResult`).
pub type RefineResultFunc = Box<dyn Fn(RefinementBuilder) -> RefinementBuilder>;

/// One parameter of a function (go-cty: `function.Parameter`).
#[derive(Debug, Default, Clone)]
pub struct Parameter {
    /// Optional name, for documentation only (go-cty: `Parameter.Name`).
    pub name: String,
    /// Optional description (go-cty: `Parameter.Description`).
    pub description: String,
    /// The type arguments must conform to; `Type::dynamic()` permits any type
    /// (go-cty: `Parameter.Type`).
    pub ty: Option<Type>,
    /// Whether null arguments are passed through to the implementation rather
    /// than rejected (go-cty: `Parameter.AllowNull`).
    pub allow_null: bool,
    /// Whether unknown arguments are passed through to the implementation
    /// rather than short-circuiting to an unknown result
    /// (go-cty: `Parameter.AllowUnknown`).
    pub allow_unknown: bool,
    /// Whether `Value::dynamic()` is passed through to the type-check function
    /// (go-cty: `Parameter.AllowDynamicType`).
    pub allow_dynamic_type: bool,
    /// Whether marked arguments are passed through to the implementation
    /// rather than auto-unmarked with marks propagated to the result
    /// (go-cty: `Parameter.AllowMarked`).
    pub allow_marked: bool,
}

/// The specification a [`Function`] is built from (go-cty: `function.Spec`).
///
/// Unset optional fields use `Default`; `type_fn` and `impl_fn` are required.
pub struct Spec {
    /// Optional description of the function (go-cty: `Spec.Description`).
    pub description: String,
    /// The positional parameters (go-cty: `Spec.Params`).
    pub params: Vec<Parameter>,
    /// Optional specification for variadic arguments (go-cty: `Spec.VarParam`).
    pub var_param: Option<Parameter>,
    /// Decides the return type; use [`static_return_type`] when it is fixed
    /// (go-cty: `Spec.Type`).
    pub type_fn: TypeFunc,
    /// Optional refinements holding for all results (go-cty: `Spec.RefineResult`).
    pub refine_result: Option<RefineResultFunc>,
    /// The implementation (go-cty: `Spec.Impl`).
    pub impl_fn: ImplFunc,
}

impl std::fmt::Debug for Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spec")
            .field("description", &self.description)
            .field("params", &self.params)
            .field("var_param", &self.var_param)
            .finish_non_exhaustive()
    }
}

/// A [`TypeFunc`] that always returns the given type
/// (go-cty: `function.StaticReturnType`).
pub fn static_return_type(ty: Type) -> TypeFunc {
    Box::new(move |_args| Ok(ty.clone()))
}

/// A callable cty function (go-cty: `function.Function`).
#[derive(Debug, Clone)]
pub struct Function {
    _priv: (),
}

impl Function {
    /// Builds a function from its specification (go-cty: `function.New`).
    pub fn new(spec: Spec) -> Function {
        let _ = spec;
        todo!()
    }

    /// The return type for arguments of the given types
    /// (go-cty: `Function.ReturnType`).
    pub fn return_type(&self, arg_types: &[Type]) -> Result<Type, Error> {
        let _ = arg_types;
        todo!()
    }

    /// The return type for the given argument values
    /// (go-cty: `Function.ReturnTypeForValues`).
    pub fn return_type_for_values(&self, args: &[Value]) -> Result<Type, Error> {
        let _ = args;
        todo!()
    }

    /// Calls the function with the given arguments (go-cty: `Function.Call`).
    pub fn call(&self, args: &[Value]) -> Result<Value, Error> {
        let _ = args;
        todo!()
    }

    /// The positional parameters (go-cty: `Function.Params`).
    pub fn params(&self) -> Vec<Parameter> {
        todo!()
    }

    /// The variadic parameter specification, if any (go-cty: `Function.VarParam`).
    pub fn var_param(&self) -> Option<Parameter> {
        todo!()
    }

    /// The function's description (go-cty: `Function.Description`).
    pub fn description(&self) -> String {
        todo!()
    }

    /// A copy of this function with the descriptions replaced; `param_descs`
    /// must either be empty or have one entry per parameter, including the
    /// variadic parameter if any (go-cty: `Function.WithNewDescriptions`).
    pub fn with_new_descriptions(&self, func_desc: &str, param_descs: &[&str]) -> Function {
        let _ = (func_desc, param_descs);
        todo!()
    }
}

/// Wraps a function so that its results are always unknown (though correctly
/// typed), for analysis-only evaluation (go-cty: `function.Unpredictable`).
pub fn unpredictable(f: Function) -> Function {
    let _ = f;
    todo!()
}

/// Creates an argument error blaming the argument at the given index
/// (go-cty: `function.NewArgErrorf` / `function.NewArgError`).
pub fn new_arg_error(index: usize, message: impl Into<String>) -> Error {
    let _ = (index, message.into());
    todo!()
}
