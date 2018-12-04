use serde_json::Map;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

pub type Capabilities = Map<String, Value>;
pub type Cookies = Vec<Cookie>;


pub enum Frame {
    Elem(String),
    Id(i32),
    None,
}

pub enum WebDriverLocator {
    ID,
    CSS,
    Xpath,
    LinkText,
    Tag,
    PartialLinkText,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub secure: Option<bool>,
    #[serde(rename = "httpOnly")]
    pub http_only: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timeouts {
    //TODO check time type
    pub script: i32,
    #[serde(rename = "pageLoad")]
    pub page_load: i32,
    pub implicit: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteScript {
    pub script: String,
    pub args: Vec<Value>,
}


//TODO add ErrorKind
#[derive(Serialize, Deserialize, Debug)]
pub struct WebDriverError {
    pub error: String,
    pub message: String,

    pub stacktrace: String,
    pub data: Option<Map<String, Value>>,
}

impl Display for WebDriverError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        //TODO write stacktrace and data
        writeln!(f, "error: {}, message: {}", self.error, self.message)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub ready: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSession {
    pub capabilities: Capabilities,
    #[serde(rename = "sessionId")]
    pub session_id: String,
}
