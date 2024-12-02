use itermore::prelude::*;

type Level = i32;
type Report = Vec<Level>;

fn parse_input(input: String) -> Vec<Report> {
    input
        .split("\n")
        .into_iter()
        .filter(|x| x.trim().len() > 0)
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Report>()
        })
        .collect::<Vec<Report>>()
}

fn part_01(reports: Vec<Report>) -> i32 {
    let mut safe_count = 0;

    for report in reports {
        assert!(report.len() > 1);

        let mut safe = true;

        let ascending = report[0] < report[1];

        for i in 0..report.len() - 1 {
            if ascending != (report[i] < report[i + 1]) {
                safe = false;
                break;
            }

            let diff = report[i].abs_diff(report[i + 1]);
            safe = diff >= 1 && diff <= 3;
            if !safe {
                break;
            }
        }

        if safe {
            safe_count += 1;
        }
    }

    safe_count
}

fn part_02(reports: Vec<Report>) -> i32 {
    let mut safe_count = 0;

    for report in reports {
        assert!(report.len() > 1);

        let mut damper_safe = false;
        let indexes = 0..report.len();

        for i in indexes.clone() {
            let levels = indexes
                .clone()
                .into_iter()
                .filter(|x| *x != i)
                .map(|x| report[x])
                .collect::<Vec<_>>();

            let mut safe = true;
            let ascending = levels[0] < levels[1];

            for [j, k] in levels.iter().array_windows() {
                if ascending != (j < k) {
                    safe = false;
                    break;
                }

                let diff = j.abs_diff(*k);

                safe = diff >= 1 && diff <= 3;

                if !safe {
                    break;
                }
            }

            damper_safe = safe;
            if damper_safe {
                break;
            }
        }

        if damper_safe {
            safe_count += 1;
        }
    }

    safe_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = include_str!("../inputs/day02/sample.txt");
        let reports = parse_input(input.to_string());

        let safe_count = part_01(reports);
        assert_eq!(safe_count, 2);
    }

    #[test]
    fn part2_sample() {
        let input = include_str!("../inputs/day02/sample.txt");
        let reports = parse_input(input.to_string());

        let safe_count = part_02(reports);
        assert_eq!(safe_count, 4);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day02/input.txt");
        let reports = parse_input(input.to_string());

        let safe_count = part_01(reports);
        assert_eq!(safe_count, 332);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day02/input.txt");
        let reports = parse_input(input.to_string());

        let safe_count = part_02(reports);
        assert_eq!(safe_count, 398);
    }
}
