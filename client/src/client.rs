use crate::{proxy::ApphubDamonProxy, transaction::GTransaction};
use futures::{StreamExt, future};
use gio::prelude::ListModelExt;
use glib::prelude::*;
use once_cell::sync::Lazy;
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
    pub fn bind_dbus_signals(&self) {
        // Setup message Passing
        enum Msg {
            TransactionAdded(types::Transaction),
            Progress(u32, f64),
            Error(u32, String),
            Done(u32),
        }
        let (sender, receiver) = glib::MainContext::channel::<Msg>(glib::Priority::default());

        // Spawn the async runtime to pull for signals from dbus
        RUNTIME.spawn(async move {
            let conn = zbus::Connection::session()
                .await
                .expect("Failed to connect to the session bus");
            let proxy = ApphubDamonProxy::new(&conn)
                .await
                .expect("Failed to create proxy")
                ;
            
            // Pulling for each signal needs to be done in a separate task
            // otherwise the first task will block the second one
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

            // proxy.receive_all_signals().await.unwrap().for_each(|m| {
            //     println!("Got message");
            //     future::ready(())
            // }).await;

            let transaction_added_task = {
                let sender = sender.clone();
                let mut proxy = proxy.receive_transaction_added().await.unwrap();

                async move {
                    loop {
                        println!("Listening for added");
                        if let Some(v) = proxy.next().await {
                            println!("Got transaction added");
                            let args = v.args().unwrap();
                            let transaction = args.transaction;
                            sender.send(Msg::TransactionAdded(transaction)).unwrap();
                        } else {
                            println!("No more transactions");
                        }
                    }
                }
            };

            let transaction_error_task = {
                let sender = sender.clone();
                let mut proxy = proxy.receive_transaction_error().await.unwrap();
                async move {
                    while let Some(v) = proxy.next().await {
                        let args = v.args().unwrap();
                        let id = args.transaction_id;
                        let error = args.error;
                        sender.send(Msg::Error(id, error)).unwrap();
                    }
                }
            };

            let transaction_done_task = {
                let sender = sender.clone();
                let mut proxy = proxy.receive_transaction_done().await.unwrap();
                async move {
                    println!("Listening for done");
                    while let Some(v) = proxy.next().await {
                        let args = v.args().unwrap();
                        let id = args.transaction_id;
                        sender.send(Msg::Done(id)).unwrap();
                    }
                }
            };

            
            tokio::join!(
                progress_changed_task,
                transaction_added_task,
                transaction_error_task,
                transaction_done_task,
            );
        });

        // Pull form the channel and update the model
        // (yes we are pulling from data that is being pulled in a separate thread glib is weird)
        receiver.attach(None, {
            let trans = self.transactions.clone();
            move |msg| {
                println!("Got message");
                match msg {
                    Msg::TransactionAdded(transaction) => {
                        if find_transaction(&trans, transaction.id).is_some() {
                            panic!("Transaction {} already exists", transaction.id);
                        }
                        trans.append(&GTransaction::from_t(transaction));
                    }
                    Msg::Progress(id, progress) => {
                        if let Some((_, transaction)) = find_transaction(&trans, id) {
                            transaction.set_progress(progress);
                        }
                    }
                    Msg::Error(id, error) => {
                        if let Some((_, transaction)) = find_transaction(&trans, id) {
                            if !error.is_empty() {
                                transaction.set_error(error);
                            }
                        }
                    }
                    Msg::Done(id) => {
                        if let Some((i, transaction)) = find_transaction(&trans, id) {
                            transaction.set_done(true);
                            trans.remove(i);
                        }
                    }
                }
                return glib::ControlFlow::Continue;
            }
        });
    }
}

fn find_transaction(ls: &gio::ListStore, id: u32) -> Option<(u32, GTransaction)> {
    for i in 0..ls.n_items() {
        if let Some(item) = ls.item(i) {
            let transaction = item.downcast_ref::<GTransaction>().unwrap();
            if transaction.id() == id {
                return Some((i, transaction.clone()));
            }
        }
    }
    None
}
