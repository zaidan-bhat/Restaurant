use std::thread;
use std::sync::Arc;

pub mod app;
fn main() {

    // Running tests
    


    let restaurant = Arc::new(app::Restaurant::new());

    // Spawn two threads to perform operations concurrently
    let restaurant_clone = Arc::clone(&restaurant);
    let thread1 = thread::spawn(move || {
        let _result= restaurant_clone.add_order(String::from("Pizza"), 1, 1);
    });

    let restaurant_clone = Arc::clone(&restaurant);
    let thread2 = thread::spawn(move || {
        let _result= restaurant_clone.add_order(String::from("Pizza"), 2, 1);
    });

    let restaurant_clone = Arc::clone(&restaurant);
    let thread3 = thread::spawn(move || {
        let _result= restaurant_clone.add_order(String::from("Pizza"), 3, 1);
    });

    let restaurant_clone = Arc::clone(&restaurant);
    let thread4 = thread::spawn(move || {
        let _result= restaurant_clone.add_order(String::from("Burger"), 2, 2);
    });

    let restaurant_clone = Arc::clone(&restaurant);
    let thread5 = thread::spawn(move || {
        let _result= restaurant_clone.add_order(String::from("Coffe"), 1, 1);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();
    thread5.join().unwrap();


    let restaurant_clone = Arc::clone(&restaurant);
    let thread6 = thread::spawn(move || {
        let result= restaurant_clone.query_table(1).unwrap();
        print!("\n Orders for table 1: \n {} \n", result);
    });

    thread6.join().unwrap();

    let restaurant_clone = Arc::clone(&restaurant);
    let thread7 = thread::spawn(move || {
        let _result= restaurant_clone.remove_order(1, "Pizza", 1);
    });

    thread7.join().unwrap();

    let restaurant_clone = Arc::clone(&restaurant);
    let thread8= thread::spawn(move || {
        let result= restaurant_clone.query_item_for_table(2, "Burger").unwrap();
        println!("\n Burger order for table 2: \n{} \n", result);
    });


    let restaurant_clone = Arc::clone(&restaurant);
    let thread9= thread::spawn(move || {
        let result= restaurant_clone.show_all_orders();
        println!("\n All orders for all tables: \n{} \n", result);
    });
    

    
    thread8.join().unwrap();
    thread9.join().unwrap();
    
}
