mod crow_core;
mod vm_manager;

fn main() {
    println!("Welcome to Crow sandbox :)");

    let _vm_name = String::from(r"win10x64");
    //vm_manager::start_vm_machine(&path);
    //vm_manager::start_vm_machine(&vm_name);
    crow_core::init();

    match crow_core::calculate_hash_file("Cargo.toml") {
        Ok(hash) => crow_core::init_for_any_file(hash.as_str()),
        Err(e) => println!("Error: {}", e),
    }

    //crow_core::init_for_any_file(hash);
}
