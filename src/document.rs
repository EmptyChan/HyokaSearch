use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Document<'a> {
    pub doc_id: usize, // 文档 id
    pub title: &'a str,
    pub description: &'a str,
    pub content: &'a str,
}
impl Document<'_> {
    pub fn new(doc_id: usize, title: &str, description: &str, content: &str) -> Document {
        Document {
            doc_id,
            title,
            description,
            content
        }
    }

    pub fn get_title(&self) -> &str {
        self.title
    }

    pub fn get_description(&self) -> &str {
        self.description
    }

    pub fn get_content(&self) -> &str {
        self.content
    }
}