use phper::{
    classes::{ClassEntity, ClassEntry},
    errors::{exception_class, Throwable},
};

// The exception class name of extension.
const EXCEPTION_CLASS_NAME: &str = "HttpClient\\HttpClientException";

pub fn make_exception_class() -> ClassEntity<()> {
    let mut class = ClassEntity::new(EXCEPTION_CLASS_NAME);
    // The `extends` is same as the PHP class `extends`.
    class.extends(exception_class);
    class
}

#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    #[error(transparent)]
    Reqwest(reqwest::Error),

    #[error("should call '{method_name}()' before call 'body()'")]
    ResponseAfterRead { method_name: String },

    #[error("should not call 'body()' multiple times")]
    ResponseHadRead,
}

impl Throwable for HttpClientError {
    fn get_class(&self) -> &ClassEntry {
        ClassEntry::from_globals(EXCEPTION_CLASS_NAME).unwrap_or_else(|_| exception_class())
    }
}

impl From<HttpClientError> for phper::Error {
    fn from(e: HttpClientError) -> Self {
        phper::Error::throw(e)
    }
}
