use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

fn code_occurrences(code: &str) -> HashSet<u32> {
    code.chars()
        .fold(HashMap::new(), |mut lc, l| {
            *lc.entry(l).or_insert(0) += 1;
            lc
        })
        .values()
        .cloned()
        .collect()
}

fn codes_occurrences_count(codes: &str) -> HashMap<u32, u32> {
    codes
        .lines()
        .map(code_occurrences)
        .fold(HashMap::new(), |oc, hs: HashSet<u32>| {
            hs.into_iter().fold(oc, |mut oc, o| {
                *oc.entry(o).or_insert(0) += 1;
                oc
            })
        })
}

fn checksum(codes: &str) -> u32 {
    let oc = codes_occurrences_count(codes);
    oc.get(&2).unwrap_or(&0) * oc.get(&3).unwrap_or(&0)
}

fn common_chars(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|(ac, bc)| ac == bc)
        .map(|(a, _)| a)
        .collect()
}

fn find_codes_common_chars(codes: &str) -> String {
    use itertools::Itertools;

    codes
        .lines()
        .cartesian_product(codes.lines().skip(1))
        .map(|(a, b)| (a, common_chars(a, b)))
        .filter(|(a, b)| a.len() - 1 == b.len())
        .map(|(_, b)| b)
        .nth(0)
        .unwrap()
        .to_string()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    println!("{}", checksum(&input));
    println!("{}", find_codes_common_chars(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use super::{
        checksum, code_occurrences, codes_occurrences_count, common_chars, find_codes_common_chars,
    };

    #[test]
    fn test_code_occurrences() {
        let tests: Vec<(&str, Vec<u32>)> = vec![
            ("abcdef", [1].to_vec()),
            ("bababc", [1, 2, 3].to_vec()),
            ("abbcde", [1, 2].to_vec()),
            ("abcccd", [1, 3].to_vec()),
            ("aabcdd", [1, 2].to_vec()),
            ("abcdee", [1, 2].to_vec()),
            ("ababab", [3].to_vec()),
        ];

        tests.iter().for_each(|(str, os)| {
            assert_eq!(
                code_occurrences(str),
                HashSet::from_iter(os.iter().cloned())
            );
        });
    }

    static CODES: &str = "abcdef\n\
                          bababc\n\
                          abbcde\n\
                          abcccd\n\
                          aabcdd\n\
                          abcdee\n\
                          ababab";

    #[test]
    fn test_codes_occurences_count() {
        let oc = codes_occurrences_count(CODES);

        assert_eq!(*oc.get(&2).unwrap(), 4);
        assert_eq!(*oc.get(&3).unwrap(), 3);
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum(CODES), 12);
    }

    #[test]
    fn test_common_chars() {
        let tests: Vec<(&str, &str, &str)> = vec![
            ("asdf", "asdf", "asdf"),
            ("asdf", "asdg", "asd"),
            ("asdf", "asgf", "asf"),
            ("asdf", "agdf", "adf"),
            ("asdf", "gsdf", "sdf"),
            ("asdf", "zzzz", ""),
        ];

        tests.iter().for_each(|(a, b, t)| {
            assert_eq!(common_chars(a, b), t.to_string());
        })
    }

    #[test]
    fn test_find_codes_common_chars() {
        let codes: &str = "abcde\n\
                           fghij\n\
                           klmno\n\
                           pqrst\n\
                           fguij\n\
                           axcye\n\
                           wvxyz";

        assert_eq!(find_codes_common_chars(codes), "fgij".to_string());
    }
}
