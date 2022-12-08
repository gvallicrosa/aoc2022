#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum PlayResult {
    Loose,
    Draw,
    Win,
}

fn compute_result(other: &Play, me: &Play) -> PlayResult {
    match other {
        Play::Rock => match me {
            Play::Rock => PlayResult::Draw,
            Play::Paper => PlayResult::Win,
            Play::Scissors => PlayResult::Loose,
        },
        Play::Paper => match me {
            Play::Rock => PlayResult::Loose,
            Play::Paper => PlayResult::Draw,
            Play::Scissors => PlayResult::Win,
        },
        Play::Scissors => match me {
            Play::Rock => PlayResult::Win,
            Play::Paper => PlayResult::Loose,
            Play::Scissors => PlayResult::Draw,
        },
    }
}

fn points_from_play(p: &Play) -> u64 {
    match p {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    }
}

fn gives_that_result(other: &Play, result: &PlayResult) -> Play {
    match other {
        Play::Rock => match result {
            PlayResult::Loose => Play::Scissors,
            PlayResult::Draw => Play::Rock,
            PlayResult::Win => Play::Paper,
        },
        Play::Paper => match result {
            PlayResult::Loose => Play::Rock,
            PlayResult::Draw => Play::Paper,
            PlayResult::Win => Play::Scissors,
        },
        Play::Scissors => match result {
            PlayResult::Loose => Play::Paper,
            PlayResult::Draw => Play::Scissors,
            PlayResult::Win => Play::Rock,
        },
    }
}

fn main() -> Result<(), std::io::Error> {
    // A = X -> Rock     -> 1 Point
    // B = Y -> Paper    -> 2 Points
    // C = Z -> Scissors -> 3 Points
    // Loose -> 0 Points
    // Draw  -> 3 Points
    // Win   -> 6 Points
    let mut points = 0;

    // X -> Loose
    // Y -> Draw
    // Z -> Win
    let mut points_second = 0;

    // for line in include_str!("input_example.txt")
    for line in include_str!("input.txt").replace("\r\n", "\n").split("\n") {
        let other_me: Vec<_> = line.split(" ").collect();
        let other = match other_me[0] {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => unreachable!(),
        };
        // first
        let me = match other_me[1] {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => unreachable!(),
        };
        points += match compute_result(&other, &me) {
            PlayResult::Loose => points_from_play(&me) + 0,
            PlayResult::Draw => points_from_play(&me) + 3,
            PlayResult::Win => points_from_play(&me) + 6,
        };
        // second
        let result = match other_me[1] {
            "X" => PlayResult::Loose,
            "Y" => PlayResult::Draw,
            "Z" => PlayResult::Win,
            _ => unreachable!(),
        };
        points_second += match result {
            PlayResult::Loose => points_from_play(&gives_that_result(&other, &result)) + 0,
            PlayResult::Draw => points_from_play(&gives_that_result(&other, &result)) + 3,
            PlayResult::Win => points_from_play(&gives_that_result(&other, &result)) + 6,
        };
    }
    println!("points {}", points);
    println!("points_second {}", points_second);

    Ok(())
}
