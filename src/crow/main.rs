mod arguments_managment;
mod crow_core;
mod vm_manager;
mod submit_manager;

fn main() {
    let arg_config = arguments_managment::run();

    crow_core::init();

    //println!("{}", arg_config.file_name);

    crow_core::init_for_any_file(String::from(&arg_config.file_name));

    // do it various checks about vm's
    //vm_manager::start_vm(String::from(&arg_config.vm_name));

    // send file to vm
    //
    submit_manager::send_command(String::from(&arg_config.vm_ip));

    let _vm_name = String::from(r"win10x64");
}
