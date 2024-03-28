use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer};
use esp_idf_svc::wifi::{AccessPointConfiguration, BlockingWifi, ClientConfiguration, EspWifi, WifiDriver};
use embedded_svc::wifi::Configuration;
use embedded_svc::http::Method;

use std::str::from_utf8;
use std::{thread::sleep, time::Duration};

use std::sync::{Arc, Mutex};

use esp_idf_svc::nvs::EspDefaultNvs;

use esp_idf_svc::netif::EspNetif;
use esp_idf_svc::netif::NetifConfiguration;
use esp_idf_svc::ipv4::{
    ClientConfiguration as IpClientConfiguration, ClientSettings as IpClientSettings,
    Configuration as IpConfiguration, Mask, Subnet,
};

use esp_idf_svc::netif::NetifStack;

use std::net::Ipv4Addr;
use std::str::FromStr;


pub struct WifiSetup<'a>{
    pub wifi: BlockingWifi<EspWifi<'a>>,
    ssid: Arc<Mutex<String>>,
    password: Arc<Mutex<String>>,
    flash_partition: EspDefaultNvs,
}

impl<'a> WifiSetup<'a> {
    pub fn new(wifi_driver: WifiDriver<'a>,sysloop: EspSystemEventLoop,static_ip: &str, flash_partition: EspDefaultNvs) -> anyhow::Result<Self> {
        let ssid = Arc::new(Mutex::new(String::new()));
        let password = Arc::new(Mutex::new(String::new()));

        let esp_wifi: EspWifi<'a> = EspWifi::wrap_all(
            wifi_driver,
            EspNetif::new_with_conf(&NetifConfiguration {
                ip_configuration: IpConfiguration::Client(IpClientConfiguration::Fixed(
                    IpClientSettings {
                        ip: Ipv4Addr::from_str(static_ip)?,
                        subnet: Subnet {
                            gateway: Ipv4Addr::from_str("192.168.0.1")?,
                            mask: Mask(u8::from_str("24")?),
                        },
                        // Can also be set to Ipv4Addrs if you need DNS
                        dns: None,
                        secondary_dns: None,
                    },
                )),
                ..NetifConfiguration::wifi_default_client()
            })?,
            EspNetif::new(NetifStack::Ap)?,
        )?;

