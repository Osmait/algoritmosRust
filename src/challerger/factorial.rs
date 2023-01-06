
pub fn iterative_factorial(n:usize)->usize{
    let mut fact = 1;
    for i in 2..n+1 {
        fact *= i;
    }
    fact
}

pub fn recur_factorial(n:usize)->usize{
    if n == 1 {
        return n;
    }
    return recur_factorial(n-1) * n;


    
}
