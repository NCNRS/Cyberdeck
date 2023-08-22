/// User for the app. Currently just a username and password hash.
// @Todo In the future we might want to add roles 
#[derive(Debug, Default, Clone)]
struct User {
    hash: String,
    name: String,
}


