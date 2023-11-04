use std::collections::HashMap;
use rand::Rng;

#[derive(Debug)]
pub struct Restaurant {
    tables: HashMap<u32, HashMap<String, (u32, u32)>>,
}

#[derive(Debug)]
pub enum RestaurantError {
    TableNotFound,
    ItemNotFound,
    InsufficientQuantity,
}

impl Restaurant {
    pub fn new() -> Self {
        Restaurant {
            tables: HashMap::new(),
        }
    }

    pub fn add_order(&self, item: String, table_number: u32, quantity: u32) -> Result<Restaurant, RestaurantError> {
        let mut new_tables = self.tables.clone(); // Clone to avoid mutation
        let table_entry = new_tables.entry(table_number).or_insert(HashMap::new());
        let mut rng = rand::thread_rng();
        let preparation_time = rng.gen_range(5..=15);
        let item_entry = table_entry.entry(item).or_insert((preparation_time, 0));
        let (_, current_quantity) = item_entry;
        *current_quantity += quantity;

        Ok(Restaurant { tables: new_tables })
    }

    pub fn remove_order(&self, table_number: u32, item_name: &str, quantity: u32) -> Result<Restaurant, RestaurantError> {
        let mut new_tables = self.tables.clone(); // Clone to avoid mutation
        
        if let Some(table_orders) = new_tables.get_mut(&table_number) {
            if let Some(item_entry) = table_orders.get_mut(item_name) {
                let (_preparation_time, current_quantity) = item_entry;
                if *current_quantity > quantity {
                    *current_quantity -= quantity;
                } else {
                    table_orders.remove(item_name);
                }
                Ok(Restaurant { tables: new_tables })
            } else {
                Err(RestaurantError::ItemNotFound)
            }
        } else {
            Err(RestaurantError::TableNotFound)
        }
    }

    pub fn query_item_for_table(&self, table_number: u32, item_name: &str) -> Result<String, RestaurantError> {
        if let Some(table_orders) = self.tables.get(&table_number) {
            if let Some((preparation_time, quantity)) = table_orders.get(item_name) {
                let result = serde_json::json!({
                    "item_name": item_name,
                    "table_number": table_number,
                    "quantity": quantity,
                    "preparation_time": preparation_time,
                });
                Ok(result.to_string())
            } else {
                Err(RestaurantError::ItemNotFound)
            }
        } else {
            Err(RestaurantError::TableNotFound)
        }
    }
    
    pub fn query_table(&self, table_number: u32) -> Result<String, RestaurantError> {
        if let Some(table_orders) = self.tables.get(&table_number) {
            let orders: Vec<_> = table_orders
                .iter()
                .map(|(item_name, (preparation_time, quantity))| {
                    serde_json::json!({
                        "item_name": item_name,
                        "quantity": quantity,
                        "preparation_time": preparation_time,
                    })
                })
                .collect();

            let result = serde_json::json!({
                "table_number": table_number,
                "orders": orders,
            });

            Ok(result.to_string())
        } else {
            Err(RestaurantError::TableNotFound)
        }
    }

    pub fn show_all_orders(&self) -> Result<String, RestaurantError> {
        let all_orders: Vec<_> = self.tables
            .iter()
            .map(|(table_number, table_orders)| {
                let orders: Vec<_> = table_orders
                    .iter()
                    .map(|(item_name, (preparation_time, quantity))| {
                        serde_json::json!({
                            "item_name": item_name,
                            "quantity": quantity,
                            "preparation_time": preparation_time,
                        })
                    })
                    .collect();

                serde_json::json!({
                    "table_number": table_number,
                    "orders": orders,
                })
            })
            .collect();

        let result = serde_json::json!({
            "all_orders": all_orders,
        });

        Ok(result.to_string())
    }
}
