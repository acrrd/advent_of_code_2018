use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Claim {
    fn new(id: u32, x: u32, y: u32, w: u32, h: u32) -> Claim {
        Claim { id, x, y, w, h }
    }
}

fn parse_claim(line: &str) -> Claim {
    let parts: Vec<&str> = line.split(' ').collect();

    let id = parts[0][1..].parse::<u32>().unwrap();
    let cords_len = parts[2].len();
    let cords: Vec<u32> = (parts[2][..cords_len - 1])
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let dims: Vec<u32> = parts[3]
        .split('x')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    Claim::new(id, cords[0], cords[1], dims[0], dims[1])
}

fn register_claim(reg: HashMap<(u32, u32), u32>, claim: &Claim) -> HashMap<(u32, u32), u32> {
    use itertools::Itertools;

    (0..claim.w)
        .into_iter()
        .cartesian_product(0..claim.h)
        .fold(reg, |mut reg, (x, y)| {
            let cord = (claim.x + x, claim.y + y);
            *reg.entry(cord).or_insert(0) += 1;
            reg
        })
}

fn register_claims(claimsstr: &str) -> HashMap<(u32, u32), u32> {
    claimsstr
        .lines()
        .map(parse_claim)
        .fold(HashMap::new(), |r, c| register_claim(r, &c))
}

fn count_overlapping_claims(claimstr: &str) -> u32 {
    register_claims(claimstr).values().filter(|n| **n > 1).map(|_| 1).sum()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    println!("{}", count_overlapping_claims(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count_overlapping_claims, parse_claim, register_claim, register_claims, Claim};
    use std::collections::HashMap;

    #[test]
    fn test_parse_claim() {
        let tests = vec![
            ("#1 @ 1,1: 1x1", Claim::new(1, 1, 1, 1, 1)),
            ("#12 @ 12,12: 12x12", Claim::new(12, 12, 12, 12, 12)),
            (
                "#123 @ 123,123: 123x123",
                Claim::new(123, 123, 123, 123, 123),
            ),
            (
                "#1234 @ 1234,1234: 1234x1234",
                Claim::new(1234, 1234, 1234, 1234, 1234),
            ),
        ];

        tests
            .iter()
            .for_each(|(s, c)| assert_eq!(parse_claim(s), *c));
    }

    #[test]
    fn test_register_claim_rect1() {
        let claim = Claim::new(1, 1, 1, 1, 3);
        let mut result = HashMap::new();
        result.insert((1, 1), 1);
        result.insert((1, 2), 1);
        result.insert((1, 3), 1);

        assert_eq!(register_claim(HashMap::new(), &claim), result);
    }

    #[test]
    fn test_register_claim_rect2() {
        let claim = Claim::new(1, 1, 1, 3, 1);
        let mut result = HashMap::new();
        result.insert((1, 1), 1);
        result.insert((2, 1), 1);
        result.insert((3, 1), 1);

        assert_eq!(register_claim(HashMap::new(), &claim), result);
    }

    #[test]
    fn test_register_claim_same_claim() {
        let claims = vec![Claim::new(1, 1, 1, 1, 3), Claim::new(1, 1, 1, 1, 3)];
        let mut result = HashMap::new();
        result.insert((1, 1), 2);
        result.insert((1, 2), 2);
        result.insert((1, 3), 2);

        assert_eq!(
            claims
                .into_iter()
                .fold(HashMap::new(), |r, c| register_claim(r, &c)),
            result
        );
    }

    #[test]
    fn test_register_claim_overlapping_claim() {
        let claims = vec![Claim::new(1, 1, 1, 1, 3), Claim::new(1, 1, 1, 3, 1)];
        let mut result = HashMap::new();
        result.insert((1, 1), 2);
        result.insert((1, 2), 1);
        result.insert((1, 3), 1);
        result.insert((2, 1), 1);
        result.insert((3, 1), 1);

        assert_eq!(
            claims
                .into_iter()
                .fold(HashMap::new(), |r, c| register_claim(r, &c)),
            result
        );
    }

    /*
    ........
    ...2222.
    ...2222.
    .11XX22.
    .11XX22.
    .111133.
    .111133.
    ........
    */
    #[test]
    fn test_register_claim_example() {
        let claims = vec![
            Claim::new(1, 1, 3, 4, 4),
            Claim::new(2, 3, 1, 4, 4),
            Claim::new(3, 5, 5, 2, 2),
        ];

        let result = claims
            .into_iter()
            .fold(HashMap::new(), |r, c| register_claim(r, &c));

        vec![
            Claim::new(1, 1, 3, 2, 4),
            Claim::new(1, 1, 5, 6, 2),
            Claim::new(1, 3, 1, 4, 2),
            Claim::new(1, 5, 1, 2, 6),
            Claim::new(1, 5, 5, 2, 2),
        ]
        .into_iter()
        .for_each(|claim| {
            register_claim(HashMap::new(), &claim)
                .iter()
                .for_each(|(cord, claims_n)| {
                    assert_eq!(result.get(cord).unwrap(), claims_n);
                });
        });

        vec![Claim::new(1, 3, 3, 2, 2), Claim::new(1, 3, 3, 2, 2)]
            .iter()
            .fold(HashMap::new(), |r, c| register_claim(r, &c))
            .iter()
            .for_each(|(cord, claims_n)| {
                println!("{:?} {}", cord, claims_n);
                assert_eq!(result.get(cord).unwrap(), claims_n);
            });
    }

    #[test]
    fn test_register_claims_example() {
        let claims = "#1 @ 1,3: 4x4\n\
                      #2 @ 3,1: 4x4\n\
                      #3 @ 5,5: 2x2";

        let result = register_claims(claims);

        vec![
            Claim::new(1, 1, 3, 2, 4),
            Claim::new(1, 1, 5, 6, 2),
            Claim::new(1, 3, 1, 4, 2),
            Claim::new(1, 5, 1, 2, 6),
            Claim::new(1, 5, 5, 2, 2),
        ]
        .into_iter()
        .for_each(|claim| {
            register_claim(HashMap::new(), &claim)
                .iter()
                .for_each(|(cord, claims_n)| {
                    assert_eq!(result.get(cord).unwrap(), claims_n);
                });
        });

        vec![Claim::new(1, 3, 3, 2, 2), Claim::new(1, 3, 3, 2, 2)]
            .iter()
            .fold(HashMap::new(), |r, c| register_claim(r, &c))
            .iter()
            .for_each(|(cord, claims_n)| {
                println!("{:?} {}", cord, claims_n);
                assert_eq!(result.get(cord).unwrap(), claims_n);
            });
    }

    #[test]
    fn test_count_overlapping_claims() {
        let claims = "#1 @ 1,3: 4x4\n\
                      #2 @ 3,1: 4x4\n\
                      #3 @ 5,5: 2x2";

        assert_eq!(count_overlapping_claims(claims), 4);
    }
}
