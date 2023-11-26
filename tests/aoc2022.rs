use aoc2023_rust::{
    self,
    solutions::aoc2022::five::{solve_a, solve_b},
};

#[test]
pub fn test_one_a() {
    assert_eq!(
        aoc2023_rust::solutions::aoc2022::one::solve_a("./inputs/aoc2022/1.txt"),
        70764
    );
}
#[test]
pub fn test_one_b() {
    assert_eq!(
        aoc2023_rust::solutions::aoc2022::one::solve_b("./inputs/aoc2022/1.txt"),
        203905
    );
}

#[test]
pub fn test_two_a() {
    assert_eq!(
        aoc2023_rust::solutions::aoc2022::two::solve_a("./inputs/aoc2022/2_test.txt"),
        15
    );

    assert_eq!(
        aoc2023_rust::solutions::aoc2022::two::solve_a("./inputs/aoc2022/2.txt"),
        13809
    );
}

#[test]
pub fn test_two_b() {
    assert_eq!(
        aoc2023_rust::solutions::aoc2022::two::solve_b("./inputs/aoc2022/2_test.txt"),
        12
    );

    assert_eq!(
        aoc2023_rust::solutions::aoc2022::two::solve_b("./inputs/aoc2022/2.txt"),
        12316
    );
}

#[test]
pub fn test_three_a() {
    assert_eq!(
        aoc2023_rust::solutions::aoc2022::three::solve_a("./inputs/aoc2022/3_test.txt"),
        157
    );

    assert_eq!(
        aoc2023_rust::solutions::aoc2022::three::solve_a("./inputs/aoc2022/3.txt"),
        8233
    );
}

#[test]
pub fn test_three_b() {
    assert_eq!(
        aoc2023_rust::solutions::aoc2022::three::solve_b("./inputs/aoc2022/3_test.txt"),
        70
    );

    assert_eq!(
        aoc2023_rust::solutions::aoc2022::three::solve_b("./inputs/aoc2022/3.txt"),
        2821
    );
}

#[test]
pub fn test_four_a() {
    assert_eq!(
        aoc2023_rust::solutions::aoc2022::four::solve_a("./inputs/aoc2022/4_test.txt"),
        2
    );

    assert_eq!(
        aoc2023_rust::solutions::aoc2022::four::solve_a("./inputs/aoc2022/4.txt"),
        605
    );
}

#[test]
pub fn test_four_b() {
    assert_eq!(
        aoc2023_rust::solutions::aoc2022::four::solve_b("./inputs/aoc2022/4_test.txt"),
        4
    );

    assert_eq!(
        aoc2023_rust::solutions::aoc2022::four::solve_b("./inputs/aoc2022/4.txt"),
        914
    );
}

#[test]
pub fn test_five_a() {
    assert_eq!(solve_a("./inputs/aoc2022/5_test.txt"), String::from("CMZ"));
    assert_eq!(solve_a("./inputs/aoc2022/5.txt"), String::from("VGBBJCRMN"));
}

#[test]
pub fn test_five_b() {
    assert_eq!(solve_b("./inputs/aoc2022/5_test.txt"), String::from("MCD"));
    assert_eq!(solve_b("./inputs/aoc2022/5.txt"), String::from("LBBVJBRMH"));
}
