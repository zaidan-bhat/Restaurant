use crate::app::Restaurant;

pub mod app;
fn main() {
    let restaurant = Restaurant::new();

    // Add some sample orders
    let restaurant= restaurant.add_order(String::from("Pizza"), 1, 1);
    let restaurant= restaurant.add_order(String::from("Pizza"), 1, 1);
    let restaurant= restaurant.add_order(String::from("Salad"), 1, 1);
    let restaurant= restaurant.add_order(String::from("Burger"), 2, 1);

    restaurant.print_all_orders();

    let restaurant= restaurant.remove_order(1, "Pizza", 1);
    restaurant.query_table(1);
}
