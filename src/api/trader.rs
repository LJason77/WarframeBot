//! 虚空商人

use gettextrs::gettext;

use crate::models::trader::Trader;

use super::{get_cache, get_eta, get_node, get_url, need_update};

pub async fn get_trader() -> String {
    // 读取缓存
    let mut trader = match get_cache::<Trader>("voidTrader").await {
        Ok(trader) => trader,
        Err(err) => return err,
    };

    if need_update(&trader.activation) && !trader.active || need_update(&trader.expiry) {
        trader = match get_url("voidTrader", None).await {
            Ok(trader) => trader,
            Err(err) => return err,
        };
    }

    let mut trader_str = format!("{}\n", get_node(&trader.location));
    if trader.active {
        let mut items = String::new();
        for item in trader.inventory {
            items.push_str(
                format!(
                    "<strong>{}</strong>\n金币：{}  |  现金：{}\n\n",
                    gettext(item.item),
                    item.ducats,
                    item.credits
                )
                .as_str(),
            );
        }
        trader_str.push_str(
            format!("距离虚空商人离开：{}\n\n{}", get_eta(&trader.expiry), items)
                .as_str(),
        );
    } else {
        trader_str.push_str(
            format!("距离虚空商人抵达：{}", get_eta(&trader.activation)).as_str(),
        );
    }

    trader_str
}
