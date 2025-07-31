pub mod Chip8;
pub mod Helpers;
pub mod NonBlockingReader;
pub mod Ram;

pub trait Restart {
    fn restart(&mut self) -> ();
}
