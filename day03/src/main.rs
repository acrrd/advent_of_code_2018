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

fn main() {}

#[cfg(test)]
mod tests {
    use super::{parse_claim, Claim};

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
}
