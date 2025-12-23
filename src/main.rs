use std::io;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor};


fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.trim().to_string()
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Toggle,
    SetLow,
    SetHigh,
    Read,
    Exit,
    None
}

fn clear_terminal() {
    execute!(
        io::stdout(), 
        Clear(ClearType::FromCursorUp),
        cursor::MoveTo(0, 0)
    ).unwrap();
}

#[derive(Debug, Clone, Copy)]
enum Byte {
    Hex(u16),
    Binary(u16),
}

impl Byte {
    fn byte(&self) -> u16 {
        match self {
            Byte::Hex(hex) => *hex,
            Byte::Binary(binary) => *binary,
        }
    }
}

#[derive(Debug, Clone)]
enum Register {
    Stack([Byte; 10]),
    Heap([Byte; 10]),
}

#[derive(Debug, Clone)]
struct Memory {
    mask: Byte,
    register: Register,
    pointer: usize,
    bit: u16,
    action: Action,
}

impl Memory {
    fn new(register: &Register, mask: &Byte) -> Memory {
        Memory {
            mask: mask.clone(),
            register: register.clone(),
            pointer: 0,
            bit: 0,
            action: Action::None,
        }
    }

    fn toggle(&mut self, pointer: usize, bit: u16) {
        match &mut self.register {
            Register::Stack(stack) => {
                match stack[pointer] {
                    Byte::Hex(hex) => {
                        stack[pointer] = Byte::Hex(hex ^ (self.mask.byte() << bit));
                    },
                    Byte::Binary(bin) => {
                        stack[pointer] = Byte::Binary(bin ^ (self.mask.byte() << bit));
                    },
                }
            },
            Register::Heap(heap) => {
                match heap[pointer] {
                    Byte::Hex(hex) => {
                        heap[pointer] = Byte::Hex(hex ^ (self.mask.byte() << bit));
                    },
                    Byte::Binary(bin) => {
                        heap[pointer] = Byte::Binary(bin ^ (self.mask.byte() << bit));
                    },
                }
            },
        }
    }

    fn set_low(&mut self, pointer: usize, bit: u16) {
        match &mut self.register {
            Register::Stack(stack) => {
                match stack[pointer] {
                    Byte::Hex(hex) => {
                        stack[pointer] = Byte::Hex(hex & !(self.mask.byte() << bit));
                    },
                    Byte::Binary(bin) => {
                        stack[pointer] = Byte::Binary(bin & !(self.mask.byte() << bit));
                    },
                }
            },            
            Register::Heap(heap) => {
                match heap[pointer] {
                    Byte::Hex(hex) => {
                        heap[pointer] = Byte::Hex(hex & !(self.mask.byte() << bit));
                    },
                    Byte::Binary(bin) => {
                        heap[pointer] = Byte::Binary(bin & !(self.mask.byte() << bit));
                    },
                }
            },
        }
    }

    fn set_high(&mut self, pointer: usize, bit: u16) {
        match &mut self.register {
            Register::Stack(stack) => {
                match stack[pointer] {
                    Byte::Hex(hex) => {
                        stack[pointer] = Byte::Hex(hex | (self.mask.byte() << bit));
                    },
                    Byte::Binary(bin) => {
                        stack[pointer] = Byte::Binary(bin | (self.mask.byte() << bit));
                    },
                }
            },  
            Register::Heap(heap) => {
                match heap[pointer] {
                    Byte::Hex(hex) => {
                        heap[pointer] = Byte::Hex(hex | (self.mask.byte() << bit));
                    },
                    Byte::Binary(bin) => {
                        heap[pointer] = Byte::Binary(bin | (self.mask.byte() << bit));
                    },
                }
            },
        }
    }

    fn read(&self, pointer: usize, bit: u16) {
        match &self.register {
            Register::Stack(stack) => {
                let mut tmp_byte: u16 = stack[pointer].byte() >> bit;
                tmp_byte = tmp_byte & self.mask.byte();
                if tmp_byte == 1 {
                    println!("Bit Status: High\n");
                } else if tmp_byte == 0 {
                    println!("Bit Status: Low\n");
                } else {
                    println!("ReadError: An Error Occurred")
                }
            },
            Register::Heap(heap) => {
                let mut tmp_byte: u16 = heap[pointer].byte() >> bit;
                tmp_byte = tmp_byte & self.mask.byte();
                if tmp_byte == 1 {
                    println!("Bit Status: Low\n");
                } else if tmp_byte == 0 {
                    println!("Bit Status: High\n");
                }
            },
        }
    }

