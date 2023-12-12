use types::Transaction;

#[zbus::dbus_proxy(
    default_service = "dev.bedsteler20.ApphubDamon",
    interface = "dev.bedsteler20.ApphubDamon",
    default_path = "/dev/bedsteler20/ApphubDamon"
)]
trait _ApphubDamon {
    fn list_transactions(&self) -> zbus::Result<Vec<Transaction>>;
    fn get_transaction(&self, transaction_id: u32) -> zbus::Result<Transaction>;
    fn get_app_info(&self, app_id: &str) -> zbus::Result<types::AppInfo>;
    fn is_app_installed(&self, app_id: &str) -> zbus::Result<bool>;
    fn create_transaction(
        &self,
        app_id: &str,
        install_location: types::InstallLocation,
        transaction_type: types::TransactionType,
    ) -> zbus::Result<u32>;

    fn install(&self, transaction_id: u32) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn progress_changed(&self, transaction_id: u32, progress: f64) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn transaction_added(&self, transaction: Transaction) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn transaction_error(&self, transaction_id: u32, error: String) -> zbus::Result<()>;

    #[dbus_proxy(signal)]
    fn transaction_done(&self, transaction_id: u32) -> zbus::Result<()>;
}

pub use _ApphubDamonProxy as ApphubDamonProxy;
