pub struct Transaction {
    pub id: u32,
    pub from_user_id: u32,
    pub to_user_id: u32,
    pub amount: f64,
    pub timestamp: String,
    pub status: TransactionStatus,
}

#[derive(Debug)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed(String),
}