use serde::{Deserialize, Serialize};

/// 任务
#[derive(Serialize, Deserialize)]
pub struct Variant {
    /// 节点
    pub node: String,
    /// 任务类型
    #[serde(rename = "missionType")]
    pub mission_type: String,
    /// 状态
    pub modifier: String,
}

/// 突击
#[derive(Serialize, Deserialize)]
pub struct Sortie {
    /// 阵营
    pub faction: String,
    /// 阵营领导者
    pub boss: String,
    /// 结束时间
    pub expiry: String,
    /// 任务
    pub variants: Vec<Variant>,
}
