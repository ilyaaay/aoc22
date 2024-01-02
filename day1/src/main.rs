fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("./src/input.txt")?;

    println!("First part: {}", first_part(&input));
    println!("Second part: {}", second_part(&input));

    Ok(())
}

fn first_part(input: &String) -> i32 {
    let mut vec_sum: Vec<i32> = Vec::new();
    let mut sum = 0;

    for i in input.lines() {
        if i.is_empty() {
            vec_sum.push(sum);
            sum = 0;
            continue;
        }

        sum += i.trim().parse::<i32>().unwrap_or(0);
    }

    vec_sum.sort();

    *vec_sum.last().unwrap()
}

fn second_part(input: &String) -> i32 {
    let mut vec_sum: Vec<i32> = Vec::new();
    let mut sum = 0;

    for i in input.lines() {
        if i.is_empty() {
            vec_sum.push(sum);
            sum = 0;
            continue;
        }

        sum += i.trim().parse::<i32>().unwrap_or(0);
    }

    vec_sum.sort();
    vec_sum.reverse();

    vec_sum[0] + vec_sum[1] + vec_sum[2]
}

#[cfg(test)]
mod test {
    use crate::*;

    static TEST: &str = r#"1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000"#;

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(&TEST.to_string()), 24000);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(&TEST.to_string()), 41000);
    }
}
