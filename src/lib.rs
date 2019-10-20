pub mod ast;
pub mod codegen;
pub mod ir;
pub mod resolve;
pub mod primitives;
pub mod tycheck;
pub mod runtime;
pub mod dino_std;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use snafu::{Snafu, ResultExt};

use crate::codegen::CExecutableProgram;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not read '{}': {}", path.display(), source))]
    IOError {
        path: PathBuf,
        source: io::Error,
    },
    #[snafu(display("Parse error while reading '{}': {}", path.display(), source))]
    ParseError {
        path: PathBuf,
        source: ast::ParseError,
    },
    #[snafu(display("In '{}': {}", path.display(), source))]
    DuplicateDecl {
        path: PathBuf,
        source: resolve::DuplicateDecl,
    },
    #[snafu(display("In '{}': {}", path.display(), source))]
    TypeError {
        path: PathBuf,
        source: tycheck::Error,
    },
    #[snafu(display("In '{}': {}", path.display(), source))]
    CodeGenerationError {
        path: PathBuf,
        source: codegen::Error,
    },
}

/// Compiles the given file into executable code
pub fn compile_executable<P: AsRef<Path>>(path: P) -> Result<CExecutableProgram, Error> {
    let path = path.as_ref();
    let input_program = fs::read_to_string(path)
        .with_context(|| IOError {path: path.to_path_buf()})?;
    let program = ast::Program::parse(&input_program)
        .with_context(|| ParseError {path: path.to_path_buf()})?;
    let mut decls = resolve::ProgramDecls::new(program)
        .with_context(|| DuplicateDecl {path: path.to_path_buf()})?;
    insert_prelude(&mut decls);
    let program_ir = tycheck::infer_and_check(&decls)
        .with_context(|| TypeError {path: path.to_path_buf()})?;
    let code = codegen::executable(&program_ir, &decls)
        .with_context(|| CodeGenerationError {path: path.to_path_buf()})?;

    Ok(code)
}

