fn main() -> Result<(), std::io::Error> {
    let mut forest: Vec<Vec<u32>> = Vec::new();
    let lines: Vec<_> = include_str!("input.txt").split('\n').collect();
    // let lines: Vec<_> = include_str!("input_example.txt").split('\n').collect();
    for line in lines {
        // get line
        forest.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }
    // for r in 0..forest.len() {
    //     for c in 0..forest[0].len() {
    //         print!("{} ", forest[r][c]);
    //     }
    //     println!();
    // }

    // check visibility
    let mut visible = 0;
    for r in 0..forest.len() {
        for c in 0..forest[0].len() {
            // println!("{} {}", r, c);
            if r == 0 || r == forest.len() - 1 {
                // println!("  visible (row)");
                visible += 1;
                continue;
            }
            if c == 0 || c == forest[0].len() - 1 {
                // println!("  visible (col)");
                visible += 1;
                continue;
            }
            // normal tree
            let current_height = forest[r][c];
            // from top
            let mut vis = true;
            for heights in forest.iter().take(r) {
                if heights[c] >= current_height {
                    vis = false;
                    break;
                }
            }
            // println!("  vis top {}", vis);
            // from bottom
            if !vis {
                let mut vis_temp = true;
                for heights in forest.iter().skip(r + 1) {
                    if heights[c] >= current_height {
                        vis_temp = false;
                        break;
                    }
                }
                vis |= vis_temp;
                // println!("  vis bottom {}", vis_temp);
            }
            // from left
            if !vis {
                let mut vis_temp = true;
                for k in 0..c {
                    if forest[r][k] >= current_height {
                        vis_temp = false;
                        break;
                    }
                }
                vis |= vis_temp;
                // println!("  vis left {}", vis_temp);
            }
            // from right
            if !vis {
                let mut vis_temp = true;
                for k in c + 1..forest[0].len() {
                    if forest[r][k] >= current_height {
                        vis_temp = false;
                        break;
                    }
                }
                vis |= vis_temp;
                // println!("  vis right {}", vis_temp);
            }
            // still visible
            if vis {
                // println!("  visible");
                visible += 1;
            }
        }
    }
    dbg!(visible);

    // scenic score
    let mut scenic_score = 0;
    for r in 0..forest.len() {
        for c in 0..forest[0].len() {
            // normal tree
            let current_height = forest[r][c];
            let mut scores = vec![0; 4];
            // from top
            {
                let mut limit = false;
                for (k, heights) in forest.iter().take(r).rev().enumerate() {
                    if heights[c] >= current_height {
                        scores[0] = k + 1;
                        limit = true;
                        break;
                    }
                }
                if !limit {
                    scores[0] = r;
                }
            }
            // from bottom
            {
                let mut limit = false;
                for (k, heights) in forest.iter().skip(r + 1).enumerate() {
                    if heights[c] >= current_height {
                        scores[1] = k + 1;
                        limit = true;
                        break;
                    }
                }
                if !limit {
                    scores[1] = forest.len() - r - 1;
                }
            }
            // from left
            {
                let mut limit = false;
                for k in (0..c).rev() {
                    if forest[r][k] >= current_height {
                        // dbg!(c, k);
                        scores[2] = c - k;
                        limit = true;
                        break;
                    }
                }
                if !limit {
                    scores[2] = c;
                }
            }
            // from right
            {
                let mut limit = false;
                for k in c + 1..forest[0].len() {
                    if forest[r][k] >= current_height {
                        scores[3] = k - c;
                        limit = true;
                        break;
                    }
                }
                if !limit {
                    scores[3] = forest[0].len() - c - 1;
                }
            }
            // score
            let score = scores.iter().product::<usize>();
            println!("{} {} scores {:?} => {}", r, c, scores, score);
            if score > scenic_score {
                scenic_score = score;
            }
        }
    }
    dbg!(scenic_score);

    Ok(())
}
