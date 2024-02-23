//* HashMap Practice
//* HashMap can retrieve data very fast
//* Data is printed in random order just like `Java`, so be careful!

use std::collections::HashMap;

pub fn stock_info() {
  let mut stock: HashMap<String, u8> = HashMap::new();

  stock.insert("Table".to_owned(), 5);
  stock.insert("Chair".to_owned(), 10);
  stock.insert("Bed".to_owned(), 3);
  stock.insert("Couch".to_owned(), 0);

  let mut total_items: u8 = 0;

  //* When iterating over a Hash Map, the parameters are automatically borrowed
  //* So instead of writing `0`, you must write `&0`
  for (item, quantity) in stock.iter() {
    let stock_status: String = if quantity == &0 {
      "item is out of stock".to_owned()
    } else {
      //* Format Macro Returns a `String` just like Java `String.format()`
      format!("{}", quantity)
    };

    print!("Item: {}, Status: {} \n", item, stock_status);

    total_items += quantity;
  }

  print!("Hash Map: {:?} \n", stock);
  print!("Hash Map Size: {} \n", stock.len());
  print!("Hash Map Contains 'Bed': {} \n", stock.contains_key("Bed"));
  print!("Hash Map Keys: {:?} \n", stock.keys());
  print!("Hash Map Values: {:?} \n", stock.values());
  print!("Total Stock Number: {} \n", total_items);
}
