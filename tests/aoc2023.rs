#[test]
pub fn test_one_a() {
    assert_eq!(
        142,
        aoc::solutions::aoc2023::one::solve_a("./inputs/aoc2023/1_test.txt")
    );

    assert_eq!(
        55090,
        aoc::solutions::aoc2023::one::solve_a("./inputs/aoc2023/1.txt")
    );
}

#[test]
pub fn test_one_b() {
    assert_eq!(
        281,
        aoc::solutions::aoc2023::one::solve_b("./inputs/aoc2023/1_test_b.txt")
    );

    assert_eq!(
        54845,
        aoc::solutions::aoc2023::one::solve_b("./inputs/aoc2023/1.txt")
    );
}

#[test]
pub fn test_two_a() {
    assert_eq!(
        8,
        aoc::solutions::aoc2023::two::solve_a("./inputs/aoc2023/2_test.txt")
    );

    assert_eq!(
        2283,
        aoc::solutions::aoc2023::two::solve_a("./inputs/aoc2023/2.txt")
    );
}

#[test]
pub fn test_two_b() {
    assert_eq!(
        2286,
        aoc::solutions::aoc2023::two::solve_b("./inputs/aoc2023/2_test.txt")
    );

    assert_eq!(
        78669,
        aoc::solutions::aoc2023::two::solve_b("./inputs/aoc2023/2.txt")
    );
}

#[test]
pub fn test_three_a() {
    assert_eq!(
        4361,
        aoc::solutions::aoc2023::three::solve_a("./inputs/aoc2023/3_test.txt")
    );

    assert_eq!(
        551094,
        aoc::solutions::aoc2023::three::solve_a("./inputs/aoc2023/3.txt")
    );
}
#[test]
pub fn test_three_b() {
    assert_eq!(
        467835,
        aoc::solutions::aoc2023::three::solve_b("./inputs/aoc2023/3_test.txt")
    );

    assert_eq!(
        80179647,
        aoc::solutions::aoc2023::three::solve_b("./inputs/aoc2023/3.txt")
    );
}

#[test]
pub fn test_four_a() {
    assert_eq!(
        13,
        aoc::solutions::aoc2023::four::solve_a("./inputs/aoc2023/4_test.txt")
    );

    assert_eq!(
        25010,
        aoc::solutions::aoc2023::four::solve_a("./inputs/aoc2023/4.txt")
    );

    assert_eq!(
        13,
        aoc::solutions::aoc2023::four::solve_a("./inputs/aoc2023/4_test.txt")
    );

    assert_eq!(
        25010,
        aoc::solutions::aoc2023::four::solve_a("./inputs/aoc2023/4.txt")
    );
}

#[test]
pub fn test_four_b() {
    assert_eq!(
        30,
        aoc::solutions::aoc2023::four::solve_b("./inputs/aoc2023/4_test.txt")
    );

    assert_eq!(
        9924412,
        aoc::solutions::aoc2023::four::solve_b("./inputs/aoc2023/4.txt")
    );
}

#[test]
pub fn test_five_a() {
    assert_eq!(
        35,
        aoc::solutions::aoc2023::five::solve_a("./inputs/aoc2023/5_test.txt")
    );

    // assert_eq!(
    //     0,
    //     aoc::solutions::aoc2023::five::solve_a("./inputs/aoc2023/5.txt")
    // );
}
