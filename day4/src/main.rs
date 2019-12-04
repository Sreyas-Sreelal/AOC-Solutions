use std::collections::HashSet;

fn main() {
    let (start, end) = (147_981, 691_423);
    let mut count = 0;
    let (mut sorted, mut stop, mut element);

    for x in start..=end {
        let integers: Vec<i32> = x
            .to_string()
            .chars()
            .map(|x| x.to_string().parse().unwrap())
            .collect();

        let mut set = HashSet::new();
        if integers.iter().all(|x| set.insert(x)) {
            continue;
        }

        sorted = integers.clone();
        sorted.sort();
        if sorted != integers {
            continue;
        }

        stop = 0;
        element = -1;
        for x in sorted.iter() {
            if element != *x {
                if stop == 2 {
                    break;
                }
                stop = 1;
                element = *x;
            } else {
                stop += 1;
            }
        }
        if stop != 2 {
            continue;
        }

        count += 1;
    }

    println!("{}", count);
}
