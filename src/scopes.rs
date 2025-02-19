use reqwest::Client;
use serde::{Deserialize, Serialize};
use xlsxwriter::*;
use regex::Regex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scope {
    #[serde(rename = "data")]
    data: Vec<Datum>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Datum {
    #[serde(rename = "attributes")]
    attributes: Attributes,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Attributes {
    #[serde(rename = "asset_type")]
    asset_type: String,

    #[serde(rename = "asset_identifier")]
    asset_identifier: String,

    #[serde(rename = "eligible_for_bounty")]
    eligible_for_bounty: bool,

    #[serde(rename = "eligible_for_submission")]
    eligible_for_submission: bool,

}

#[derive(Serialize, Deserialize, Debug)]
struct Assets{
    handler: String,
    asset: String,
}

pub async fn get_all_assets(client: Client, user:&str, key:&str,handle: Vec<String>){
    // let mut scopes:Scope;
    let mut assets:Vec<Assets> = vec![];
    for (i, hand) in handle.iter().enumerate() {
        println!("正在获取第{}个: {:?}", i + 1, hand);
        let url = format!("https://api.hackerone.com/v1/hackers/programs/{}/structured_scopes", hand);
        let resp = client.get(url).basic_auth(user, Some(key)).send().await.unwrap();
        if !resp.status().is_success() {
            eprintln!("{}", resp.status());
        }
        let scopes: Scope = serde_json::from_str(&resp.text().await.unwrap()).unwrap();

        for datum in scopes.data {
            if datum.attributes.eligible_for_bounty && datum.attributes.eligible_for_submission
                && (!datum.attributes.asset_type.eq("WILDCARD") || datum.attributes.asset_identifier != "URL")
                && is_domain_or_wildcard(datum.attributes.asset_identifier.as_str())
            {
                assets.push(Assets {
                    handler: hand.to_string(),
                    asset: datum.attributes.asset_identifier.to_string(),
                });
            }
        }
    }


    println!("正在写入xlsx文件");
    // 创建 Excel 文件
    let workbook = Workbook::new("assets.xlsx").unwrap();
    let mut sheet = workbook.add_worksheet(None).unwrap();
    // 写入表头
    sheet.write_string(0, 0, "Handler", None).unwrap();
    sheet.write_string(0, 1, "Asset", None).unwrap();

    // 遍历 `assets` 并写入 Excel
    for (i, asset) in assets.iter().enumerate() {
        sheet.write_string((i + 1) as u32, 0, &asset.handler, None).unwrap();
        sheet.write_string((i + 1) as u32, 1, &asset.asset, None).unwrap();
    }

    // 关闭 Excel 文件
    workbook.close().unwrap();
}

fn is_domain_or_wildcard(assets: &str) -> bool {
    // 先去掉 "http://" 或 "https://"
    let domain = assets.trim_start_matches("http://")
        .trim_start_matches("https://");

    // 直接排除 "com." 开头的字符串
    if domain.starts_with("com.") {
        return false;
    }

    // 定义域名正则
    let domain_regex = Regex::new(r"^(\*\.)?([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,63}(/.*)?$").unwrap();

    domain_regex.is_match(domain)


    // (domain.starts_with("*.") || domain.contains('.')) && !domain.chars().any(|c| !c.is_ascii_alphanumeric() && c != '.' && c != '-')
}


#[cfg(test)]
mod tests {
    use crate::scopes::is_domain_or_wildcard;

    #[test]
    fn test_is_domain_or_wildcard() {
        println!("{:?}", is_domain_or_wildcard("https://help.glassdoor.com*"));
    }
}