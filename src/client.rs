use phper::{
    alloc::ToRefOwned,
    classes::{ClassEntity, StaticStateClass, Visibility},
    functions::Argument,
};
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{ACCEPT, CONTENT_TYPE},
};
use std::{convert::Infallible, mem::take, time::Duration};

use crate::errors::HttpClientError;
use crate::request::REQUEST_BUILDER_CLASS;

const HTTP_CLIENT_BUILDER_CLASS_NAME: &str = "HttpClient\\HttpClientBuilder";

const HTTP_CLIENT_CLASS_NAME: &str = "HttpClient\\HttpClient";

// The static StaticStateClass is bound to ClassEntity of HttpClient, When the class registered,
// the StaticStateClass will be initialised, so you can use it to initialise stateful objects, etc.
static HTTP_CLIENT_CLASS: StaticStateClass<Option<Client>> = StaticStateClass::null();

pub fn make_client_builder_class() -> ClassEntity<ClientBuilder> {
    // `new_with_default_state_constructor` means initialize the state of `ClientBuilder` as
    // `Default::default`.
    let mut class = ClassEntity::new_with_default_state_constructor(HTTP_CLIENT_BUILDER_CLASS_NAME);

    class
        .add_method("timeout", Visibility::Public, |this, arguments| {
            let ms = arguments[0].expect_long()?;
            let state = this.as_mut_state();
            let builder: ClientBuilder = take(state);
            *state = builder.timeout(Duration::from_millis(ms as u64));
            Ok::<_, phper::Error>(this.to_ref_owned())
        })
        .argument(Argument::by_val("ms"));

    class
        .add_method("cookie_store", Visibility::Public, |this, arguments| {
            let enable = arguments[0].expect_bool()?;
            let state = this.as_mut_state();
            let builder: ClientBuilder = take(state);
            *state = builder.cookie_store(enable);
            Ok::<_, phper::Error>(this.to_ref_owned())
        })
        .argument(Argument::by_val("enable"));

    // Inner call the `ClientBuilder::build`, and wrap the result `Client` in
    // Object.
    class.add_method("build", Visibility::Public, |this, _arguments| {
        let state = take(this.as_mut_state());
        let client = ClientBuilder::build(state).map_err(HttpClientError::Reqwest)?;
        let mut object = HTTP_CLIENT_CLASS.init_object()?;
        *object.as_mut_state() = Some(client);
        Ok::<_, phper::Error>(object)
    });

    class
}

pub fn make_client_class() -> ClassEntity<Option<Client>> {
    let mut class =
        ClassEntity::<Option<Client>>::new_with_default_state_constructor(HTTP_CLIENT_CLASS_NAME);

    class.bind(&HTTP_CLIENT_CLASS);

    class.add_method("__construct", Visibility::Private, |_, _| {
        Ok::<_, Infallible>(())
    });

    class
        .add_method("get", Visibility::Public, |this, arguments| {
            let url = arguments[0].expect_z_str()?.to_str().unwrap();
            let client: &Client = this.as_state().as_ref().unwrap();
            let request_builder = client
                .get(url)
                .header(ACCEPT, "application/json")
                .header(CONTENT_TYPE, "application/json");
            let mut object = REQUEST_BUILDER_CLASS.init_object()?;
            *object.as_mut_state() = Some(request_builder);
            Ok::<_, phper::Error>(object)
        })
        .argument(Argument::by_val("url"));

    class
        .add_method("post", Visibility::Public, |this, arguments| {
            let url = arguments[0].expect_z_str()?.to_str().unwrap();
            let client = this.as_state().as_ref().unwrap();
            let request_builder = client.post(url);
            let mut object = REQUEST_BUILDER_CLASS.init_object()?;
            *object.as_mut_state() = Some(request_builder);
            Ok::<_, phper::Error>(object)
        })
        .argument(Argument::by_val("url"));

    class
}
