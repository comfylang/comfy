use chumsky::span::SimpleSpan;
use comfy_types::{Argument, Expr, Statements, Type};
use comfy_utils::inc_indent;

use crate::compiler::Error;

use super::{ComfyType, CompileResult, State};

fn typed_name(st: &mut State, name: &str, ty: &Type, expr: &Expr) -> CompileResult<String> {
    let expr_ty = expr.resolve_type(st).unwrap_or_else(|_s| {
        st.errors.push(Error::Compile(
            "Cannot infer type of expression".to_owned(),
            expr.span(),
        ));

        Type::Unknown(expr.span())
    });

    let cty = match ty {
        Type::Unknown(_) => expr_ty.to_cpp(st)?,
        _ => ty.to_cpp(st)?,
    };

    let is_default = if let Expr::Unknown = expr {
        false
    } else {
        true
    };

    let type_name = format!("{} {}", cty.0, name);

    let arr_desc = if cty.1 .0 {
        let size = match cty.1 .1 {
            Some(size) => size.to_string(),
            None => "".to_owned(),
        };

        format!("[{}]", size)
    } else {
        "".to_owned()
    };

    let assign_default = if is_default {
        format!(" = {}", expr.to_cpp(st)?)
    } else {
        "".to_owned()
    };

    Ok(format!("{}{}{}", type_name, arr_desc, assign_default,))
}

impl ComfyType<String> for Statements {
    fn to_cpp(&self, st: &mut State) -> CompileResult<String> {
        Ok(match self {
            Statements::ExpressionStatement(e, _) => format!("{};", e.to_cpp(st)?),
            Statements::LetStatement(name, ty, expr, _) => {
                format!("{};", typed_name(st, name, ty, expr)?)
            }
            Statements::FunctionDeclaration(_access_modifier, name, args, ty, body, _s) => {
                let cty = ty.to_cpp(st)?;

                if cty.1 .0 {
                    // TODO: support arrays
                    return Err(Error::Compile("Cannot return arrays".to_owned(), ty.span()));
                }

                let cargs = args.to_cpp(st)?;
                let cbody = body.to_cpp(st)?;

                format!(
                    "{} {}({}) {{\n{}\n}}\n",
                    cty.0,
                    name,
                    cargs,
                    inc_indent(cbody)
                )
            }
            Statements::ReturnStatement(e, _) => format!("return {};", e.to_cpp(st)?),
        })
    }

    fn span(&self) -> SimpleSpan {
        match self {
            Statements::ExpressionStatement(_, s) => *s,
            Statements::LetStatement(_, _, _, s) => *s,
            Statements::FunctionDeclaration(_, _, _, _, _, s) => *s,
            Statements::ReturnStatement(_, s) => *s,
        }
    }

    fn resolve_type(&self, _: &mut State) -> CompileResult<Type> {
        todo!()
    }
}

impl ComfyType<String> for Vec<Statements> {
    fn to_cpp(&self, st: &mut State) -> CompileResult<String> {
        Ok(self
            .iter()
            .map(|s| s.to_cpp(st))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"))
    }

    fn span(&self) -> SimpleSpan {
        let start = self.first().unwrap().span().start;
        let end = self.last().unwrap().span().end;

        SimpleSpan::new(start, end)
    }

    fn resolve_type(&self, _state: &mut State) -> CompileResult<Type> {
        todo!()
    }
}

impl ComfyType<String> for Argument {
    fn to_cpp(&self, st: &mut State) -> CompileResult<String> {
        typed_name(st, &self.0, &self.1, &self.2)
    }

    fn span(&self) -> SimpleSpan {
        self.3
    }

    fn resolve_type(&self, _: &mut State) -> CompileResult<Type> {
        Ok(self.1.clone())
    }
}

impl ComfyType<String> for Vec<Argument> {
    fn to_cpp(&self, st: &mut State) -> CompileResult<String> {
        Ok(self
            .iter()
            .map(|a| a.to_cpp(st))
            .collect::<Result<Vec<_>, _>>()?
            .join(", "))
    }

    fn span(&self) -> SimpleSpan {
        let start = self.first().unwrap().span().start;
        let end = self.last().unwrap().span().end;

        SimpleSpan::new(start, end)
    }

    fn resolve_type(&self, _: &mut State) -> CompileResult<Type> {
        todo!()
    }
}
