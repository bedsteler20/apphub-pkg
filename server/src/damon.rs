use crate::imp;
use async_std::{sync::Mutex, task};
use libflatpak::glib;
use types::TransactionType;
use types::{AppInfo, InstallLocation, Transaction};
use zbus::{dbus_interface, fdo, SignalContext};

pub struct ApphubDamon {
    store: Mutex<Vec<Transaction>>,
}

impl ApphubDamon {
    pub async fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
        }
    }
}

#[dbus_interface(name = "dev.bedsteler20.ApphubDamon")]
impl ApphubDamon {
    async fn get_app_info(&self, app_id: &str) -> fdo::Result<AppInfo> {
        match imp::get_app_info(app_id) {
            Ok(Some(app_info)) => Ok(app_info),
            Ok(None) => Err(fdo::Error::Failed("App not found".into())),
            Err(e) => Err(fdo::Error::Failed(e.to_string())),
        }
    }

    async fn is_app_installed(&self, app_id: &str) -> fdo::Result<bool> {
        match imp::is_app_installed(app_id) {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(e) => Err(fdo::Error::Failed(e.to_string())),
        }
    }

    async fn list_transactions(&self) -> fdo::Result<Vec<Transaction>> {
        let store = self.store.lock().await;
        Ok(store.clone())
    }

    async fn get_transaction(&self, transaction_id: u32) -> fdo::Result<Transaction> {
        let store = self.store.lock().await;
        let transaction = store.iter().find(|t| t.id == transaction_id).unwrap();
        Ok(transaction.clone())
    }

    // This crates a new transaction and returns the transaction id
    // The transaction id is will be use when calling install, uninstall, update, etc.
    // This is so that the client get the id before the transaction starts
    // because actually running the transaction will occupy the thread handling the request
    // and the client will not be able to get the id. This way the client can get the id
    // and track the progress of the transaction while its running
    async fn create_transaction(
        &self,
        app_id: &str,
        install_location: InstallLocation,
        transaction_type: TransactionType,
        #[zbus(signal_context)] ctx: SignalContext<'_>,
    ) -> fdo::Result<u32> {
        let mut store = self.store.lock().await;
        let transaction = Transaction::new(app_id, install_location, transaction_type);
        let id = transaction.id;
        store.push(transaction.clone());
        println!("Transaction added");
        Self::transaction_added(&ctx, transaction.clone())
            .await
            .unwrap();
        Ok(id)
    }

    async fn install(
        &self,
        transaction_id: u32,
        #[zbus(signal_context)] ctx: SignalContext<'_>,
    ) -> fdo::Result<()> {
        enum Msg {
            Progress(f64),
            Error(String),
            Done,
        }

        let (sender, receiver) = async_std::channel::unbounded::<Msg>();

        task::spawn(async move {
            loop {
                task::sleep(std::time::Duration::from_millis(1000)).await;
                let r = glib::random_int_range(0, 100) as f64;
                let progress = r / 100.0;
                sender.send(Msg::Progress(progress)).await.unwrap();
            }
        });

        loop {
            let msg = receiver.recv().await.unwrap();
            match msg {
                Msg::Progress(progress) => {
                    let mut store = self.store.lock().await;
                    let transaction = store.iter_mut().find(|t| t.id == transaction_id).unwrap();
                    transaction.progress = progress;

                    Self::progress_changed(&ctx, transaction_id, progress)
                        .await
                        .unwrap();
                }
                Msg::Error(error) => {
                    let mut store = self.store.lock().await;
                    let transaction = store.iter_mut().find(|t| t.id == transaction_id).unwrap();
                    transaction.error = error;
                }
                Msg::Done => {
                    let mut store = self.store.lock().await;
                    Self::transaction_done(&ctx, transaction_id).await.unwrap();
                    store.retain(|t| t.id != transaction_id);
                    break;
                }
            }
        }

        Ok(())
    }

    #[dbus_interface(signal)]
    async fn progress_changed(
        ctx: &SignalContext<'_>,
        transaction_id: u32,
        progress: f64,
    ) -> Result<(), zbus::Error>;

    #[dbus_interface(signal)]
    async fn transaction_added(
        ctx: &SignalContext<'_>,
        transaction: Transaction,
    ) -> Result<(), zbus::Error>;

    #[dbus_interface(signal)]
    async fn transaction_error(
        ctx: &SignalContext<'_>,
        transaction_id: u32,
        error: String,
    ) -> Result<(), zbus::Error>;

    #[dbus_interface(signal)]
    async fn transaction_done(
        ctx: &SignalContext<'_>,
        transaction_id: u32,
    ) -> Result<(), zbus::Error>;
}
