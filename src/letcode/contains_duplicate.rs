use std::{ collections::HashSet};

fn contains_duplicate(nums:Vec<i32>)->bool {
    let mut map  = HashSet::new();
    for &n in nums.iter() {
        if map.contains(&n){
            return  true;
        }
        map.insert(n);
    }
    return false
}

#[cfg(test)]
mod tests {
use  super::*;



    #[test]
    fn duplicate() {
        let data = vec![1,2,3,4];
        let result = contains_duplicate(data);
        assert_eq!(result,false)
    }


}