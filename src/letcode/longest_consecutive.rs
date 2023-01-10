use std::collections::HashSet;

fn longest_consecutive(nums:Vec<i32>)->i32 {
    let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());

    let mut max_cnt = 0;

    for n  in &set {
        if !set.contains(&(n-1)) {
            let mut next = n + 1;
            let mut cnt = 1;
            while set.contains(&next) {
                cnt += 1;
                next+=1;

            }
            max_cnt = max_cnt.max(cnt);

        }
    }
    max_cnt
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_longest() {
        let data = vec![100,4,200,1,3,2];
        let expectec:i32 =4; 
        assert_eq!(longest_consecutive(data),expectec)
    }
   

}