        let wifi: BlockingWifi<EspWifi<'a>> = BlockingWifi::wrap(
            esp_wifi,
            sysloop,
        )?;


        Ok(Self {
            wifi,
            ssid,
            password,
            flash_partition,
        })
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        if self.wifi.is_connected()? {
            self.wifi.disconnect()?;
            println!("# WifiSetup stop:\n\tWiFi disconnected");
        }
        if self.wifi.is_started()? {
            self.wifi.stop()?;
            println!("# WifiSetup stop:\n\tWiFi stopped");
        }
        Ok(())
    }

    fn start_access_point(&mut self, ssid: &str, password: &str) -> anyhow::Result<()> {
        self.stop()?;

        self.wifi.set_configuration(&Configuration::Mixed(ClientConfiguration::default(),AccessPointConfiguration{
            ssid: ssid.try_into().unwrap(),
            password: password.try_into().unwrap(),
            ..Default::default()
        }))?;

        self.wifi.start()?;
        println!("# WifiSetup start_access_point:\n\tWiFi started\n\t\tSSID: {}\tPassword: {}", ssid, password);

        Ok(())
    }

    fn wait_on_user_input(&mut self,ssid: &str, password: &str, login_page: &String) -> anyhow::Result<()> {
        let _ = self.start_access_point(ssid, password);

        let ssid_clone = self.ssid.clone();
        let password_clone = self.password.clone();
        *ssid_clone.lock().unwrap() = "".to_string();
        *password_clone.lock().unwrap() = "".to_string();
        println!("# WifiSetup wait_on_user_input:\n\tSSID and Password reset");

        let mut httpserver = EspHttpServer::new(&HttpServerConfig::default())?;

        httpserver.fn_handler("/", Method::Get, move |request| {
            let html = login_page;
            let mut response = request.into_ok_response()?;
            response.write(html.as_bytes())?;
            Ok::<(), anyhow::Error>(())
        })?;

        let ssid_clone = self.ssid.clone();
        let password_clone = self.password.clone();
        httpserver.fn_handler("/wifi-setup", Method::Post, move |request| {
            let ssid_from_web = request.header("ssid").unwrap_or_default();
            let password_from_web = request.header("password").unwrap_or_default();
            *ssid_clone.lock().unwrap() = ssid_from_web.to_string();
            *password_clone.lock().unwrap() = password_from_web.to_string();
            Ok::<(), anyhow::Error>(())
        })?;

        let ssid_clone = self.ssid.clone();
        let password_clone = self.password.clone();
        println!("# WifiSetup wait_on_user_input:\n\t Waiting for user input");
        loop {
            sleep(Duration::from_millis(1000));
            if !ssid_clone.lock().unwrap().is_empty() && !password_clone.lock().unwrap().is_empty() {
                break;
            }
        }

        self.stop()?;
        Ok(())
    }

    fn start_client(&mut self, ssid: &str, password: &str) -> anyhow::Result<()> {
        self.stop()?;

        // # WiFi setup
        self.wifi.set_configuration(&Configuration::Mixed(ClientConfiguration{
            ssid: ssid.try_into().unwrap(),
            password: password.try_into().unwrap(),
            ..Default::default()
        },AccessPointConfiguration{
            ssid: "Autobus CBA nr 5".try_into().unwrap(),
            password: "po co ci to".try_into().unwrap(),
            ..Default::default()
        }))?;

        self.wifi.start()?;
        println!("# WifiSetup start_client:\n\tWiFi started\n\t\tSSID: {}\tPassword: {}", ssid, password);

        self.wifi.connect()?;
        println!("# WiFiSetup start_client:\n\tWiFi connected\n\t\tSSID: {}\tPassword: {}", ssid, password);
        Ok(())
    }

    fn try_to_login(&mut self) -> anyhow::Result<()> {
        let ssid_binding = self.ssid.lock().unwrap().clone();
        let password_binding = self.password.lock().unwrap().clone();
        let ssid_val = ssid_binding.as_str();
        let password_val = password_binding.as_str();

        if ssid_val.is_empty() || password_val.is_empty() {
            return Err(anyhow::anyhow!("SSID or password is empty"));
        }

        println!("# WifiSetup try_to_login:\n\tTrying to login with SSID: {}, Password: {}", ssid_val, password_val);
        let mut i = 0;
        let mut success = false;
        while i < 3 {
            i += 1;
            println!("# WifiSetup authentication_loop:\n\tLogin attempt: {}", i);
            match self.start_client(ssid_val, password_val) {
                Ok(_) => {
                    success = true;
                    break;
                }
                Err(e) => {
                    println!("# WifiSetup authentication_loop:\n\tLogin failed: {}", e);
                }
            }
        }
        if success {
            return Ok(());
        }
        Err(anyhow::anyhow!("Login failed"))
    }

    fn save_credentials_to_flash(&mut self) -> anyhow::Result<()> {
        println!("# WifiSetup save_credentials_to_flash:\n\tSaving credentials to flash\n\t\tSSID: {}\tPassword: {}", self.ssid.lock().unwrap(), self.password.lock().unwrap());

        self.flash_partition.set_str("ssid", &self.ssid.lock().unwrap())?;
        self.flash_partition.set_str("password", &self.password.lock().unwrap())?;

        Ok(())
    }

    pub fn authentication_loop(&mut self,ap_ssid: &str, ap_password: &str, ap_login_page: String) -> anyhow::Result<()> {
        self.stop()?;
        let ap_login_page_clone = ap_login_page.clone();

        println!("# WifiSetup authentication_loop:\n\tEntering authentication loop");
        loop {
            self.wait_on_user_input(ap_ssid, ap_password, &ap_login_page_clone)?;

            match self.try_to_login() {
                Ok(_) => {
                    println!("# WifiSetup authentication_loop:\n\tLogin successful");
                    self.save_credentials_to_flash()?;
                    break;
                }
                Err(e) => {
                    println!("# WifiSetup authentication_loop:\n\tLogin failed: {}", e)
                }
            }
        };
        Ok(())
    }

    pub fn login_from_flash(&mut self) -> anyhow::Result<()> {
        let ssid_clone = self.ssid.clone();
        let password_clone = self.password.clone();

        let mut ssid_buff = [0u8; 32];
        let mut password_buff = [0u8; 64];

        let ssid: String = match self.flash_partition.get_str("ssid",&mut ssid_buff)? {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        let password = match self.flash_partition.get_str("password",&mut password_buff)? {
            Some(p) => p.to_string(),
            None => "".to_string(),
        };

        println!("# WifiSetup login_from_flash:\n\tSSID: {}\tPassword: {}\n\tBuff: s:{}\tp:{}", ssid, password,from_utf8(&mut ssid_buff)?,from_utf8(&mut password_buff)?);

        *ssid_clone.lock().unwrap() = ssid;
        *password_clone.lock().unwrap() = password;

        println!("# WifiSetup login_from_flash:\n\tSSID: {}\tPassword: {}", &self.ssid.lock().unwrap(), &self.password.lock().unwrap());
        self.try_to_login()?;
        Ok(())
    }

}