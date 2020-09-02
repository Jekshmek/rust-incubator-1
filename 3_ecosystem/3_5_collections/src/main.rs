use im::OrdMap;
use std::borrow::Cow;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct User {
    id: u64,
    nickname: Cow<'static, str>,
}

trait UserRepository {
    fn get_user(&self, id: u64) -> Option<User>;

    fn get_users(&self, ids: &[u64]) -> HashMap<u64, User>;

    fn get_users_by_nickname<'b>(&self, nickname: impl Into<&'b str>) -> HashSet<User>;
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
}

impl UserRepository for UserStorage {
    fn get_user(&self, id: u64) -> Option<User> {
        self.users.get(&id).cloned()
    }

    fn get_users(&self, ids: &[u64]) -> HashMap<u64, User, RandomState> {
        ids.iter()
            .filter_map(|id| self.get_user(*id).map(|u| (u.id, u)))
            .collect()
    }

    fn get_users_by_nickname<'b>(
        &self,
        nickname: impl Into<&'b str>,
    ) -> HashSet<User, RandomState> {
        let nickname = nickname.into();

        self.users
            .values()
            .filter(|u| u.nickname == nickname)
            .cloned()
            .collect()
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
    dbg!(storage.get_users(&[1, 2, 3, 4, 5]));
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
        assert_eq!(s.get_user(0).unwrap(), user);
    }

    #[test]
    fn get_by_ids() {
        let mut s = UserStorage::default();
        let users = sample_users();

        for user in users.clone() {
            s.insert(user);
        }

        s.get_users(&[0, 1, 2, 3, 4])
            .into_iter()
            .for_each(|(_, u)| assert!(users.iter().any(|u1| *u1 == u)));
    }

    #[test]
    fn get_by_nickname() {
        let mut s = UserStorage::default();
        let users = sample_users();

        for user in users.clone() {
            s.insert(user);
        }

        s.get_users_by_nickname("first")
            .into_iter()
            .for_each(|u| assert!(users.iter().any(|u1| *u1 == u)));
    }

    fn sample_users() -> Vec<User> {
        vec![
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
        ]
    }
}
