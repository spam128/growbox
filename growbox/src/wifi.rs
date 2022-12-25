use std::error::Error;
use wifi::{WiFi, WiFiConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wifi = WiFi::new(WiFiConfig::default())?;

    // Ustaw dane dostępowe do sieci WiFi
    let ssid = "nazwa_sieci";
    let password = "haslo";

    // Połącz się z siecią WiFi
    wifi.connect(ssid, password)?;

    // Sprawdź, czy połączenie zostało nawiązane
    if wifi.is_connected() {
        println!("Połączenie z siecią WiFi zostało nawiązane!");
    } else {
        println!("Nie udało się połączyć z siecią WiFi!");
    }

    Ok(())
}
