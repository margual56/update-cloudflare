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

mod zone;

use clap::Parser;
use colored::Colorize;
use zone::get_all;

async fn get_ip(client: &reqwest::Client) -> Result<String, reqwest::Error> {
    match client
        .get("http://ipv4.icanhazip.com")
        .send()
        .await?
        .text()
        .await
    {
        Err(e) => Err(e),
        Ok(v) => Ok(v.trim().replace("\n", "")),
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    /// Lists all records associated with you account and exits
    list: bool,

    /// List of records (subdomains) to update with your current public IP
    #[arg(short, long)]
    records: Vec<String>,

    #[arg(short, long, env = "CLOUDFLARE_EMAIL")]
    /// Email for account admin
    email: String,

    #[arg(short, long, env = "CLOUDFLARE_KEY")]
    /// Global API key
    key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = reqwest::Client::new();

    if args.list {
        for zone in get_all(&client, &args.email, &args.key).await.unwrap() {
            for record in zone
                .get_records(&client, &args.email, &args.key)
                .await
                .unwrap()
            {
                println!("{}", record.name.yellow());
            }
        }

        return Ok(());
    }

    let record_queue = args.records.as_slice();

    let mut zone_queue: std::collections::HashSet<String> = std::collections::HashSet::new();
    for record in record_queue {
        let spl = record.split(".").collect::<Vec<&str>>();

        zone_queue.insert(spl[spl.len() - 2..].join("."));
    }

    println!("Set of zones to update: {:?}", zone_queue);

    let current_ip = get_ip(&client).await.unwrap();

    for zone in get_all(&client, &args.email, &args.key)
        .await
        .unwrap()
        .into_iter()
        .filter(|zone| zone_queue.contains(&zone.name))
    {
        for record in zone
            .get_records(&client, &args.email, &args.key)
            .await
            .unwrap()
            .into_iter()
            .filter(|record| record_queue.contains(&record.name))
        {
            if record.content == current_ip {
                println!("Record {} is already up to date :)", record.name.yellow());
            } else {
                print!("Updating {}...\t\t", record.name.yellow());

                if !record
                    .update_ip(&client, &zone.id, &current_ip, &args.email, &args.key)
                    .await
                {
                    println!("{}", format!("{}", "ERROR".red().bold()));
                } else {
                    println!("{}", format!("{}", "Success :)".bright_green().italic()));
                }
            }
        }
    }

    Ok(())
}
