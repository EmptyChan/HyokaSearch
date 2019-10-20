use super::index::Index;
#[derive(Debug)]
pub struct Search<'a> {
    //    score: f64, // 分数 title *
    pub id: &'a str,
}
// TODO: 搜索
// id: 索引id
// keyword: 索引词
// documents: 关联的文档