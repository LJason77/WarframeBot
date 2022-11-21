use serde::{Deserialize, Serialize};

/// 任务
#[derive(Serialize, Deserialize)]
pub struct Mission {
    /// 节点
    pub node: String,
    /// 任务类型
    #[serde(rename = "type")]
    pub mission_type: String,
}

/// 执刑官猎杀
#[derive(Serialize, Deserialize)]
pub struct Archon {
    /// 领导者
    pub boss: String,
    /// 结束时间
    pub expiry: String,
    /// 任务
    pub missions: Vec<Mission>,
}
