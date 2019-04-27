use std::collections::LinkedList;
use std::io::{self, Read};

fn get_digits(mut n: usize) -> Vec<u8> {
    let mut digits = Vec::with_capacity(2);
    let mut push = |n| {
        digits.push((n % 10) as u8);
    };

    if n == 0 {
        push(n);
    }

    while n != 0 {
        push(n);
        n /= 10;
    }

    digits.into_iter().rev().collect()
}

struct Scoreboard {
    scores: Vec<u8>,
    a_idx: usize,
    b_idx: usize,
}

struct ScoreboardIter<'a> {
    scoreboard: &'a mut Scoreboard,
    iter_pos: usize,
}

impl Scoreboard {
    fn new() -> Scoreboard {
        Scoreboard {
            scores: vec![3, 7],
            a_idx: 0,
            b_idx: 1,
        }
    }

    fn len(&self) -> usize {
        self.scores.len()
    }

    fn iter<'a>(&'a mut self) -> ScoreboardIter<'a> {
        ScoreboardIter {
            scoreboard: self,
            iter_pos: 0,
        }
    }
}

impl<'a> Iterator for ScoreboardIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let a_idx = self.scoreboard.a_idx;
        let b_idx = self.scoreboard.b_idx;

        if self.iter_pos == self.scoreboard.len() {
            let a_score = self.scoreboard.scores[a_idx];
            let b_score = self.scoreboard.scores[b_idx];
            let new_score = a_score + b_score;
            let mut digits = get_digits(new_score as usize);
            self.scoreboard.scores.append(&mut digits);

            let len = self.scoreboard.len();
            self.scoreboard.a_idx = (a_idx + a_score as usize + 1) % len;
            self.scoreboard.b_idx = (b_idx + b_score as usize + 1) % len;
        }

        let digit = self.scoreboard.scores[self.iter_pos];
        self.iter_pos += 1;

        Some(digit)
    }
}

fn next_10_scores(limit: usize) -> Vec<u8> {
    Scoreboard::new().iter().skip(limit).take(10).collect()
}

fn find_in_scoreboard(target: &Vec<u8>) -> usize {
    let mut scoreboard = Scoreboard::new();
    let mut window: LinkedList<u8> = scoreboard.iter().take(target.len()).collect();

    let mut it = scoreboard.iter();
    loop {
        if window.iter().enumerate().all(|(idx, &v)| target[idx] == v) {
            return scoreboard.len() - target.len();
        }

        // the iterator will always return a score
        let digit = it.next().unwrap();
        window.pop_front();
        window.push_back(digit);
    }
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input.trim();

    let limit = input.parse::<usize>().expect("Number of recipes");

    next_10_scores(limit).iter().for_each(|score| {
        print!("{}", score);
    });
    println!("");

    let digits = get_digits(limit);
    println!("{}", find_in_scoreboard(&digits));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{find_in_scoreboard, get_digits, next_10_scores, Scoreboard};

    #[test]
    fn test_get_digits() {
        (0..10).for_each(|n| {
            let digits = get_digits(n);
            assert_eq!(digits.len(), 1);
            assert_eq!(get_digits(n)[0], n as u8);
        });

        (1..10).for_each(|d| {
            (0..10).for_each(|n| {
                let digits = get_digits(d * 10 + n);
                assert_eq!(digits.len(), 2);
                assert_eq!(digits[0], d as u8);
                assert_eq!(digits[1], n as u8);
            });
        });
    }

    #[test]
    fn test_compute_scoreboard() {
        let scoreboard = |n| Scoreboard::new().iter().take(n).collect::<Vec<_>>();

        assert_eq!(scoreboard(0), vec![]);
        assert_eq!(scoreboard(2), vec![3, 7]);
        assert_eq!(scoreboard(5), vec![3, 7, 1, 0, 1]);
        assert_eq!(scoreboard(6), vec![3, 7, 1, 0, 1, 0]);
        assert_eq!(scoreboard(10), vec![3, 7, 1, 0, 1, 0, 1, 2, 4, 5]);
        assert_eq!(
            scoreboard(14),
            vec![3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9]
        );
        assert_eq!(
            scoreboard(19),
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

    #[test]
    fn test_find_in_scoreboard() {
        assert_eq!(find_in_scoreboard(&vec![5, 1, 5, 8, 9]), 9);
        assert_eq!(find_in_scoreboard(&vec![0, 1, 2, 4, 5]), 5);
        assert_eq!(find_in_scoreboard(&vec![9, 2, 5, 1, 0]), 18);
        assert_eq!(find_in_scoreboard(&vec![5, 9, 4, 1, 4]), 2018);
    }
}
