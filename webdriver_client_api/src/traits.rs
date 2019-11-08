extern crate serde_json;


use crate::structs::*;
use crate::httpdecorator::HttpExecutorPathDecorator;

use serde_json::Value;
//TODO make webdriver specific
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::clone::Clone;

pub trait LocatorStrategy {
    fn get_using_str(&self) -> &str;
}

impl LocatorStrategy for WebDriverLocator {
    fn get_using_str(&self) -> &str {
        match self {
            WebDriverLocator::ID => "id",
            WebDriverLocator::CSS => "css selector",
            WebDriverLocator::Xpath => "xpath",
            WebDriverLocator::Tag => "tag name",
            WebDriverLocator::LinkText => "link text",
            WebDriverLocator::PartialLinkText => "partial link text",
        }
    }
}

//TODO add drop for session
pub trait Session<H: HttpExecutor> {
    fn get_session_id(self) -> String;
    fn get_http_executor(self) -> Box<HttpExecutorPathDecorator<H>>;
    fn delete_session(&self) -> Result<(), WebDriverError>;
    fn get_timeouts(&self) -> Result<Timeouts, WebDriverError>;
    fn set_timeouts(&self, timeouts: Timeouts) -> Result<(), WebDriverError>;
    fn navigate_to(&self, url: &str) -> Result<(), WebDriverError>;
    fn get_current_url(&self) -> Result<String, WebDriverError>;
    fn back(&self) -> Result<(), WebDriverError>;
    fn forward(&self) -> Result<(), WebDriverError>;
    fn refresh(&self) -> Result<(), WebDriverError>;
    fn get_title(&self) -> Result<String, WebDriverError>;
    fn get_window_handle(&self) -> Result<String, WebDriverError>;
    fn close_window(&self) -> Result<Vec<String>, WebDriverError>;
    fn switch_to_window(&self, handle: &str) -> Result<(), WebDriverError>;
    fn get_window_handles(&self) -> Result<Vec<String>, WebDriverError>;
    fn switch_to_frame(&self, frame: Frame) -> Result<(), WebDriverError>;
    fn switch_to_parent(&self) -> Result<(), WebDriverError>;
    fn get_window_rect(&self) -> Result<Rect, WebDriverError>;
    fn set_window_rect(&self, rect: Rect) -> Result<Rect, WebDriverError>;
    fn maximize(&self) -> Result<Rect, WebDriverError>;
    fn minimize(&self) -> Result<Rect, WebDriverError>;
    fn fullscreen(&self) -> Result<Rect, WebDriverError>;
    fn get_active_element(&self) -> Result<Box<dyn Element<H>>, WebDriverError>;
    fn find_element(
        &self,
        using: &dyn LocatorStrategy,
        value: &str,
    ) -> Result<Box<dyn Element<H>>, WebDriverError>;
    fn find_elements(
        &self,
        using: &dyn LocatorStrategy,
        value: &str,
    ) -> Result<Vec<Box<dyn Element<H>>>, WebDriverError>;
    fn get_page_source(&self) -> Result<String, WebDriverError>;
    //TODO maybe extract and make generic
    fn execute_sync(&self, script: ExecuteScript) -> Result<Value, WebDriverError>;
    fn execute_async(&self, script: ExecuteScript) -> Result<(), WebDriverError>;
    fn get_cookies(&self) -> Result<Cookies, WebDriverError>;
    fn get_cookie(&self, name: &str) -> Result<Value, WebDriverError>;
    fn add_cookie(&self, name: Cookies) -> Result<(), WebDriverError>;
    fn delete_cookie(&self, name: &str) -> Result<(), WebDriverError>;
    fn delete_all_cookies(&self) -> Result<(), WebDriverError>;
    //TODO make action builder
    fn perform_actions(&self) -> Result<(), WebDriverError>;
    fn release_actions(&self) -> Result<(), WebDriverError>;
    fn dismiss_alert(&self) -> Result<(), WebDriverError>;
    fn accept_alert(&self) -> Result<(), WebDriverError>;
    fn get_alert_text(&self) -> Result<String, WebDriverError>;
    fn set_alert_text(&self, text: &str) -> Result<(), WebDriverError>;
    fn take_screenshot(&self) -> Result<String, WebDriverError>;
}

pub trait Element<H: HttpExecutor> {
    fn get_reference_id(&self) -> &str;
    fn get_http_executor(self) -> Box<HttpExecutorPathDecorator<HttpExecutorPathDecorator<H>>>;
    fn find_element(
        &self,
        using: &dyn LocatorStrategy,
        value: &str,
    ) -> Result<Box<dyn Element<H>>, WebDriverError>;
    fn find_elements(
        &self,
        using: &dyn LocatorStrategy,
        value: &str,
    ) -> Result<Vec<Box<dyn Element<H>>>, WebDriverError>;
    fn is_selected(&self) -> Result<bool, WebDriverError>;
    fn get_attribute(&self, name: &str) -> Result<String, WebDriverError>;
    fn get_property(&self, name: &str) -> Result<String, WebDriverError>;
    fn get_css_value(&self, name: &str) -> Result<String, WebDriverError>;
    fn get_text(&self) -> Result<String, WebDriverError>;
    fn get_tag_name(&self) -> Result<String, WebDriverError>;
    fn get_rect(&self) -> Result<Rect, WebDriverError>;
    fn is_enabled(&self) -> Result<bool, WebDriverError>;
    fn click(&self) -> Result<(), WebDriverError>;
    fn clear(&self) -> Result<(), WebDriverError>;
    //TODO check param text in runtime
    fn send_keys(&self, text: &str) -> Result<(), WebDriverError>;

    fn take_screenshot(&self) -> Result<String, WebDriverError>;
}

pub trait WebDriver<H: HttpExecutor> {
    fn create_session(self, caps: Capabilities) -> Result<Box<dyn Session<H>>, WebDriverError>;
    fn get_http_executor(self) -> Box<H>;
    fn status(&self) -> Status;
}

pub trait HttpExecutor: Clone {
    fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, WebDriverError>;
    fn post<T: DeserializeOwned, S: Serialize>(&self, path: &str, body: S) -> Result<T, WebDriverError>;
    fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, WebDriverError>;
}

