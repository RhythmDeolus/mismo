use super::Statement;
#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters_list: Vec<Box<dyn Statement>>,
    pub body: Box<dyn Statement>,
}
impl Statement for FunctionDeclaration {
    fn desugar(&self) -> Box<dyn Statement> {
        let body = self.body.desugar();
        Box::new(FunctionDeclaration{
            name: self.name.clone(),
            parameters_list: self.parameters_list.iter().map(|x| x.desugar()).collect(),
            body
        })
    }
}
