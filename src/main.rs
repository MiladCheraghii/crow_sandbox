
mod vm_manager;
mod crow_core;


fn main() {
    println!("Welcome to Crow sandbox :)");

    let vm_name = String::from(r"win10x64");
    //vm_manager::start_vm_machine(&path);
	//vm_manager::start_vm_machine(&vm_name);
    crow_core::init();
    crow_core::init_for_any_file("fderfe");
    match crow_core::calculate_hash_file("Cargo.toml") {
    Ok(hash) => println!("Hash: {}", hash),
    Err(e) => println!("Error: {}", e),
    }
}
