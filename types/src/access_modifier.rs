use chumsky::span::SimpleSpan;

#[derive(Debug, Clone)]
pub enum AccessModifier {
    Public(SimpleSpan),
    Private(SimpleSpan),
    Protected(SimpleSpan),
}
