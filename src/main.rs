use std::io;
use std::io::prelude::*;
use std::env;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

//TODO: REFORMAT DESIGN -- Semi Done
//TODO: Add More. 

fn pause() {
    //credit for pause function goes to 'DroidLogician' on users.rust-lang.org 
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}


fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("\n");
    println!("\n");
    println!("\n");
    println!("------------------------------------------------");
    println!("   Hello {:?}                                   ", sys.host_name());
    println!("   Os Type: {}                                  ", env::consts::OS);
    println!("   Kernel Version: {:?}                         ", sys.kernel_version());
    println!("   Processors: {}                               ", sys.processors().len());
    println!("   Total Memory: {} KB                          ", sys.total_memory());
    println!("   Total Swap: {} KB                            ", sys.total_swap());
    println!("------------------------------------------------");
    println!("\n");
    println!("\n");
    println!("\n");
    pause();
}
