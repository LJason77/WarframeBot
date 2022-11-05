use serde::{Deserialize, Serialize};

/// 警报
#[derive(Serialize, Deserialize)]
pub struct Alert {
    /// 标签
    pub tag: String,
    /// 任务
    pub mission: Mission,
    /// 结束时间
    pub expiry: String,
}

/// 任务
#[derive(Serialize, Deserialize)]
pub struct Mission {
    /// 介绍
    pub description: String,
    /// 节点
    pub node: String,
    /// 任务类型
    #[serde(rename = "type")]
    pub mission_type: String,
    /// 派系
    pub faction: String,
    /// 最小等级
    #[serde(rename = "minEnemyLevel")]
    pub min_enemy_level: u16,
    /// 最大等级
    #[serde(rename = "maxEnemyLevel")]
    pub max_enemy_level: u16,
    /// 最大波数
    #[serde(rename = "maxWaveNum")]
    pub max_wave_num: u16,
    /// 奖励
    pub reward: Reward,
}

/// 奖励
#[derive(Serialize, Deserialize)]
pub struct Reward {
    /// 类型
    #[serde(rename = "itemString")]
    pub items: String,
}
