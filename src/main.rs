use std::{env, thread::sleep, time::Duration};

fn get_current_ip() -> Option<String> {
    let response = match reqwest::blocking::get("https://api.ipify.org") {
        Ok(r) => r,
        Err(_) => return None,
    };

    match response.status().is_success() {
        true => Some(response.text().unwrap()),
        false => None,
    }
}

fn update_records(api_key: &str, domain: &str, record: &str, current_ip: &str) -> bool {
    let data = serde_json::json!({
        "override": false,
        "zone": [
            {
                "name": record,
                "type": "A",
                "ttl": 300,
                "records": [{"content": current_ip}],
            }
        ],
    });

    let response = match reqwest::blocking::Client::new()
        .put(&format!(
            "https://developers.hostinger.com/api/dns/v1/zones/{}",
            domain,
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(data.to_string())
        .send()
    {
        Ok(resp) => resp,
        Err(_) => return false,
    };

    match response.status().is_success() {
        true => true,
        false => {
            println!(
                "Failed to update DNS records. {}",
                response
                    .text()
                    .unwrap_or_else(|_| "Unknown error".to_string())
            );

            false
        }
    }
}

fn sync_dns(api_key: &str, domain: &str, record: &str) {
    let current_ip = match get_current_ip() {
        Some(ip) => ip,
        None => {
            println!("Failed to retrieve current IP address.");
            return;
        }
    };

    match update_records(api_key, domain, record, &current_ip) {
        true => println!("DNS records updated successfully to {}", current_ip),
        false => println!("Failed to update DNS records."),
    }
}

fn main() {
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let domain = env::var("DOMAIN").expect("DOMAIN not set");
    let record = env::var("RECORD").expect("RECORD not set");
    let commit_hash = env::var("COMMIT_HASH").unwrap_or_else(|_| "unknown".to_string());

    println!("version: {}", commit_hash);
    println!("Starting DNS sync...");

    loop {
        sync_dns(&api_key, &domain, &record);

        sleep(Duration::from_secs(300));
    }
}
