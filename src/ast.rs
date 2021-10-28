#[derive(Debug)]
pub enum Expression {
    String(String),
    Array(Vec<Expression>),
    Block(Box<Block>),
    Boolean(bool),
}

#[derive(Debug)]
pub struct Block {
    pub key: String,
    pub value: Expression,
}
