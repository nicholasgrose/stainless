use std::collections::HashMap;

use velcro::hash_map;

use crate::web::schema::User;

#[derive(Default, Clone)]
pub struct Database {
    users: HashMap<i32, User>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            users: hash_map! {
                1: User {
                    id: 1,
                    name: "Aron".to_string(),
                },
                2: User {
                    id: 2,
                    name: "Bea".to_string(),
                },
                3: User {
                    id: 3,
                    name: "Carl".to_string(),
                },
                4: User {
                    id: 4,
                    name: "Dora".to_string(),
                },
            },
        }
    }

    pub fn get_user(&self, id: &i32) -> Option<&User> {
        self.users.get(id)
    }
}

impl juniper::Context for Database {}
