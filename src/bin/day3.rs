#[derive(Debug)]
struct Number {
    rows: std::ops::RangeInclusive<i32>,
    cols: std::ops::RangeInclusive<i32>,
    num: i32,
}

#[derive(Debug)]
struct Symbol {
    row: i32,
    col: i32,
    symbol: char,
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn parse(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut bytes = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(move |(j, c)| (i, j, c))
        })
        .peekable();
    while let Some(&(row, col, next)) = bytes.peek() {
        match next {
            b'.' => {
                bytes.next();
            }
            num if num.is_ascii_digit() => {
                let mut bytes_vec = vec![];
                while let Some(&(_, _, next)) = bytes.peek() {
                    if !next.is_ascii_digit() {
                        break;
                    }
                    bytes_vec.push(*bytes.next().unwrap().2);
                }
                let len = bytes_vec.len();
                numbers.push(Number {
                    rows: (row as i32 - 1)..=(row as i32 + 1),
                    cols: (col as i32 - 1)..=(col as i32 + len as i32),
                    num: std::str::from_utf8(&bytes_vec).unwrap().parse().unwrap(),
                });
            }
            _ => {
                symbols.push(Symbol {
                    row: row as i32,
                    col: col as i32,
                    symbol: *bytes.next().unwrap().2 as char,
                });
            }
        }
    }

    (numbers, symbols)
}

fn main() {
    let input = include_str!("../../input/day3.txt");
    let (nums, symbols) = parse(input);

    let sum: i32 = nums
        .iter()
        .filter(|num| {
            symbols
                .iter()
                .any(|symbol| num.rows.contains(&symbol.row) && num.cols.contains(&symbol.col))
        })
        .map(|num| num.num)
        .sum();
    println!("Sum of part numbers: {sum}");

    let sum: i32 = symbols
        .iter()
        .filter(|symbol| symbol.symbol == '*')
        .map(|symbol| {
            let adjacent: Vec<_> = nums
                .iter()
                .filter(|num| num.rows.contains(&symbol.row) && num.cols.contains(&symbol.col))
                .collect();
            if adjacent.len() == 2 {
                adjacent.iter().map(|num| num.num).product()
            } else {
                0
            }
        })
        .sum();
    println!("Sum of gear rations: {sum}");
}
