use std::{env, sync::Arc};

use hambands::{band::types::Hertz, search::get_band_for_frequency};
use prometheus_exporter::prometheus::{register_counter, register_counter_vec};
use rbn_lib::RbnClient;

fn main() {
    // Read environment
    let expose_host = env::var("EXPORT_HOST").unwrap_or("0.0.0.0".to_string());
    let expose_port = env::var("EXPORT_PORT").unwrap_or("9814".to_string());
    let endpoint = format!("{}:{}", expose_host, expose_port).parse().unwrap();

    // Set up the prometheus exporter
    println!("Setting up export server on {:?}", endpoint);
    let _exporter = prometheus_exporter::start(endpoint).unwrap();

    // Register data outputs
    let total_spots =
        register_counter!("rbn_spots_total", "Spots on the Reverse Beacon Network").unwrap();
    let spots = register_counter_vec!(
        "rbn_spots_count",
        "Spots on each band",
        &["band", "mode", "spotter"]
    )
    .unwrap();

    // Connect to the network
    println!("Connecting to RBN");
    let mut rbn = RbnClient::new_default_addr("N0CALL".to_string());

    // Handle incoming data
    let _thread = rbn.start(Arc::new(move |data| {
        // Incr the total packet counter
        total_spots.inc();

        // Convert frequency to band
        let frequency_hz: Hertz = (data.frequency * 1000.0) as Hertz;
        let band_name = match get_band_for_frequency(frequency_hz) {
            Ok(x) => x.name,
            Err(_) => "unknown",
        };

        // Increment the appropriate counter
        spots
            .with_label_values(&[band_name, &data.mode, &data.spotter])
            .inc();
    }));

    // Join the thread
    println!("Joining data thread");
    _thread.unwrap().join().unwrap();
}
