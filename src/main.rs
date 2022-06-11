use nom::character::is_digit;
use nom::byte::streaming::take_while;
use nom::IResult;

fn main() {
    
    
}

fn jud_digit(char: char) -> bool {
    is_digit(chr as u8)
}

#[test]
fn test_split_digit() {
    let s = "123abc";
    let r: IResult<&str, &str> = take_while(jud_digit)(s);

    let remain_expanded = "abc";
    let digit_expected ="123";
    assert_eq!(Ok((remain_expanded, digit_expected)), r)ß
}
