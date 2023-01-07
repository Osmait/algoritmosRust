use std::collections::HashMap;

fn two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
    let mut map = HashMap::new();
    for (i,n) in nums.into_iter().enumerate()  {
        let diff = target -n;
        if let Some(&j) = map.get(&diff){
            return vec![i as i32, j as i32];
        }else {
            map.insert(n, i);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn two_sum_test() {
        let data=vec![4,7,11,15];
        let target = 11;
        let result = vec![1,0];

        assert_eq!(two_sum(data, target),result)
    }
}