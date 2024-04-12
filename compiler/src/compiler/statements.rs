use comfy_types::{Argument, Expr, Statements};

use super::{CompileResult, ToC};

impl ToC<String> for Statements {
    fn to_c(&self) -> CompileResult<String> {
        Ok(match self {
            Statements::ExpressionStatement(e) => format!("{};", e.to_c()?),
            Statements::LetStatement(name, ty, expr) => {
                let cty = ty.to_c()?;

                if cty.1 .0 {
                    let size = match cty.1 .1 {
                        Some(size) => size.to_string(),
                        None => "".to_owned(),
                    };

                    format!("{} {}[{}] = {};", cty.0, name, size, expr.to_c()?)
                } else {
                    format!("{} {} = {};", cty.0, name, expr.to_c()?)
                }
            }
            Statements::FunctionDeclaration(access_modifier, name, args, ty, body) => {
                // let cty = ty.to_c()?;

                todo!()
            }
        })
    }
}

impl ToC<String> for Vec<Statements> {
    fn to_c(&self) -> CompileResult<String> {
        Ok(self
            .iter()
            .map(|s| s.to_c())
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"))
    }
}

impl ToC<String> for Argument {
    fn to_c(&self) -> CompileResult<String> {
        let is_default = if let Expr::Unknown = self.2 {
            true
        } else {
            false
        };

        let ty = self.1.to_c()?;
        let name = &self.0;

        Ok(format!(
            "{} {}{}",
            ty.0,
            name,
            if is_default {
                format!(" = {}", self.2.to_c()?)
            } else {
                "".to_owned()
            }
        ))
    }
}

impl ToC<String> for Vec<Argument> {
    fn to_c(&self) -> CompileResult<String> {
        Ok(self
            .iter()
            .map(|a| a.to_c())
            .collect::<Result<Vec<_>, _>>()?
            .join(", "))
    }
}
