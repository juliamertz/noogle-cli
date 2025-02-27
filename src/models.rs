use fuzzy_matcher::skim;
use serde::{Deserialize, Serialize};

pub type ValuePath = Vec<String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionType {
    Lambda,
    Attribute,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Docs(pub Vec<Doc>);

impl std::ops::Deref for Docs {
    type Target = Vec<Doc>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Docs {
    pub fn get_by_title(&self, title: impl AsRef<str>) -> Option<&Doc> {
        self.iter().find(|e| e.meta.title == title.as_ref())
    }

    pub fn fuzzy_search(&self, query: &str) -> Vec<(&Doc, i64)> {
        let matcher = skim::SkimMatcherV2::default().ignore_case();
        self.iter()
            .filter_map(|item| {
                matcher
                    .fuzzy(&item.meta.title, query, false)
                    .map(|(score, _)| (item, score))
            })
            .collect::<Vec<_>>()
    }

    pub fn fuzzy_search_sorted(&self, query: &str) -> Vec<(&Doc, i64)> {
        let mut filtered = self.fuzzy_search(query);
        filtered.sort_by(|(_, a), (_, b)| b.cmp(a));
        filtered
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Doc {
    pub meta: Meta,
    pub content: Option<ContentSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSource {
    pub content: Option<String>,
    pub source: Option<SourceOrigin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceOrigin {
    pub position: Option<FilePosition>,
    pub path: Option<ValuePath>,
    pub pos_type: Option<PositionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePosition {
    pub file: String,
    pub line: i64,
    pub column: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimopMeta {
    pub name: Option<String>,
    pub args: Option<Vec<String>>,
    pub experimental: Option<bool>,
    pub arity: Option<i32>,
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
