use mc_protocol::{SlpConfig, ping_java, resolve_address};

#[tokio::main]
async fn main() {
    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "mc.hypixel.net".to_string());

    println!("Resolving {address}...");

    match resolve_address(&address).await {
        Ok(resolved) => {
            println!(
                "  -> {} (IP: {}, port: {}, SRV: {})",
                resolved.resolved_host, resolved.ip, resolved.port, resolved.srv_found
            );

            println!("\nPinging via SLP...");
            match ping_java(
                &resolved.ip,
                resolved.port,
                &resolved.hostname,
                &SlpConfig::default(),
            )
            .await
            {
                Ok(response) => {
                    println!("  Online!");
                    println!(
                        "  Version: {} (protocol {})",
                        response.version.name, response.version.protocol
                    );
                    println!(
                        "  Players: {}/{}",
                        response.players.online, response.players.max
                    );
                    println!("  MOTD: {}", response.description.to_plain_text());
                    println!("  Latency: {}ms", response.latency_ms);
                    println!(
                        "  Favicon: {}",
                        if response.favicon.is_some() {
                            "yes"
                        } else {
                            "no"
                        }
                    );

                    if !response.players.sample.is_empty() {
                        println!("  Sample players:");
                        for p in &response.players.sample {
                            println!("    - {} ({})", p.name, p.id);
                        }
                    }
                }
                Err(e) => eprintln!("  SLP failed: {e}"),
            }
        }
        Err(e) => eprintln!("DNS resolution failed: {e}"),
    }
}
