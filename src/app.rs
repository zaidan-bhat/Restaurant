use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;
use serde_json::{json, Map, Value};

#[derive(Debug)]
pub enum RestaurantError {
    TableNotFound,
    ItemNotFound,
    InsufficientQuantity,
}

#[derive(Debug)]
pub struct Restaurant {
    tables: Mutex<HashMap<u32, HashMap<String, (u32, u32)>>>,
}

impl Restaurant {
    pub fn new() -> Self {
        Restaurant {
            tables: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_order(&self, item: String, table_number: u32, quantity: u32) -> Result<(), RestaurantError> {
        let mut new_tables = self.tables.lock().unwrap();
        let table_entry = new_tables.entry(table_number).or_insert(HashMap::new());
        let mut rng = rand::thread_rng();
        let preparation_time = rng.gen_range(5..=15);
        let item_entry = table_entry.entry(item).or_insert((preparation_time, 0));
        let (_, current_quantity) = item_entry;
        *current_quantity += quantity;
        Ok(())
    }

    pub fn remove_order(&self, table_number: u32, item_name: &str, quantity: u32) -> Result<(), RestaurantError> {
        let mut new_tables = self.tables.lock().unwrap();
        if let Some(table_orders) = new_tables.get_mut(&table_number) {
            if let Some(item_entry) = table_orders.get_mut(item_name) {
                let (_preparation_time, current_quantity) = item_entry;
                if *current_quantity > quantity {
                    *current_quantity -= quantity;
                    Ok(())
                } else {
                    table_orders.remove(item_name);
                    Ok(())
                }
            } else {
                Err(RestaurantError::ItemNotFound)
            }
        } else {
            Err(RestaurantError::TableNotFound)
        }
    }

    pub fn query_table(&self, table_number: u32) -> Result<String, RestaurantError> {
        let tables = self.tables.lock().unwrap();
        if let Some(table_orders) = tables.get(&table_number) {
            let result: Map<String, Value> = table_orders
                .iter()
                .map(|(item_name, (preparation_time, quantity))| {
                    (
                        item_name.to_string(),
                        json!({
                            "item_name": item_name,
                            "quantity": quantity,
                            "preparation_time": preparation_time,
                        }),
                    )
                })
                .collect();
    
            Ok(json!(result).to_string())
        } else {
            Err(RestaurantError::TableNotFound)
        }
    }

    pub fn query_item_for_table(&self, table_number: u32, item_name: &str) -> Result<String, RestaurantError> {
        let tables = self.tables.lock().unwrap();
        if let Some(table_orders) = tables.get(&table_number) {
            if let Some(item_entry) = table_orders.get(item_name) {
                let (preparation_time, quantity) = item_entry;
                let result: Value = json!({
                    "table_number": table_number,
                    "item_name": item_name,
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
    
    pub fn show_all_orders(&self) -> String {
        let tables = self.tables.lock().unwrap(); // Lock before accessing
    
        let result: Map<String, Value> = tables
            .iter()
            .map(|(table_number, table_orders)| {
                let table_json: Map<String, Value> = table_orders
                    .iter()
                    .map(|(item_name, (preparation_time, quantity))| {
                        (
                            item_name.to_string(),
                            json!({
                                "item_name": item_name,
                                "quantity": quantity,
                                "preparation_time": preparation_time,
                            }),
                        )
                    })
                    .collect();
    
                (
                    table_number.to_string(),
                    json!({
                        "table_number": table_number,
                        "orders": table_json,
                    }),
                )
            })
            .collect();
    
        json!(result).to_string()
    }
}





