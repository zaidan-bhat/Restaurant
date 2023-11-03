use crate::app::Restaurant ;
fn main() {
    let restaurant = Restaurant::new();

    // Add some sample orders
    restaurant= restaurant.add_order("Pizza", 1, 1);
    restaurant= restaurant.add_order("Salad", 1, 1);
    restaurant= restaurant.add_order("Burger", 2, 1);

    restaurant.print
}
