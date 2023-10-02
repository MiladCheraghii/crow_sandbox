// This module is vm's handler that managment every think that is about Vms.

use std::process::Command;
use std::str::from_utf8;



pub fn start_vm_machine(machine_name : &str) {
	let mut vmrun_command = std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
	vmrun_command.arg(r"start");
	vmrun_command.arg(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\win10x64.vmx");
	vmrun_command.spawn().expect("vmrun command failed to start");
}

pub fn stop_vm_machine(machine_name : &str) {
	let mut vmrun_command = std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
	vmrun_command.arg(r"stop");
	vmrun_command.arg(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\win10x64.vmx");
	vmrun_command.spawn().expect("vmrun command failed to start");
}

pub fn check_vm_machine_status(machine_name : &str) {}

pub fn revert_to_clean_state_vm_machine(machine_name : &str) {
	let mut vmrun_command = std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
	vmrun_command.arg(r"revertToSnapshot");
	vmrun_command.arg(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\win10x64.vmx");
	vmrun_command.spawn().expect("vmrun command failed to start");
}
	// we need that knows vms state for any operation.
