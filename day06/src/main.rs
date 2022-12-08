use std::collections::VecDeque;

fn main() -> Result<(), std::io::Error> {
    let num_different = 4;
    let num_different = 14;
    let mut queue: VecDeque<char> = VecDeque::new();
    // for line in include_str!("input_example.txt")
    for line in include_str!("input.txt")
    .replace("\r\n", "\n").split('\n') {
        for (i, c) in line.chars().enumerate() {
            // add
            queue.push_back(c);
            // empty
            if queue.len() > num_different {
                queue.pop_front();
            }
            // check
            if queue.len() == num_different {
                // dbg!(&i, &queue);
                let mut duplicate = false;
                'outer: for j in 0..queue.len() {
                    for k in j + 1..queue.len() {
                        if queue[j] == queue[k] {
                            duplicate = true;
                            break 'outer;
                        }
                    }
                }
                if !duplicate {
                    println!("found at {}", i +1);
                    queue = VecDeque::new();
                    break;
                }
            }
        }
    }

    Ok(())
}
