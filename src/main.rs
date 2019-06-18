use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::multi::separated_nonempty_list;
use nom::IResult;

fn main() {
    let expr = vec!["12+7", "3-14", "2*10+7", "1+2*7+4"];

    for e in expr {
        println!("{}:\n    {:?}", e, expression(e));
    }
}

type AddSum = Vec<SubSum>;
type SubSum = Vec<MulSum>;
type MulSum = Vec<DivSum>;
type DivSum = Vec<f32>;

fn expression(i: &str) -> IResult<&str, AddSum> {
    separated_nonempty_list(char('+'), sub)(i)
}

fn sub(i: &str) -> IResult<&str, SubSum> {
    separated_nonempty_list(char('-'), mul)(i)
}

fn mul(i: &str) -> IResult<&str, MulSum> {
    separated_nonempty_list(char('*'), div)(i)
}

fn div(i: &str) -> IResult<&str, DivSum> {
    separated_nonempty_list(char('/'), number)(i)
}

fn number(i: &str) -> IResult<&str, f32> {
    nom::combinator::map(digit1, |s: &str| s.parse().unwrap())(i)
}
