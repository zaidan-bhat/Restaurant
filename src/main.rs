use restaurant::Restaurant;  // Use the Restaurant struct from the app module

fn main() {
    let restaurant = Restaurant::new();

    // Add some sample orders
    let restaurant= restaurant.add_order(String::from("Pizza"), 1, 1).unwrap();
    let restaurant= restaurant.add_order(String::from("Pizza"), 1, 1).unwrap();
    let restaurant= restaurant.add_order(String::from("Salad"), 1, 1).unwrap();
    let restaurant= restaurant.add_order(String::from("Burger"), 2, 1).unwrap();

    let result = restaurant.show_all_orders().unwrap();
    println!("\n Here are the details of all orders: \n{}", result);
    let restaurant= restaurant.remove_order(1, "Pizza", 1).unwrap();
    let result= restaurant.query_table(1).unwrap();
    println!("\n Here are the details of orders for table 1: \n{}", result);
    let result= restaurant.query_item_for_table(2, "Burger").unwrap();
    println!("\n Here are the details of item Burger for table 2: \n{}", result);
}
