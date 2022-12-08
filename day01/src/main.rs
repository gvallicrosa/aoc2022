fn main() -> Result<(), std::io::Error> {
    let mut biggest = 0;
    let mut three_biggest = Vec::new();

    // for group in include_str!("input_example.txt")
    for group in include_str!("input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
    {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u64>().unwrap();
            sum += value;
        }
        // get biggest
        if sum > biggest {
            biggest = sum;
        }
        // get 3 biggest
        let mut changed = false;
        if three_biggest.len() < 3 {
            three_biggest.push(sum);
            changed = true;
        } else if three_biggest[0] < sum {
            three_biggest[0] = sum;
            changed = true;
        }
        if changed {
            three_biggest.sort();
        }
    }
    println!("biggest is {}", biggest);
    println!("three biggest is {}", three_biggest.iter().sum::<u64>());

    Ok(())
}
