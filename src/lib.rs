use adblock::Engine;
use adblock::lists::ParseOptions;
use adblock::request::Request;

use pyo3::prelude::*;


/// DevHybrid-DE: 
/// Compatibility update for Python 3.12 and 3.13:
///
/// Update of the implementation of check_network_urls() 
///
/// The former "engine.check_network_urls()" of Rusts adblock 
/// is deprecated and not exisiting anmyore. This version use the new
/// engine.check_network_request(&request) instead.
///
/// Further changes of dependencies and build processes in:
/// cargo.toml, pyproject.toml and in github actions


/// Adblocker class
/// Hold the adblocker engine loaded with the rules
///
/// input:
///     rules: List[str] -> list of strings that represent the rules to be applied
///
/// example:
///     braveblock.Adblocker(
///         rules=[
///             "-advertisement-icon.",
///             "-advertisement/script.",
///         ]
///     )
#[pyclass]
struct Adblocker {
    engine: Engine,
}

#[pymethods]
impl Adblocker {
    #[new]
    fn new(rules: Vec<String>) -> Self {
        Adblocker {
            engine: Engine::from_rules(&rules, ParseOptions::default()),
        }
    }

    /// The function that determines whether a specific request should be blocked based on the loaded rules
    ///
    /// input:
    ///     url: str -> The URL of the resource being requested
    ///     source_url: str -> The URL of the page that initiated the request
    ///     request_type: str -> The type of the requested resource. Can be one of:
    ///         "beacon", "csp_report", "document", "font", "image", "imageset", "main_frame",
    ///         "media", "object_subrequest", "object", "other", "ping", "script", "speculative",
    ///         "stylesheet", "sub_frame", "subdocument", "web_manifest", "websocket", "xbl",
    ///         "xhr", "xml_dtd", "xmlhttprequest", "xslt"
    ///
    /// returns:
    ///     bool -> True if the request matches a blocking rule, otherwise False
    ///
    /// example:
    ///     adblocker.check_network_urls(
    ///         url="http://example.com/ads/banner.jpg",
    ///         source_url="http://example.com/",
    ///         request_type="image",
    ///     )
    fn check_network_urls(
        &self,
        url: &str,
        source_url: &str,
        request_type: &str,
    ) -> PyResult<bool> {
        match Request::new(url, source_url, request_type) {
            Ok(request) => {
                let result = self.engine.check_network_request(&request);
                Ok(result.matched)
            }
            Err(_) => Ok(false), // Return false if request creation fails
        }
    }
}

/// Braveblock is a Python library that implements an adblocker based on Brave's browser adblocker written in Rust.
/// This library provides Python bindings for the original adblocker written by Brave.
/// Original code can be found here: https://github.com/brave/adblock-rust
#[pymodule]
fn braveblock(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Adblocker>()?;
    Ok(())
}
