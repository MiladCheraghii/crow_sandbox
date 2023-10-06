mod crow_core;
mod vm_manager;
mod arguments_managment;

fn main() {
    arguments_managment::run();

    let _vm_name = String::from(r"win10x64");
    
    crow_core::init();

    match crow_core::calculate_hash_file("src/crow_core.rs") {
        Ok(hash) => crow_core::init_for_any_file(hash.as_str()),
        Err(e) => println!("Error: {}", e),
    }
}