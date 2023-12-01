#[test]
pub fn test_one_a() {
    assert_eq!(
        142,
        aoc2023_rust::solutions::aoc2023::one::solve_a("./inputs/aoc2023/1_test.txt")
    );

    assert_eq!(
        55090,
        aoc2023_rust::solutions::aoc2023::one::solve_a("./inputs/aoc2023/1.txt")
    );
}

#[test]
pub fn test_one_b() {
    assert_eq!(
        281,
        aoc2023_rust::solutions::aoc2023::one::solve_b("./inputs/aoc2023/1_test_b.txt")
    );

    assert_eq!(
        54845,
        aoc2023_rust::solutions::aoc2023::one::solve_b("./inputs/aoc2023/1.txt")
    );
}