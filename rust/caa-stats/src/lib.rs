use chrono::{Duration, NaiveTime};

pub const TIME_FMT: &str = "%H|%M|%S";

pub fn stati(strg: &str) -> String {
    if strg.is_empty() {
        return strg.into();
    }

    let t0 = NaiveTime::from_hms(0, 0, 0);

    let mut ts: Vec<Duration> = strg
        .split(", ")
        .filter_map(|t| NaiveTime::parse_from_str(t, TIME_FMT).ok())
        .map(|t| t - t0)
        .collect();

    ts.sort_unstable();

    let range = t0 + (*ts.last().unwrap() - *ts.first().unwrap());

    let n = ts.len();

    let avg = ts.iter().map(Duration::num_seconds).sum::<i64>() / (n as i64);
    let avg = t0 + Duration::seconds(avg);

    let median = t0
        + if n % 2 == 0 {
            Duration::seconds((ts[n / 2 - 1] + ts[n / 2]).num_seconds() / 2)
        } else {
            ts[n / 2]
        };

    format!(
        "Range: {} Average: {} Median: {}",
        range.format(TIME_FMT),
        avg.format(TIME_FMT),
        median.format(TIME_FMT)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(strg: &str, exp: &str) -> () {
        println!(" str: {:?};", strg);
        let ans = stati(strg);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(
            "01|15|59, 1|47|16, 01|17|20, 1|32|34, 2|17|17",
            "Range: 01|01|18 Average: 01|38|05 Median: 01|32|34",
        );
        dotest(
            "02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00, 2|31|41",
            "Range: 00|31|17 Average: 02|26|18 Median: 02|22|00",
        );
        dotest(
            "02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|32|34, 2|17|17",
            "Range: 00|31|17 Average: 02|27|10 Median: 02|24|57",
        );
    }
}
