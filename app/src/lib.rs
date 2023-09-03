#![allow(unused_must_use, non_upper_case_globals)]

use std::str::FromStr;
use sysinfo::Signal::*;
use sysinfo::{
    CpuExt, NetworkExt, NetworksExt, Pid, ProcessExt, Signal, System, SystemExt, UserExt,
};
use log::{info, warn};
use std::fs;

const signals: &[Signal] = &[
    Hangup,
    Interrupt,
    Quit,
    Illegal,
    Trap,
    Abort,
    Bus,
    FloatingPointException,
    Kill,
    User1,
    Segv,
    User2,
    Pipe,
    Alarm,
    Term,
    Child,
    Continue,
    Stop,
    TSTP,
    TTIN,
    TTOU,
    Urgent,
    XCPU,
    XFSZ,
    VirtualAlarm,
    Profiling,
    Winch,
    IO,
    Power,
    Sys,
];

pub fn print_help(mut writer: impl std::io::Write) {
    writeln!(writer, "== Help menu ==");
    writeln!(writer, "help               : show this menu");
    writeln!(
        writer,
        "signals            : show the available signals"
    );
    writeln!(
        writer,
        "refresh            : reloads all processes' information"
    );
    writeln!(
        writer,
        "refresh [pid]      : reloads corresponding process' information"
    );
    writeln!(
        writer,
        "refresh_disks      : reloads only disks' information"
    );
    writeln!(
        writer,
        "refresh_users      : reloads only users' information"
    );
    writeln!(
        writer,
        "show [pid | name]  : show information of the given process \
         corresponding to [pid | name]"
    );
    writeln!(
        writer,
        "kill [pid] [signal]: send [signal] to the process with this \
         [pid]. 0 < [signal] < 32"
    );
    writeln!(
        writer,
        "cpus               : Displays CPUs state"
    );
    writeln!(
        writer,
        "memory             : Displays memory state"
    );
    writeln!(
        writer,
        "temperature        : Displays components' temperature"
    );
    writeln!(
        writer,
        "disks              : Displays disks' information"
    );
    writeln!(
        writer,
        "network            : Displays network' information"
    );
    writeln!(
        writer,
        "all                : Displays all process name and pid"
    );
    writeln!(
        writer,
        "uptime             : Displays system uptime"
    );
    writeln!(
        writer,
        "boot_time          : Displays system boot time"
    );
    writeln!(
        writer,
        "vendor_id          : Displays CPU vendor id"
    );
    writeln!(writer, "brand              : Displays CPU brand");
    writeln!(
        writer,
        "load_avg           : Displays system load average"
    );
    writeln!(
        writer,
        "frequency          : Displays CPU frequency"
    );
    writeln!(writer, "users              : Displays all users");
    writeln!(
        writer,
        "system             : Displays system information (such as name, version and hostname)"
    );
    writeln!(
        writer,
        "pid                : Display this example's PID"
    );
    writeln!(writer, "quit               : Exit the program");
}

// pub fn check_networking() {
//     log!("");    
// }

pub fn check_os() {
    let data = fs::read_to_string("/etc/os-release").expect("Unable to read file");
    println!("{}", data);
    println!("info log");
    info!("{}", data);
}

