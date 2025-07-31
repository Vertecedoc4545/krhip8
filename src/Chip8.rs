use crate::Helpers::*;
use crate::NonBlockingReader::NonblockingBufReader;
use crate::Ram;
use crate::Restart;
use rand::random;
use stack_stack::Stack;
use std::io::prelude::*;
use std::io::{stdout, Stdout};
use std::write;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Chip8 {
    pc: u16,
    ir: u16,
    pub ram: Ram::Ram,
    v: [u8; 16],
    display: [bool; 2048],
    stack: Stack<u16, 16>,
    pub keys: [u8; 16],
    pub delay_timer: u8,
    pub soud_timer: u8,
    stdin: NonblockingBufReader,
    pub outbuff: RawTerminal<Stdout>,
}

#[derive(Debug)]
enum Chip8Errors {
    UndefinedInstruction,
    StackOverflow,
}

impl Chip8 {
    fn draw_sprite(&mut self, x: usize, y: usize, spheight: u16) -> () {
        let xpos = self.v[x] % 64;
        let ypos = self.v[y] % 32;
        self.v[0xF] = 0;

        let mut row: usize = 0;
        let mut col;

        while row < spheight as usize {
            if row + ypos as usize > 31 {
                break;
            }

            col = 0;

            while col < 8 {
                if col + xpos as usize > 63 {
                    break;
                }

                if ((self.ram[self.ir as usize + row] as u16 >> (7 - col as u16)) & 1) == 1 {
                    if self.display[index(xpos as usize + col, ypos as usize + row)] == true {
                        self.v[0xF] = 1;
                        self.display[index(xpos as usize + col, ypos as usize + row)] = false;
                    } else {
                        self.display[index(xpos as usize + col, ypos as usize + row)] = true;
                    }
                }
                col += 1;
            }
            row += 1;
        }
    }

    fn clear_screen(&mut self) -> () {
        write!(
            self.outbuff,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
        )
        .unwrap();
        self.outbuff.flush().unwrap();
        //} else {
        //    print!("\x1b[0;0H");
        //}
        //println!("somo clean    ");
        //        print!("\x1B[2J\x1B[H");
    }

    pub fn render(&mut self) -> () {
        self.clear_screen();
        let mut scrn: Stack<u8, 22688> = Stack::new();

        for i in 0..2048 {
            if i % 64 == 0 {
                for c in (*b"\x1b[1G").into_iter() {
                    scrn.push(c).unwrap();
                }
                for c in (*b"\n").into_iter() {
                    scrn.push(c).unwrap();
                }
            }

            if self.display[i] {
                for c in (*b"\x1b[47m  \x1b[0m").into_iter() {
                    scrn.push(c).unwrap();
                }
            } else {
                for c in (*b"\x1b[40m  \x1b[0m").into_iter() {
                    scrn.push(c).unwrap();
                }
            }
        }
        write!(
            self.outbuff,
            "{} {}",
            std::str::from_utf8(scrn.as_slice()).unwrap(),
            scrn.len()
        )
        .unwrap();
        self.outbuff.flush().unwrap();
    }

