#[test]
fn test_converter() {
    assert_eq!(1, computer_start());
    assert_eq!(1, computer_shutdown());
}
mod processors; //Processors is a folder with two modules cpu and gpu. We declare cpu and gpu in processors/mod.rs as pub.
mod psu; //PSU is in a file psu.rs. We declare the moduel as a submodule in this file
use processors::cpu::Process;
use processors::gpu::Process as GraphicalProcess; //Alias since both are named similarly in their individual module
use psu::power_on; // It is finally used here

pub fn computer_start() -> u32 {
    let first_process = Process { pid: 23 };
    first_process.pass_process();
    let _graphic_render = GraphicalProcess { pid: 34 };
    power_on();
    1
}

pub fn computer_shutdown() -> u32 {
    psu::power_off();
    1
}
