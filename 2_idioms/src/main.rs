mod store {
    use std::borrow::Cow;
    use std::collections::btree_map::Entry;
    use std::collections::BTreeMap;

    use crate::store::coin::Coin;
    use crate::store::private::VendingMachineStateSecure;

    pub mod coin {
        use std::convert::TryFrom;

        #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        pub enum Coin {
            One = 1,
            Two = 2,
            Five = 5,
            Ten = 10,
            Twenty = 20,
            Fifty = 50,
        }

        impl TryFrom<u8> for Coin {
            type Error = CoinError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    1 => Ok(Coin::One),
                    2 => Ok(Coin::Two),
                    5 => Ok(Coin::Five),
                    10 => Ok(Coin::Ten),
                    20 => Ok(Coin::Twenty),
                    50 => Ok(Coin::Fifty),
                    _ => Err(CoinError::NoSuchCoin),
                }
            }
        }

        pub enum CoinError {
            NoSuchCoin,
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Product<'a> {
        price: usize,
        name: Cow<'a, str>,
    }

    #[derive(Clone, Copy, Debug)]
    struct PriceAndAmount {
        price: usize,
        amount: usize,
    }

    #[derive(Clone, Debug)]
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

        pub fn add_products<'b, C, I>(
            machine: &mut Self,
            products: I,
        ) -> Result<(), VendingMachineError>
        where
            'b: 'a,
            C: Into<Cow<'b, str>>,
            I: IntoIterator<Item = (C, usize)>,
        {
            for (name, price) in products.into_iter() {
                Self::add_product(machine, name, price)?;
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

        pub fn choose(
            mut self,
            product: impl Into<Cow<'a, str>>,
        ) -> Result<VendingMachine<'a, Paying<'a>>, VendingMachineError> {
            let product_name = product.into();
            let product = self.products.entry(product_name);

            let product = match product {
                Entry::Occupied(x) => Product {
                    price: x.get().price,
                    name: x.key().clone(),
                },
                Entry::Vacant(_) => return Err(VendingMachineError::NoProduct),
            };

            Ok(VendingMachine {
                products: self.products,
                space_left: self.space_left,
                coins: self.coins,
                state: Paying {
                    product,
                    payed: Vec::new(),
                },
            })
        }
    }

    impl<'a> VendingMachine<'a, Paying<'a>> {
        pub fn insert_coin(&mut self, coin: Coin) {
            self.state.payed.push(coin);
        }

        pub fn inset_coins<I: IntoIterator<Item = Coin>>(&mut self, coins: I) {
            for coin in coins.into_iter() {
                Self::insert_coin(self, coin);
            }
        }

        pub fn get_product(
            mut self,
        ) -> Result<(VendingMachine<'a, Ready>, Product<'a>, Vec<Coin>), VendingMachineError>
        {
            let payed = self
                .state
                .payed
                .iter()
                .fold(0, |acc, coin| acc + *coin as usize);

            let rest = payed
                .checked_sub(self.state.product.price)
                .ok_or(VendingMachineError::NotEnoughMoney)?;

            let rest_coins = Self::calc_rest(rest, Cow::Borrowed(&self.coins));

            let product_name = self.state.product.name.clone();
            if let Some(rest_coins) = rest_coins {
                let (_, data) = self
                    .products
                    .iter_mut()
                    .find(|(name, _)| **name == product_name)
                    .unwrap();

                data.amount -= 1;

                if data.amount == 0 {
                    self.products.remove(&*product_name);
                }

                return Ok((
                    VendingMachine {
                        products: self.products,
                        space_left: self.space_left,
                        coins: self.coins,
                        state: Ready,
                    },
                    Product {
                        price: self.state.product.price,
                        name: self.state.product.name,
                    },
                    rest_coins,
                ));
            }

            Err(VendingMachineError::CantGiveRest(self.state.payed))
        }

        fn calc_rest(rest: usize, coins: Cow<BTreeMap<Coin, usize>>) -> Option<Vec<Coin>> {
            Self::calc_rest_internal(rest, coins, 0, Cow::Owned(Vec::new()))
        }

        fn calc_rest_internal(
            rest: usize,
            mut coins: Cow<BTreeMap<Coin, usize>>,
            cur_sum: usize,
            rest_coins: Cow<Vec<Coin>>,
        ) -> Option<Vec<Coin>> {
            if rest == cur_sum {
                return Some(Vec::clone(&rest_coins));
            }

            if cur_sum < rest {
                if let Some((coin, amount)) = coins.clone().to_mut().iter_mut().rev().next() {
                    return if *amount == 0 {
                        coins.to_mut().remove(coin);
                        Self::calc_rest_internal(rest, coins, cur_sum, rest_coins)
                    } else {
                        *amount -= 1;

                        let new_sum = cur_sum + *coin as usize;
                        let mut new_rest_coins = rest_coins.clone();
                        new_rest_coins.to_mut().push(*coin);

                        Self::calc_rest_internal(rest, coins.clone(), cur_sum, rest_coins).or_else(
                            || Self::calc_rest_internal(rest, coins, new_sum, new_rest_coins),
                        )
                    };
                }
            }

            None
        }
    }

    #[derive(Clone, Debug)]
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

    pub struct Paying<'a> {
        product: Product<'a>,
        payed: Vec<Coin>,
    }
    impl<'a> VendingMachineState for Paying<'a> {}

    mod private {
        use super::*;

        pub trait VendingMachineStateSecure {}

        impl VendingMachineStateSecure for Ready {}
        impl<'a> VendingMachineStateSecure for Paying<'a> {}
    }

    #[cfg(test)]
    mod tests {
        // TODO: test everything
    }
}

fn main() {
    println!("Implement me!");
}
