// FIXME: Make me pass! Diff budget: 25 lines.

enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

fn convert(event: Duration) -> u64 {
    match event {
        Duration::MilliSeconds(s) => s,
        Duration::Seconds(s) => s as u64 * 1000,
        Duration::Minutes(s) => s as u64 * 60 * 1000,
    }
}

fn main() {
    assert_eq!(convert(Duration::Seconds(120)), convert(Duration::Minutes(2)));
    assert_eq!(convert(Duration::Seconds(420)), convert(Duration::Minutes(7)));
    assert_eq!(convert(Duration::MilliSeconds(420000)), convert(Duration::Minutes(7)));
    assert_eq!(convert(Duration::MilliSeconds(43000)), convert(Duration::Seconds(43)));
}
