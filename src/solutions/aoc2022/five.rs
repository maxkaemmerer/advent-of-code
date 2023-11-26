use crate::common::file_to_lines;

type Layout = Vec<Vec<char>>;

fn print_layout(layout: &Layout) {
    println!("Layout:");
    layout
        .iter()
        .enumerate()
        .for_each(|(index, stack)| println!("{index} {:?}", stack));
}

#[derive(Debug)]
struct CargoMove {
    amount: usize,
    source: usize,
    target: usize,
}

pub fn solve_a(path: &str) -> String {
    solve_with_stacking_order(path, false)
}

pub fn solve_b(path: &str) -> String {
    solve_with_stacking_order(path, true)
}

fn solve_with_stacking_order(path: &str, preserve_stacking_order: bool) -> String {
    let lines = file_to_lines(path);

    let mut split_definitions = lines.split(String::is_empty);

    let mut raw_layout: Vec<String> = split_definitions
        .next()
        .expect("input is malformed. could not parse layout")
        .to_owned();

    raw_layout.reverse();
    let mut layout_iterator = raw_layout.iter();

    let stacks = layout_iterator
        .next()
        .expect("input is malformed. could not parse stack indicators");

    let number_of_stacks = stacks.chars().filter(|c| !c.is_whitespace()).count();

    println!("Number of Stack {}", number_of_stacks);

    let mut layout: Layout = (0..number_of_stacks).map(|_| vec![]).collect();
    layout_iterator.for_each(|layout_line| {
        let cargo = line_to_cargo(layout_line, number_of_stacks);
        cargo
            .iter()
            .enumerate()
            .for_each(|(stack_index, cargo_box)| {
                if let Some(existing_box) = cargo_box {
                    layout[stack_index].push(existing_box.clone());
                }
            });
    });

    let moves: Vec<CargoMove> = split_definitions
        .next()
        .expect("input is malformed. could not parse moves")
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let _ = parts.next();
            let op_amount = parts.next().and_then(|c| c.parse::<usize>().ok());
            let _ = parts.next();
            let op_source = parts.next().and_then(|c| c.parse::<usize>().ok());
            let _ = parts.next();
            let op_destination = parts.next().and_then(|c| c.parse::<usize>().ok());

            if let (Some(amount), Some(source), Some(target)) =
                (op_amount, op_source, op_destination)
            {
                return CargoMove {
                    amount,
                    source: source - 1,
                    target: target - 1,
                };
            }

            panic!("input is malformed cant parge line '{}' to move", line);
        })
        .collect();

    print_layout(&layout);

    moves
        .iter()
        .for_each(|cargo_move| move_cargo(&mut layout, cargo_move, preserve_stacking_order));

    print_layout(&layout);

    let top_cargo: Vec<char> = layout
        .iter()
        .map(|stack| stack.last().unwrap_or(&' ').clone())
        .collect();

    String::from_iter(top_cargo.iter().filter(|c| !c.is_whitespace()))
}

fn take_x_cargo(
    layout: &mut Layout,
    stack: usize,
    amount: usize,
    preserve_stacking_order: bool,
) -> Vec<char> {
    let mut removed_cargo: Vec<char> = (0..amount)
        .map(|index| {
            println!("{:?} {index}", layout[stack]);
            layout[stack]
                .pop()
                .expect("malformed input, tried to move nonexistent cargo")
        })
        .collect();

    if preserve_stacking_order {
        removed_cargo.reverse();
    }

    removed_cargo
}

fn place_cargo(layout: &mut Layout, stack: usize, cargo: Vec<char>) {
    cargo.iter().for_each(|piece_of_cargo| {
        layout[stack].push(piece_of_cargo.clone());
    });
}

fn move_cargo(layout: &mut Layout, cargo_move: &CargoMove, preserve_stacking_order: bool) {
    println!("Move {:?}", cargo_move);
    let taken_cargo = take_x_cargo(
        layout,
        cargo_move.source,
        cargo_move.amount,
        preserve_stacking_order,
    );
    place_cargo(layout, cargo_move.target, taken_cargo);

    print_layout(layout);
}

fn line_to_cargo(line: &String, stacks: usize) -> Vec<Option<char>> {
    (0..stacks)
        .map(|index| index + 1 + (3 * index))
        .map(|index_in_line| {
            line.chars()
                .skip(index_in_line)
                .take(1)
                .next()
                .filter(|c| !c.is_whitespace())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::line_to_cargo;

    #[test]
    fn should_corrently_extract_crates() {
        assert_eq!(
            line_to_cargo(&"    [D]    ".to_string(), 3),
            vec![None, Some('D'), None]
        );
        assert_eq!(
            line_to_cargo(&"[N] [C]    ".to_string(), 3),
            vec![Some('N'), Some('C'), None]
        );
        assert_eq!(
            line_to_cargo(&"[Z] [M] [P]".to_string(), 3),
            vec![Some('Z'), Some('M'), Some('P')]
        );
    }
}
