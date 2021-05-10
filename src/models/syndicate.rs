use serde::{Deserialize, Serialize};

/// 任务
#[derive(Serialize, Deserialize)]
pub struct Job {
	/// 任务类型
	#[serde(rename = "type")]
	pub mission_type: String,
	/// 奖励池
	#[serde(rename = "rewardPool")]
	pub reward_pool: Vec<String>,
	/// 敌人等级
	#[serde(rename = "enemyLevels")]
	pub enemy_levels: Vec<u8>,
}

/// 赏金
#[derive(Serialize, Deserialize)]
pub struct Bounty {
	/// 剩余时间
	pub expiry: String,
	/// 集团
	pub syndicate: String,
	/// 任务
	pub jobs: Vec<Job>,
}
