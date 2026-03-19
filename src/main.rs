use sysinfo::{System,Networks};
use std::process::Command;

#[derive(Debug)]
struct WiFi{
    name: String,
    received: u64,
    transmitted: u64,
}

struct Info{
    cpu_usage: f32,
    ram_usage: f64,
    wifi_usage: Vec<WiFi>,
}

fn main() {
    if sysinfo::IS_SUPPORTED_SYSTEM{
        let mut sys = System::new_all();
        
        const GIB: f64 = 1_073_741_824.0;
        const GB : f64 = 1_000_000_000.0;

        let unit = GIB;
        
        let mut network = Networks::new_with_refreshed_list();
        
        loop {
            let mut wifi = Vec::new();

            sys.refresh_all();
            network.refresh(true);
            
            for net in network.list(){
                let net_info = net.1;
                wifi.push(
                    WiFi {
                        name: net.0.to_string(),
                        received: net_info.received(),
                        transmitted: net_info.transmitted(),
                    }
                );
            }
            
            let info = Info{
                cpu_usage:sys.global_cpu_usage(),
                ram_usage:sys.used_memory() as f64 / unit,
                wifi_usage:wifi,
            };

            println!("Uso da cpu: {:.2}%",info.cpu_usage);
            println!("Uso da memória: {:.2}GiB ",info.ram_usage);

            for wifi in info.wifi_usage{
                if wifi.received != 0 && wifi.transmitted != 0 {
                    println!("{} - Recebido: {:?} B/s Enviado: {:?} B/s",wifi.name,wifi.received,wifi.transmitted);
                }
            }
            

            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

            Command::new("clear").status().unwrap();
        }
    } else {
        println!("Sistema não suportado :(")
    }
}
