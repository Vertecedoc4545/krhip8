use krhip8::Chip8::Chip8;
use std::fs::read;
use std::thread::sleep;
use std::time::Duration;
use std::env::args;

fn main() {
    //print!("\x1B[2J\x1B[H");
    let mut ibmromname = "/home/edwjuaard/Downloads/test_opcode.ch8".to_string();

    if let Some(f) = args().nth(1) {
        ibmromname = f; 
    } else {
    }
        println!("{}",ibmromname);
    let ibmrom = read(ibmromname).unwrap();

    let mut chip8: Chip8 = Default::default();

    for i in 0..ibmrom.len() {
        chip8.ram[i + 0x200] = ibmrom[i];
    }

    loop {
        let _ = chip8.handle_input(false);
//        sleep(Duration::from_millis(1));
        chip8.do_cycle();
        chip8.render();
        //chip8.clear_keys();
    } 
    
}
