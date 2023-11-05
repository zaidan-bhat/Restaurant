# Restaurant App

This repository hosts a restaurant application designed to manage menu items, their preparation times, and orders. The application enables a quick overview of all or specific items in its list and facilitates the removal of specific orders as needed.

## Getting Started

1. **Environment:**
   - Make sure you have rustc and cargo installed

2. **Clone the repository:**
   - Clone the repository  using
   ```bash
   $ git clone git@github.com:zaidan-bhat/Restaurant.git
   ```
   - Checkout to the main Branch using
   ```bash
   $ git checkout main
   ```

To execute the code after cloning the repository, use the following command:

```bash
$ cargo run
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

## Functional Programming Principles

The code adheres to functional programming principles, ensuring that methods have no side effects. Methods like add_order and remove_order return a new Restaurant instance instead of modifiying the original one, promoting immutability in the calling code (main.rs). The absence of the "mut" keyword in main.rs reinforces this approach.


## Important note

This code is not thread safe. For instance if we call the add order method on the Restaurant instance from two different threads, the result from one of them would overwrite the result from other. So we would end up with inconsistent result. We cannot ensure consistensy. The methods need to be called sequencially and not concurrently.
