/* src/api/update.rs */

use crate::constants::API_UPDATE_URL;
use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub date: String,
    pub changes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    code: i32,
    data: UpdateData,
}

#[derive(Debug, Deserialize)]
struct UpdateData {
    version: String,
    date: String,
    changelog: String,
}

pub fn fetch_update_info() -> Result<UpdateInfo> {
    let response = reqwest::blocking::get(API_UPDATE_URL)
        .and_then(|r| r.json::<ApiResponse>())
        .map_err(|e| crate::error::CatoolsError::ApiError(format!("获取更新信息失败: {}", e)))?;

    if response.code != 200 {
        return Err(crate::error::CatoolsError::ApiError(
            "API 返回错误".to_string(),
        ));
    }

    // Parse changelog into list
    let changes: Vec<String> = response
        .data
        .changelog
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .collect();

    Ok(UpdateInfo {
        version: response.data.version,
        date: response.data.date,
        changes,
    })
}
