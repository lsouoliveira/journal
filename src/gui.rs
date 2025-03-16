use crate::api;
use crate::db;

use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Application, Entry};

use glib::clone;
use glib::Object;

use std::cell::RefCell;

pub struct Journal {
    app: Application,
}

impl Journal {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id("com.github.Journal")
            .build();

        Self { app }
    }

    pub fn run(&mut self) -> glib::ExitCode {
        let args: Vec<&str> = vec![];

        self.app.connect_activate(Self::setup_window);
        self.app.run_with_args(&args)
    }

    fn setup_window(app: &Application) {
        let window = JournalWindow::new(app);
        window.present()
    }
}

mod imp {
    use crate::api;

    use gtk::subclass::prelude::*;
    use gtk::{glib, Entry};
    use std::cell::{OnceCell, RefCell};

    #[derive(Default)]
    pub struct JournalWindow {
        pub entry: OnceCell<Entry>,
        pub client: OnceCell<RefCell<api::Client>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for JournalWindow {
        const NAME: &'static str = "JournalWindow";
        type Type = super::JournalWindow;
        type ParentType = gtk::ApplicationWindow;
    }

    impl ObjectImpl for JournalWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            obj.setup_client();
            obj.setup_ui();
            obj.setup_callbacks();
        }
    }

    impl WidgetImpl for JournalWindow {}

    impl WindowImpl for JournalWindow {}

    impl ApplicationWindowImpl for JournalWindow {}
}

glib::wrapper! {
    pub struct JournalWindow(ObjectSubclass<imp::JournalWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl JournalWindow {
    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", app).build()
    }

    fn setup_client(&self) {
        let conn = db::connect();
        let entries_service = api::EntriesService::new(conn);
        let client = api::Client::new(entries_service);

        self.imp().client.set(RefCell::new(client)).unwrap()
    }

    fn setup_ui(&self) {
        let entry = Entry::builder()
            .placeholder_text("What are you doing right now?")
            .width_chars(40)
            .build();

        self.imp().entry.set(entry.clone()).unwrap();

        self.set_child(Some(&entry));
    }

    fn setup_callbacks(&self) {
        let entry = self.imp().entry.get().unwrap();

        entry.connect_activate(clone!(
            #[weak(rename_to = window)]
            self,
            move |_| window.add_entry()
        ));
    }

    fn add_entry(&self) {
        let message = self.imp().entry.get().unwrap().text();

        if message.is_empty() {
            return;
        }

        let new_entry = api::EntryCreate {
            message: message.to_string(),
        };

        if let Some(client) = self.imp().client.get() {
            client.borrow_mut().entries_service.create_entry(new_entry);
        }

        self.close()
    }
}
