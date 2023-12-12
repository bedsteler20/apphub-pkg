mod imp {
    use std::cell::{Cell, RefCell};

    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib::subclass::types::ObjectSubclass;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::GTransaction)]
    pub struct ApphubTransaction {
        #[property(get, set)]
        app_id: RefCell<Option<String>>,
        #[property(get, set)]
        progress: Cell<f64>,
        #[property(get, set)]
        error: RefCell<Option<String>>,
        #[property(get, set)]
        id: Cell<u32>,
        #[property(get, set)]
        done: Cell<bool>,
        #[property(get, set, builder(super::GInstallLocation::default()))]
        install_location: Cell<super::GInstallLocation>,
        #[property(get, set, builder(super::GTransactionType::default()))]
        transaction_type: Cell<super::GTransactionType>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ApphubTransaction {
        const NAME: &'static str = "ApphubTransaction";
        type Type = super::GTransaction;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ApphubTransaction {}
}

// ====== Public Interface ======
glib::wrapper! {
    pub struct GTransaction(ObjectSubclass<imp::ApphubTransaction>);
}

impl GTransaction {
    pub fn from_t(transaction: types::Transaction) -> Self {
        let err = if transaction.error.is_empty() {
            None
        } else {
            Some(transaction.error)
        };
        glib::Object::builder()
            .property("id", transaction.id)
            .property("app-id", transaction.app_id)
            .property("progress", transaction.progress)
            .property("error", err)
            .property(
                "install-location",
                Into::<GInstallLocation>::into(transaction.install_location),
            )
            .property(
                "transaction-type",
                Into::<GTransactionType>::into(transaction.transaction_type),
            )
            .build()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "ApphubInstallLocation")]
pub enum GInstallLocation {
    #[default]
    System,
    User,
}

impl From<types::InstallLocation> for GInstallLocation {
    fn from(value: types::InstallLocation) -> Self {
        match value {
            types::InstallLocation::System => GInstallLocation::System,
            types::InstallLocation::User => GInstallLocation::User,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "ApphubTransactionType")]
pub enum GTransactionType {
    #[default]
    Install,
    Update,
    Uninstall,
}

impl From<types::TransactionType> for GTransactionType {
    fn from(value: types::TransactionType) -> Self {
        match value {
            types::TransactionType::Install => GTransactionType::Install,
            types::TransactionType::Uninstall => GTransactionType::Uninstall,
            types::TransactionType::Update => GTransactionType::Update,
        }
    }
}
