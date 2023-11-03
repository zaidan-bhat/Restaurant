use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;

pub struct Restaurant {
    tables: Mutex<HashMap<u32, HashMap<String, (u32, u32)>>>,
}

impl Restaurant {
    pub fn new() -> Self {
        Restaurant {
            tables: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_order(&self, item: String, table_number: u32, quantity: u32) {
        let mut new_tables = self.tables.lock().unwrap(); // Lock before accessing
        let table_entry = new_tables.entry(table_number).or_insert(HashMap::new());
        let mut rng = rand::thread_rng();
        let preparation_time = rng.gen_range(5..=15);
        let item_entry = table_entry.entry(item).or_insert((preparation_time, 0));
        let (_, current_quantity) = item_entry;
        *current_quantity += quantity;
    }

    pub fn remove_order(&self, table_number: u32, item_name: &str, quantity: u32) {
        let mut new_tables = self.tables.lock().unwrap(); // Lock before accessing
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
    }

    pub fn query_table(&self, table_number: u32) {
        let tables = self.tables.lock().unwrap(); // Lock before accessing
        if let Some(table_orders) = tables.get(&table_number) {
            println!("Orders for Table {}:", table_number);
            for (item_name, (preparation_time, quantity)) in table_orders.iter() {
                println!("    * {}: {} ({})", item_name, quantity, preparation_time);
            }
        } else {
            println!("Table {} not found.", table_number);
        }
    }

    pub fn print_all_orders(&self) {
        let tables = self.tables.lock().unwrap(); // Lock before accessing
        for (table_number, table_orders) in tables.iter() {
            println!("Table {}", table_number);
            for (item_name, (preparation_time, quantity)) in table_orders.iter() {
                println!("    * {}: {} ({})", item_name, quantity, preparation_time);
            }
        }
    }
}
