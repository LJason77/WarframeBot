//! 赏金

use gettextrs::gettext;

use crate::models::syndicate::Bounty;

use super::{get_cache, get_eta, get_url, need_update};

/// 读取任务
fn get_job(title: &str, bounty: &Bounty) -> String {
    let mut jobs_str = String::new();
    jobs_str.push_str(format!("{}赏金 | {}\n\n", title, get_eta(&bounty.expiry)).as_str());
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
async fn read_cache(syndicate: &str) -> Result<Bounty, String> {
    let mut bounties = match get_cache::<Vec<Bounty>>("syndicateMissions").await {
        Ok(bounties) => bounties,
        Err(err) => return Err(err),
    };

    let mut bounty = bounties.into_iter().find(|bounty| bounty.syndicate == syndicate).unwrap();
    if need_update(&bounty.expiry) {
        bounties = match get_url::<Vec<Bounty>>("syndicateMissions", None).await {
            Ok(bounties) => bounties,
            Err(err) => return Err(err),
        };
        bounty = bounties.into_iter().find(|bounty| bounty.syndicate == syndicate).unwrap();
    }
    Ok(bounty)
}

/// 希图斯赏金
pub async fn get_cetus() -> String {
    let cetus = match read_cache("Ostrons").await {
        Ok(cetus) => cetus,
        Err(err) => return err,
    };

    get_job("希图斯", &cetus)
}

/// 福尔图娜赏金
pub async fn get_fortuna() -> String {
    let fortuna = match read_cache("Solaris United").await {
        Ok(fortuna) => fortuna,
        Err(err) => return err,
    };

    get_job("福尔图娜", &fortuna)
}

/// 殁世幽都赏金
pub async fn get_necralisk() -> String {
    let necralisk = match read_cache("Entrati").await {
        Ok(necralisk) => necralisk,
        Err(err) => return err,
    };

    get_job("殁世幽都", &necralisk)
}
