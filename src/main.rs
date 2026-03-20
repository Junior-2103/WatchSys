use sysinfo::{System,Disks,Networks};
use std::process::Command;
// use std::collections::HashMap;

#[derive(Debug)]
struct WiFi{
    name: String,
    received: u64,
    transmitted: u64,
}

struct Disk{
    name: String,
    total: f64,
    used: f64
}

struct Info{
    cpu_usage: f32,
    ram_usage: f64,
    disk_usage: Vec<Disk>,
    wifi_usage: Vec<WiFi>,
    // top_process: HashMap<String, i32>,
}

fn main() {
    if sysinfo::IS_SUPPORTED_SYSTEM{
        let mut sys = System::new_all();
        
        const GIB: f64 = 1_073_741_824.0;
        const GB : f64 = 1_000_000_000.0;

        let unit = GIB;
        
        let mut disks = Disks::new_with_refreshed_list();
        let mut network = Networks::new_with_refreshed_list();
        
        
        print!("{:?}\n",disks.first());
        loop {
            let mut wifi_vec = Vec::new();
            let mut disks_vec = Vec::new();

            sys.refresh_all();
            network.refresh(true);
            disks.refresh(true);
            
            for net in network.list(){
                let net_info = net.1;
                wifi_vec.push(
                    WiFi {
                        name: net.0.to_string(),
                        received: net_info.received(),
                        transmitted: net_info.transmitted(),
                    }
                );
            }

            for disk in disks.list(){
                let disk_usage = disk.total_space().saturating_sub(disk.available_space());
                disks_vec.push(
                    Disk {
                        name: disk.name().to_string_lossy().to_string(),
                        total: disk.total_space() as f64 / unit,
                        used: disk_usage as f64 / unit,
                    }
                );
            }
            
            let info = Info{
                cpu_usage:sys.global_cpu_usage(),
                ram_usage:sys.used_memory() as f64 / unit,
                disk_usage: disks_vec,
                wifi_usage:wifi_vec,
            };

            println!("Uso da cpu: {:.2}%",info.cpu_usage);
            println!("Uso da memória: {:.2}GiB ",info.ram_usage);

            for wifi in info.wifi_usage{
                if wifi.received != 0 && wifi.transmitted != 0 {
                    println!("{} - Recebido: {:?} B/s Enviado: {:?} B/s",wifi.name,wifi.received,wifi.transmitted);
                }
            }

            for disk in info.disk_usage {
                println!("{} - {:.2} / {:.2}",disk.name,disk.used,disk.total)
            }
            

            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

            Command::new("clear").status().unwrap();
        }
    } else {
        println!("Sistema não suportado :(")
    }
}