pub fn system_info(input: &str, sys: &mut System, mut writer: impl std::io::Write) -> bool {
    match input.trim() {
        "help" => print_help(writer),
        "refresh_disks" => {
            writeln!(writer, "Refreshing disk list...");
            sys.refresh_disks_list();
            writeln!(writer, "Done.");
        }
        "refresh_users" => {
            writeln!(writer, "Refreshing user list...");
            sys.refresh_users_list();
            writeln!(writer, "Done.");
        }
        "signals" => {
            let mut nb = 1i32;

            for sig in signals {
                writeln!(writer, "{:2}:{:?}", nb, sig);
                nb += 1;
            }
        }
        "cpus" => {
            // Note: you should refresh a few times before using this, so that usage statistics
            // can be ascertained
            writeln!(
                writer,
                "number of physical cores: {}",
                sys.physical_core_count()
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "Unknown".to_owned()),
            );
            writeln!(
                writer,
                "total process usage: {}%",
                sys.global_cpu_info().cpu_usage()
            );
            for proc_ in sys.cpus() {
                writeln!(writer, "{:?}", proc_);
            }
        }
        "memory" => {
            writeln!(writer, "total memory: {} KB", sys.total_memory());
            writeln!(writer, "used memory : {} KB", sys.used_memory());
            writeln!(writer, "total swap  : {} KB", sys.total_swap());
            writeln!(writer, "used swap   : {} KB", sys.used_swap());
        }
        "quit" | "exit" => return true,
        "all" => {
            for (pid, proc_) in sys.processes() {
                writeln!(
                    writer,
                    "{}:{} status={:?}",
                    pid,
                    proc_.name(),
                    proc_.status()
                );
            }
        }
        "frequency" => {
            writeln!(
                writer,
                "{} MHz",
                sys.global_cpu_info().frequency()
            );
        }
        "vendor_id" => {
            writeln!(
                writer,
                "vendor ID: {}",
                sys.cpus()[0].vendor_id()
            );
        }
        "brand" => {
            writeln!(writer, "brand: {}", sys.cpus()[0].brand());
        }
        "load_avg" => {
            let load_avg = sys.load_average();
            writeln!(writer, "one minute     : {}%", load_avg.one);
            writeln!(writer, "five minutes   : {}%", load_avg.five);
            writeln!(writer, "fifteen minutes: {}%", load_avg.fifteen);
        }
        e if e.starts_with("show ") => {
            let tmp: Vec<&str> = e.split(' ').collect();

            if tmp.len() != 2 {
                writeln!(
                    writer,
                    "show command takes a pid or a name in parameter!"
                );
                writeln!(writer, "example: show 1254");
            } else if let Ok(pid) = Pid::from_str(tmp[1]) {
                match sys.process(pid) {
                    Some(p) => writeln!(writer, "{:?}", *p),
                    None => writeln!(writer, "pid \"{:?}\" not found", pid),
                };
            } else {
                let proc_name = tmp[1];
                for proc_ in sys.processes_by_name(proc_name) {
                    writeln!(writer, "==== {} ====", proc_.name());
                    writeln!(writer, "{:?}", proc_);
                }
            }
        }
        "temperature" => {
            for component in sys.components() {
                writeln!(writer, "{:?}", component);
            }
        }
        "network" => {
            for (interface_name, data) in sys.networks().iter() {
                writeln!(
                    writer,
                    "{}:\n  input data  (new / total): {} / {} B\n  output data (new / total): {} / {} B",
                    interface_name,
                    data.received(),
                    data.total_received(),
                    data.transmitted(),
                    data.total_transmitted(),
                );
            }
        }
        "show" => {
            writeln!(
                writer,
                "'show' command expects a pid number or a process name"
            );
        }
        e if e.starts_with("kill ") => {
            let tmp: Vec<&str> = e.split(' ').collect();

            if tmp.len() != 3 {
                writeln!(
                    writer,
                    "kill command takes the pid and a signal number in parameter!"
                );
                writeln!(writer, "example: kill 1254 9");
            } else {
                let pid = Pid::from_str(tmp[1]).unwrap();
                let signal = i32::from_str(tmp[2]).unwrap();

                if signal < 1 || signal > 31 {
                    writeln!(
                        writer,
                        "Signal must be between 0 and 32 ! See the signals list with the \
                         signals command"
                    );
                } else {
                    match sys.process(pid) {
                        Some(p) => {
                            if let Some(res) =
                                p.kill_with(*signals.get(signal as usize - 1).unwrap())
                            {
                                writeln!(writer, "kill: {}", res,);
                            } else {
                                writeln!(
                                    writer,
                                    "kill: signal not supported on this platform"
                                );
                            }
                        }
                        None => {
                            writeln!(writer, "pid not found");
                        }
                    };
                }
            }
        }
        "disks" => {
            for disk in sys.disks() {
                writeln!(writer, "{:?}", disk);
            }
        }
        "users" => {
            for user in sys.users() {
                writeln!(writer, "{:?}", user.name());
            }
        }
        "boot_time" => {
            writeln!(writer, "{} seconds", sys.boot_time());
        }
        "uptime" => {
            let up = sys.uptime();
            let mut uptime = sys.uptime();
            let days = uptime / 86400;
            uptime -= days * 86400;
            let hours = uptime / 3600;
            uptime -= hours * 3600;
            let minutes = uptime / 60;
            writeln!(
                writer,
                "{} days {} hours {} minutes ({} seconds in total)",
                days,
                hours,
                minutes,
                up,
            );
        }
        x if x.starts_with("refresh") => {
            if x == "refresh" {
                writeln!(writer, "Getting processes' information...");
                sys.refresh_all();
                writeln!(writer, "Done.");
            } else if x.starts_with("refresh ") {
                writeln!(writer, "Getting process' information...");
                if let Some(pid) = x
                    .split(' ')
                    .filter_map(|pid| pid.parse().ok())
                    .take(1)
                    .next()
                {
                    if sys.refresh_process(pid) {
                        writeln!(writer, "Process `{}` updated successfully", pid);
                    } else {
                        writeln!(
                            writer,
                            "Process `{}` couldn't be updated...",
                            pid
                        );
                    }
                } else {
                    writeln!(writer, "Invalid [pid] received...");
                }
            } else {
                writeln!(
                    writer,
                    "\"{}\": Unknown command. Enter 'help' if you want to get the commands' \
                     list.",
                    x
                );
            }
        }
        "pid" => {
            writeln!(
                writer,
                "PID: {}",
                sysinfo::get_current_pid().expect("failed to get PID")
            );
        }
        "system" => {
            writeln!(
                writer,
                "System name:              {}\n\
                 System kernel version:    {}\n\
                 System OS version:        {}\n\
                 System OS (long) version: {}\n\
                 System host name:         {}",
                sys.name().unwrap_or_else(|| "<unknown>".to_owned()),
                sys.kernel_version()
                    .unwrap_or_else(|| "<unknown>".to_owned()),
                sys.os_version().unwrap_or_else(|| "<unknown>".to_owned()),
                sys.long_os_version()
                    .unwrap_or_else(|| "<unknown>".to_owned()),
                sys.host_name().unwrap_or_else(|| "<unknown>".to_owned()),
            );
        }
        e => {
            writeln!(
                writer,
                "\"{}\": Unknown command. Enter 'help' if you want to get the commands' \
                 list.",
                e
            );
        }
    }
    false
}
