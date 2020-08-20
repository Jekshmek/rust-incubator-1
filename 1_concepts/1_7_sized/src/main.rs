use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
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

#[derive(Clone, Debug)]
pub struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

trait UserRepository<K> {
    type Error;

    fn get_user(&self, key: &K) -> Option<&User>;

    fn add_user(&mut self, key: K, user: User) -> Result<(), Self::Error>;

    fn update_user(&mut self, key: K, user: User) -> Result<(), Self::Error>;

    fn remove_user(&mut self, key: &K) -> Result<User, Self::Error>;
}

struct StaticUserRepository<K, S>
where
    S: Storage<K, User>,
{
    storage: S,
    _key: PhantomData<K>,
}

impl<K, S> StaticUserRepository<K, S>
where
    S: Storage<K, User>,
{
    fn new(storage: S) -> Self {
        StaticUserRepository {
            storage,
            _key: PhantomData,
        }
    }
}

impl<K, S> UserRepository<K> for StaticUserRepository<K, S>
where
    S: Storage<K, User>,
{
    type Error = &'static str;

    fn get_user(&self, key: &K) -> Option<&User> {
        self.storage.get(key)
    }

    fn add_user(&mut self, key: K, user: User) -> Result<(), Self::Error> {
        if self.storage.get(&key).is_some() {
            return Err("User with that key already exists!");
        }

        self.storage.set(key, user);

        Ok(())
    }

    fn update_user(&mut self, key: K, user: User) -> Result<(), Self::Error> {
        if self.storage.get(&key).is_none() {
            return Err("No user with that key!");
        }

        self.storage.set(key, user);

        Ok(())
    }

    fn remove_user(&mut self, key: &K) -> Result<User, Self::Error> {
        self.storage.remove(key).ok_or("No user with that key!")
    }
}

trait Command {}

trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &mut Self::Context) -> Self::Result;
}

struct CreateUser {
    key: i32,
}

impl Command for CreateUser {}

impl CommandHandler<CreateUser> for User {
    type Context = dyn UserRepository<i32, Error = &'static str>;
    type Result = Result<(), &'static str>;

    fn handle_command(&self, cmd: &CreateUser, ctx: &mut Self::Context) -> Self::Result {
        ctx.add_user(cmd.key, self.clone())
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_user() {
        let storage: UserStorage<i32> = UserStorage::default();
        let mut repo = StaticUserRepository::new(storage);

        let user = User {
            id: 0,
            email: "user@email.com".into(),
            activated: true,
        };
        let cmd = CreateUser { key: 1 };

        user.handle_command(&cmd, &mut repo).unwrap();

        assert_eq!(repo.get_user(&1).unwrap().id, 0);
    }
}
