use std::io::{self, Read};
use std::collections::LinkedList;

fn get_digits(mut n: usize) -> Vec<u8> {
    let mut digits = Vec::with_capacity(2);
    let mut push = |n| { digits.push((n%10) as u8); };

    if n == 0 {
        push(n);
    }

    while n != 0 {
        push(n);
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
        let mut digits = get_digits(new_score as usize);
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

fn find_in_scoreboard(target: &Vec<u8>) -> usize  {
    let mut scoreboard: Vec<u8> = Vec::new();
    scoreboard.push(3);
    scoreboard.push(7);

    let mut a_idx = 0;
    let mut b_idx = 1;

    let mut window: LinkedList<u8> = LinkedList::new();
    loop {

        let a_score = scoreboard[a_idx];
        let b_score = scoreboard[b_idx];
        let new_score = a_score + b_score;
        let mut digits = get_digits(new_score as usize);
        for (offset, &d) in digits.iter().enumerate() {
            window.push_back(d);
            if window.len() > target.len() {
                window.pop_front();
            }
            if window.iter().enumerate().all(|(idx, &v)| target[idx] == v) {
                return scoreboard.len() + offset + 1 - target.len();
            }
        }

        scoreboard.append(&mut digits);

        a_idx = (a_idx + a_score as usize + 1) % scoreboard.len();
        b_idx = (b_idx + b_score as usize + 1) % scoreboard.len();
    }

}


fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input.trim();

    let limit = input.parse::<usize>().expect("Number of recipes");

    next_10_scores(limit).iter().for_each(|score| { print!("{}", score); });
    println!("");

    let digits = get_digits(limit);
    println!("{:?} {}", digits, find_in_scoreboard(&digits));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{find_in_scoreboard, compute_scoreboard, get_digits, next_10_scores};

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

    #[test]
    fn test_find_in_scoreboard() {
        assert_eq!(find_in_scoreboard(&vec![5,1,5,8,9]), 9);
        assert_eq!(find_in_scoreboard(&vec![0,1,2,4,5]), 5);
        assert_eq!(find_in_scoreboard(&vec![9,2,5,1,0]), 18);
        assert_eq!(find_in_scoreboard(&vec![5,9,4,1,4]), 2018);
    }
}
