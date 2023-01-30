#[derive(Debug)]
pub enum TokenVariant {
    Native,
    Custom
}

#[derive(Debug)]
pub enum TokenType {
    Fungible,
    NonFungible
}

#[derive(Debug)]
pub struct Metadata {
    _type : TokenType,
    total_supply : u64,
    circulating_supply : Option<u64>
}

#[derive(Debug)]
pub struct Token {
    ID : u64,
    name : &'static str,
    variant : TokenVariant,
    symbol : &'static str,
    metadata : Metadata
}

impl Token {
    // pub fn createToken(token_type : TokenType) -> Token {
    //     Token {

    //     }
    // }
}