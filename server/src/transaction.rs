#[derive(Clone, serde::Serialize, zvariant::Type)]
pub struct  Transaction {
    pub id: u32,
    pub app_id: String,
    pub status: String,
    pub progress: f64,
    pub error: String,
}
