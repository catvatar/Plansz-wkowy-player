use esp_idf_hal::peripherals::Peripherals;

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::WifiDriver;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer};
use embedded_svc::http::Method;
use std::{thread::sleep, time::Duration};

// mod button;

mod http_server;
use http_server::WifiSetup;

use esp_idf_svc::nvs::EspDefaultNvs;


fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi_setup = WifiSetup::new(
        WifiDriver::new(peripherals.modem, sysloop.clone(), None)?,
        sysloop, 
        "192.168.0.69",
        EspDefaultNvs::new(nvs,"flash memory",true)?,
    )?;

    match wifi_setup.login_from_flash() {
        Ok(_) => {
            println!("# WiFi credentials loaded from NVS");
        }
        Err(_) => {
            println!("# WiFi credentials not found in NVS");
            wifi_setup.authentication_loop("Planszowkowy Player", "", password_input_index_html())?;
        }
    }

    let mut httpserver = EspHttpServer::new(&HttpServerConfig::default())?;

    httpserver.fn_handler("/", Method::Get, |request| {
        let html = playlist_select_intex_html();
        let mut response = request.into_ok_response()?;
        response.write(html.as_bytes())?;
        Ok::<(), anyhow::Error>(())
    })?;

    loop{
        sleep(Duration::from_secs(1));
    }
}


fn password_input_index_html() -> String {
    let html = include_str!("password_input.html");
    html.to_string()
}

fn playlist_select_intex_html() -> String {
    let html = include_str!("playlist_select.html");
    html.to_string()
}