#[derive(Debug)]
pub enum Expression {
    Nil,
    Boolean(bool),
    String(String),
    Array(Vec<Expression>),
    Block(Box<Block>),
    Map(Vec<Block>),
}

#[derive(Debug)]
pub struct Block {
    pub key: String,
    pub value: Expression,
}
