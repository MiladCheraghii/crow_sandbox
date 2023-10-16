// This module is vm's handler that managment every think that is about Vms.

static mut VM_PATH: &str = r"C:\Users\Milad\Documents\Virtual Machines\";

pub fn start_vm(_machine_name: String) {
    let mut vm_name = String::from(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\");
    vm_name.push_str(&_machine_name);
    vm_name.push_str(".vmx");
    let mut vm =
        std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
    vm.arg(r"start");
    vm.arg(vm_name);
    vm.spawn();
}

pub fn stop_vm(_machine_name: String) {
    let mut vm_name = String::from(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\");
    vm_name.push_str(&_machine_name);
    vm_name.push_str(".vmx");
    let mut vm =
        std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");

    vm.arg(r"stop");
    vm.arg(vm_name);
    vm.spawn();
}
/*
pub fn check_vm_status(_machine_name: String) {
    let mut vm_name = String::from(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\");
    vm_name.push_str(&_machine_name);
    vm_name.push_str(".vmx");
    let mut vm =
        std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
    vm.arg(r"start");
    vm.arg(vm_name);
    vm.spawn();
}*/

pub fn revert_to_clean_state(_machine_name: String) {
    let mut vm_name = String::from(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\");
    vm_name.push_str(&_machine_name);
    vm_name.push_str(".vmx");
    let mut vm =
        std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
    vm.arg(r"revertToSnapshot");
    vm.arg(vm_name);
    vm.spawn();
}

pub fn total_runing_vm() {
    let _vm = std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun")
        .arg("list")
        .arg("-l")
        .spawn();
}

pub fn list_all_snapshots(_machine_name: String) {
    let mut vm_name = String::from(r"C:\Users\Milad\Documents\Virtual Machines\win10x64\");
    vm_name.push_str(&_machine_name);
    vm_name.push_str(".vmx");
    let mut vm =
        std::process::Command::new(r"C:\Program Files (x86)\VMware\VMware Workstation\vmrun");
    vm.arg(r"listSnapshots");
    vm.arg(vm_name);
    vm.spawn();
}

// we need that knows vms state for any operation.
// all fn must have a error handling
