use colored::Colorize;
use sysinfo::{Disks, RefreshKind, System};

pub fn execute(_args: &[&str]) {
    let mut sys = System::new_all(); // Initialize sysinfo

    let command = "sysinfo"; // This would come from input logic

    match command {
        "sysinfo" => {
            // Refresh system information
            sys.refresh_specifics(RefreshKind::everything());

            // Print system information
            println!("{}", "System Information: ".cyan().bold());

            // Os Info
            println!("OS: {}", System::name().unwrap_or("Unknown".to_string()));
            println!(
                "Host: {}",
                System::host_name().unwrap_or("Unknown".to_string())
            );

            // CPU Info
            let cpu_count = sys.cpus().len();
            let cpu_usage = sys.global_cpu_usage();
            println!(
                "CPUs: {} cores ({}% usage)",
                cpu_count.to_string().green(),
                format!("{:.1}", cpu_usage).yellow()
            );

            // Memory info
            let total_mem = sys.total_memory() / 1024 / 1024;
            let used_mem = sys.used_memory() / 1024 / 1024;
            println!(
                "Memory: {} MB total, {} MB used ({}%used)",
                total_mem.to_string().green(),
                used_mem.to_string().yellow(),
                (used_mem * 100 / total_mem).to_string().red()
            );

            // Disk info
            println!("Disks:");
            let disks = Disks::new_with_refreshed_list();

            {
                for disk in disks.list() {
                    let total_space = disk.total_space() / 1024 / 1024 / 1024;
                    let available_space = disk.available_space() / 1024 / 1024 / 1024;
                    println!(
                        "{} ({}): {} GB free / {} GB total",
                        disk.kind(),
                        disk.name().to_string_lossy(),
                        available_space.to_string().green(),
                        total_space.to_string()
                    );
                }
            }
        }
        _ => {
            println!("{}: '{}'", "Unknown Command".red(), command);
        }
    }
}
