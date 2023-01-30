use crate::tokens::_tokens::{Token, TokenType, TokenVariant, Metadata};
use crate::accounts::_accounts::Account;

pub const NATIVE_TOKEN : Token = Token {
    ID : 34,
    name : "Xeleron",
    variant : TokenVariant::Native,
    symbol : "XEN",
    metadata : Metadata {
        _type : TokenType::Fungible,
        total_supply : 100_000_000,
        circulating_supply : None
    }
};

pub const NATIVE_ACCOUNT : Account = Account {
    pub_key : "",
    sec_key : "",
    storage : None
};

pub const MAGIC_KEY : &'static str = "Xeleron";