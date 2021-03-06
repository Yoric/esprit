#![recursion_limit="32"]
extern crate unjson;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate joker;
extern crate easter;

mod tag;
pub mod error;
pub mod result;
mod stmt;
mod expr;
mod id;
mod node;
mod fun;
mod patt;
mod obj;
mod decl;
mod prog;
mod lit;
mod util;

use serde::de::Error;
use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use easter::stmt::Script;
use unjson::ty::Object;
pub use prog::IntoScript;
use util::Serialization;

pub struct ESTreeScript(Script);

impl<'a> Deserialize<'a> for ESTreeScript {
    fn deserialize<D>(de: D) -> ::std::result::Result<Self, D::Error> where D: Deserializer<'a> {
        let json: Object = Deserialize::deserialize(de)?;
        match json.into_script() {
            Ok(script) => Ok(ESTreeScript(script)),
            Err(err)   => Err(D::Error::custom(&format!("{}", err)[..]))
        }
    }
}

impl Serialize for ESTreeScript {
    fn serialize<S: Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
        let json = json!({
            "type": "Program",
            "sourceType": "script",
            "loc": null,
            "body": Serialization::new(&self.0.items),
        });
        json.serialize(serializer)
    }
}

/*
pub struct ESTreeStmt(Stmt);

impl Deserialize for ESTreeStmt {
    fn deserialize<D: Deserializer>(de: &mut D) -> ::std::result::Result<Self, D::Error> {
        let json: Object = Deserialize::deserialize(de)?;
        match json.into_stmt() {
            Ok(stmt) => Ok(ESTreeStmt(stmt)),
            Err(err) => Err(D::Error::syntax(&format!("{}", err)[..]))
        }
    }
}

pub struct ESTreeExpr(Expr);

impl Deserialize for ESTreeExpr {
    ...
}
*/

