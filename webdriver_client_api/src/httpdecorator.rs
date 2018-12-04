use traits::HttpExecutor;
use serde::de::DeserializeOwned;
use structs::WebDriverError;
use serde::Serialize;

impl<H: HttpExecutor> HttpExecutor for HttpExecutorPathDecorator<H> {
    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, WebDriverError> {
        self.inner.get(format!("{}/{}", &self.path, path).trim_end_matches("/"))
    }

    fn post<T: DeserializeOwned, S: Serialize>(&self, path: &str, body: S) -> Result<T, WebDriverError> {
        self.inner.post(format!("{}/{}", &self.path, path).trim_end_matches("/"), body)
    }

    fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, WebDriverError> {
        self.inner.delete(format!("{}/{}", &self.path, path).trim_end_matches("/"))
    }
}

#[derive(Clone)]
pub struct HttpExecutorPathDecorator<H: HttpExecutor + Clone> {
    path: String,
    inner: Box<H>,
}

impl<H: HttpExecutor> HttpExecutorPathDecorator<H> {
    pub fn new(facade: Box<H>, path: String) -> HttpExecutorPathDecorator<H> {
        HttpExecutorPathDecorator {
            path,
            inner: facade,
        }
    }
    pub fn into_inner(self) -> Box<H>{
        self.inner
    }
}

