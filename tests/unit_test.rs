#[cfg(test)]
mod tests {
    use restaurant::Restaurant;

    #[test]
    fn test_add_order() {
        let restaurant = Restaurant::new();
        let result = restaurant.add_order(String::from("Burger"), 1, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_order() {
        let restaurant = Restaurant::new();
        let restaurant= restaurant.add_order(String::from("Pizza"), 1, 3).unwrap();
        let result = restaurant.remove_order(1, "Pizza", 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_item_for_table() {
        let restaurant = Restaurant::new();
        let restaurant = restaurant.add_order(String::from("Salad"), 2, 1).unwrap();
        let result = restaurant.query_item_for_table(2, "Salad");
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_table() {
        let restaurant = Restaurant::new();
        let restaurant = restaurant.add_order(String::from("Drink"), 3, 4).unwrap();
        let result = restaurant.query_table(3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_all_orders() {
        let restaurant = Restaurant::new();
        let restaurant = restaurant.add_order(String::from("Dessert"), 4, 2).unwrap();
        let result = restaurant.show_all_orders();
        assert!(result.is_ok());
    }
}
