#[cfg(test)]
mod tests {
    use restaurant::Restaurant;

    #[test]
    fn test_add_order() {
        let restaurant = Restaurant::new();
        let result = restaurant.add_order("Burger".to_string(), 1, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remove_order() {
        let restaurant = Restaurant::new();
        restaurant.add_order("Pizza".to_string(), 2, 3).unwrap();
        
        let result = restaurant.remove_order(2, "Pizza", 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_table() {
        let restaurant = Restaurant::new();
        restaurant.add_order("Salad".to_string(), 3, 1).unwrap();

        let result = restaurant.query_table(3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_item_for_table() {
        let restaurant = Restaurant::new();
        restaurant.add_order("Pasta".to_string(), 4, 2).unwrap();

        let result = restaurant.query_item_for_table(4, "Pasta");
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_all_orders() {
        let restaurant = Restaurant::new();
        restaurant.add_order("Drink".to_string(), 5, 1).unwrap();

        let result = restaurant.show_all_orders();
        assert!(!result.is_empty());
    }
}
