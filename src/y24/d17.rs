use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete as ch;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{preceded, tuple};
use nom::IResult;

type Int = u64;

#[derive(Default, Debug)]
struct Register {
    registers: Vec<Int>,
}

impl Register {
    fn literal_operand(&self, v: u8) -> Int {
        v as Int
    }

    fn combo_operand(&self, v: u8) -> Int {
        if v <= 3 {
            v as Int
        } else {
            self.registers[(v - 4) as usize]
        }
    }

    fn resolve_operand(&self, op: Operand) -> Int {
        match op {
            Operand::Literal(v) => self.literal_operand(v),
            Operand::Combo(v) => self.combo_operand(v),
        }
    }

    fn a_register(&self) -> Int {
        self.registers[0]
    }
    fn b_register(&self) -> Int {
        self.registers[1]
    }
    fn c_register(&self) -> Int {
        self.registers[2]
    }
    fn write(&mut self, i: usize, value: Int) {
        self.registers[i] = value;
    }
}

#[derive(Default, Debug)]
struct Program {
    i_ptr: usize,
    jumped: bool,
    register: Register,
    instructions: Vec<u8>,
    output: Vec<u8>,
}

impl Program {
    pub fn new(registers: Vec<Int>, instructions: Vec<u8>) -> Self {
        Self {
            register: Register { registers },
            instructions,
            jumped: false,
            i_ptr: 0,
            output: vec![],
        }
    }
}

enum Operand {
    Literal(u8),
    Combo(u8),
}

const A_REG: usize = 0;
const B_REG: usize = 1;
const C_REG: usize = 2;

const A_REG_COMBO: u8 = 4;
const B_REG_COMBO: u8 = 5;
const C_REG_COMBO: u8 = 6;

#[derive(Clone, Debug)]
struct Input {
    registers: Vec<Int>,
    instructions: Vec<u8>,
}
impl Input {
    fn parse(i: &str) -> IResult<&str, Self> {
        // More like for verification, not really necessary, just as an exercise
        let parse_register = preceded(
            preceded(preceded(tag("Register "), ch::alpha1), tag(": ")),
            ch::u64,
        );
        let (i, registers) = separated_list0(ch::newline, parse_register)(i)?;
        let (i, _) = tuple((ch::newline, ch::newline))(i)?;
        let (i, instructions) = preceded(tag("Program: "), separated_list1(tag(","), ch::u8))(i)?;

        Ok((
            i,
            Self {
                registers,
                instructions,
            },
        ))
    }
}

impl Program {
    fn div_instruction(&mut self, operand: Operand, out_reg: usize) {
        let num = self.register.a_register();
        let shift = self.register.resolve_operand(operand);
        self.register.write(out_reg, num >> shift);
    }

    fn xor_instruction(&mut self, op1: Operand, op2: Operand) {
        let val1 = self.register.resolve_operand(op1);
        let val2 = self.register.resolve_operand(op2);
        let ans = val1 ^ val2;
        self.register.write(B_REG, ans);
    }

    fn bst_instruction(&mut self, op1: Operand) {
        let val = self.register.resolve_operand(op1);
        let ans = val & 7;
        self.register.write(B_REG, ans);
    }

    fn jnz_instruction(&mut self, op: Operand) {
        if self.register.a_register() == 0 {
            return;
        }
        self.i_ptr = self.register.resolve_operand(op) as usize; // Should be literal operand
        self.jumped = true;
    }

    fn out_instruction(&mut self, op: Operand) {
        let value = self.register.resolve_operand(op);
        let ans = (value & 7) as u8;
        self.output.push(ans);
    }

    fn do_instruction(&mut self, instruction: u8, operand: u8) {
        match instruction {
            0 => self.div_instruction(Operand::Combo(operand), A_REG),
            1 => self.xor_instruction(Operand::Combo(B_REG_COMBO), Operand::Literal(operand)),
            2 => self.bst_instruction(Operand::Combo(operand)),
            3 => self.jnz_instruction(Operand::Literal(operand)),
            4 => self.xor_instruction(Operand::Combo(B_REG_COMBO), Operand::Combo(C_REG_COMBO)),
            5 => self.out_instruction(Operand::Combo(operand)),
            6 => self.div_instruction(Operand::Combo(operand), B_REG),
            7 => self.div_instruction(Operand::Combo(operand), C_REG),
            _ => panic!("Unexpected instruction {}", instruction),
        }
    }

    fn simulate(&mut self) -> Vec<u8> {
        while self.i_ptr < self.instructions.len() {
            assert_eq!(self.i_ptr % 2, 0); // Not sure though
                                           // println!("{:?}", self);
            self.do_instruction(
                self.instructions[self.i_ptr],
                self.instructions[self.i_ptr + 1],
            );

            if !self.jumped {
                self.i_ptr += 2;
            } else {
                self.jumped = false;
            }
        }

        self.output.clone()
    }
}

/**
Program: 2,4,1,5,7,5,1,6,0,3,4,1,5,5,3,0

2,4,
1,5,
7,5,
1,6,
0,3,
4,1,
5,5,
3,0

bst 4 | B := A & 7
bxl 5 | B := B ^ 5
cdv 5 | C := A >> B
bxl 6 | B := B ^ 6
adv 3 | A := A >> 3
bxc 1 | B := B ^ C
out 5 | Out.push(B & 7)
jnz 0 | Jump to 0 while A > 0

*/

// 3 bits * 16

fn rec(instructions: &Vec<u8>, a: Int, i: usize) -> Option<Int> {
    let b = (a & 7) ^ 5;
    let c = a >> b;
    let b = (b ^ 6) ^ c;

    let b = (b & 7) as u8;

    if i == instructions.len() || b == instructions[i] {
        if i == 0 {
            return Some(a);
        }
        for x in 0..8 {
            let new_a = (a << 3) | x;
            if let Some(a) = rec(instructions, new_a, i - 1) {
                return Some(a);
            }
        }
    }

    None
}

fn solve_bruteforce(instructions: Vec<u8>) -> Int {
    let mut a = 0;
    loop {
        if instructions == Program::new(vec![a, 0, 0], instructions.clone()).simulate() {
            break;
        }
        a += 1;
    }
    a
}

fn solve_2(instructions: Vec<u8>) -> Int {
    let a_register = if instructions == vec![ 2,4,1,5,7,5,1,6,0,3,4,1,5,5,3,0] {
        rec(&instructions, 0, instructions.len()).expect("Not found")
    } else {
        solve_bruteforce(instructions.clone())
    };
    let simulated = Program::new(vec![a_register, 0, 0], instructions.clone()).simulate();
    assert_eq!(simulated, instructions);
    a_register
}

fn solve_1(input: Input) -> String {
    let mut program = Program::new(input.registers, input.instructions);
    let vec = program.simulate();
    vec.into_iter().join(",")
}

pub fn run_all(content: &str) -> (String, Int) {
    let (_, input) = Input::parse(content).expect("Failed to parse!");
    let ans1 = solve_1(input.clone());
    let ans2 = solve_2(input.instructions);

    (ans1, ans2)
}
pub fn run(content: &str) -> Int {
    let (_, input) = Input::parse(content).expect("Failed to parse!");
    let ans2 = solve_2(input.instructions);
    ans2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse() {
        let (_a, input) = Input::parse(
            r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        )
        .expect("Failed to parse!");
        assert_eq!(input.registers, vec![729, 0, 0]);
        assert_eq!(input.instructions, vec![0, 1, 5, 4, 3, 0]);
        dbg!(_a);
    }
}
