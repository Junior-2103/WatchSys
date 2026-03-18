use sysinfo::{System,Disks};
use std::io::Write;

struct Info{
    cpu_usage: f32,
    ram_usage: u64,
}

fn main() {
    if sysinfo::IS_SUPPORTED_SYSTEM{
        let mut sys = System::new_all();
        
        const GIB: u64 = 1_073_741_824;
        const GB : u64 = 1_000_000_000;

        let unit = GB;
        
        loop {
            sys.refresh_all();
            
            let info = Info{
                cpu_usage:sys.global_cpu_usage(),
                ram_usage:sys.used_memory() / unit,
            };

            print!("\rUso da cpu: {:.2}%, Uso da memória: {:.2}GB ",info.cpu_usage,info.ram_usage);
            std::io::stdout().flush().unwrap();
            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    } else {
        println!("Sistema não suportado :(")
    }
}
