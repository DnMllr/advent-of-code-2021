use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space0, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub type LineOfNumbers = Vec<u8>;
pub type Board = Vec<LineOfNumbers>;
pub type Boards = Vec<Board>;

fn parse_number(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse()
}

fn number(input: &str) -> IResult<&str, u8> {
    map_res(digit1, parse_number)(input)
}

fn number_line(input: &str) -> IResult<&str, LineOfNumbers> {
    separated_list1(tag(","), number)(input)
}

fn board_line(input: &str) -> IResult<&str, LineOfNumbers> {
    separated_list1(space1, number)(input)
}

fn board(input: &str) -> IResult<&str, Board> {
    separated_list1(terminated(newline, space0), board_line)(input)
}

fn boards(input: &str) -> IResult<&str, Boards> {
    separated_list1(many1(terminated(newline, space0)), board)(input)
}

fn total_input(input: &str) -> IResult<&str, (LineOfNumbers, Boards)> {
    tuple((terminated(number_line, newline), preceded(newline, boards)))(input)
}

#[derive(Debug)]
pub struct Parse {
    pub numbers: LineOfNumbers,
    pub boards: Boards,
}

pub fn parse(input: &str) -> color_eyre::Result<Parse> {
    let (_, (numbers, boards)) = total_input(input).map_err(|e| e.to_owned())?;

    Ok(Parse { numbers, boards })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

        parse(input).expect("should be able to successfully parse the test input");
    }
}
