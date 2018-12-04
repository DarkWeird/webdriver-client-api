extern crate log;
extern crate webdriver_client_api as wda;

use log::*;
use serde_json::Map;
use serde_json::Value;

use wda::traits::HttpExecutor;
use wda::traits::Element;
use wda::httpdecorator::HttpExecutorPathDecorator;
use wda::traits::LocatorStrategy;
use wda::WEB_ELEMENT_IDENTIFIER;
use wda::structs::Frame::Elem;
use wda::structs::Frame::Id;
use wda::structs::Status;
use wda::structs::NewSession;
use wda::structs::WebDriverError;
use wda::structs::Frame;
use wda::structs::Rect;
use wda::traits::WebDriver;
use wda::traits::Session;
use wda::structs::Timeouts;
use wda::structs::ExecuteScript;
use wda::structs::Cookies;

impl<H: HttpExecutor + 'static> Element<H> for DefaultElement<H> {
    fn get_reference_id(&self) -> &str {
        self.ref_id.as_str()
    }

    fn get_http_executor(self) -> Box<HttpExecutorPathDecorator<HttpExecutorPathDecorator<H>>> {
        self.http
    }


    fn find_element(&self, using: &LocatorStrategy, value: &str) -> Result<Box<Element<H>>, WebDriverError> {
        let mut request = Map::new();
        request.insert("using".to_string(), Value::String(using.get_using_str().to_string()));
        request.insert("value".to_string(), Value::String(value.to_string()));


        let parent_executor = self.clone().http.clone();

        //TODO handle exception
        self.http.post::<Value, Map<String, Value>>("element", request).map(|v| Box::new(
            DefaultElement::<H>::new(
                parent_executor.into_inner(),
                v.get(WEB_ELEMENT_IDENTIFIER)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            )
        ) as Box<Element<H>>)
    }

    fn find_elements(&self, using: &LocatorStrategy, value: &str) -> Result<Vec<Box<Element<H>>>, WebDriverError> {
        let mut request = Map::new();
        request.insert("using".to_string(), Value::String(using.get_using_str().to_string()));
        request.insert("value".to_string(), Value::String(value.to_string()));

        //TODO handle exception
        self.http.post::<Value, Map<String, Value>>("elements", request)
            .map(move |v| {
                let vec: Vec<Box<Element<H>>> = v.get(WEB_ELEMENT_IDENTIFIER).unwrap()
                    .as_array().unwrap()
                    .into_iter()
                    .map(move |e|
                        Box::new(
                            DefaultElement::<H>::new(
                                (self.clone().http.clone()).into_inner(),
                                e.as_str().unwrap().to_string())
                        ) as Box<Element<H>>
                    ).collect();
                vec
            }
            )
    }

    fn is_selected(&self) -> Result<bool, WebDriverError> {
        self.http.get("selected")
    }

    fn get_attribute(&self, name: &str) -> Result<String, WebDriverError> {
        self.http.get(format!("attribute/{}", name).as_str())
    }

    fn get_property(&self, name: &str) -> Result<String, WebDriverError> {
        self.http.get(format!("property/{}", name).as_str())
    }

    fn get_css_value(&self, name: &str) -> Result<String, WebDriverError> {
        self.http.get(format!("css/{}", name).as_str())
    }

    fn get_text(&self) -> Result<String, WebDriverError> {
        self.http.get("text")
    }

    fn get_tag_name(&self) -> Result<String, WebDriverError> {
        self.http.get("name")
    }

    fn get_rect(&self) -> Result<Rect, WebDriverError> {
        self.http.get("rect")
    }

    fn is_enabled(&self) -> Result<bool, WebDriverError> {
        self.http.get("enabled")
    }

    fn click(&self) -> Result<(), WebDriverError> {
        self.http.post("click", Value::Object(Map::new()))
    }

    fn clear(&self) -> Result<(), WebDriverError> {
        self.http.post("clear", Value::Object(Map::new()))
    }

    fn send_keys(&self, text: &str) -> Result<(), WebDriverError> {
        let mut request = Map::new();
        request.insert("text".to_string(), Value::String(text.to_string()));
        self.http.post("value", request)
    }

    fn take_screenshot(&self) -> Result<String, WebDriverError> {
        self.http.get("screenshot")
    }
}


