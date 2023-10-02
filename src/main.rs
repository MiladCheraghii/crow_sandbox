
mod vm_manager;
mod crow_core;


fn main() {
    println!("Welcome to Crow sandbox :)");

    let path = String::from(r"C:\\Users\\Milad\\Documents\\Virtual Machines\\win10x64\\");
    //vm_manager::start_vm_machine(&path);
	vm_manager::stop_vm_machine(&path);
}
