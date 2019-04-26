use std::io::{self, Read};

fn get_digits(mut n: u8) -> Vec<u8> {
    let mut digits = Vec::with_capacity(2);

    if n == 0 {
        digits.push(n % 10);
    }

    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits.into_iter().rev().collect()
}

// the scoreboard can be bigger then 'limit'
// because we always add all the digits
fn compute_scoreboard(limit: usize) -> Vec<u8> {
    let mut scoreboard: Vec<u8> = Vec::with_capacity(limit * 2);
    scoreboard.push(3);
    scoreboard.push(7);

    let mut a_idx = 0;
    let mut b_idx = 1;

    while scoreboard.len() < limit {
        let a_score = scoreboard[a_idx];
        let b_score = scoreboard[b_idx];
        let new_score = a_score + b_score;
        let mut digits = get_digits(new_score);
        scoreboard.append(&mut digits);

        a_idx = (a_idx + a_score as usize + 1) % scoreboard.len();
        b_idx = (b_idx + b_score as usize + 1) % scoreboard.len();
    }

    scoreboard
}

fn next_10_scores(limit: usize) -> Vec<u8> {
    compute_scoreboard(limit + 10)
        .into_iter()
        .skip(limit)
        .take(10)
        .collect()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input.trim();

    let limit = input.parse::<usize>().expect("Number of recipes");

    next_10_scores(limit).iter().for_each(|score| { print!("{}", score); });
    println!("");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{compute_scoreboard, get_digits, next_10_scores};

    #[test]
    fn test_get_digits() {
        (0..10).for_each(|n| {
            let digits = get_digits(n);
            assert_eq!(digits.len(), 1);
            assert_eq!(get_digits(n)[0], n);
        });

        (1..10).for_each(|d| {
            (0..10).for_each(|n| {
                let digits = get_digits(d * 10 + n);
                assert_eq!(digits.len(), 2);
                assert_eq!(digits[0], d);
                assert_eq!(digits[1], n);
            });
        });
    }

    #[test]
    fn test_compute_scoreboard() {
        assert_eq!(compute_scoreboard(0), vec![3, 7]);
        assert_eq!(compute_scoreboard(5), vec![3, 7, 1, 0, 1, 0]);
        assert_eq!(compute_scoreboard(6), vec![3, 7, 1, 0, 1, 0]);
        assert_eq!(compute_scoreboard(10), vec![3, 7, 1, 0, 1, 0, 1, 2, 4, 5]);
        assert_eq!(
            compute_scoreboard(14),
            vec![3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9]
        );
        assert_eq!(
            compute_scoreboard(19),
            vec![3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9]
        );
    }

    #[test]
    fn test_next_10_scores() {
        let scoreboard = next_10_scores(9);
        assert_eq!(scoreboard, vec![5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);

        let scoreboard = next_10_scores(5);
        assert_eq!(scoreboard, vec![0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);

        let scoreboard = next_10_scores(18);
        assert_eq!(scoreboard, vec![9, 2, 5, 1, 0, 7, 1, 0, 8, 5]);

        let scoreboard = next_10_scores(2018);
        assert_eq!(scoreboard, vec![5, 9, 4, 1, 4, 2, 9, 8, 8, 2]);
    }
}
