fn product_except_self(mut nums:Vec<i32>)-> Vec<i32> {
    let mut res = vec![1; nums.len()];

    for i in 1..nums.len(){
        res[i] = nums[i -1] * res[i -1];
    }
    let mut rigth =1;
    for(i,n) in res.iter_mut().enumerate().rev(){
        *n = *n * rigth;
        rigth *= nums[i];

    }
    res
}


#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_product() {
        let data = vec![1,2,3,4];
        let result = vec![24,12,8,6];
        assert_eq!(product_except_self(data),result)

    }
}