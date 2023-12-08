#![allow(dead_code)]

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod util;

fn main() {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("run") => match args.next().as_deref() {
            Some("1") => {
                util::run_solution(1, day_1::part_1::solution);
                util::run_solution(1, day_1::part_2::solution);
            }
            Some("2") => {
                util::run_solution(2, day_2::part_1::solution);
                util::run_solution(2, day_2::part_2::solution);
            }
            Some("3") => {
                util::run_solution(3, day_3::part_1::solution);
                util::run_solution(3, day_3::part_2::solution);
            }
            Some("4") => {
                util::run_solution(4, day_4::part_1::solution);
                util::run_solution(4, day_4::part_2::solution);
            }
            Some("5") => {
                util::run_solution(5, day_5::part_1::solution);
                util::run_solution(5, day_5::part_2::solution);
            }
            Some("6") => {
                util::run_solution(6, day_6::part_1::solution);
                util::run_solution(6, day_6::part_2::solution);
            }
            Some("7") => {
                util::run_solution(7, day_7::part_1::solution);
                util::run_solution(7, day_7::part_2::solution);
            }
            Some("8") => {
                util::run_solution(8, day_8::part_1::solution);
                util::run_solution(8, day_8::part_2::solution);
            }
            _ => panic!("unknown day"),
        },
        Some("bench") => match args.next().as_deref() {
            Some("1") => {
                util::bench_solution(1, day_1::part_1::solution);
                util::bench_solution(1, day_1::part_2::solution);
            }
            Some("2") => {
                util::bench_solution(2, day_2::part_1::solution);
                util::bench_solution(2, day_2::part_2::solution);
            }
            Some("3") => {
                util::bench_solution(3, day_3::part_1::solution);
                util::bench_solution(3, day_3::part_2::solution);
            }
            Some("4") => {
                util::bench_solution(4, day_4::part_1::solution);
                util::bench_solution(4, day_4::part_2::solution);
            }
            Some("5") => {
                util::bench_solution(5, day_5::part_1::solution);
                util::bench_solution(5, day_5::part_2::solution);
            }
            Some("6") => {
                util::bench_solution(6, day_6::part_1::solution);
                util::bench_solution(6, day_6::part_2::solution);
            }
            Some("7") => {
                util::bench_solution(7, day_7::part_1::solution);
                util::bench_solution(7, day_7::part_2::solution);
            }
            Some("8") => {
                util::bench_solution(8, day_8::part_1::solution);
                util::bench_solution(8, day_8::part_2::solution);
            }
            Some("all") => {
                util::bench_solution(1, day_1::part_1::solution);
                util::bench_solution(1, day_1::part_2::solution);
                util::bench_solution(2, day_2::part_1::solution);
                util::bench_solution(2, day_2::part_2::solution);
                util::bench_solution(3, day_3::part_1::solution);
                util::bench_solution(3, day_3::part_2::solution);
                util::bench_solution(4, day_4::part_1::solution);
                util::bench_solution(4, day_4::part_2::solution);
                util::bench_solution(5, day_5::part_1::solution);
                util::bench_solution(5, day_5::part_2::solution);
                util::bench_solution(6, day_6::part_1::solution);
                util::bench_solution(6, day_6::part_2::solution);
                util::bench_solution(7, day_7::part_1::solution);
                util::bench_solution(7, day_7::part_2::solution);
                util::bench_solution(8, day_8::part_1::solution);
                util::bench_solution(8, day_8::part_2::solution);
            }
            _ => panic!("unknown day"),
        },
        _ => panic!("unknown command"),
    };
}
