use serde::{Deserialize, Serialize};

pub type ValuePath = Vec<String>;

#[derive(Debug, Serialize, Deserialize)]
pub enum PositionType {
    Lambda,
    Attribute,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Doc {
    pub meta: Meta,
    pub content: Option<ContentSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentSource {
    content: Option<String>,
    source: Option<SourceOrigin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceOrigin {
    position: Option<FilePosition>,
    path: Option<ValuePath>,
    pos_type: Option<PositionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePosition {
    file: String,
    line: i64,
    column: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimopMeta {
    name: Option<String>,
    args: Option<Vec<String>>,
    experimental: Option<bool>,
    arity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
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
