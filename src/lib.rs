pub mod client;
pub mod couchdb;
pub mod errors;
pub mod request;
pub mod response;

use crate::{
    client::{make_client_builder_class, make_client_class},
    couchdb::make_couchdb_class,
    errors::make_exception_class,
    request::make_request_builder_class,
    response::make_response_class,
};

use phper::{modules::Module, php_get_module};

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    module.add_class(make_exception_class());
    module.add_class(make_client_class());
    module.add_class(make_client_builder_class());
    module.add_class(make_request_builder_class());
    module.add_class(make_response_class());
    module.add_class(make_couchdb_class());

    module
}
