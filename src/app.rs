pub mod app{
    use std::collections::HashMap;
    use rand::Rng;

    pub struct Restaurant {
        tables: HashMap<u32, HashMap<String, (u32, u32)>>,
    }

    impl Restaurant {
        fn new() -> Self {
            Restaurant {
                tables: HashMap::new(),
            }
        }

        fn add_order(&self, item: String, table_number: u32, quantity: u32) -> Restaurant {
            let preparation_time = rand::thread_rng().gen_range(5..=15);

            let new_tables = self
                .tables
                .iter()
                .cloned()
                .map(|(num, table)| {
                    (
                        *num,
                        table
                            .iter()
                            .cloned()
                            .map(|(item_name, (time, current_quantity))| {
                                if *num == table_number && item_name == item {
                                    (
                                        item_name,
                                        (time, current_quantity + quantity),
                                    )
                                } else {
                                    (item_name, (preparation_time, current_quantity))
                                }
                            })
                            .collect(),
                    )
                })
                .collect();

            Restaurant { tables: new_tables }
        }


        fn print_all_orders(&self) {
            for (table_number, table_orders) in restaurant.tables.iter() {
                println!("Table {}", table_number);
                for (item_name, (preparation_time, quantity)) in table_orders.iter() {
                    println!("    * {}: {} ({})", item_name, quantity, preparation_time);
                }
            }
        }
    }

}
// use std::collections::HashMap;
// use rand::Rng;

// struct Restaurant {
//     tables: HashMap<u32, HashMap<String, (u32, u32)>>,
// }

// impl Restaurant {
//     fn new() -> Self {
//         Restaurant {
//             tables: HashMap::new(),
//         }
//     }

//     fn add_order(&self, item: String, table_number: u32, quantity: u32) -> Restaurant {
//         let mut new_tables = self.tables.clone(); // Clone to avoid mutation
//         let table_entry = new_tables.entry(table_number).or_insert(HashMap::new());
//         let mut rng = rand::thread_rng();
//         let preparation_time = rng.gen_range(5..=15);
//         let item_entry = table_entry.entry(item).or_insert((preparation_time, 0));
//         let (_, current_quantity) = item_entry;
//         *current_quantity += quantity;

//         Restaurant { tables: new_tables }
//     }

//     // Other methods for removal and querying can follow a similar pattern
// }


