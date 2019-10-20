use super::document::Document;
use md5;
use std::error::Error;

#[derive(Debug)]
pub struct Index<'a> {
//    score: f64, // 分数 title *
    pub id: &'a str,
    pub keyword: &'a str,
    pub documents: Vec<&'a str>,
    pub version: usize,
}
// TODO: 索引
// id: 索引id
// keyword: 索引词
// documents: 关联的文档id
impl<'a> Index<'a> {
    /// 初始化
    /// # Examples
    /// ```
    /// let index = Index::new("周杰伦");
    /// ```
    pub fn new(keyword: &str) -> Index {
        let id = match std::str::from_utf8(
            md5::compute(keyword.as_bytes()).as_ref()) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        let documents = Vec::new();
        Index {
            id,
            keyword,
            documents,
            version: 0,
        }
    }

    pub fn insert_doc(&mut self, doc_id: &str) -> Result<(), Box<dyn Error>> {
        self.documents.push(doc_id);
        Ok(())
    }
}