    fn full_read(&self, pointer: usize) {
        match &self.register {
            Register::Stack(stack) => {
                match stack[pointer] {
                    Byte::Hex(hex) => {
                        println!("Modified: Stack[{}] | Byte: {:#06X}\n", pointer, hex);
                    },
                    Byte::Binary(bin) => {
                        println!("Modified: Stack[{}] | Byte: {:#018b}\n", pointer, bin);
                    },
                }
            },
            Register::Heap(heap) => {
                match heap[pointer] {
                    Byte::Hex(hex) => {
                        println!("Modified: Heap[{}] | Byte: {:#06X}\n", pointer, hex);
                    },
                    Byte::Binary(bin) => {
                        println!("Modified: Heap[{}] | Byte: {:#018b}\n", pointer, bin);
                    },
                }
            },
        }
    }

    fn action(&mut self, action: Action, pointer: usize, bit: u16) {
        clear_terminal();
        self.pointer = pointer;
        self.bit = bit;
        self.action = action;

        match action {
            Action::Toggle => {
                self.toggle(pointer, bit);
            },
            Action::Read => {
                self.read(pointer, bit);
            },
            Action::SetHigh => {
                self.set_high(pointer, bit);
            },
            Action::SetLow => {
                self.set_low(pointer, bit);
            },
            _ => {},
        }
    }

    fn last_action(&self) {
        println!("Last Action: {:?}", &self.action);
        println!("Modified Register Pointer: {}", self.pointer);
        println!("Changed Bit: {}", self.bit);
        self.full_read(self.pointer);
    }
}

fn user_action() -> (Action, usize, u16) {
    let action: Action;
    let mut pointer: usize;
    let mut bit: u16;

    // Prompts the user to choose an action.
    action = {
        loop {
            let action: u8 = input("Choose an action: \n\t1. Toggle\n\t2. Read\n\t3. Set High\n\t4. Set Low\n\t5. Exit").parse().unwrap();
            if action == 1 {
                break Action::Toggle
            } else if action == 2 {
                break Action::Read
            } else if action == 3 {
                break Action::SetHigh
            } else if action == 4 {
                break Action::SetLow
            } else if action == 5 {
                break Action::Exit
            } else {
                clear_terminal();
                println!("Invalid choice");
                continue
            }
        }
    };

    match action {
        Action::Exit => {
            clear_terminal();
            return (action, 0, 0);
        },
        _ => {}
    }
    // Prompts the user to enter a pointer.
    clear_terminal();
    pointer = loop {
        pointer = input("Enter a pointer [Byte Location in Memory (0 - 9)]: ").parse().unwrap();
        if pointer > 9 {
            clear_terminal();
            println!("MemoryError: Invalid pointer");
            continue;
        } else {
            break pointer
        }
    };

    // Prompts the user to enter a bit.
    clear_terminal();
    bit = loop {
        bit = input("Enter a bit [0 - 15]: ").parse().unwrap();
        if bit > 15 {
            clear_terminal();
            println!("RegisterError: Invalid bit");
            continue;
        } else {
            break bit
        }
    };

    // Return
    (action, pointer, bit)
}

fn main() {
    clear_terminal();
    
    // Optional Memory Parameters
    let _mask1: Byte = Byte::Hex(0x0001);
    let _register1: Register = Register::Heap([Byte::Hex(0x0000); 10]);

    // Default Memory Parameters
    let mask2: Byte = Byte::Binary(0b0000_0001);
    let register2: Register = Register::Stack([Byte::Binary(0b0000_0000); 10]);
    
    // initializes a Memory to store data in Registers (Stack, Heap).
    let mut memory: Memory = Memory::new(&register2, &mask2);

    loop {
        // Gets the user's action.
        memory.last_action();
        let (action, pointer, bit): (Action, usize, u16) = user_action();
        clear_terminal();
        
        // Executes the user's action.
        match action {
            // If the user chooses to exit, exit the program.
            Action::Exit => break println!("Exited Program!"),
            _ => memory.action(action, pointer, bit),
        }
    }
}
