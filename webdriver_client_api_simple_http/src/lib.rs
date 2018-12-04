extern crate webdriver_client_api as wda;
extern crate log;
extern crate ureq;
extern crate serde_json;
extern crate serde;

use log::*;

use ureq::get;
use ureq::delete;
use ureq::post;
use ureq::Response;
use ureq::Request;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use wda::structs::WebDriverError;
use wda::traits::HttpExecutor;


#[derive(Clone)]
pub struct HttpClient {
    base_url: String,
}


impl HttpExecutor for HttpClient {
    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, WebDriverError>
        where
                for<'de> T: Deserialize<'de>,
    {
        let mut req: Request = get(&format!("{}/{}", self.base_url, path))
            .set("Accept", "application/json;charset=UTF-8")
            .build();
        info!("Sending GET request:{:?}", req);
        let res: ureq::Response = req.call();
        info!("Receive GET response:{:?}", res);
        self.extract(res)
    }

    fn post<T: DeserializeOwned, S: Serialize>(&self, path: &str, body: S) -> Result<T, WebDriverError>

    {
        let mut req: Request = post(&format!("{}/{}", self.base_url, path))
            .set("Accept", "application/json;charset=UTF-8")
            .build();
        info!("Sending POST request:{:?}", req);
        let body = serde_json::to_value(body).unwrap();
        debug!("Request Body:{}", body);
        let res: ureq::Response = req.send_json(body);
        info!("Receive POST response:{:?}", res);
        self.extract(res)
    }

    fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, WebDriverError>
        where
                for<'de> T: Deserialize<'de>,
    {
        let mut req: Request = delete(&format!("{}/{}", self.base_url, path))
            .set("Accept", "application/json;charset=UTF-8")
            .build();
        info!("Sending DELETE request:{:?}", req);
        let res: ureq::Response = req.call();
        info!("Receive DELETE response:{:?}", res);
        self.extract(res)
    }
}


impl HttpClient {
    pub fn new(base_url: String) -> Self {
        HttpClient { base_url }
    }

    fn extract<T>(&self, res: Response) -> Result<T, WebDriverError>
        where
                for<'de> T: Deserialize<'de>,
    {
        //TODO make error handling
        let content = res.into_string().unwrap();
        debug!("RAW: {:?}", content);
        let json: Value = serde_json::from_str(&content).unwrap();
        debug!("RAW Value: {:?}", json);
        let json_value = json.get("value").unwrap().clone();
        debug!("Value in JSON:{:?}", json_value);
        match serde_json::from_value::<T>(json_value.clone()) {
            Ok(v) => Ok(v),
            Err(e) => {
                warn!("{}", e);
                match serde_json::from_value::<WebDriverError>(json_value) {
                    Ok(e) => Err(e),
                    Err(e) => {
                        error!("Cannot read response: {}", e);
                        //TODO make normal IoError
                        Err(WebDriverError {
                            error: e.to_string(),
                            message: String::new(),
                            stacktrace: String::new(),
                            data: None,
                        })
                    }
                }
            }
        }
    }
}
