use serde::{Deserialize, Serialize};

/// 新闻
#[derive(Serialize, Deserialize)]
pub struct New {
    /// 标题
    pub message: String,
    /// 链接
    pub link: String,
}
