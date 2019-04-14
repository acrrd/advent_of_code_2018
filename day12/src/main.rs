struct Pots {
    list: Vec<bool>,
    zero_pos: usize,
}

impl Pots {
    fn new(pots: &[bool]) -> Pots {
        if pots.len() == 0 {
            panic!("There must be at least one pot");
        }
        Pots {
            list: Vec::from(pots),
            zero_pos: 0,
        }
    }
}

// ensure that there are at least 4 empty pots at the begin and at the end
fn ensure_empty_pots(pots: Pots) -> Pots {
    let first_full = pots.list.iter().take(4).position(|p| *p).map(|p| 4 - p);
    let last_full = pots
        .list
        .iter()
        .rev()
        .take(4)
        .position(|p| *p)
        .map(|p| (p as i32 - 4).abs() as usize);

    let to_add_at =
        |pos: Option<usize>| if let Some(p) = pos { (0..p) } else { (0..0) }.map(|_| false);

    let list = to_add_at(first_full)
        .chain(pots.list.into_iter())
        .chain(to_add_at(last_full))
        .collect::<Vec<_>>();

    let zero_pos = pots.zero_pos + first_full.unwrap_or(0);

    Pots { list, zero_pos }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{Pots, ensure_empty_pots};

    const T: bool = true;
    const F: bool = false;

    #[test]
    fn test_ensure_empty_posts_one_true() {
        let pots = Pots::new(&[T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 4);

        let pots = Pots::new(&[F, T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 3);

        let pots = Pots::new(&[F, F, T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 2);

        let pots = Pots::new(&[F, F, F, T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 1);

        let pots = Pots::new(&[F, F, F, F, T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 0);

        let pots = Pots::new(&[F, F, F, F, F, T]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 0);

        let pots = Pots::new(&[T, F]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 4);

        let pots = Pots::new(&[T, F, F]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 4);

        let pots = Pots::new(&[T, F, F, F, F]);
        let pots = ensure_empty_pots(pots);

        assert_eq!(pots.list, vec![F, F, F, F, T, F, F, F, F]);
        assert_eq!(pots.zero_pos, 4);
    }
}
