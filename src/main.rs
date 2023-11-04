use crate::app::Restaurant;

pub mod app;
fn main() {
    let restaurant = Restaurant::new();

    // Add some sample orders
    let restaurant= restaurant.add_order(String::from("Pizza"), 1, 1).unwrap();
    let restaurant= restaurant.add_order(String::from("Pizza"), 1, 1).unwrap();
    let restaurant= restaurant.add_order(String::from("Salad"), 1, 1).unwrap();
    let restaurant= restaurant.add_order(String::from("Burger"), 2, 1).unwrap();

    restaurant.print_all_orders();

    let restaurant= restaurant.remove_order(1, "Pizza", 1).unwrap();
    let _result= restaurant.query_table(1);
    let _result= restaurant.query_item_for_table(2, "Burger");
}
