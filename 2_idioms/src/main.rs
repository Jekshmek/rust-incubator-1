mod store {
    use crate::store::private::VendingMachineStateSecure;
    use std::borrow::Cow;
    use std::collections::BTreeMap;

    #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
    pub enum Coin {
        One = 1,
        Two = 2,
        Five = 5,
        Ten = 10,
        Twenty = 20,
        Fifty = 50,
    }

    #[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Product<'a> {
        price: usize,
        name: Cow<'a, str>,
    }

    struct PriceAndAmount {
        price: usize,
        amount: usize,
    }

    pub struct VendingMachine<'a, State> {
        products: BTreeMap<Cow<'a, str>, PriceAndAmount>,
        space_left: usize,
        coins: BTreeMap<Coin, usize>,
        state: State,
    }

    impl<'a> VendingMachine<'a, Ready> {
        pub fn new(capacity: usize) -> Self {
            VendingMachine {
                products: BTreeMap::new(),
                space_left: capacity,
                coins: BTreeMap::new(),
                state: Ready,
            }
        }

        pub fn add_product<'b: 'a>(
            machine: &mut Self,
            name: impl Into<Cow<'b, str>>,
            price: usize,
        ) -> Result<(), VendingMachineError> {
            if machine.space_left == 0 {
                return Err(VendingMachineError::OutOfFreeSpace);
            }

            let name = name.into();
            let entry = machine
                .products
                .iter_mut()
                .find(|(product_name, _)| **product_name == name);

            if let Some((_, data)) = entry {
                if data.price == price {
                    data.amount += 1;
                    machine.space_left -= 1;
                } else {
                    return Err(VendingMachineError::SameProductNameDifferentPrice);
                }
            } else {
                machine
                    .products
                    .insert(name, PriceAndAmount { price, amount: 0 });
                machine.space_left -= 1;
            }

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
                .position(|(name, _)| name == product_name)
                .ok_or(VendingMachineError::NoProduct)?;

            Ok(VendingMachine {
                products: self.products,
                space_left: self.space_left,
                coins: self.coins,
                state: Paying {
                    product_id,
                    payed: Vec::new(),
                },
            })
        }
    }

    pub enum VendingMachineError {
        SameProductNameDifferentPrice,
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
