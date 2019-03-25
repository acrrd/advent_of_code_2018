#[derive(PartialEq, Debug)]
enum Event {
    Start(u32), // start of a shift
    Asleep(u8),
    Awake(u8),
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

fn main() {}

#[cfg(test)]
mod tests {
    use super::{parse_event, parse_events,Event};

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
}
