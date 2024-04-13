use chumsky::span::SimpleSpan;
use comfy_types::{Argument, Expr, Statements, Type};
use comfy_utils::inc_indent;

use crate::compiler::Error;

use super::{ComfyType, CompileResult, State};

fn typed_name(st: &mut State, name: &str, ty: &Type, expr: &Expr) -> CompileResult<String> {
    let cty = ty.to_c(st)?;

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
        format!(" = {}", expr.to_c(st)?)
    } else {
        "".to_owned()
    };

    Ok(format!("{}{}{}", type_name, arr_desc, assign_default,))
}

impl ComfyType<String> for Statements {
    fn to_c(&self, st: &mut State) -> CompileResult<String> {
        Ok(match self {
            Statements::ExpressionStatement(e, _) => format!("{};", e.to_c(st)?),
            Statements::LetStatement(name, ty, expr, _) => {
                format!("{};", typed_name(st, name, ty, expr)?)
            }
            Statements::FunctionDeclaration(_access_modifier, name, args, ty, body, s) => {
                let cty = ty.to_c(st)?;

                if cty.1 .0 {
                    // TODO: support arrays
                    return Err(Error::Compile("Cannot return arrays".to_owned(), ty.span()));
                }

                let cargs = args.to_c(st)?;
                let cbody = body.to_c(st)?;

                format!(
                    "{} {}({}) {{\n{}\n}}\n",
                    cty.0,
                    name,
                    cargs,
                    inc_indent(cbody)
                )
            }
            Statements::ReturnStatement(e, _) => format!("return {};", e.to_c(st)?),
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
}

impl ComfyType<String> for Vec<Statements> {
    fn to_c(&self, st: &mut State) -> CompileResult<String> {
        Ok(self
            .iter()
            .map(|s| s.to_c(st))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"))
    }

    fn span(&self) -> SimpleSpan {
        let start = self.first().unwrap().span().start;
        let end = self.last().unwrap().span().end;

        SimpleSpan::new(start, end)
    }
}

impl ComfyType<String> for Argument {
    fn to_c(&self, st: &mut State) -> CompileResult<String> {
        typed_name(st, &self.0, &self.1, &self.2)
    }

    fn span(&self) -> SimpleSpan {
        self.3
    }
}

impl ComfyType<String> for Vec<Argument> {
    fn to_c(&self, st: &mut State) -> CompileResult<String> {
        Ok(self
            .iter()
            .map(|a| a.to_c(st))
            .collect::<Result<Vec<_>, _>>()?
            .join(", "))
    }

    fn span(&self) -> SimpleSpan {
        let start = self.first().unwrap().span().start;
        let end = self.last().unwrap().span().end;

        SimpleSpan::new(start, end)
    }
}
