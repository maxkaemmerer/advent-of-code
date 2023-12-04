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

#[test]
pub fn test_two_a() {
    assert_eq!(
        8,
        aoc2023_rust::solutions::aoc2023::two::solve_a("./inputs/aoc2023/2_test.txt")
    );

    assert_eq!(
        2283,
        aoc2023_rust::solutions::aoc2023::two::solve_a("./inputs/aoc2023/2.txt")
    );
}

#[test]
pub fn test_two_b() {
    assert_eq!(
        2286,
        aoc2023_rust::solutions::aoc2023::two::solve_b("./inputs/aoc2023/2_test.txt")
    );

    assert_eq!(
        78669,
        aoc2023_rust::solutions::aoc2023::two::solve_b("./inputs/aoc2023/2.txt")
    );
}

#[test]
pub fn test_three_a() {
    assert_eq!(
        4361,
        aoc2023_rust::solutions::aoc2023::three::solve_a("./inputs/aoc2023/3_test.txt")
    );

    assert_eq!(
        551094,
        aoc2023_rust::solutions::aoc2023::three::solve_a("./inputs/aoc2023/3.txt")
    );
}
#[test]
pub fn test_three_b() {
    assert_eq!(
        467835,
        aoc2023_rust::solutions::aoc2023::three::solve_b("./inputs/aoc2023/3_test.txt")
    );

    assert_eq!(
        80179647,
        aoc2023_rust::solutions::aoc2023::three::solve_b("./inputs/aoc2023/3.txt")
    );
}

#[test]
pub fn test_four_a() {
    assert_eq!(
        13,
        aoc2023_rust::solutions::aoc2023::four::solve_a("./inputs/aoc2023/4_test.txt")
    );

    assert_eq!(
        25010,
        aoc2023_rust::solutions::aoc2023::four::solve_a("./inputs/aoc2023/4.txt")
    );

    assert_eq!(
        13,
        aoc2023_rust::solutions::aoc2023::four::solve_a("./inputs/aoc2023/4_test.txt")
    );

    assert_eq!(
        25010,
        aoc2023_rust::solutions::aoc2023::four::solve_a("./inputs/aoc2023/4.txt")
    );
}

#[test]
pub fn test_four_b() {
    assert_eq!(
        30,
        aoc2023_rust::solutions::aoc2023::four::solve_b("./inputs/aoc2023/4_test.txt")
    );

    assert_eq!(
        9924412,
        aoc2023_rust::solutions::aoc2023::four::solve_b("./inputs/aoc2023/4.txt")
    );
}
