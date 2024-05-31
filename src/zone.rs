/*
 * MIT License
 *
 * Copyright (c) 2024 Marcos Gutiérrez Alonso
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub content: String,
}

impl Record {
    pub async fn update_ip(
        &self,
        client: &reqwest::Client,
        zone_id: &str,
        new_ip: &str,
        auth_email: &str,
        auth_key: &str,
    ) -> bool {
        match client
            .patch(format!(
                "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                zone_id, self.id
            ))
            .header(CONTENT_TYPE, "application/json")
            .header("X-Auth-Email", auth_email)
            .header("X-Auth-Key", auth_key)
            .body(format!(
                "{{\"content\": \"{}\", \"name\": \"{}\", \"type\": \"A\"}}",
                new_ip, self.name
            ))
            .send()
            .await
        {
            Err(_) => false,
            Ok(v) => v.error_for_status().is_ok(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
}

impl Zone {
    pub async fn get_records(
        &self,
        client: &reqwest::Client,
        auth_email: &str,
        auth_key: &str,
    ) -> Result<Vec<Record>, reqwest::Error> {
        let records: ApiResult<Record> = client
            .get(format!(
                "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
                self.id
            ))
            .header(CONTENT_TYPE, "application/json")
            .header("X-Auth-Email", auth_email)
            .header("X-Auth-Key", auth_key)
            .send()
            .await?
            .json::<ApiResult<Record>>()
            .await?;

        Ok(records.result)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ApiResult<T> {
    result: Vec<T>,
}

pub async fn get_all(
    client: &reqwest::Client,
    auth_email: &str,
    auth_key: &str,
) -> Result<Vec<Zone>, reqwest::Error> {
    let zones: ApiResult<Zone> = client
        .get("https://api.cloudflare.com/client/v4/zones")
        .header(CONTENT_TYPE, "application/json")
        .header("X-Auth-Email", auth_email)
        .header("X-Auth-Key", auth_key)
        .send()
        .await?
        .json::<ApiResult<Zone>>()
        .await?;

    Ok(zones.result)
}

impl<T> IntoIterator for ApiResult<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.result.into_iter()
    }
}
