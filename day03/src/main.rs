fn get_value(c: char) -> u16 {
    let val_upper = (c as u16) - ('A' as u16) + 1; // A = 1
    if val_upper <= 26 {
        val_upper + 26
    } else {
        (c as u16) - ('a' as u16) + 1 // a = 1
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut priorities = 0;
    for line in include_str!("input_example.txt")
        // for line in include_str!("input.txt")
        .replace("\r\n", "\n")
        .split('\n')
    {
        let rucksack: Vec<_> = line.chars().collect();
        // println!("rucksack: {:?}", rucksack);
        let half = rucksack.len() / 2;
        // println!("{} => {}", rucksack.len(), half);
        'outer: for i in 0..half {
            let this = rucksack[i];
            for j in 0..half {
                let other = rucksack[j + half];

                if this == other {
                    // println!("{:?} {}", this, this as u8);
                    let toadd = get_value(this);
                    println!("{}", toadd);
                    priorities += toadd;
                    break 'outer;
                }
            }
        }
    }
    println!("priorities {}", priorities);

    let mut badges = 0;
    // let lines: Vec<_> = include_str!("input_example.txt").split('\n').collect();
    let lines: Vec<_> = include_str!("input.txt").split('\n').collect();
    for group in lines.chunks(3).map(|line| line.to_owned()) {
        'outer: for c0 in group[0].chars() {
            for c1 in group[1].chars() {
                if c0 == c1 {
                    for c2 in group[2].chars() {
                        if c1 == c2 {
                            badges += get_value(c0);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    println!("badges {}", badges);

    Ok(())
}
