use alloc::vec;
use alloc::vec::Vec;
use itertools::Itertools;
use rtt_target::rprintln;

enum Operation {
    Square,
    Mul(usize),
    Add(usize),
}

pub struct Monkey {
    bag: Vec<usize>,
    op: Operation,
    div: usize,
    yay: usize,
    nay: usize,
    ins: usize,
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
            bag: items,
            op: operation,
            div: test,
            yay: test_true,
            nay: test_false,
            ins: 0,
        });
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].bag.pop() {
                monkeys[i].ins += 1;

                item = match monkeys[i].op {
                    Operation::Square => item * item,
                    Operation::Mul(operand) => item * operand,
                    Operation::Add(operand) => item + operand,
                } / 3;

                let target_monkey = if item % monkeys[i].div == 0 {
                    monkeys[i].yay
                } else {
                    monkeys[i].nay
                };

                monkeys[target_monkey].bag.push(item);
            }
        }
    }

    rprintln!(
        "d11a: {}",
        monkeys
            .iter()
            .map(|monkey| monkey.ins)
            .sorted()
            .rev()
            .take(2)
            .product::<usize>()
    );
}

/// Measured speed: 16,701,845us.
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
            bag: items,
            op: operation,
            div: test,
            yay: test_true,
            nay: test_false,
            ins: 0,
        });
    }

    let lcm = monkeys.iter().map(|monkey| monkey.div).product::<usize>();
    let mut bags = vec![vec![]; monkeys.len()];

    (0..10000).for_each(|_| {
        monkeys.iter_mut().enumerate().for_each(|(i, m)| {
            m.bag.append(&mut bags[i]);
            m.ins += m.bag.len();
            m.bag.drain(..).for_each(|mut item| {
                item = match m.op {
                    Operation::Square => ((item as u64 * item as u64) % lcm as u64) as usize,
                    Operation::Mul(operand) => (item * operand) % lcm,
                    Operation::Add(operand) => item + operand,
                };
                bags[if item % m.div == 0 { m.yay } else { m.nay }].push(item);
            });
        })
    });

    rprintln!(
        "d11b: {}",
        monkeys
            .iter()
            .map(|monkey| monkey.ins)
            .sorted()
            .rev()
            .take(2)
            .map(|inspection| inspection as u64)
            .product::<u64>()
    );
}
