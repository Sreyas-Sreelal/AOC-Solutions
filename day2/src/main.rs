fn main() {
    let ointcodes: Vec<i64> = include_str!("../input")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut intcodes = ointcodes.clone();
            intcodes[1] = noun;
            intcodes[2] = verb;
            let mut idx = 0;
            loop {
                let opcode = intcodes[idx] as usize;
                if opcode == 99 {
                    break;
                }
                let op1 = intcodes[idx + 1] as usize;
                let op2 = intcodes[idx + 2] as usize;
                let result = intcodes[idx + 3] as usize;
                match opcode {
                    1 => {
                        intcodes[result] = intcodes[op1] + intcodes[op2];
                    }
                    2 => {
                        intcodes[result] = intcodes[op1] * intcodes[op2];
                    }

                    _ => {}
                }
                idx += 4;
            }

            if intcodes[0] == 19_690_720 {
                println!("{:?}", 100 * intcodes[1] + intcodes[2]);
                return;
            }
        }
    }
}
