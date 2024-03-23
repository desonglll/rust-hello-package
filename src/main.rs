use hello_package::eat_at_restaurant;
mod front_of_house;
fn main() {
    let yummy: String = eat_at_restaurant();
    println!("yummy: {:?}", yummy);
    let seat: String = hello_package::front_of_house::hosting::seat_at_table();
    println!("seat: {:?}", seat);
}
