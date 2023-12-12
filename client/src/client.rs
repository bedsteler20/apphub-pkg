use crate::{
    proxy::{self, ApphubDamonProxy},
    transaction::{self, GTransaction},
};
use futures::StreamExt;
use glib::object::TypedObjectRef;
use once_cell::sync::{Lazy, OnceCell};
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

// ====== GObject Boilerplate ======
pub struct Client {
    pub transactions: gio::ListStore,
    // pub apps_with_updates: gio::ListStore,
}

impl Client {
    pub fn new() -> Client {
        Client {
            transactions: gio::ListStore::new::<GTransaction>(),
            // apps_with_updates: gio::ListStore::new(),
        }
    }

    #[allow(deprecated)]
    pub fn setup(&self) {
        enum Msg {
            TransactionAdded(types::Transaction),
            Progress(u32, f64),
        }
        let (sender, receiver) = glib::MainContext::channel::<Msg>(glib::Priority::default());
        RUNTIME.spawn(async move {
            let conn = zbus::Connection::session()
                .await
                .expect("Failed to connect to the session bus");
            let proxy = ApphubDamonProxy::new(&conn)
                .await
                .expect("Failed to create proxy");

            let progress_changed_task = {
                let sender = sender.clone();
                let mut proxy = proxy.receive_progress_changed().await.unwrap();
                async move {
                    while let Some(v) = proxy.next().await {
                        let args = v.args().unwrap();
                        let id = args.transaction_id;
                        let progress = args.progress;
                        sender.send(Msg::Progress(id, progress)).unwrap();
                    }
                }
            };

            let transaction_added_task = {
                let sender = sender.clone();
                let mut proxy = proxy.receive_transaction_added().await.unwrap();
                async move {
                    while let Some(v) = proxy.next().await {
                        let args = v.args().unwrap();
                        let transaction = args.transaction;
                        sender.send(Msg::TransactionAdded(transaction)).unwrap();
                    }
                }
            };

            tokio::join!(progress_changed_task, transaction_added_task,);
        });

        let this = self.clone();
        receiver.attach(None, |msg| {
            match msg {
                Msg::TransactionAdded(transaction) => {
                    this.transactions.append::<GTransaction>(&GTransaction::from_t(transaction));
                }
                Msg::Progress(id, progress) => {
                }
            }
            return glib::ControlFlow::Continue;
        });
    }
}
