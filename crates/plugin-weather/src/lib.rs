mod api;
#[allow(warnings)]
mod bindings;

use crate::bindings::exports::repl::api::plugin::Guest;
use crate::bindings::repl::api::http_client;
use crate::bindings::repl::api::transport;

use crate::api::get_weather_from_body;

struct Component;

impl Guest for Component {
    fn name() -> String {
        "weather".to_string()
    }

    fn man() -> String {
        r#"
NAME
    weather - Get the weather for a given city (built with Rust🦀)

USAGE
    weather <city>

DESCRIPTION
    Get the weather for a given city.

        "#
        .to_string()
    }

    fn run(payload: String) -> Result<transport::PluginResponse, ()> {
        match http_client::get(
            format!("https://wttr.in/{}?format=j1", payload).as_str(),
            &[],
        ) {
            Ok(response) => {
                // todo: add more ok status codes - put that on the host side
                if response.status != 200 {
                    return Ok(transport::PluginResponse {
                        status: transport::ReplStatus::Error,
                        stdout: None,
                        stderr: Some(format!(
                            "Error fetching weather - status code:{}",
                            response.status
                        )),
                    });
                }
                match get_weather_from_body(response.body.as_str()) {
                    Ok(weather) => Ok(transport::PluginResponse {
                        status: transport::ReplStatus::Success,
                        stdout: Some(weather),
                        stderr: None,
                    }),
                    Err(e) => Ok(transport::PluginResponse {
                        status: transport::ReplStatus::Error,
                        stdout: None,
                        stderr: Some(format!("Error parsing result: {}", e.to_string())),
                    }),
                }
            }
            Err(e) => Ok(transport::PluginResponse {
                status: transport::ReplStatus::Error,
                stdout: None,
                stderr: Some(format!("Error fetching weather: {}", e.to_string())),
            }),
        }
    }
}

bindings::export!(Component with_types_in bindings);
