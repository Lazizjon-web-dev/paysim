pub enum Command {
    CreateUser {
        username: String,
        password: String,
        initial_balance: f64,
    },
    
    /// Send money to another user
    Send {
        sender: String,
        password: String,
        recipient: String,
        amount: f64,
    },
    
    /// Show transaction history
    History {
        username: String,
    },
    
    /// Show account balance
    Balance {
        username: String,
    },
    
    /// List all users
    ListUsers,
}