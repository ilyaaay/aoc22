use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("./src/input.txt")?;

    println!("First part: {}", first_part(&input));
    println!("Second part: {}", second_part(&input));

    Ok(())
}

fn first_part(raw_content: &String) -> i32 {
    let mut result = 0;
    let mut duplicate_chars_vec = Vec::<char>::new();
    let mut duplicate_char: char = '\0';

    raw_content.lines().for_each(|x| {
        let (first_part, second_part) = x.split_at(x.len() / 2);

        for ch in first_part.chars() {
            second_part.contains(ch).then(|| duplicate_char = ch);
        }

        duplicate_chars_vec.push(duplicate_char);
    });

    let lowercase_map = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
    ]);

    let uppercase_map = HashMap::from([
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    for i in duplicate_chars_vec {
        let mut x = lowercase_map.get(&i);

        x.is_none().then(|| x = uppercase_map.get(&i));

        result += x.unwrap();
    }

    result
}

fn second_part(raw_content: &String) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_part() {
        let test_data = "abcabd".to_string();

        assert_eq!(first_part(&test_data), 2);
    }

    #[test]
    fn test_second_part() {
        assert!(true);
    }
}
