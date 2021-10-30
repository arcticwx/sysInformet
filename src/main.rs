use std::env;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

//TODO: REFORMAT DESIGN
//TODO: Add More. 

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("\n");
    println!("\n");
    println!("\n");
    println!("|------------------------------------------------|");
    println!("|   Hello {:?}                |", sys.host_name());
    println!("|   Os Type: {}                             |", env::consts::OS);
    println!("|   Kernel Version: {:?}                |", sys.kernel_version());
    println!("|   Total Memory: {} KB                     |", sys.total_memory());
    println!("|   Processors: {}                               |", sys.processors().len());
    println!("|   Total Swap: {}                          |", sys.total_swap());
    println!("|------------------------------------------------|");
    println!("\n");
    println!("\n");
    println!("\n");
}
