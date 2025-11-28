use std::io;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor};

fn set_high(mut register: u16, mut mask: u16) -> u16 {
    clear_terminal();
    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    loop {
        let bit_index: u32 = input("Set [High] | Bit Index [0 - 15]: ").parse().expect("Expected An Integer: u32");
        if bit_index > 15 {
            clear_terminal();
            println!("{} -> [{}]\n", "Invalid Index", bit_index);
            println!("{:#018b} | {:#06X} | {}\n", register, register, register);
            continue;
        } else {
            mask = mask << bit_index;
            register = register | mask;
            break register;
        }
    }
}

fn set_low(mut register: u16, mut mask: u16) -> u16 {
    clear_terminal();
    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    loop {
        let bit_index: u32 = input("Set [Low] | Bit Index [0 - 15]: ").parse().expect("Expected An Integer: u32");
        if bit_index > 15 {
            clear_terminal();
            println!("{} -> [{}]\n", "Invalid Index", bit_index);
            println!("{:#018b} | {:#06X} | {}\n", register, register, register);
            continue;
        } else {
            mask = mask << bit_index;
            register = register & !mask;
            break register;
        }
    }
}

fn toggle(mut register: u16, mut mask: u16) -> u16 {
    clear_terminal();
    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    loop {
        let bit_index: u32 = input("Toggle | Bit Index [0 - 15]: ").parse().expect("Expected An Integer: u32");
        if bit_index > 15 {
            clear_terminal();
            println!("{} -> [{}]\n", "Invalid Index", bit_index);
            println!("{:#018b} | {:#06X} | {}\n", register, register, register);
            continue;
        } else {
            mask = mask << bit_index;
            register = register ^ mask;
            break register;
        }
    }
}

fn read_bit(register: u16, mut mask: u16) -> u16 {
    clear_terminal();
    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    loop {
        let action: String = input("A. Read Bit\nB. Quit\n");
        clear_terminal();
        println!("{:#018b} | {:#06X} | {}\n", register, register, register);
            if (action == "A") | (action == "a") {
                let bit_index: u32 = input("Read Bit | Bit Index [0 - 15]: ").parse().expect("Expected An Integer: u32");
                clear_terminal();
                println!("{:#018b} | {:#06X} | {}\n", register, register, register);
                if bit_index > 15 {
                    clear_terminal();
                    println!("{} -> [{}]\n", "Invalid Index", bit_index);
                    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
                    continue;
                } else {
                    mask = mask << bit_index;
                    let mut tmp_register: u16 = register & mask;
                    tmp_register = tmp_register >> bit_index;
                    if tmp_register == 1 {
                        println!("{} -> [1] | [{}]\n", "Bit", "High");
                    } else if tmp_register == 0 {
                        println!("{} -> [0] | [{}]\n", "Bit", "Low")
                    } else {
                        println!("{} -> [{:#06X}] | [{:#06X}]\n", "ReadError", tmp_register, register);
                        continue;
                    }
                }
            } else if (action == "B") | (action == "b") | (action == "q") | (action == "Q") {
                break register;
            } else {
                clear_terminal();
                println!("{} -> [{}]\n", "Invalid Operation", action);
                println!("{:#018b} | {:#06X} | {}\n", register, register, register);
                continue;
            }
        }
    }

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buf: String = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.trim().to_string()
}

fn clear_terminal() {
    execute!(
        io::stdout(), 
        Clear(ClearType::FromCursorUp),
        cursor::MoveTo(0, 0)
    ).unwrap();
}

fn options(prompt: &str, register: u16) -> String {
    loop {
        let choice: String = input(prompt);
        if (choice == "t") | (choice == "T") {
            break "toogle".to_string();
        } else if (choice == "h") | (choice == "H") {
            break "set high".to_string();
        } else if (choice == "l") | (choice == "L") {
            break "set low".to_string();
        } else if (choice == "r") | (choice == "R") {
            break "read bit".to_string();
        } else if (choice == "q") | (choice == "Q") {
            break "quit".to_string();
        } else {
            clear_terminal();
            println!("{} -> [{}]\n", "Invalid Operation", choice);
            println!("{:#018b} | {:#06X} | {}\n", register, register, register);
            continue;
        }
    }
}

fn main() {
    let mut register: u16 = 0x0000;
    let mask: u16 = 0x0001;

    clear_terminal();
    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    'main: loop {
        let choice: String = options(
            "[Options]:\n  Toggle: T\n  Set High: H\n  Set Low: L\n  Read Bit: R\n  Quit: Q\n", 
            register
        );
        register = {
            if choice == "toogle" {
                toggle(register, mask)
            } else if choice == "set high" {
                set_high(register, mask)
            } else if choice == "set low" {
                set_low(register, mask)
            } else if choice == "read bit" {
                read_bit(register, mask);
                register
            } else if choice == "quit" {
                break 'main;
            } else {
                continue;
            }
        };
        clear_terminal();
        println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    }
    clear_terminal();
    println!("{:#018b} | {:#06X} | {}\n", register, register, register);
    println!("{}", "Register Operations -> Completed!");
}
