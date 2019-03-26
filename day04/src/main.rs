use std::collections::HashMap;
use std::io::{self, Read};

#[derive(PartialEq, Debug)]
enum Event {
    Start(u32), // start of a shift
    Asleep(u8),
    Awake(u8),
}

struct Guard {
    id: u32,
    minutes_slept: [u32; 60],
}

impl Guard {
    fn new(id: u32) -> Guard {
        Guard {
            id,
            minutes_slept: [0; 60],
        }
    }
}

fn parse_guard_id(cs: Vec<&str>) -> u32 {
    cs[3][1..].parse::<u32>().expect("Error parsing guard id")
}

fn parse_minutes(cs: Vec<&str>) -> u8 {
    cs[1][3..5].parse::<u8>().expect("Error parsing minutes")
}

fn parse_event(line: &str) -> Event {
    let cs: Vec<&str> = line.split(" ").collect();

    match cs[2] {
        "Guard" => Event::Start(parse_guard_id(cs)),
        "falls" => Event::Asleep(parse_minutes(cs)),
        "wakes" => Event::Awake(parse_minutes(cs)),
        _ => panic!("Event {} not reconized", cs[2]),
    }
}

// return events _in chronological order_
fn parse_events<'a>(input: &'a str) -> impl Iterator<Item = Event> + 'a {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort_unstable();
    lines.into_iter().map(parse_event)
}

fn get_guards_stats(events: impl Iterator<Item = Event>) -> HashMap<u32, Guard> {
    #[derive(Debug)]
    enum State {
        Init,
        CurrentGuard(u32),
        Asleep(u32, u8), // guard_id, minute
    };

    events
        .fold(
            (HashMap::new(), State::Init),
            |(mut guards, state), e| match e {
                Event::Start(id) => {
                    guards.entry(id).or_insert(Guard::new(id));
                    let next = match state {
                        State::Init | State::CurrentGuard(_) => State::CurrentGuard(id),
                        _ => panic!("Invalid state {:?} with event {:?}", state, e),
                    };
                    (guards, next)
                }
                Event::Asleep(minute) => {
                    let next = match state {
                        State::CurrentGuard(id) => State::Asleep(id, minute),
                        _ => panic!("Invalid state {:?} with event {:?}", state, e),
                    };
                    (guards, next)
                }
                Event::Awake(end_minute) => {
                    let next = match state {
                        State::Asleep(id, begin_minute) => {
                            let guard = guards
                                .get_mut(&id)
                                .expect(&format!("Cannot found Gaurd #{}", id));
                            (begin_minute..end_minute).for_each(|minute| {
                                guard.minutes_slept[minute as usize] += 1;
                            });
                            State::CurrentGuard(id)
                        }
                        _ => panic!("Invalid state {:?} with event {:?}", state, e),
                    };
                    (guards, next)
                }
            },
        )
        .0
}

fn find_sleepiest_guard(guards: &HashMap<u32, Guard>) -> &Guard {
    guards
        .values()
        .max_by_key(|guard| guard.minutes_slept.iter().sum::<u32>())
        .expect("Cannot find a guard")
}

fn find_favourite_minute(guard: &Guard) -> (u32, u32) {
    let (minute, times) = guard
        .minutes_slept
        .iter()
        .enumerate()
        .max_by_key(|&(_, times)| times)
        .expect("Cannot find favourite minute");

    (minute as u32, *times)
}

