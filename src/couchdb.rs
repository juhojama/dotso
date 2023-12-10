use phper::{
    classes::{ClassEntity, Visibility},
    functions::Argument,
};
use std::convert::Infallible;

const COUCHDB_CLASS_NAME: &str = "Dotso\\CouchDB";

pub fn make_couchdb_class() -> ClassEntity<()> {
    let mut class = ClassEntity::new(COUCHDB_CLASS_NAME);

    class
        .add_method("__construct", Visibility::Public, |_, _| {
            Ok::<_, Infallible>(())
        })
        .argument(Argument::by_val("databaseHost"));

    class
}
