use super::{AnyExpressionEnum, Expression};
#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}
impl Expression for Identifier {
    fn is_assignable(&self) -> bool {
        true
    }
    fn into_any_expression_enum(self) -> super::AnyExpressionEnum {
        super::AnyExpressionEnum::Identifier(self)
    }
    fn desugar(self) -> AnyExpressionEnum {
        self.into_any_expression_enum()
    }
    fn my_clone(&self) -> AnyExpressionEnum {
        Identifier {
            name: self.name.clone()
        }.into_any_expression_enum()
    }
}

