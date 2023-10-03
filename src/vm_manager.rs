// This module is vm's handler that managment every think that is about Vms.

use std::process::Command;
use std::str::*;

pub fn start_vm_machine(machine_name : &str) {
	let mut vm = std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
	vm.arg(r"start");
	vm.arg(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\win10x64.vmx");
	vm.spawn().expect("vmrun command failed to start");
}

pub fn stop_vm_machine(machine_name : &str) {
	let mut vm = std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
	vm.arg(r"stop");
	vm.arg(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\win10x64.vmx");
	vm.spawn().expect("vmrun command failed to start");
}

pub fn check_vm_machine_status(machine_name : &str) {}

pub fn revert_to_clean_state_vm_machine(machine_name : &str) {
	let mut vm = std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
	vm.arg(r"revertToSnapshot");
	vm.arg(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\win10x64.vmx");
	vm.spawn().expect("vmrun command failed to start");
}

pub fn total_runing_vm_machine() {
	let vm = std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun").arg("list").arg("-l").spawn();
}

pub fn list_all_machine_snapshots(machine_name : &str) {
	let mut vm = std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
	vm.arg(r"listSnapshots");
	vm.arg(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\win10x64.vmx");
	vm.spawn().expect("vmrun command failed to start");
}
	// we need that knows vms state for any operation.
