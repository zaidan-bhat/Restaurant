use std::thread;
use std::sync::Arc;

pub mod app;

fn main() {
    let restaurant = Arc::new(app::Restaurant::new());

    // Spawn two threads to perform operations concurrently
    let restaurant_clone = Arc::clone(&restaurant);
    let thread1 = thread::spawn(move || {
        restaurant_clone.add_order(String::from("Pizza"), 1, 1);
    });

    let restaurant_clone = Arc::clone(&restaurant);
    let thread2 = thread::spawn(move || {
        restaurant_clone.add_order(String::from("Pizza"), 2, 1);
    });

    let restaurant_clone = Arc::clone(&restaurant);
    let thread3 = thread::spawn(move || {
        restaurant_clone.add_order(String::from("Pizza"), 3, 1);
    });

    let restaurant_clone = Arc::clone(&restaurant);
    let thread4 = thread::spawn(move || {
        restaurant_clone.add_order(String::from("Burger"), 2, 2);
    });

    let restaurant_clone = Arc::clone(&restaurant);
    let thread5 = thread::spawn(move || {
        restaurant_clone.add_order(String::from("Coffe"), 1, 1);
    });



    let restaurant_clone = Arc::clone(&restaurant);
    let thread6 = thread::spawn(move || {
        restaurant_clone.remove_order(1, "Pizza", 1);
        restaurant_clone.query_table(1);
    });

    let restaurant_clone = Arc::clone(&restaurant);
    let thread7= thread::spawn(move || {
        restaurant_clone.print_all_orders();
    });
    // Wait for threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();
    thread5.join().unwrap();
    thread6.join().unwrap();
    thread7.join().unwrap();
    // thread8.join().unwrap();
    // thread9.join().unwrap();
    // thread10.join().unwrap();
    // Print all orders after concurrent operations
}
