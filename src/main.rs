#![allow(dead_code)]

mod day_1;
mod day_2;
mod day_3;
mod util;

fn main() {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("run") => {
            match args.next().as_deref() {
                Some("1") => {
                    util::run_solution(1, day_1::part_1::solution);
                    util::run_solution(1, day_1::part_2::solution);
                },
                Some("2") => {
                    util::run_solution(2, day_2::part_1::solution);
                    util::run_solution(2, day_2::part_2::solution);
                },
                Some("3") => {
                    util::run_solution(3, day_3::part_1::solution);
                    util::run_solution(3, day_3::part_2::solution);
                }
                _ => panic!("unknown day"),
            }
        }
        Some("bench") => {
            match args.next().as_deref() {
                Some("1") => {
                    util::bench_solution(1, day_1::part_1::solution);
                    util::bench_solution(1, day_1::part_2::solution);
                },
                Some("2") => {
                    util::bench_solution(2, day_2::part_1::solution);
                    util::bench_solution(2, day_2::part_2::solution);
                },
                Some("3") => {
                    util::bench_solution(3, day_3::part_1::solution);
                    util::bench_solution(3, day_3::part_2::solution);
                },
                Some("all") => {
                    util::bench_solution(1, day_1::part_1::solution);
                    util::bench_solution(1, day_1::part_2::solution);
                    util::bench_solution(2, day_2::part_1::solution);
                    util::bench_solution(2, day_2::part_2::solution);
                    util::bench_solution(3, day_3::part_1::solution);
                    util::bench_solution(3, day_3::part_2::solution);
                }
                _ => panic!("unknown day"),
            }
        }
        _ => panic!("unknown command"),
    };
}
