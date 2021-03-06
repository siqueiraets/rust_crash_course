#[derive(Debug, PartialEq)]
pub struct Time {
    ms: u64,
}

impl Time {
    fn new(ms: u64) -> Self {
        Time { ms }
    }
}

fn in_hours(t: Time) -> u64 {
    unimplemented!();
}

fn greater(t1: Time, t2: Time) -> Time {
    unimplemented!();
}

// implement without cloning
fn greatest(v: Vec<Time>) -> Time {
    unimplemented!();
}

// references
fn time_diff_in_ms(t1: &Time, t2: &Time) -> u64 {
    unimplemented!();
}

fn greatest_ref(v: &Vec<Time>) -> &Time {
    unimplemented!();
}

fn main() {}

mod test {
    use super::*;
    #[test]
    fn test_in_hours() {
        let t = Time {
            ms: 3600 * 1000 * 5,
        };
        assert_eq!(5, in_hours(t));
    }

    #[test]
    fn test_greater() {
        let t1 = Time::new(5);
        let t2 = Time::new(1);
        let res = greater(t1, t2);
        assert_eq!(5, res.ms);
    }

    #[test]
    fn test_greatest() {
        let v = vec![Time::new(1), Time::new(5)];
        let res = greatest(v);
        assert_eq!(5, res.ms);
    }

    #[test]
    fn test_time_diff() {
        let t1 = Time::new(1);
        let t2 = Time::new(10);
        assert_eq!(9, time_diff_in_ms(&t1, &t2));
        assert_eq!(9, time_diff_in_ms(&t2, &t1));
    }

    #[test]
    fn test_greatest_ref() {
        let v = vec![Time::new(1), Time::new(5)];
        let res = greatest_ref(&v);
        assert_eq!(5, res.ms);
    }
}
