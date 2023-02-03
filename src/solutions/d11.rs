use alloc::vec::Vec;
use itertools::Itertools;
use rtt_target::rprintln;

enum Operation<T> {
    Square,
    Mul(T),
    Add(T),
}

pub struct Monkey<T> {
    items: Vec<T>,
    operation: Operation<T>,
    test: T,
    test_true: usize,
    test_false: usize,
    inspections: usize,
}

/// Measured speed: 34,021us.
pub fn p1(_memory: &mut [u8], input: &mut [u8]) {
    let mut monkeys = Vec::new();
    for monkey in input.split(|&byte| byte == b'\n').array_chunks::<7>() {
        let mut item = 0;
        let mut items = Vec::new();
        for &byte in monkey[1][18..].iter().chain(&[b'\n']) {
            match byte {
                b',' | b'\n' => {
                    items.push(item);
                    item = 0;
                }
                b' ' => {}
                _ => item = item * 10 + (byte - b'0') as usize,
            }
        }

        let mut num = 0;
        let operation;
        if monkey[2][23] == b'+' {
            for &byte in monkey[2][25..].iter() {
                num = num * 10 + (byte - b'0') as usize;
            }
            operation = Operation::Add(num);
        } else if monkey[2][25] == b'o' {
            operation = Operation::Square;
        } else {
            for &byte in monkey[2][25..].iter() {
                num = num * 10 + (byte - b'0') as usize;
            }
            operation = Operation::Mul(num);
        }

        let mut test = 0;
        for &byte in monkey[3][21..].iter() {
            test = test * 10 + (byte - b'0') as usize;
        }

        let mut test_true = 0;
        for &byte in monkey[4][29..].iter() {
            test_true = test_true * 10 + (byte - b'0') as usize;
        }

        let mut test_false = 0;
        for &byte in monkey[5][30..].iter() {
            test_false = test_false * 10 + (byte - b'0') as usize;
        }

        monkeys.push(Monkey {
            items,
            operation,
            test,
            test_true,
            test_false,
            inspections: 0,
        });
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop() {
                monkeys[i].inspections += 1;

                item = match monkeys[i].operation {
                    Operation::Square => item * item,
                    Operation::Mul(operand) => item * operand,
                    Operation::Add(operand) => item + operand,
                } / 3;

                let target_monkey = if item % monkeys[i].test == 0 {
                    monkeys[i].test_true
                } else {
                    monkeys[i].test_false
                };

                monkeys[target_monkey].items.push(item);
            }
        }
    }

    rprintln!(
        "d11a: {}",
        monkeys
            .iter()
            .map(|monkey| monkey.inspections)
            .sorted()
            .rev()
            .take(2)
            .product::<usize>()
    );
}

/// Measured speed: 26,111,888us.
pub fn p2(_memory: &mut [u8], input: &mut [u8]) {
    let mut monkeys = Vec::new();
    for monkey in input.split(|&byte| byte == b'\n').array_chunks::<7>() {
        let mut item = 0;
        let mut items = Vec::new();
        for &byte in monkey[1][18..].iter().chain(&[b'\n']) {
            match byte {
                b',' | b'\n' => {
                    items.push(item);
                    item = 0;
                }
                b' ' => {}
                _ => item = item * 10 + (byte - b'0') as u64,
            }
        }

        let mut num = 0;
        let operation;
        if monkey[2][23] == b'+' {
            for &byte in monkey[2][25..].iter() {
                num = num * 10 + (byte - b'0') as u64;
            }
            operation = Operation::Add(num);
        } else if monkey[2][25] == b'o' {
            operation = Operation::Square;
        } else {
            for &byte in monkey[2][25..].iter() {
                num = num * 10 + (byte - b'0') as u64;
            }
            operation = Operation::Mul(num);
        }

        let mut test = 0;
        for &byte in monkey[3][21..].iter() {
            test = test * 10 + (byte - b'0') as u64;
        }

        let mut test_true = 0;
        for &byte in monkey[4][29..].iter() {
            test_true = test_true * 10 + (byte - b'0') as usize;
        }

        let mut test_false = 0;
        for &byte in monkey[5][30..].iter() {
            test_false = test_false * 10 + (byte - b'0') as usize;
        }

        monkeys.push(Monkey {
            items,
            operation,
            test,
            test_true,
            test_false,
            inspections: 0,
        });
    }

    let common_multiple = monkeys.iter().map(|monkey| monkey.test).product::<u64>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop() {
                monkeys[i].inspections += 1;

                item = match monkeys[i].operation {
                    Operation::Square => item * item,
                    Operation::Mul(operand) => item * operand,
                    Operation::Add(operand) => item + operand,
                } % common_multiple;

                let target_monkey = if item % monkeys[i].test == 0 {
                    monkeys[i].test_true
                } else {
                    monkeys[i].test_false
                };

                monkeys[target_monkey].items.push(item);
            }
        }
    }

    rprintln!(
        "d11b: {}",
        monkeys
            .iter()
            .map(|monkey| monkey.inspections)
            .sorted()
            .rev()
            .take(2)
            .map(|inspection| inspection as u64)
            .product::<u64>()
    );
}