fn insert_prelude(decls: &mut resolve::ProgramDecls) {
    //TODO: Figure out how to do this properly without hard coding things
    use crate::ast::*;

    let prims = &decls.prims;
    let decls = &mut decls.top_level_decls;

    decls.insert_func(Function::new_extern("unit_eq", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Unit},
            FuncParam {name: "right", ty: Ty::Unit},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("print_unit", FuncSig {
        return_type: Ty::Unit,
        params: vec![
            FuncParam {name: "value", ty: Ty::Unit},
        ],
    })).unwrap();

    decls.insert_func(Function::new_extern("bool_eq", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("bool")},
            FuncParam {name: "right", ty: Ty::Named("bool")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bool_and", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("bool")},
            FuncParam {name: "right", ty: Ty::Named("bool")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bool_or", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("bool")},
            FuncParam {name: "right", ty: Ty::Named("bool")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bool_not", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "value", ty: Ty::Named("bool")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("print_bool", FuncSig {
        return_type: Ty::Unit,
        params: vec![
            FuncParam {name: "value", ty: Ty::Named("bool")},
        ],
    })).unwrap();

    decls.insert_method(prims.int(), "eq", Function::new_extern("int__eq", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
            FuncParam {name: "right", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_method(prims.int(), "gt", Function::new_extern("int__gt", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
            FuncParam {name: "right", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_method(prims.int(), "gte", Function::new_extern("int__gte", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
            FuncParam {name: "right", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_method(prims.int(), "lt", Function::new_extern("int__lt", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
            FuncParam {name: "right", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_method(prims.int(), "lte", Function::new_extern("int__lte", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
            FuncParam {name: "right", ty: Ty::Named("int")},
        ],
    })).unwrap();

    decls.insert_method(prims.int(), "add", Function::new_extern("int__add", FuncSig {
        return_type: Ty::Named("int"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
            FuncParam {name: "other", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_method(prims.int(), "sub", Function::new_extern("int__sub", FuncSig {
        return_type: Ty::Named("int"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
            FuncParam {name: "right", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_method(prims.int(), "mul", Function::new_extern("int__mul", FuncSig {
        return_type: Ty::Named("int"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
            FuncParam {name: "right", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_method(prims.int(), "div", Function::new_extern("int__div", FuncSig {
        return_type: Ty::Named("int"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
            FuncParam {name: "right", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_method(prims.int(), "rem", Function::new_extern("int__rem", FuncSig {
        return_type: Ty::Named("int"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
            FuncParam {name: "right", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_method(prims.int(), "neg", Function::new_extern("int__neg", FuncSig {
        return_type: Ty::Named("int"),
        params: vec![
            FuncParam {name: "self", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("print_int", FuncSig {
        return_type: Ty::Unit,
        params: vec![
            FuncParam {name: "value", ty: Ty::Named("int")},
        ],
    })).unwrap();

    decls.insert_func(Function::new_extern("add_real", FuncSig {
        return_type: Ty::Named("real"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("real")},
            FuncParam {name: "right", ty: Ty::Named("real")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("sub_real", FuncSig {
        return_type: Ty::Named("real"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("real")},
            FuncParam {name: "right", ty: Ty::Named("real")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("print_real", FuncSig {
        return_type: Ty::Unit,
        params: vec![
            FuncParam {name: "value", ty: Ty::Named("real")},
        ],
    })).unwrap();

    decls.insert_func(Function::new_extern("add_complex", FuncSig {
        return_type: Ty::Named("complex"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("complex")},
            FuncParam {name: "right", ty: Ty::Named("complex")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("add_real_complex", FuncSig {
        return_type: Ty::Named("complex"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("real")},
            FuncParam {name: "right", ty: Ty::Named("complex")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("add_complex_real", FuncSig {
        return_type: Ty::Named("complex"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("complex")},
            FuncParam {name: "right", ty: Ty::Named("real")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("sub_complex", FuncSig {
        return_type: Ty::Named("complex"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("complex")},
            FuncParam {name: "right", ty: Ty::Named("complex")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("sub_real_complex", FuncSig {
        return_type: Ty::Named("complex"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("real")},
            FuncParam {name: "right", ty: Ty::Named("complex")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("sub_complex_real", FuncSig {
        return_type: Ty::Named("complex"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("complex")},
            FuncParam {name: "right", ty: Ty::Named("real")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("print_complex", FuncSig {
        return_type: Ty::Unit,
        params: vec![
            FuncParam {name: "value", ty: Ty::Named("complex")},
        ],
    })).unwrap();

    decls.insert_func(Function::new_extern("bstr_len", FuncSig {
        return_type: Ty::Named("int"),
        params: vec![
            FuncParam {name: "value", ty: Ty::Named("bstr")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bstr_eq", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("bstr")},
            FuncParam {name: "right", ty: Ty::Named("bstr")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bstr_gt", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("bstr")},
            FuncParam {name: "right", ty: Ty::Named("bstr")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bstr_gte", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("bstr")},
            FuncParam {name: "right", ty: Ty::Named("bstr")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bstr_lt", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("bstr")},
            FuncParam {name: "right", ty: Ty::Named("bstr")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bstr_lte", FuncSig {
        return_type: Ty::Named("bool"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("bstr")},
            FuncParam {name: "right", ty: Ty::Named("bstr")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bstr_concat", FuncSig {
        return_type: Ty::Named("bstr"),
        params: vec![
            FuncParam {name: "left", ty: Ty::Named("bstr")},
            FuncParam {name: "right", ty: Ty::Named("bstr")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bstr_slice", FuncSig {
        return_type: Ty::Named("bstr"),
        params: vec![
            FuncParam {name: "string", ty: Ty::Named("bstr")},
            FuncParam {name: "start", ty: Ty::Named("int")},
            FuncParam {name: "end", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("bstr_get", FuncSig {
        return_type: Ty::Named("bstr"),
        params: vec![
            FuncParam {name: "string", ty: Ty::Named("bstr")},
            FuncParam {name: "index", ty: Ty::Named("int")},
        ],
    })).unwrap();
    decls.insert_func(Function::new_extern("print_bstr", FuncSig {
        return_type: Ty::Unit,
        params: vec![
            FuncParam {name: "value", ty: Ty::Named("bstr")},
        ],
    })).unwrap();

    decls.insert_func(Function::new_extern("read_line_bstr", FuncSig {
        return_type: Ty::Named("bstr"),
        params: Vec::new(),
    })).unwrap();
}
