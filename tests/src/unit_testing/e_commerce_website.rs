#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Product {
    Shampoo,
    Book,
}

type Quantity = u32;

struct Store {
    inventories: HashMap<Product, Quantity>,
}

impl Store {
    fn new() -> Self {
        Self {
            inventories: HashMap::new(),
        }
    }

    fn get_inventory(&self, product: Product) -> Quantity {
        *self.inventories.get(&product).unwrap_or(&0)
    }

    fn has_enough_inventory(&self, product: Product, quantity: Quantity) -> bool {
        self.get_inventory(product) >= quantity
    }

    fn add_inventory(&mut self, product: Product, quantity: Quantity) {
        let inventory = self.get_inventory(product);
        let new_inventory = inventory + quantity;
        self.inventories.insert(product, new_inventory);
    }
}

struct Customer;

impl Customer {
    fn new() -> Self {
        Self
    }

    fn purchase(&self, store: &mut Store, product: Product, quantity: Quantity) -> bool {
        if !store.has_enough_inventory(product, quantity) {
            return false;
        }

        let inventory = store.get_inventory(product);
        let new_inventory = inventory - quantity;
        store.inventories.insert(product, new_inventory);

        true
    }
}

#[cfg(test)]
mod detroit_school {
    use super::*;

    #[test]
    fn purchase_succeeds_when_enough_inventory() {
        let mut store = Store::new();
        store.add_inventory(Product::Shampoo, 10);
        let customer = Customer::new();

        let success = customer.purchase(&mut store, Product::Shampoo, 5);

        assert_eq!(true, success);
        assert_eq!(5, store.get_inventory(Product::Shampoo));
    }

    #[test]
    fn purchase_fails_when_not_enough_inventory() {
        let mut store = Store::new();
        store.add_inventory(Product::Shampoo, 10);
        let customer = Customer::new();

        let success = customer.purchase(&mut store, Product::Shampoo, 15);

        assert_eq!(false, success);
        assert_eq!(10, store.get_inventory(Product::Shampoo));
    }
}

#[cfg(test)]
mod london_school {
    // TODO
}
