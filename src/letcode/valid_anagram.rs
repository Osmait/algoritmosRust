use std::collections::HashMap;

fn is_anagram(s:String,t:String)-> bool{
    if t.len() != s.len() {
        return false
    }
    let mut map :HashMap<char,i64> = HashMap::new();

    for (a,b) in s.chars().zip(t.chars()) {
        *map.entry(a).or_default() +=1;
        *map.entry(b).or_default() -= 1;
        }

        map.into_values().all(|data| data == 0)

}

#[cfg(test)]
mod tests{
   

    use super::*;


    #[test]
    fn anagrams() {
        let data = String::from("acca");
        let data2 = String::from("ccaa");
        assert_eq!(is_anagram(data,data2),true)


    }
}