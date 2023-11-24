use aoc2023_rust;

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
