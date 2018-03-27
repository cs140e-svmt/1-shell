// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        let durations = [self, other];
        let mut iter = durations.iter().map(|d| match **d {
            MilliSeconds(ms) => ms,
            Seconds(s) => s as u64 * 1000,
            Minutes(m) => m as u64 * 60 * 1000
        });
        iter.next() == iter.next()
    }
}

use Duration::*;

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
