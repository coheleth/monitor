extern crate termsize;
extern crate unicode_width;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::{ thread, time };
use prettytable::{Table,format};

use sysinfo::{ NetworkExt, Pid, Process, ProcessExt, System, SystemExt };
use termsize::Size;
use unicode_width::UnicodeWidthStr;

#[macro_use] extern crate prettytable;

const ESCAPE: char = 27 as char;


fn get_terminal_size() -> Option<Size> {
    return termsize::get().map(|size| { return size; });
}

struct Monitor {
    sys: System,
}

impl Monitor {
    pub fn init() -> Monitor {
        let mut sys = System::new_all();
        Monitor { sys }
    }

    pub fn sort_processes(&self, mode: &str) -> Vec<(&Pid, &Process)> {
        let mut v = Vec::from_iter(self.sys.processes());
        if mode == "cpu" {
            v.sort_by(|&(_, a), &(_, b)| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
        }   
        if mode == "mem" {
            v.sort_by(|&(_, a), &(_, b)| b.memory().partial_cmp(&a.memory()).unwrap());
        }
        return v;
    }

    pub fn processes(&mut self) {
        self.sys.refresh_all();
        let cols = get_terminal_size().unwrap().cols;

        println!("{esc}[41m", esc = ESCAPE); // Sets colour to red
        println!("\r{}", " ".to_string().repeat((get_terminal_size().unwrap().cols - 1) as usize));

        print!("\r{esc}[1F ", esc = ESCAPE);

        print!("\rPID");

        print!("\r{esc}[8C ", esc = ESCAPE);

        print!("Process");

        print!("{esc}[{last}G ", esc = ESCAPE, last=cols);
        print!("{esc}[12D ", esc = ESCAPE);

        print!(" CPU%");

        print!("{esc}[{last}G ", esc = ESCAPE, last=cols);
        print!("{esc}[6D ", esc = ESCAPE);

        print!(" MEM%");

        println!("{esc}[0m\r", esc = ESCAPE);

        // print!("{esc}[40m", esc = ESCAPE);

        for (pid, process) in self.sort_processes("cpu").iter().take(5) {
            let p_id: String = pid.to_string();
            let name: String = process.name().to_string();
            let cpu: String = format!("{: >5.2}", process.cpu_usage() / self.sys.cpus().len() as f32);
            let mem: String = format!("{: >5.2}", (process.memory() as f32 / self.sys.total_memory() as f32) * 100.0);
            
            print!("\r{}", " ".to_string().repeat((get_terminal_size().unwrap().cols - 1) as usize));
            print!("\r{}", p_id);

            print!("\r{esc}[8C ", esc = ESCAPE); 

            print!("{}", name);

            print!("{esc}[{last}G ", esc = ESCAPE, last=cols);
            print!("{esc}[12D ", esc = ESCAPE);

            print!("{}", cpu);

            print!("{esc}[{last}G ", esc = ESCAPE, last=cols);
            print!("{esc}[6D ", esc = ESCAPE);

            print!("{}", mem);

            print!("{esc}[1E ", esc = ESCAPE);

        }
    }
}


fn main() {
    let mut moni = Monitor::init();
    print!("{esc}[?25l{esc}[2J{esc}[3J{esc}[1;1H", esc = ESCAPE);
    loop {
        print!("{esc}[3J{esc}[1;1H", esc = ESCAPE);
        // print!("{esc}[2J{esc}[3J{esc}[1;1H", esc = ESCAPE);
        println!("{esc}[0m", esc = ESCAPE);
        println!("{}", moni.sys.name().unwrap());
        moni.processes();
        thread::sleep(time::Duration::from_millis(1500));
    }
}