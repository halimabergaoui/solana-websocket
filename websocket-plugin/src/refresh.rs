use std::sync::Arc;
use std::sync::Mutex;
use solana_program::pubkey::Pubkey;
use reqwest::Client;
use std::str::FromStr;
use std::time::Duration;
use reqwest::StatusCode;

pub async fn start_refresher(pubkeys: Arc<Mutex<Vec<Option<Pubkey>>>>) {
    let client = Client::new();

    loop {
        tokio::time::sleep(Duration::from_secs(2)).await;

        let response = client.get("http://localhost:3000/pubkey").send().await;

        if let Ok(response) = response {
            if response.status().is_success() {
                if let Ok(pubkeys_str) = response.text().await {
                    let pubkeys_str = pubkeys_str.trim().split(',').map(|s| s.to_string()).collect::<Vec<String>>();

                    let mut pubkeys_guard = pubkeys.lock().unwrap();
                    pubkeys_guard.clear();
                    for pubkey_str in pubkeys_str {
                        println!("new pubkey {}", pubkey_str);
                        if let Ok(new_pubkey) = Pubkey::from_str(&pubkey_str) {
                            pubkeys_guard.push(Some(new_pubkey));
                            println!("Refreshed pubkey to: {}", new_pubkey);
                        }
                    }
                }
            } else {
                println!("Failed to fetch new pubkeys: HTTP request was not successful");
            }
        } else {
            println!("Failed to fetch new pubkeys: Error making HTTP request");
        }
    }
}
