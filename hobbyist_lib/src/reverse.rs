pub fn reverse(input:&str) -> String{
    let mut res = "".to_string();
    for letter in input.chars().rev(){
        res.push(letter);
    }
    res

}

//Another form of reverse 
use unicode_segmentation::UnicodeSegmentation;

fn overengineered_rev(input : &str) -> String{
    UnicodeSegmentation::graphemes(input, true).rev().collect::<String>()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_reverse(){
        assert_eq!("oob".to_string(), reverse("boo"));
    }

    #[test]
    fn test_overengine(){
        assert_eq!("tac", overengineered_rev("cat"));
    }

}

