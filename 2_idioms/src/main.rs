mod store {
    use crate::store::private::VendingMachineStateSecure;
    use std::borrow::Cow;
    use std::collections::HashMap;

    #[derive(Debug, Hash, PartialEq, Eq)]
    pub enum Coin {
        One = 1,
        Two = 2,
        Five = 5,
        Ten = 10,
        Twenty = 20,
        Fifty = 50,
    }

    #[derive(Clone, Debug, Hash, PartialOrd, PartialEq)]
    pub struct Product<'a> {
        price: u16,
        name: Cow<'a, str>,
    }

    impl<'a> Product<'a> {
        pub fn new(price: u16, name: Cow<'a, str>) -> Self {
            Product { price, name }
        }
    }

    pub struct VendingMachine<'a, State> {
        products: Vec<Product<'a>>,
        coins: HashMap<Coin, usize>,
        state: State,
    }

    impl<'a> VendingMachine<'a, Ready> {
        pub fn new(capacity: usize) -> Self {
            VendingMachine {
                products: Vec::with_capacity(capacity),
                coins: HashMap::new(),
                state: Ready,
            }
        }

        pub fn add_product<'b: 'a>(
            machine: &mut Self,
            product: Product<'b>,
        ) -> Result<(), VendingMachineError> {
            if machine.products.len() == machine.products.capacity() {
                return Err(VendingMachineError::OutOfFreeSpace);
            }

            machine.products.push(product);
            Ok(())
        }

        pub fn add_coin(machine: &mut Self, coin: Coin) {
            machine
                .coins
                .entry(coin)
                .and_modify(|amount| *amount += 1)
                .or_default();
        }

        pub fn add_coins<I: IntoIterator<Item = Coin>>(machine: &mut Self, coins: I) {
            for coin in coins.into_iter() {
                VendingMachine::add_coin(machine, coin);
            }
        }

        pub fn choose<'b>(
            self,
            product: impl Into<&'b str>,
        ) -> Result<VendingMachine<'a, Paying>, VendingMachineError> {
            let product_name = product.into();
            let product_id = self
                .products
                .iter()
                .position(|item| item.name == product_name)
                .ok_or(VendingMachineError::NoProduct)?;

            Ok(VendingMachine {
                products: self.products,
                coins: self.coins,
                state: Paying {
                    product_id,
                    payed: Vec::new(),
                },
            })
        }
    }

    pub enum VendingMachineError {
        OutOfFreeSpace,
        NoProduct,
        NotEnoughMoney,
        CantGiveRest(Vec<Coin>),
    }

    pub trait VendingMachineState: VendingMachineStateSecure {}

    pub struct Ready;
    impl VendingMachineState for Ready {}

    pub struct Paying {
        product_id: usize,
        payed: Vec<Coin>,
    }
    impl VendingMachineState for Paying {}

    mod private {
        use super::*;

        pub trait VendingMachineStateSecure {}

        impl VendingMachineStateSecure for Ready {}
        impl VendingMachineStateSecure for Paying {}
    }
}

fn main() {
    println!("Implement me!");
}
