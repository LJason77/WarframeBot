//! 午夜电波

use gettextrs::gettext;

use crate::models::nightwave::Nightwave;

use super::{get_cache, get_eta, get_url, need_update};

pub async fn get_nightwave() -> String {
    // 读取缓存
    let mut nightwave = match get_cache::<Nightwave>("nightwave").await {
        Ok(nightwave) => nightwave,
        Err(err) => return err,
    };

    for challenge in nightwave.challenges.clone() {
        if need_update(&challenge.expiry) {
            nightwave = match get_url::<Nightwave>("nightwave", None).await {
                Ok(nightwave) => nightwave,
                Err(err) => return err,
            };
        }
    }

    let mut challenges = String::new();
    for challenge in nightwave.challenges {
        challenges.push_str(
            format!(
                r#"
					<strong>{}  |  {}</strong>
					声望：{}  |  剩余：{}
					"#,
                gettext(&challenge.title),
                gettext(&challenge.desc),
                &challenge.reputation,
                get_eta(&challenge.expiry)
            )
            .as_str(),
        );
    }

    format!("电波剩余时间：{}\n  {}", get_eta(&nightwave.expiry), challenges)
}
