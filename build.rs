use rkyv::rancor::Error;
use rkyv::{Archive, Serialize};
use serde::Deserialize;
use std::fs;

pub type ValuePath = Vec<String>;

#[derive(Archive, Debug, Clone, Serialize, Deserialize)]
pub enum PositionType {
    Lambda,
    Attribute,
}

#[derive(Archive, Debug, Serialize, Deserialize)]
pub struct Docs(pub Vec<Doc>);

#[derive(Archive, Debug, Serialize, Deserialize)]
pub struct Doc {
    pub meta: Meta,
    pub content: Option<ContentSource>,
}

#[derive(Archive, Debug, Clone, Serialize, Deserialize)]
pub struct ContentSource {
    pub content: Option<String>,
    pub source: Option<SourceOrigin>,
}

#[derive(Archive, Debug, Clone, Serialize, Deserialize)]
pub struct SourceOrigin {
    pub position: Option<FilePosition>,
    pub path: Option<ValuePath>,
    pub pos_type: Option<PositionType>,
}

#[derive(Archive, Debug, Clone, Serialize, Deserialize)]
pub struct FilePosition {
    pub file: String,
    pub line: i64,
    pub column: i64,
}

#[derive(Archive, Debug, Serialize, Deserialize)]
pub struct PrimopMeta {
    pub name: Option<String>,
    pub args: Option<Vec<String>>,
    pub experimental: Option<bool>,
    pub arity: Option<i32>,
}

#[derive(Archive, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
    pub path: ValuePath,
    pub aliases: Option<Vec<ValuePath>>,
    pub is_primop: Option<bool>,
    pub is_functor: Option<bool>,
    pub primop_meta: Option<PrimopMeta>,
    pub attr_position: Option<FilePosition>,
    pub attr_expr: Option<String>,
    pub lambda_position: Option<FilePosition>,
    pub lambda_expr: Option<String>,
    pub count_applied: Option<i64>,
    pub content_meta: Option<SourceOrigin>,
    pub signature: Option<String>,
}

fn main() {
    let data: Docs = serde_json::from_str(include_str!("./data.json")).unwrap();
    let bytes = rkyv::to_bytes::<Error>(&data).unwrap();
    fs::write("./data", bytes).unwrap();
}
