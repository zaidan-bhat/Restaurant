use std::collections::HashMap;
use rand::Rng;

pub struct Restaurant {
    tables: HashMap<u32, HashMap<String, (u32, u32)>>,
}

impl Restaurant {
    pub fn new() -> Self {
        Restaurant {
            tables: HashMap::new(),
        }
    }

    pub fn add_order(&self, item: String, table_number: u32, quantity: u32) -> Restaurant {
        let mut new_tables = self.tables.clone(); // Clone to avoid mutation
        let table_entry = new_tables.entry(table_number).or_insert(HashMap::new());
        let mut rng = rand::thread_rng();
        let preparation_time = rng.gen_range(5..=15);
        let item_entry = table_entry.entry(item).or_insert((preparation_time, 0));
        let (_, current_quantity) = item_entry;
        *current_quantity += quantity;

        Restaurant { tables: new_tables }
    }

    pub fn remove_order(&self, table_number: u32, item_name: &str, quantity: u32) -> Restaurant {
        let mut new_tables = self.tables.clone(); // Clone to avoid mutation
        if let Some(table_orders) = new_tables.get_mut(&table_number) {
            if let Some(item_entry) = table_orders.get_mut(item_name) {
                let (_preparation_time, current_quantity) = item_entry;
                if *current_quantity > quantity {
                    *current_quantity -= quantity;
                } else {
                    table_orders.remove(item_name);
                }
            }
        }

        Restaurant { tables: new_tables }
    }

    pub fn query_table(&self, table_number: u32) {
        if let Some(table_orders) = self.tables.get(&table_number) {
            println!("Orders for Table {}:", table_number);
            for (item_name, (preparation_time, quantity)) in table_orders.iter() {
                println!("    * {}: {} ({})", item_name, quantity, preparation_time);
            }
        } else {
            println!("Table {} not found.", table_number);
        }
    }

    
    pub fn print_all_orders(&self) {
        for (table_number, table_orders) in self.tables.iter() {
            println!("Table {}", table_number);
            for (item_name, (preparation_time, quantity)) in table_orders.iter() {
                println!("    * {}: {} ({})", item_name, quantity, preparation_time);
            }
        }
    }
    
}