impl<H: HttpExecutor + 'static> Session<H> for DefaultSession<H> {
    fn get_session_id(self) -> String {
        self.session_id
    }
    fn get_http_executor(self) -> Box<HttpExecutorPathDecorator<H>> {
        self.http
    }

    fn delete_session(&self) -> Result<(), WebDriverError> {
        self.http.delete("")
    }

    fn get_timeouts(&self) -> Result<Timeouts, WebDriverError> {
        self.http.get("timeouts")
    }

    fn set_timeouts(&self, timeouts: Timeouts) -> Result<(), WebDriverError> {
        self.http.post("timeouts", timeouts)
    }

    fn navigate_to(&self, url: &str) -> Result<(), WebDriverError> {
        let mut body = Map::new();
        body.insert("url".to_string(), Value::String(url.to_string()));
        self.http.post("url", body)
    }

    fn get_current_url(&self) -> Result<String, WebDriverError> {
        self.http.get("url")
    }

    fn back(&self) -> Result<(), WebDriverError> {
        self.http.post("back", Value::Object(Map::new()))
    }

    fn forward(&self) -> Result<(), WebDriverError> {
        self.http.post("forward", Value::Object(Map::new()))
    }

    fn refresh(&self) -> Result<(), WebDriverError> {
        self.http.post("refresh", Value::Object(Map::new()))
    }

    fn get_title(&self) -> Result<String, WebDriverError> {
        self.http.get("title")
    }

    fn get_window_handle(&self) -> Result<String, WebDriverError> {
        self.http.get("window")
    }

    fn close_window(&self) -> Result<Vec<String>, WebDriverError> {
        self.http.delete("window")
    }

    fn switch_to_window(&self, handle: &str) -> Result<(), WebDriverError> {
        let mut body = Map::new();
        body.insert("handle".to_string(), Value::String(handle.to_string()));
        self.http.post("window", body)
    }

    fn get_window_handles(&self) -> Result<Vec<String>, WebDriverError> {
        self.http.get("window/handles")
    }

    fn switch_to_frame(&self, frame: Frame) -> Result<(), WebDriverError> {
        let mut body = Map::new();
        let id :Value = match frame {
            Elem(e) => {
                let mut elem_repr = Map::new();
                elem_repr.insert(WEB_ELEMENT_IDENTIFIER.to_string(), Value::String(e));
                Value::Object(elem_repr)
            },
            Id(n) => Value::from(n),
            Frame::None => Value::Null
        };
        body.insert("id".to_string(), id);
        self.http.post("frame", Value::Object(body))
    }

    fn switch_to_parent(&self) -> Result<(), WebDriverError> {
        self.http.post("frame/parent", Value::Object(Map::new()))
    }

    fn get_window_rect(&self) -> Result<Rect, WebDriverError> {
        self.http.get("window/rect")
    }

    fn set_window_rect(&self, rect: Rect) -> Result<Rect, WebDriverError> {
        self.http.post("window/rect", rect)
    }

    fn maximize(&self) -> Result<Rect, WebDriverError> {
        self.http.post("window/maximize", Value::Object(Map::new()))
    }

    fn minimize(&self) -> Result<Rect, WebDriverError> {
        self.http.post("window/minimize", Value::Object(Map::new()))
    }

    fn fullscreen(&self) -> Result<Rect, WebDriverError> {
        self.http.post("window/fullscreen", Value::Object(Map::new()))
    }

    fn get_active_element(&self) -> Result<Box<Element<H>>, WebDriverError> {
        let parent_executor = self.clone().get_http_executor();

        //TODO handle exception

        self.http.get::<Value>("element/active").map(|v| Box::new(
            DefaultElement::<H>::new(
                parent_executor,
                v.get(WEB_ELEMENT_IDENTIFIER)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            )
        ) as Box<Element<H>>)
    }

    fn find_element(&self, using: &LocatorStrategy, value: &str) -> Result<Box<Element<H>>, WebDriverError> {
        let mut request = Map::new();
        request.insert("using".to_string(), Value::String(using.get_using_str().to_string()));
        request.insert("value".to_string(), Value::String(value.to_string()));


        let parent_executor = self.clone().get_http_executor();

        //TODO handle exception
        self.http.post::<Value, Map<String, Value>>("element", request).map(|v| Box::new(
            DefaultElement::<H>::new(
                parent_executor,
                v.get(WEB_ELEMENT_IDENTIFIER)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            )
        ) as Box<Element<H>>)
    }

    fn find_elements(&self, using: &LocatorStrategy, value: &str) -> Result<Vec<Box<Element<H>>>, WebDriverError> {
        let mut request = Map::new();
        request.insert("using".to_string(), Value::String(using.get_using_str().to_string()));
        request.insert("value".to_string(), Value::String(value.to_string()));


        //TODO handle exception
        self.http.post::<Value, Map<String, Value>>("elements", request)
            .map(move |v| {
                let vec: Vec<Box<Element<H>>> = v.get(WEB_ELEMENT_IDENTIFIER).unwrap()
                    .as_array().unwrap()
                    .iter()
                    .map(move |e|
                        Box::new(
                            DefaultElement::<H>::new(
                                self.clone().get_http_executor(),
                                e.as_str().unwrap().to_string())
                        ) as Box<Element<H>>
                    ).collect();
                vec
            }
            )
    }

    fn get_page_source(&self) -> Result<String, WebDriverError> {
        self.http.get("source")
    }

    fn execute_sync(&self, script: ExecuteScript) -> Result<Value, WebDriverError> {
        self.http.post("execute/sync", script)
    }

    fn execute_async(&self, script: ExecuteScript) -> Result<(), WebDriverError> {
        self.http.post("execute/async", script)
    }

    fn get_cookies(&self) -> Result<Cookies, WebDriverError> {
        self.http.get("cookie")
    }

    fn get_cookie(&self, name: &str) -> Result<Value, WebDriverError> {
        self.http.get(format!("cookie/{}", name).as_str())
    }

    fn add_cookie(&self, cookies: Cookies) -> Result<(), WebDriverError> {
        self.http.post("cookie", cookies)
    }

    fn delete_cookie(&self, name: &str) -> Result<(), WebDriverError> {
        self.http.delete(format!("cookie/{}", name).as_str())
    }

    fn delete_all_cookies(&self) -> Result<(), WebDriverError> {
        self.http.delete("cookie")
    }

    fn perform_actions(&self) -> Result<(), WebDriverError> {
        unimplemented!()
    }

    fn release_actions(&self) -> Result<(), WebDriverError> {
        self.http.delete("actions")
    }

    fn dismiss_alert(&self) -> Result<(), WebDriverError> {
        self.http.post("alert/dismiss", Value::Object(Map::new()))
    }

    fn accept_alert(&self) -> Result<(), WebDriverError> {
        self.http.post("alert/accept", Value::Object(Map::new()))
    }

    fn get_alert_text(&self) -> Result<String, WebDriverError> {
        self.http.get("alert/text")
    }

    fn set_alert_text(&self, text: &str) -> Result<(), WebDriverError> {
        let mut map = Map::new();
        map.insert("text".to_string(), Value::String(text.to_string()));
        self.http.post("alert/text", map)
    }

    fn take_screenshot(&self) -> Result<String, WebDriverError> {
        self.http.get("screenshot")
    }
}

