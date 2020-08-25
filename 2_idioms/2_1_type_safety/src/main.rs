use crate::utils::post;
use crate::utils::user;
use crate::utils::Post;

mod utils {
    use crate::utils::post::{Body, Title};
    use crate::utils::private::PostStateSecured;

    pub mod post {
        #[derive(Clone, Debug, PartialEq)]
        pub struct Id(u64);

        impl Id {
            pub fn new(data: u64) -> Self {
                Id(data)
            }
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct Title(String);

        impl Title {
            pub fn new(data: String) -> Self {
                Title(data)
            }
        }

        #[derive(Clone, Debug, PartialEq)]
        pub struct Body(String);

        impl Body {
            pub fn new(data: String) -> Self {
                Body(data)
            }
        }
    }

    pub mod user {
        #[derive(Clone, Debug, PartialEq)]
        pub struct Id(u64);

        impl Id {
            pub fn new(data: u64) -> Self {
                Id(data)
            }
        }
    }

    pub trait PostState: PostStateSecured {}

    #[derive(Clone)]
    pub struct Post<S: PostState> {
        id: post::Id,
        user_id: user::Id,
        title: post::Title,
        body: post::Body,
        state: S,
    }

    pub struct New;
    impl PostState for New {}

    pub struct Unmoderated;
    impl PostState for Unmoderated {}

    pub struct Published;
    impl PostState for Published {}

    pub struct Deleted;
    impl PostState for Deleted {}

    mod private {
        use super::*;

        pub trait PostStateSecured {}

        impl PostStateSecured for New {}
        impl PostStateSecured for Unmoderated {}
        impl PostStateSecured for Published {}
        impl PostStateSecured for Deleted {}
    }

    impl<S: PostState> Post<S> {
        fn transform<T: PostState>(self, to: T) -> Post<T> {
            Post {
                id: self.id,
                user_id: self.user_id,
                title: self.title,
                body: self.body,
                state: to,
            }
        }
    }

    impl Post<New> {
        pub fn new(id: post::Id, user_id: user::Id, title: post::Title, body: post::Body) -> Self {
            Post {
                id,
                user_id,
                title,
                body,
                state: New,
            }
        }

        pub fn publish(self) -> Post<Unmoderated> {
            self.transform(Unmoderated)
        }
    }

    impl Post<Unmoderated> {
        pub fn allow(self) -> Post<Published> {
            self.transform(Published)
        }

        pub fn deny(self) -> Post<Deleted> {
            self.transform(Deleted)
        }
    }

    impl Post<Published> {
        pub fn delete(self) -> Post<Deleted> {
            self.transform(Deleted)
        }
    }
}

fn main() {
    let post = Post::new(
        post::Id::new(0),
        user::Id::new(0),
        post::Title::new("Test".into()),
        post::Body::new("Test post".into()),
    );

    let deleted_post = post.publish().allow().delete();
}
