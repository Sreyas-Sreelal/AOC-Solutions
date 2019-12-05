fn process_param(mode: i64, param: i64, codes: &[i64]) -> i64 {
    if mode == 1 {
        param
    } else {
        codes[param as usize]
    }
}

fn main() {
    let mut intcodes: Vec<i64> = include_str!("../input")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut idx = 0;
    let mut initial = 5;

    while idx < intcodes.len() {
        let mut op1 = intcodes[idx + 1];
        let opcode = intcodes[idx] % 100;
        if opcode == 99 {
            break;
        } else if opcode == 4 {
            initial = intcodes[op1 as usize];
            idx += 2;
            continue;
        } else if opcode == 3 {
            intcodes[op1 as usize] = initial;
            idx += 2;
            continue;
        }
        op1 = process_param((intcodes[idx] / 100) % 10, intcodes[idx + 1], &intcodes);
        let op2 = process_param((intcodes[idx] / 1000) % 10, intcodes[idx + 2], &intcodes);
        let op3 = intcodes[idx + 3];

        match opcode {
            1 => {
                intcodes[op3 as usize] = op1 + op2;
                idx += 4;
            }
            2 => {
                intcodes[op3 as usize] = op1 * op2;
                idx += 4;
            }
            5 => {
                if op1 != 0 {
                    idx = op2 as usize;
                } else {
                    idx += 3;
                }
            }
            6 => {
                if op1 == 0 {
                    idx = op2 as usize;
                } else {
                    idx += 3;
                }
            }
            7 => {
                intcodes[op3 as usize] = i64::from(op1 < op2);
                idx += 4;
            }
            8 => {
                intcodes[op3 as usize] = i64::from(op1 == op2);
                idx += 4;
            }
            _ => {}
        }
    }
    println!("output {}", initial);
}
