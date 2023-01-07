use std::vec;

use crate::challerger::factorial;

mod data_structure;
mod searching;
mod sorting;
mod challerger;
mod letcode;
fn main() {
    let mut ve1 = vec![6, 5, 3, 2, 8, 1];

    let mut lista_str = vec![20, 40, 10, 15, 60];
    // sorting::bucket_sort::bucket_sort(&mut lista_str);
    sorting::buble_sort::bubble_sort(&mut ve1);
    sorting::quick_sort(&mut lista_str);
    println!("{ve1:?}",);
    println!("{lista_str:?}",);
    let n = 5;
    let result = factorial::iterative_factorial(n);
    println!("{}",result);
    
    let result_r = factorial::recur_factorial(n);
    println!("{}",result_r);




}
