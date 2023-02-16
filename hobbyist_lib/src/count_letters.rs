fn charsum(input: &str) -> usize{
    let mut sum =0;
    for letter in input.chars(){
        let n = letter as usize - 96;
        sum += n;
    }
    sum
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn charsum_test(){
       // assert_eq!(0, charsum(""));
       // assert_eq!(1, charsum("a"));
       // assert_eq!(2, charsum("b"));
        assert_eq!(6, charsum("cab"));


    }
}