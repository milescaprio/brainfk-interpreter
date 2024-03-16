use std::rc::Rc;
use std::cell::RefCell; 
use std::io::Read;

const MEMORY_SIZE : usize = 2048;
const CHARS : [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];

#[derive(Debug)]
struct InstructionChainItem {
    command: char,
    jumpback: Option<Rc<RefCell<InstructionChainItem>>>,
    next: Option<Rc<RefCell<InstructionChainItem>>>,
}

struct ScriptRunner {
    instruction_chain: Option<Rc<RefCell<InstructionChainItem>>>,
    memory: [u8; MEMORY_SIZE],
    memory_pointer: usize,
    instruction_pointer: Option<Rc<RefCell<InstructionChainItem>>>,
}

fn compile_script(script : String) -> ScriptRunner {
    let mut curr = Rc::new(RefCell::new(InstructionChainItem {
        command: ' ',
        next: None,
        jumpback: None,
    }));
    let beginning = curr.clone();
    let mut bracket_stack : Vec<Rc<RefCell<InstructionChainItem>>> = Vec::new();
    for i in script.chars() {
        if CHARS.contains(&i) {
            curr.borrow_mut().next = Some(Rc::new(RefCell::new(InstructionChainItem { command: i, jumpback: None, next: None })));
            curr = curr.clone().borrow().next.clone().unwrap_or_else(||panic!("Failed to unwrap next?"));
            if i == '[' {
                bracket_stack.push(curr.clone());
            } else if i == ']' {
                curr.borrow_mut().jumpback = Some(bracket_stack.pop()
                    .unwrap_or_else(||panic!("Early Closed Bracket"))
                    .clone());
            }
        }
    }
    if bracket_stack.len() > 0 {
        panic!("Unmatched brackets in script");
    }
    ScriptRunner{
        instruction_chain: Some(beginning),
        memory: [0; MEMORY_SIZE],
        memory_pointer: 0,
        instruction_pointer: None,
    }
}

impl ScriptRunner {
    fn run(&mut self) {
        let mut instruction_pointer = self.instruction_chain.clone();
        loop {
            let command = instruction_pointer.unwrap().borrow().command;
            match command {
                '>' => {
                    self.memory_pointer += 1;
                },
                '<' => {
                    self.memory_pointer -= 1;
                },
                '+' => {
                    self.memory[self.memory_pointer] += 1;
                },
                '-' => {
                    self.memory[self.memory_pointer] -= 1;
                }
                '[' => {
                    
                }
                ']' => {
                    if self.memory[self.memory_pointer] != 0 {
                        instruction_pointer
                        .unwrap()
                        .replace_with(|i| i.jumpback.clone());
                            // .clone()
                            // .unwrap()
                            // .borrow()
                            // .jumpback
                            // .clone()
                            // .unwrap_or_else(||panic!("No existing jumpback in instruction chain?")));
                    }
                    continue;
                }
                '.' => {
                    print!("{}", self.memory[self.memory_pointer] as char)
                }
                ',' => {
                    let input: u8 = std::io::stdin()
                        .bytes() 
                        .next()
                        .unwrap_or_else(||panic!("No byte to read"))
                        .unwrap_or_else(|_|panic!("No byte to read"));
                    self.memory[self.memory_pointer] = input;
                }
            }
        }
            
    }
}

fn main() {
    //run_script(">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]
    // >++++++++[<++++>-] <.>+++++++++++[<++++++++>-]<-.--------.+++
    // .------.--------.[-]>++++++++[<++++>- ]<+.[-]++++++++++.".to_string());
    // println!("{:?}", compile_script("++--[[]<[+] ][]".to_string()));
}
