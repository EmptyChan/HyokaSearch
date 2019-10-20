use super::index::Index;
use super::document::Document;
use super::search::Search;
use super::constants::*;

#[derive(Debug)]
pub struct HyokaEngine {
    // TODO: 引擎
    // 处理内容
    // 1. 创建倒排索引文件（以名词为倒排【初期主要是人名实体】）
    // 2. 索引文件每秒聚合为大文件，即人名实体作为单个文件的聚合大文件，其他时候为分片小文件（类似于 cache）
    // 3. 分片小文件存在，索引才会检查并聚合，分片小文件存储当秒 PUT 的文档请求处理后的结果，即（周杰伦：{1:score}）
    // 4. 可以批量上传多个文档，但是有限制，防止单秒处理不过来分片。
    // 5. 名词聚合索引大文件，内容：文件名是人名 md5 值，内容为doc id 以及对应的 score
    // 6. score 的分值以 title累积 4分，description 累积 2 分，内容累积 1 分。然后 title 出现的次数累积 2 倍，
    // 描述累积 1 倍，内容累积 0.5 倍
    // 7. 最后的结果通过 timestamp 时间戳来倒序
}