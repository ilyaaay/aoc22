fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("./src/input.txt")?;

    println!("First part: {}", solution(&input).0);
    println!("Second part: {}", solution(&input).1);

    Ok(())
}

fn solution(input: &String) -> (i32, i32) {
    let mut solution = (0, 0);

    input.lines().for_each(|x| {
        let values = x.split_whitespace().collect::<Vec<&str>>();

        let (opponent, me) = (values[0], values[1]);

        match me {
            // rock
            "X" => match opponent {
                "A" => solution.0 += 4, // draw + rock,
                "B" => solution.0 += 1, // loss + rock,
                "C" => solution.0 += 7, // win + rock,
                _ => {}
            },
            // paper
            "Y" => match opponent {
                "A" => solution.0 += 8, // win + paper,
                "B" => solution.0 += 5, // draw + paper,
                "C" => solution.0 += 2, // loss + paper,
                _ => {}
            },
            // scissors
            "Z" => match opponent {
                "A" => solution.0 += 3, // loss + scissors,
                "B" => solution.0 += 9, // win + scissors,
                "C" => solution.0 += 6, // draw + scissors,
                _ => {}
            },
            _ => (),
        }

        match opponent {
            // rock
            "A" => match me {
                "X" => solution.1 += 3, // loss + scissors
                "Y" => solution.1 += 4, // draw + rock
                "Z" => solution.1 += 8, // win + paper
                _ => {}
            },
            // paper
            "B" => match me {
                "X" => solution.1 += 1, // loss + rock
                "Y" => solution.1 += 5, // draw + paper
                "Z" => solution.1 += 9, // win + scissors
                _ => {}
            },
            // scissors
            "C" => match me {
                "X" => solution.1 += 2, // loss + paper
                "Y" => solution.1 += 6, // draw + scissors
                "Z" => solution.1 += 7, // win + rock
                _ => {}
            },
            _ => {}
        }
    });

    solution
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_day_solution() {
        let data = r#"A Y
            B X
            C Z"#
            .to_string();

        assert_eq!(solution(&data), (15, 12));
    }
}