fn find_frequent_minute(guards: &HashMap<u32, Guard>) -> (u32, u32) {
    let (id, (minute, _)) = guards
        .values()
        .map(|guard| (guard.id, find_favourite_minute(guard)))
        .max_by_key(|&(_, (_, times))| times)
        .expect("Cannot find frequent minute");

    (id, minute)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input;

    let guards = get_guards_stats(parse_events(&input));
    let guard = find_sleepiest_guard(&guards);
    let (minute, _) = find_favourite_minute(guard);

    println!("{}", guard.id * minute);

    let (guard_id, minute) = find_frequent_minute(&guards);
    println!("{}", guard_id * minute);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        find_frequent_minute,
        find_favourite_minute, find_sleepiest_guard, get_guards_stats, parse_event, parse_events,
        Event,
    };

    #[test]
    fn test_parse_event() {
        [
            (
                "[1518-11-01 00:00] Guard #10 begins shift",
                Event::Start(10),
            ),
            ("[1518-11-01 00:05] falls asleep", Event::Asleep(5)),
            ("[1518-11-01 00:25] wakes up", Event::Awake(25)),
        ]
        .iter()
        .for_each(|(line, expect)| assert_eq!(parse_event(line), *expect));
    }

    #[test]
    fn test_parse_events() {
        let input = "[1518-11-01 00:25] wakes up\n\
                     [1518-11-01 00:00] Guard #10 begins shift\n\
                     [1518-11-01 00:05] falls asleep";
        let expect = vec![Event::Start(10), Event::Asleep(5), Event::Awake(25)];

        assert_eq!(parse_events(input).collect::<Vec<Event>>(), expect);
    }

    fn check_minutes(minutes_slept: &[u32; 60], minutes: impl Iterator<Item = usize>, eq: u32) {
        minutes.for_each(|minute| {
            assert_eq!(minutes_slept[minute], eq);
        });
    }

    #[test]
    fn test_get_guards_stats_one_shift() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift\n\
                     [1518-11-01 00:05] falls asleep\n\
                     [1518-11-01 00:25] wakes up";
        let guards = get_guards_stats(parse_events(input));
        let id = 10;
        assert!(guards.contains_key(&id));

        let guard = guards.get(&id).unwrap();
        assert_eq!(guard.id, id);
        check_minutes(&guard.minutes_slept, 5..25, 1);
        check_minutes(&guard.minutes_slept, (0..5).chain(25..60), 0);
    }

    fn test_get_guards_stats_two_shifts_common(input: &str) {
        let guards = get_guards_stats(parse_events(input));
        let id = 10;
        assert!(guards.contains_key(&id));

        let guard = guards.get(&id).unwrap();
        assert_eq!(guard.id, id);
        check_minutes(&guard.minutes_slept, (5..25).chain(40..50), 1);
        check_minutes(&guard.minutes_slept, (0..5).chain(25..40).chain(50..60), 0);
    }

    #[test]
    fn test_get_guards_stats_two_shifts_one_night() {
        test_get_guards_stats_two_shifts_common(
            "[1518-11-01 00:00] Guard #10 begins shift\n\
             [1518-11-01 00:05] falls asleep\n\
             [1518-11-01 00:25] wakes up\n\
             [1518-11-01 00:40] falls asleep\n\
             [1518-11-01 00:50] wakes up",
        );
    }

    #[test]
    fn test_get_guards_stats_two_shifts_two_night() {
        test_get_guards_stats_two_shifts_common(
            "[1518-11-01 00:00] Guard #10 begins shift\n\
             [1518-11-01 00:05] falls asleep\n\
             [1518-11-01 00:25] wakes up\n\
             [1518-11-02 00:00] Guard #10 begins shift\n\
             [1518-11-02 00:40] falls asleep\n\
             [1518-11-02 00:50] wakes up",
        );
    }

    #[test]
    fn test_get_guards_stats_two_shifts_two_night_early_begin() {
        test_get_guards_stats_two_shifts_common(
            "[1518-11-01 00:00] Guard #10 begins shift\n\
             [1518-11-01 00:05] falls asleep\n\
             [1518-11-01 00:25] wakes up\n\
             [1518-11-01 00:00] Guard #10 begins shift\n\
             [1518-11-02 00:40] falls asleep\n\
             [1518-11-02 00:50] wakes up",
        );
    }

    const EXAMPLE_INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift\n\
                                 [1518-11-01 00:05] falls asleep\n\
                                 [1518-11-01 00:25] wakes up\n\
                                 [1518-11-01 00:30] falls asleep\n\
                                 [1518-11-01 00:55] wakes up\n\
                                 [1518-11-01 23:58] Guard #99 begins shift\n\
                                 [1518-11-02 00:40] falls asleep\n\
                                 [1518-11-02 00:50] wakes up\n\
                                 [1518-11-03 00:05] Guard #10 begins shift\n\
                                 [1518-11-03 00:24] falls asleep\n\
                                 [1518-11-03 00:29] wakes up\n\
                                 [1518-11-04 00:02] Guard #99 begins shift\n\
                                 [1518-11-04 00:36] falls asleep\n\
                                 [1518-11-04 00:46] wakes up\n\
                                 [1518-11-05 00:03] Guard #99 begins shift\n\
                                 [1518-11-05 00:45] falls asleep\n\
                                 [1518-11-05 00:55] wakes up";

    #[test]
    fn test_get_guards_stats_two_guards() {
        let guards = get_guards_stats(parse_events(EXAMPLE_INPUT));
        let (a, b) = (10, 99);
        assert!(guards.contains_key(&a));
        assert!(guards.contains_key(&b));

        {
            let guard = guards.get(&a).unwrap();
            assert_eq!(guard.id, a);
            check_minutes(&guard.minutes_slept, (5..24).chain(25..29).chain(30..55), 1);
            assert_eq!(guard.minutes_slept[24], 2);
            check_minutes(&guard.minutes_slept, (0..5).chain(29..30).chain(55..60), 0);
        }

        {
            let guard = guards.get(&b).unwrap();
            assert_eq!(guard.id, b);
            check_minutes(&guard.minutes_slept, (36..40).chain(50..55), 1);
            check_minutes(&guard.minutes_slept, (40..45).chain(46..50), 2);
            assert_eq!(guard.minutes_slept[45], 3);
            check_minutes(&guard.minutes_slept, (0..36).chain(55..50), 0);
        }
    }

    #[test]
    fn test_find_sleepiest_gaurd() {
        let guards = get_guards_stats(parse_events(EXAMPLE_INPUT));
        let guard = find_sleepiest_guard(&guards);

        assert_eq!(guard.id, 10);
    }

    #[test]
    fn test_find_favourite_minute() {
        let guards = get_guards_stats(parse_events(EXAMPLE_INPUT));
        let guard = find_sleepiest_guard(&guards);

        assert_eq!(find_favourite_minute(guard).0, 24);
    }

    #[test]
    fn test_find_frequent_minute() {
        let guards = get_guards_stats(parse_events(EXAMPLE_INPUT));

        assert_eq!(find_frequent_minute(&guards), (99, 45));
    }
}
