mod ascii;

#[macro_use]
extern crate log;

use env_logger::Builder;
use log::LevelFilter;

use std::{thread, time};
use sysinfo::{Process, ProcessExt, Signal, System, SystemExt};

#[cfg(target_os = "windows")]
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;

#[cfg(target_os = "windows")]
use winapi::shared::ntdef::NT_SUCCESS;

#[cfg(target_os = "windows")]
use sysinfo::Pid;

fn main() {
    let mut log_builder = Builder::from_default_env();

    log_builder.filter(None, LevelFilter::Info).init();

    ascii::logo();

    #[cfg(target_os = "windows")]
    {
        uniq_process("gta-solo.exe");
    }

    #[cfg(not(target_os = "windows"))]
    {
        uniq_process("gta-solo");
    }

    freeze("GTA5.exe", 9);

    info!("Press CTRL+C to close");
    loop {}
}

fn find_process_by_name<'a>(proc_name: &'a str, system: &'a mut System) -> Vec<&'a Process> {
    system.refresh_all();

    let processes = system
        .get_processes()
        .iter()
        .filter(|(_, process)| {
            let process_name = process.name();
            let process_cmds = process.cmd();

            process_name == proc_name || process_cmds.iter().any(|cmd| cmd.ends_with(&proc_name))
        })
        .map(|(_, proc)| proc)
        .collect::<Vec<&Process>>();

    processes
}

#[cfg(not(target_os = "windows"))]
fn stop(process: &Process) -> bool {
    let pid = process.pid();
    info!("Stopping process {:?}", pid);
    let result = process.kill(Signal::Stop);

    if result {
        info!("Process {} stopped", pid);
    } else {
        error!("Failed to stop {}", pid);
    }

    result
}

#[cfg(target_os = "windows")]
fn stop(process: &Process) -> bool {
    use ntapi::ntpsapi::NtSuspendProcess;

    if let Some(handler) = get_process_handler(process.pid()) {
        unsafe {
            let result: bool = NT_SUCCESS(NtSuspendProcess(handler));
            println!("result {:?}", result);
            CloseHandle(handler);
            result
        }
    } else {
        false
    }
}

#[cfg(not(target_os = "windows"))]
fn resume(process: &Process) -> bool {
    let pid = process.pid();

    info!("Resuming process {:?} ⏻", pid);
    let result = process.kill(Signal::Continue);

    if result {
        info!("Process {} resumed successfuly !", pid);
    } else {
        error!("Failed to resume {}", pid);
    }

    result
}

#[cfg(target_os = "windows")]
fn resume(process: &Process) -> bool {
    use ntapi::ntpsapi::NtResumeProcess;

    if let Some(handler) = get_process_handler(process.pid()) {
        unsafe {
            let result = NT_SUCCESS(NtResumeProcess(handler));
            CloseHandle(handler);
            result
        }
    } else {
        false
    }
}

fn freeze(process_name: &str, duration: u64) {
    let mut system = sysinfo::System::new_all();

    info!("Looking up for {} process 🔍", process_name);

    let (ok_processes, failed_processes): (Vec<(&Process, bool)>, Vec<(&Process, bool)>) =
        find_process_by_name(process_name, &mut system)
            .into_iter()
            .inspect(|proc| {
                info!(
                    "Process found for {} with pid ({})",
                    process_name,
                    proc.pid()
                );
            })
            .map(|proc| (proc, stop(&proc)))
            .partition(|(_, suspend_result)| *suspend_result == true);

    if ok_processes.is_empty() && failed_processes.is_empty() {
        error!(
            "Process {} not found. Is {} running ?",
            process_name, process_name
        );
    }

    if !ok_processes.is_empty() {
        let duration = time::Duration::from_secs(duration);
        info!("Sleeping for {:?} 🛏", duration);
        thread::sleep(duration);

        ok_processes.iter().for_each(|(process, _)| {
            resume(process);
        })
    }
}

fn uniq_process(process_name: &str) {
    let mut system = sysinfo::System::new_all();

    if find_process_by_name(process_name, &mut system)
        .iter()
        .any(|process| std::process::id() != (process.pid() as u32))
    {
        error!("Process already running");
        std::process::exit(0x0100);
    }
}

#[cfg(target_os = "windows")]
fn get_process_handler(pid: Pid) -> Option<HANDLE> {
    use winapi::shared::minwindef::{DWORD, FALSE};
    use winapi::um::processthreadsapi::OpenProcess;

    if pid == 0 {
        return None;
    }

    let options = PROCESS_ALL_ACCESS | PROCESS_QUERY_INFORMATION | PROCESS_VM_READ;
    let process_handler = unsafe { OpenProcess(options, FALSE, pid as DWORD) };
    if process_handler.is_null() {
        None
    } else {
        Some(process_handler)
    }
}
