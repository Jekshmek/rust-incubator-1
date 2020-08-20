use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;

pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Clone, Debug)]
pub struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

#[derive(Default, Clone)]
struct UserStorage<K: Clone>(HashMap<K, User>);

impl<K: Eq + Hash + Clone> Storage<K, User> for UserStorage<K> {
    fn set(&mut self, key: K, val: User) {
        self.0.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&User> {
        self.0.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<User> {
        self.0.remove(key)
    }
}

mod static_user_repo {
    use super::*;
    use std::marker::PhantomData;

    type Error = &'static str;

    pub struct UserRepository<K, S>
    where
        S: Storage<K, User>,
    {
        storage: S,
        _key: PhantomData<K>,
    }

    impl<K, S> UserRepository<K, S>
    where
        S: Storage<K, User>,
    {
        pub fn new(storage: S) -> Self {
            UserRepository {
                storage,
                _key: PhantomData,
            }
        }

        pub fn get_user(&self, key: &K) -> Option<&User> {
            self.storage.get(key)
        }

        pub fn add_user(&mut self, key: K, user: User) -> Result<(), Error> {
            if self.storage.get(&key).is_some() {
                return Err("User with that key already exists!");
            }

            self.storage.set(key, user);

            Ok(())
        }

        pub fn update_user(&mut self, key: K, updated_user: User) -> Result<(), Error> {
            if self.storage.get(&key).is_none() {
                return Err("No user with that key!");
            }

            self.storage.set(key, updated_user);

            Ok(())
        }

        pub fn remove_user(&mut self, key: &K) -> Result<User, Error> {
            self.storage.remove(key).ok_or("No user with that key!")
        }
    }
}

mod dynamic_user_repo {
    use super::*;
    use std::marker::PhantomData;

    type Error = &'static str;

    pub struct UserRepository<K> {
        _key: PhantomData<K>,
        storage: Box<dyn Storage<K, User>>,
    }

    impl<K> UserRepository<K> {
        pub fn new<S: 'static>(storage: S) -> Self
        where
            S: Storage<K, User>,
        {
            UserRepository {
                _key: PhantomData,
                storage: Box::new(storage),
            }
        }

        pub fn get_user(&self, key: &K) -> Option<&User> {
            self.storage.get(key)
        }

        pub fn add_user(&mut self, key: K, user: User) -> Result<(), Error> {
            if self.storage.get(&key).is_some() {
                return Err("User with that key already exists!");
            }

            self.storage.set(key, user);

            Ok(())
        }

        pub fn update_user(&mut self, key: K, updated_user: User) -> Result<(), Error> {
            if self.storage.get(&key).is_none() {
                return Err("No user with that key!");
            }

            self.storage.set(key, updated_user);

            Ok(())
        }

        pub fn remove_user(&mut self, key: &K) -> Result<User, Error> {
            self.storage.remove(key).ok_or("No user with that key!")
        }
    }
}

fn main() {
    let storage: UserStorage<i32> = UserStorage::default();

    let mut static_repo = static_user_repo::UserRepository::new(storage.clone());
    let mut dynamic_repo = dynamic_user_repo::UserRepository::new(storage);

    let user = User {
        id: 0,
        email: "user@email.com".into(),
        activated: true,
    };

    static_repo.add_user(1, user.clone()).unwrap();
    dbg!(static_repo.get_user(&1));

    dynamic_repo.add_user(1, user).unwrap();
    dbg!(dynamic_repo.get_user(&1));
}
