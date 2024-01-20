#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Product {
    Shampoo,
    Book,
}

type Quantity = u32;

#[cfg_attr(test, mockall::automock)]
trait IStore {
    fn get_inventory(&self, product: Product) -> Quantity;

    fn has_enough_inventory(&self, product: Product, quantity: Quantity) -> bool;

    fn add_inventory(&mut self, product: Product, quantity: Quantity);

    fn remove_inventory(&mut self, product: Product, quantity: Quantity) -> bool;
}

struct Store {
    inventories: HashMap<Product, Quantity>,
}

impl Store {
    fn new() -> Self {
        Self {
            inventories: HashMap::new(),
        }
    }
}

impl IStore for Store {
    fn get_inventory(&self, product: Product) -> Quantity {
        *self.inventories.get(&product).unwrap_or(&0)
    }

    fn has_enough_inventory(&self, product: Product, quantity: Quantity) -> bool {
        self.get_inventory(product) >= quantity
    }

    fn add_inventory(&mut self, product: Product, quantity: Quantity) {
        *self.inventories.entry(product).or_insert(0) += quantity;
    }

    fn remove_inventory(&mut self, product: Product, quantity: Quantity) -> bool {
        if !self.has_enough_inventory(product, quantity) {
            return false;
        }

        *self.inventories.entry(product).or_insert(0) -= quantity;

        true
    }
}

struct Customer;

impl Customer {
    fn new() -> Self {
        Self
    }

    fn purchase(&self, store: &mut impl IStore, product: Product, quantity: Quantity) -> bool {
        store.remove_inventory(product, quantity)
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
    use super::*;
    use mockall::predicate::eq;

    #[test]
    fn purchase_succeeds_when_enough_inventory() {
        let mut store_mock = MockIStore::new();
        store_mock
            .expect_remove_inventory()
            .with(eq(Product::Shampoo), eq(5))
            .times(1)
            .returning(|_, _| true);
        let customer = Customer::new();

        let success = customer.purchase(&mut store_mock, Product::Shampoo, 5);

        assert_eq!(true, success);
    }

    #[test]
    fn purchase_fails_when_not_enough_inventory() {
        let mut store_mock = MockIStore::new();
        store_mock
            .expect_remove_inventory()
            .with(eq(Product::Shampoo), eq(15))
            .times(1)
            .returning(|_, _| false);
        let customer = Customer::new();

        let success = customer.purchase(&mut store_mock, Product::Shampoo, 15);

        assert_eq!(false, success);
    }
}
