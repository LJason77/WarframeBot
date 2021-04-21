use serde::{Deserialize, Serialize};

/// 挑战
#[derive(Serialize, Deserialize)]
pub struct Challenge {
	/// 标题
	pub title: String,
	/// 描述
	pub desc: String,
	/// 声望
	pub reputation: i16,
	/// 结束时间
	#[serde(rename = "expiry")]
	pub expiry: String,
}

/// 午夜电波
#[derive(Serialize, Deserialize)]
pub struct Nightwave {
	/// 剩余时间
	pub expiry: String,
	/// 挑战
	#[serde(rename = "activeChallenges")]
	pub challenges: Vec<Challenge>,
}
