# Restaurant App (Multithreaded Version)

This repository hosts a restaurant application designed to manage menu items, their preparation times, and orders. The application enables a quick overview of all or specific items in its list and facilitates the removal of specific orders as needed.

## Getting Started

1. **Environment:**
   - Make sure you have rustc and cargo installed

2. **Clone the repository:**
   - Clone the repository  using
   ```bash
   git clone git@github.com:zaidan-bhat/Restaurant.git
   ```
   - Checkout to the Multithreaded Branch using
   ```bash
   $ git checkout Multithreaded
   ```

To execute the code after cloning the repository, use the following command:
```bash
$ cargo run
```

Expected output:

```bash
Orders for table 1: 
 {"Coffe":{"item_name":"Coffe","preparation_time":6,"quantity":1},"Pizza":{"item_name":"Pizza","preparation_time":7,"quantity":1}} 

 Burger order for table 2: 
{"item_name":"Burger","preparation_time":15,"quantity":2,"table_number":2} 


 All orders for all tables: 
{"1":{"orders":{"Coffe":{"item_name":"Coffe","preparation_time":6,"quantity":1}},"table_number":1},"2":{"orders":{"Burger":{"item_name":"Burger","preparation_time":15,"quantity":2},"Pizza":{"item_name":"Pizza","preparation_time":5,"quantity":1}},"table_number":2},"3":{"orders":{"Pizza":{"item_name":"Pizza","preparation_time":15,"quantity":1}},"table_number":3}} 
```

## Implementation Details

The restaurant functionality is implemented using a struct. Order details are stored in a HashMap named "tables," where table numbers map to a HashMap containing item details. The item details are stored as a tuple, including the preparation time and quantity ordered.

The choice of this data structure prioritizes fast lookups. Viewing all orders for a table or a specific item for a table is achieved in constant time (O(1)).

## Restaurant Struct Methods

1. **add_order:**
   - Parameters: table number, item name, quantity.
   - Description: Adds an order for the specified item and quantity to the given table number. The preparation time is randomly generated between 5 and 15.

2. **remove_order:**
   - Parameters: table number, item name, quantity.
   - Description: Removes the specified quantity of the given item from the specified table.

3. **query_table:**
   - Parameters: table number.
   - Description: Returns a JSON string containing information about all orders for the given table.

4. **query_item_for_table:**
   - Parameters: table number, item name.
   - Description: Returns details of the order for the specified item and table number in JSON format.

5. **show_all_orders:**
   - Description: Returns, in JSON format, details of all orders for all tables.

## Error Handling

Error handling mechanisms are implemented in the server-side code. However, error handling in the main.rs file is intentionally left open for customization by the client. The server ensures relevant error responses.

## Concurrency

This code is written such that it can handle multiple requests simultaneously. Multiple clients simulated using threads in the main.rs file can send requests to the server at one time. This code is written to be thread safe. So if multiple clients are trying to add orders simultaneously we will still end up with consistent data. There will be no overwrites. This has been achieved using locks. Multiple requests of different types can also be processed simultaneously.


## Important note

One very important thing to note is that if we send an add request and a query request for a table at the same time we might or might not see the add reflected in the result of the query request. This entirely depends on which thread gets the lock first. If the thread with the add request gets the lock first then we will be able to see new result. However if the thread with the query request gets the lock first then we will see the old result. 