    pub fn do_cycle(&mut self) -> () {
        let op1 = self.ram[self.pc as usize];
        let op2 = self.ram[self.pc as usize + 1];

        self.pc += 2;

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.soud_timer > 0 {
            self.soud_timer -= 1;
            // Beep;
        }

        match self.exec((op1 as u16) << 8 | op2 as u16) {
            Ok(()) => {}
            Err(_) => {
                self.ram.debug();
                panic!(
                    "pc: {} \n, opcode {:#x}",
                    self.pc,
                    (op1 as u16) << 8 | op2 as u16
                );
            }
        }
    }
    pub fn handle_input(&mut self, wait: bool) -> u8 {
        loop {
            if let Ok(Some(key)) = self.stdin.read_char_only_if_data() {
                match key {
                    27 | 3 | 4 => {
                        self.outbuff.suspend_raw_mode().unwrap();
                        let mut i = 0;
                        println!("pressed: {} ", key);
                        for val in self.keys.iter() {
                            println!("key: {:#x} = {}", i, val);
                            i += 1;
                        }
                        std::process::exit(0);
                    }
                    b'1' => {
                        self.keys[1] = 1;
                        //let mut i = 0;
                        //println!("pressed: {} ", key);
                        //for val in self.keys.iter() {
                        //    println!("key: {:#x} = {}", i, val);
                        //    i += 1;
                        //}
                        //std::process::exit(0);

                        return 1;
                    }
                    b'2' => {
                        self.keys[2] = 1;
                        return 2;
                    }
                    b'3' => {
                        self.keys[3] = 1;
                        return 3;
                    }
                    b'4' => {
                        self.keys[0xC] = 1;
                        return 0xC;
                    }
                    b'q' => {
                        self.keys[4] = 1;
                        return 4;
                    }
                    b'w' => {
                        self.keys[5] = 1;
                        return 5;
                    }
                    b'e' => {
                        self.keys[6] = 1;
                        return 6;
                    }
                    b'r' => {
                        self.keys[0xD] = 1;
                        return 0xD;
                    }
                    b'a' => {
                        self.keys[7] = 1;
                        return 7;
                    }
                    b's' => {
                        self.keys[8] = 1;
                        return 8;
                    }
                    b'd' => {
                        self.keys[9] = 1;
                        return 9;
                    }
                    b'f' => {
                        self.keys[0xE] = 1;
                        return 0xE;
                    }
                    b'z' => {
                        self.keys[0xA] = 1;
                        return 0xA;
                    }
                    b'x' => {
                        self.keys[0] = 1;
                        return 0;
                    }
                    b'c' => {
                        self.keys[0xB] = 1;
                        return 0xB;
                    }
                    b'v' => {
                        self.keys[0xF] = 1;
                        return 0xF;
                    }
                    a => {
                        println!("{} : {}\n", a, std::str::from_utf8(&[a]).unwrap());
                        break a;
                    }
                }
            } else if wait {
                continue;
            } else {
                break 0;
            }
        }
    }
    pub fn clear_keys(&mut self) -> () {
        for i in 0..16 {
            self.keys[i] = 0;
        }
    }

