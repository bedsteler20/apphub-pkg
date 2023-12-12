mod imp {
    use std::cell::{Cell, Ref, RefCell};

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
    pub fn new(id: u32, app_id: &str) -> GTransaction {
        glib::Object::builder()
            .property("id", id)
            .property("app-id", app_id)
            .property("progress", 0.0)
            .build()
    }

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
            .build()
    }

}