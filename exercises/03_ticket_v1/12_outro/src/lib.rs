// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

use core::panic;
// pub mod order {

pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {
    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Self {
        if Order::check_product_name(&product_name)
            && Order::check_quantity(&quantity)
            && Order::check_unit_price(&unit_price)
        {
            Self {
                product_name,
                quantity,
                unit_price,
            }
        } else {
            panic!("ugh");
        }
    }

    fn check_product_name(name: &String) -> bool {
        if name.len() > 0 && name.len() <= 300 {
            return true;
        }
        false
    }

    fn check_quantity(quantity: &i32) -> bool {
        if *quantity <= 0 {
            return false;
        }
        true
    }

    fn check_unit_price(price: &i32) -> bool {
        if *price > 0 {
            return true;
        }
        return false;
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, new_name: String) {
        if (Order::check_product_name(&new_name)) {
            self.product_name = new_name;
        }
    }

    pub fn set_quantity(&mut self, new_quantity: i32) {
        if (Order::check_quantity(&new_quantity)) {
            self.quantity = new_quantity;
        }
    }
    pub fn set_unit_price(&mut self, new_price: i32) {
        if (Order::check_unit_price(&new_price)) {
            self.unit_price = new_price;
        }
    }
    pub fn total(&self) -> i32 {
        return self.quantity * self.unit_price;
    }
}
// }
