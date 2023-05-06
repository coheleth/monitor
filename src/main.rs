use std::{thread, time};
use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};
// use std::env::{args, Args};

const ESCAPE: char = 27 as char;

fn main_loop(sys: &mut System) {
    // clearscreen::clear().unwrap();
    print!("{esc}[3J{esc}[1;1H", esc = ESCAPE);
    // print!("{}[3J", 27 as char);

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    // Network interfaces name, data received and data transmitted:
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }

    // Components temperature:
    println!("=> components:");
    for component in sys.components() {
        println!("{:?}", component);
    }

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    // println!("System name:             {:?}", sys.name());
    // println!("System kernel version:   {:?}", sys.kernel_version());
    // println!("System OS version:       {:?}", sys.os_version());
    // println!("System host name:        {:?}", sys.host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    // Display processes ID, name na disk usage:
    for (pid, process) in sys.processes() {
        println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }
}

fn main() {
    // let mut args: Args = args();
    let mut sys = System::new_all();
    loop {
        main_loop(&mut sys);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
