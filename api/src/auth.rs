pub struct Claims {
    pub exp: usize, // Expiry time of the token ~ should be atleast 5 days
    pub iat: usize,
    pub username: String, // username associated with the token
}

#[derive(Deserialize)]
pub struct AuthData {
    pub username: String, // Email entered during sign-in
    pub password: String, // Password entered during sign-in
    pub gamecode: String, // required to authenticate a new user
}
