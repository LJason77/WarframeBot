use serde::{Deserialize, Serialize};

/// 奖励计数
#[derive(Serialize, Deserialize)]
pub struct CountedItems {
    /// 数量
    pub count: i8,
    /// 物品
    pub key: String,
}

/// 奖励
#[derive(Serialize, Deserialize)]
pub struct Reward {
    #[serde(rename = "countedItems")]
    pub counted_items: Vec<CountedItems>,
}

/// 入侵
#[derive(Serialize, Deserialize)]
pub struct Invasion {
    /// 节点
    pub node: String,
    /// 说明
    pub desc: String,
    /// 进度
    pub completion: f32,
    /// 已完成
    pub completed: bool,
    /// 剩余时间
    pub eta: String,
    /// 入侵阵营
    #[serde(rename = "attackingFaction")]
    pub attacking_faction: String,
    /// 入侵奖励
    #[serde(rename = "attackerReward")]
    pub attacker_reward: Reward,
    /// 防守阵营
    #[serde(rename = "defendingFaction")]
    pub defending_faction: String,
    /// 防守奖励
    #[serde(rename = "defenderReward")]
    pub defender_reward: Reward,
}
