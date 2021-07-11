//! 赏金

use gettextrs::gettext;

use crate::api::{self, update_cache};
use crate::models::syndicate::Bounty;

/// 读取任务
fn get_job(title: &str, bounty: &Bounty) -> String {
    let mut jobs_str = String::new();
    jobs_str.push_str(
        format!("{}赏金 | {}\n\n", title, api::get_eta(&bounty.expiry)).as_str(),
    );
    for (i, job) in bounty.jobs.iter().enumerate() {
        let mut reward_str = String::new();
        for reward in &job.reward_pool {
            reward_str.push_str(format!("    {}\n", gettext(reward)).as_str());
        }
        jobs_str.push_str(
            format!(
                "<strong>{}. {}</strong> | {}-{}\n{}\n",
                i + 1,
                gettext(&job.mission_type),
                job.enemy_levels[0],
                job.enemy_levels[1],
                reward_str
            )
            .as_str(),
        );
    }
    jobs_str
}

/// 读取缓存
async fn read_cache(syndicate: &str) -> (String, Bounty) {
    let (mut json, mut bounties) =
        api::get_cache::<Vec<Bounty>>("syndicateMissions").await;

    let mut bounty = bounties
        .into_iter()
        .find(|bounty| bounty.syndicate == syndicate)
        .unwrap();
    if api::need_update(&bounty.expiry) {
        json = api::get_url("syndicateMissions").await;
        bounties = serde_json::from_str(&json).unwrap();
        bounty = bounties
            .into_iter()
            .find(|bounty| bounty.syndicate == syndicate)
            .unwrap();
    }
    (json, bounty)
}

/// 希图斯赏金
pub async fn get_cetus() -> String {
    let (json, cetus) = read_cache("Ostrons").await;

    update_cache(&json, "syndicateMissions");

    get_job("希图斯", &cetus)
}

/// 福尔图娜赏金
pub async fn get_fortuna() -> String {
    let (json, fortuna) = read_cache("Solaris United").await;

    update_cache(&json, "syndicateMissions");

    get_job("福尔图娜", &fortuna)
}

/// 殁世幽都赏金
pub async fn get_necralisk() -> String {
    let (json, necralisk) = read_cache("Entrati").await;

    update_cache(&json, "syndicateMissions");

    get_job("殁世幽都", &necralisk)
}
