pub trait UsersService {
    /// This procedure is used to sign up a new user 
    /// @param username The users username
    /// @param email The users email
    /// @param password The users password
    fn handle_sign_up(username: String, email: String, password: String);

    /// This procedure is used to sign in as a user 
    /// @param username The users username
    /// @param password The users password
    /// @param attempt The number of times the user has attempted to sign in
    fn handle_sign_in(username: String, password: String, attempt: i32);

    /// This event is fired when a new account signs up 
    /// @param id The newly created users id
    /// @param email The users email
    fn emit_account_created(id: i32, email: String);

    /// This event is fired when a users email is validated 
    /// @param id The users id
    /// @param timestamp A timestamp from when the email was validated
    fn emit_email_validated(id: i32, timestamp: i32);

    /// This event is fired when a user has successfully signed in 
    /// @param id The users id
    /// @param timestamp A timestamp from when the user signed in
    fn emit_sign_in_complete(id: i32, timestamp: i32);
}