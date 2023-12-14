use phper::{
    classes::{ClassEntity, Visibility},
    functions::Argument,
    objects::{ZObj, ZObject},
    values::ZVal,
    Error,
};

use phper::alloc::ToRefOwned;
use std::convert::Infallible;

const COUCHDB_CLASS_NAME: &str = "Dotso\\CouchDB";

pub fn make_couchdb_class() -> ClassEntity<()> {
    let mut class = ClassEntity::new(COUCHDB_CLASS_NAME);

    class.add_property("url", Visibility::Public, ());
    class.add_property("client", Visibility::Public, ());

    class
        .add_method(
            "__construct",
            Visibility::Public,
            move |this: &mut phper::objects::StateObj<()>, arguments: &mut [ZVal]| {
                let url = arguments[0].expect_z_str().unwrap().to_str().unwrap();
                let _type_info =
                    phper::values::ZVal::get_type_info(&arguments[1]).get_base_type_name();
                let mut argument = arguments[1].clone();
                let prop_client: &mut ZObj = argument.expect_mut_z_obj().unwrap();
                let client: ZObject = prop_client.to_ref_owned();

                this.set_property("url", url);
                this.set_property("client", client);

                Ok::<_, Infallible>(())
            },
        )
        .argument(Argument::by_val("url"))
        .argument(Argument::by_val("client"));

    class
        .add_method(
            "get",
            Visibility::Public,
            move |this, _arguments| -> phper::Result<ZVal> {
                let url = this.get_property("url");

                // 1. Get the HTTP client
                // 2. Call database in url
                // 3. Return results

                Ok::<_, Error>(url.clone())
            },
        )
        .argument(Argument::by_val("database"))
        .argument(Argument::by_val("id"));
    class
}
