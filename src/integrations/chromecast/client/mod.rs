use {
    super::types::DiscoverRes,
    mdns_sd::{ServiceDaemon, ServiceEvent},
    tokio::time::Duration,
};
const CHROMECAST_MDNS_SERVICE_IDENTIFIER: &str = "_googlecast._tcp.local.";

pub async fn chromecast_discover() -> Result<Vec<DiscoverRes>, ()> {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");
    let receiver = mdns
        .browse(CHROMECAST_MDNS_SERVICE_IDENTIFIER)
        .expect("Failed to browse");
    let mut results = Vec::<DiscoverRes>::with_capacity(10);

    while let Ok(event) = receiver.recv_timeout(Duration::from_millis(1000)) {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                let addresses = info
                    .get_addresses()
                    .iter()
                    .map(|address| address.to_string())
                    .collect::<Vec<_>>();
                println!("Resolved a new service: {}", info.get_fullname());
                println!("IP: {:?}", addresses);
                println!("[Debug]: {:?}", info);
                let dev = DiscoverRes {
                    name: info.get_fullname().to_owned(),
                    location: "".to_owned(),
                    serial: "".to_owned(),
                    mac: "".to_owned(),
                    ipv4: addresses.join(".").to_string(),
                    port: info.get_port().to_string(),
                    services: vec!["".to_string()],
                };
                println!("[Dev]: {:?}", dev);
                results.push(dev);
            }
            ServiceEvent::SearchStarted(_) => {}
            ServiceEvent::ServiceFound(_, _) => {}
            ServiceEvent::ServiceRemoved(_, _) => {}
            ServiceEvent::SearchStopped(_) => {}
        }
    }
    // Gracefully shutdown the daemon.
    std::thread::sleep(std::time::Duration::from_secs(1));
    let _nul = mdns.shutdown().unwrap();
    Ok(results)
}