struct DefaultElement<I: HttpExecutor> {
    http: Box<HttpExecutorPathDecorator<HttpExecutorPathDecorator<I>>>,
    ref_id: String,
}

impl<I: HttpExecutor> DefaultElement<I> {
    pub fn new(http: Box<HttpExecutorPathDecorator<I>>, ref_id: String) -> Self {
        DefaultElement {
            http: Box::new(
                HttpExecutorPathDecorator::<HttpExecutorPathDecorator<I>>::new(
                    http,
                    format!("element/{}", ref_id))
            ),
            ref_id,
        }
    }
}

#[derive(Clone)]
pub struct DefaultSession<I: HttpExecutor> {
    http: Box<HttpExecutorPathDecorator<I>>,
    session_id: String,
}

impl<I: HttpExecutor> DefaultSession<I> {
    pub fn new(http: Box<I>, session_id: String) -> Self {
        DefaultSession {
            http: Box::new(
                HttpExecutorPathDecorator::<I>::new(
                    http,
                    format!("session/{}", session_id))
            ),
            session_id,
        }
    }
}


struct DefaultWebDriver<H: HttpExecutor> {
    http: Box<H>
}

impl<H: HttpExecutor> DefaultWebDriver<H> {
    pub fn new(facade: Box<H>) -> Self {
        DefaultWebDriver {
            http: facade
        }
    }
}


//TODO попробовать убрать статик
impl<H: 'static + HttpExecutor> WebDriver<H> for DefaultWebDriver<H> {
    fn create_session(self, caps: Map<String, Value>) -> Result<Box<Session<H>>, WebDriverError> {
        self.http.post::<NewSession, Value>("session", Value::Object(caps))
            .map(|ns|
                Box::new(DefaultSession::<H>::new(self.get_http_executor(),
                                                  ns.session_id)) as Box<Session<H>>
            )
    }

    fn get_http_executor(self) -> Box<H> {
        self.http
    }

    fn status(&self) -> Status {
        match self.http.get::<Status>("status") {
            Ok(t) => t,
            Err(e) => {
                error!("Unexpected error from WebDriver {}", e);
                Status {
                    ready: false,
                    message: format!("[Client-side]Unexpected error from WebDriver {}", e.error),
                }
            }
        }
    }
}
