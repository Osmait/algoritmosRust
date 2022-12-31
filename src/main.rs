use std::vec;

mod sorting;
fn main() {
    let mut ve1 = vec![6, 5, 3, 2, 8, 1];
    let lista_str = ["hola", "prueba"];
    sorting::buble_sort::bubble_sort(&mut ve1);
    println!("{ve1:?}",);
    println!("{lista_str:?}",);
}
