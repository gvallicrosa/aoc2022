use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
enum Reading {
    Size,
    Header,
    Moves,
}

fn main() -> Result<(), std::io::Error> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut state: Reading = Reading::Size;
    // for line in include_str!("input_example.txt")
    for line in include_str!("input.txt").replace("\r\n", "\n").split('\n') {
        // change
        if line.is_empty() {
            state = Reading::Moves;
            dbg!(&stacks);
            continue;
        }
        // size
        if state == Reading::Size {
            // 3*x + 1*(x-1) => 4x - 1
            let num_stacks = (line.len() + 1) / 4;
            dbg!(num_stacks);
            stacks.resize(num_stacks, VecDeque::new());
            state = Reading::Header;
        }
        // header
        if state == Reading::Header {
            for (i, stack) in stacks.iter_mut().enumerate() {
                let p = 1 + 4 * i;
                let c = line.as_bytes()[p] as char;
                if c == '1' {
                    break;
                }
                if c != ' ' {
                    stack.push_back(c);
                }
            }
        }
        // moves
        if state == Reading::Moves {
            let parts: Vec<_> = line.split(' ').collect();
            let num = parts[1].parse::<usize>().unwrap();
            let src = parts[3].parse::<usize>().unwrap() - 1;
            let dst = parts[5].parse::<usize>().unwrap() - 1;
            // part1
            // for _ in 0..num {
            //     let obj = stacks[src].pop_front().unwrap();
            //     stacks[dst].push_front(obj);
            // }
            // part2
            let mut temp = VecDeque::new();
            for _ in 0..num {
                temp.push_back(stacks[src].pop_front().unwrap())
            }
            for _ in 0..num {
                stacks[dst].push_front(temp.pop_back().unwrap());
            }
        }
    }

    for stack in stacks.iter() {
        print!("{}", stack.front().unwrap());
    }
    dbg!(&stacks);
    println!();
    // dbg!(count_partial);

    Ok(())
}
