use im::OrdMap;
use std::borrow::Cow;
use std::process::id;

#[derive(Clone, Debug, Eq, PartialEq)]
struct User {
    id: u64,
    nickname: Cow<'static, str>,
}

trait UserRepository<'a, I>
where
    I: IntoIterator<Item = &'a User>,
{
    fn get_user(&'a self, id: u64) -> Option<&'a User>;

    fn get_users<T: Iterator<Item = u64>>(&'a self, ids: T) -> I;

    fn get_users_by_nickname<'b>(&'a self, nickname: impl Into<&'b str>) -> I;
}

#[derive(Default, Clone, Debug)]
struct UserStorage {
    users: OrdMap<u64, User>,
}

impl UserStorage {
    fn insert(&mut self, user: User) -> Option<User> {
        let id = user.id;
        self.users.insert(id, user)
    }

    fn iter<I>(&self, ids: I) -> UserStorageIterIds<I>
    where
        I: Iterator<Item = u64>,
    {
        UserStorageIterIds { storage: self, ids }
    }
}

impl<'storage> UserRepository<'storage, Vec<&'storage User>> for UserStorage {
    fn get_user(&'storage self, id: u64) -> Option<&'storage User> {
        self.users.get(&id)
    }

    fn get_users<T: Iterator<Item = u64>>(&'storage self, ids: T) -> Vec<&'storage User> {
        ids.map(|id| self.users.get(&id))
            .filter_map(|u| u)
            .collect()
    }

    fn get_users_by_nickname<'b>(
        &'storage self,
        nickname: impl Into<&'b str>,
    ) -> Vec<&'storage User> {
        let nickname = nickname.into();

        self.users
            .values()
            .filter(|u| u.nickname == nickname)
            .collect::<Vec<_>>()
    }
}

struct UserStorageIterIds<'storage, I> {
    storage: &'storage UserStorage,
    ids: I,
}

impl<'storage, I> Iterator for UserStorageIterIds<'storage, I>
where
    I: Iterator<Item = u64>,
{
    type Item = &'storage User;

    fn next(&mut self) -> Option<Self::Item> {
        self.ids.next().and_then(|id| self.storage.users.get(&id))
    }
}

fn main() {
    let mut storage = UserStorage::default();

    let users = vec![
        User {
            id: 0,
            nickname: "first".into(),
        },
        User {
            id: 1,
            nickname: "first".into(),
        },
        User {
            id: 2,
            nickname: "second".into(),
        },
        User {
            id: 3,
            nickname: "second".into(),
        },
    ];

    for user in users {
        storage.insert(user);
    }

    dbg!(storage.get_user(0));
    dbg!(storage.get_users(vec![1, 2, 3, 4, 5].into_iter()));
    dbg!(storage.get_users_by_nickname("first"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_get() {
        let mut s = UserStorage::default();
        let user = User {
            id: 0,
            nickname: "test".into(),
        };

        s.insert(user.clone());
        assert_eq!(*s.get_user(0).unwrap(), user);
    }
}
