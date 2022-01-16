use crate::models::users::User;

impl User {
    pub fn new(
        id: i32,
        name: String,
        email: String,
        phone: String,
        first_name: String,
        last_name: String,
    ) -> User {
        User {
            id,
            name,
            email,
            phone,
            first_name,
            last_name,
        }
    }

    pub fn show(self) {
        println!(
            "id: {}, name: {}, email: {}, phone: {}",
            self.id, self.name, self.email, self.phone
        )
    }
}

impl Default for User {
    fn default() -> User {
        User {
            id: 0,
            name: String::from(""),
            email: String::from(""),
            phone: String::from(""),
            first_name: String::from(""),
            last_name: String::from(""),
        }
    }
}
