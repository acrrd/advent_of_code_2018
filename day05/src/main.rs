use std::io::{self, Read};

fn unit_react(a: char, b: char) -> bool {
    a != b && a.eq_ignore_ascii_case(&b)
}

fn polymer_react(cs: impl Iterator<Item = char>) -> Vec<char> {
    cs.fold(Vec::new(), |mut us, u| {
        match us.last() {
            Some(top) if unit_react(*top, u) => {
                us.pop();
            }
            _ => {
                us.push(u);
            }
        };

        us
    })
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    let line = input.lines().nth(0).expect("No line to parse").chars();
    let units_n = polymer_react(line.clone()).len();

    println!("{}", units_n);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{polymer_react, unit_react};

    #[test]
    fn test_do_react() {
        assert!(unit_react('a', 'A'));
        assert!(unit_react('A', 'a'));
    }

    #[test]
    fn test_do_not_react() {
        assert!(!unit_react('a', 'a'));
        assert!(!unit_react('A', 'A'));
        assert!(!unit_react('a', 'b'));
        assert!(!unit_react('A', 'b'));
        assert!(!unit_react('A', 'B'));
        assert!(!unit_react('a', 'B'));
    }

    #[test]
    fn test_polymer_react() {
        let pr = |s: &str| polymer_react(s.chars()).into_iter().collect::<String>();

        assert_eq!(pr("aA"), "");
        assert_eq!(pr("abBA"), "");
        assert_eq!(pr("abAB"), "abAB");
        assert_eq!(pr("aabAAB"), "aabAAB");
        assert_eq!(pr("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
    }
}
