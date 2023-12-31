use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
struct Number {
    rows: std::ops::RangeInclusive<i32>,
    cols: std::ops::RangeInclusive<i32>,
    num: i32,
}

impl Number {
    fn contains(&self, symbol: &Symbol) -> bool {
        self.rows.contains(&symbol.row) && self.cols.contains(&symbol.col)
    }
}

#[derive(Debug)]
struct Symbol {
    row: i32,
    col: i32,
    symbol: char,
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn parse(input: &str) -> Result<(Vec<Number>, Vec<Symbol>)> {
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut chars = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| (i, j, c)))
        .peekable();
    while let Some(&(row, col, next)) = chars.peek() {
        match next {
            '.' => {
                chars.next();
            }
            num if num.is_ascii_digit() => {
                let mut chars_vec = vec![];
                while let Some(&(_, _, next)) = chars.peek() {
                    if !next.is_ascii_digit() {
                        break;
                    }
                    chars_vec.push(chars.next().context(anyhow!("should not happend"))?.2);
                }
                let len = chars_vec.len();
                numbers.push(Number {
                    rows: (row as i32 - 1)..=(row as i32 + 1),
                    cols: (col as i32 - 1)..=(col as i32 + len as i32),
                    num: String::from_iter(chars_vec).parse()?,
                });
            }
            _ => {
                symbols.push(Symbol {
                    row: row as i32,
                    col: col as i32,
                    symbol: chars.next().context(anyhow!("should not happen"))?.2,
                });
            }
        }
    }

    Ok((numbers, symbols))
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day3.txt");
    let (nums, symbols) = parse(input)?;

    let sum: i32 = nums
        .iter()
        .filter(|num| symbols.iter().any(|symbol| num.contains(symbol)))
        .map(|num| num.num)
        .sum();
    println!("Sum of part numbers: {sum}");

    let sum: i32 = symbols
        .iter()
        .filter(|symbol| symbol.symbol == '*')
        .map(|symbol| {
            let adjacent: Vec<_> = nums.iter().filter(|num| num.contains(symbol)).collect();
            if adjacent.len() == 2 {
                adjacent.iter().map(|num| num.num).product()
            } else {
                0
            }
        })
        .sum();
    println!("Sum of gear rations: {sum}");
    Ok(())
}