    fn exec(&mut self, uc: u16) -> Result<(), Chip8Errors> {
        match bg_id(uc) {
            0x0 => match two_end_id(uc) {
                0xE0 => {
                    self.clear_screen();
                }
                0xEE => {
                    self.pc = self.stack.pop().unwrap();
                }
                _ => {
                    return Err(Chip8Errors::UndefinedInstruction);
                }
            },
            0x1 => {
                self.pc = nnn(uc);
            }
            0x2 => match self.stack.push(self.pc) {
                Ok(()) => {
                    self.pc = nnn(uc);
                }
                Err(_) => {
                    return Err(Chip8Errors::StackOverflow);
                }
            },
            0x3 => {
                if self.v[x(uc)] == kk(uc) as u8 {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.v[x(uc)] != kk(uc) as u8 {
                    self.pc += 2;
                }
            }
            0x5 => {
                if self.v[x(uc)] == self.v[y(uc)] {
                    self.pc += 2;
                }
            }
            0x6 => {
                self.v[x(uc)] = kk(uc) as u8;
            }
            0x7 => {
                self.v[x(uc)] = self.v[x(uc)].wrapping_add(kk(uc) as u8);
            }
            0x8 => match end_id(uc) {
                0x0 => {
                    self.v[x(uc)] = self.v[y(uc)];
                }
                0x1 => {
                    self.v[x(uc)] |= self.v[y(uc)];
                }
                0x2 => {
                    self.v[x(uc)] &= self.v[y(uc)];
                }
                0x3 => {
                    self.v[x(uc)] ^= self.v[y(uc)];
                }
                0x4 => {
                    let sum = self.v[x(uc)] as u16 + self.v[y(uc)] as u16;
                    self.v[x(uc)] = (sum & 0x00FF) as u8;

                    if sum > 255 {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                }
                0x5 => {
                    let (res, bor) = self.v[x(uc)].overflowing_sub(self.v[y(uc)]);
                    let flag = if bor { 0 } else { 1 };

                    self.v[x(uc)] = res;
                    self.v[0xF] = flag;
                }
                0x6 => {
                    let carry = self.v[x(uc)] & 1;
                    self.v[x(uc)] = self.v[x(uc)] >> 1;
                    self.v[0xF] = carry;
                }
                0x7 => {
                    let (res, borr) = self.v[y(uc)].overflowing_sub(self.v[x(uc)]);
                    let flag = if borr { 0 } else { 1 };
                    self.v[x(uc)] = res;
                    self.v[0xF] = flag;
                }
                0x0E => {
                    let carry = self.v[x(uc)] >> 7;
                    self.v[x(uc)] = self.v[x(uc)] << 1;
                    self.v[0xF] = carry;
                }

                _ => {
                    return Err(Chip8Errors::UndefinedInstruction);
                }
            },
            0x9 => {
                if self.v[x(uc)] != self.v[y(uc)] {
                    self.pc += 2;
                }
            }
            0xA => {
                self.ir = nnn(uc);
            }
            0xB => {
                self.pc = nnn(uc) + self.v[0] as u16;
            }
            0xC => {
                self.v[x(uc)] = random::<u8>() & kk(uc);
            }
            0xD => {
                self.draw_sprite(x(uc), y(uc), n(uc));
                //std::process::exit(0);
            }
            0xE => match two_end_id(uc) {
                0x9E => {
                    let key = self.v[x(uc)] as usize;
                    if self.keys[key] == 1 {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    let key = self.v[x(uc)] as usize;
                    if self.keys[key] != 1 {
                        self.pc += 2;
                    }
                }
                _ => {
                    return Err(Chip8Errors::UndefinedInstruction);
                }
            },
            0xF => match two_end_id(uc) {
                0x07 => {
                    self.v[x(uc)] = self.delay_timer;
                }
                0x0A => {
                    self.v[x(uc)] = self.handle_input(true);
                }
                0x15 => {
                    self.delay_timer = self.v[x(uc)];
                }
                0x18 => {
                    self.soud_timer = self.v[x(uc)];
                }
                0x1E => {
                    self.ir += self.v[x(uc)] as u16;
                }
                0x29 => {
                    self.ir = 0x50 + (5 * self.v[x(uc)] as u16); // addres to digit x
                }
                0x33 => {
                    let mut value = self.v[x(uc)];
                    self.ram[self.ir + 2] = value % 10;
                    value /= 10;
                    self.ram[self.ir + 1] = value % 10;
                    value /= 10;
                    self.ram[self.ir] = value % 10;
                }
                0x55 => {
                    for i in 0..=x(uc) {
                        self.ram[self.ir as usize + i] = self.v[i];
                    }
                }
                0x65 => {
                    for i in 0..=x(uc) {
                        self.v[i] = self.ram[self.ir as usize + i];
                    }
                }
                _ => {
                    return Err(Chip8Errors::UndefinedInstruction);
                }
            },

            _ => {
                return Err(Chip8Errors::UndefinedInstruction);
            }
        }
        return Ok(());
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        return Self {
            pc: 0x200,
            ir: 0,
            ram: Default::default(),
            v: [0; 16],
            display: [false; 2048],
            stack: Stack::with_capacity::<16>(),
            keys: [0; 16],
            delay_timer: 0,
            soud_timer: 0,
            stdin: NonblockingBufReader::new(std::io::stdin()),
            outbuff: stdout().into_raw_mode().unwrap(),
        };
    }
}

impl Restart for Chip8 {
    fn restart(&mut self) -> () {
        self.pc = 0x200;
        self.ir = 0;
        self.ram.restart();
        self.v = [0; 16];
        self.display = [false; 2048];
        self.stack = Stack::with_capacity::<16>();
    }
}
