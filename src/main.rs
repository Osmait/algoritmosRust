use std::vec;

mod searching;
mod sorting;
fn main() {
    let mut ve1 = vec![6, 5, 3, 2, 8, 1];

    let mut lista_str = vec![20, 40, 10, 15, 60];
    // sorting::bucket_sort::bucket_sort(&mut lista_str);
    sorting::buble_sort::bubble_sort(&mut ve1);
    sorting::quick_sort(&mut lista_str);
    println!("{ve1:?}",);
    println!("{lista_str:?}",);
}
