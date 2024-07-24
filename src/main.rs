mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string<T: Container<String>>(container: &mut T, item: String) {
    container.put(item);
}

fn main() {
    let mut first_basket = Basket::new(String::from("apple"));
    let second_basket = Basket::new(10);
    let third_basket = Basket::new(true);

    let s1 = Stack::new(vec![1, 2, 3]);
    let mut s2 = Stack::new(vec![
        String::from("a"),
        String::from("b"),
        String::from("c"),
    ]);

    add_string(&mut first_basket, String::from("banana"));
    add_string(&mut s2, String::from("d"));
}
