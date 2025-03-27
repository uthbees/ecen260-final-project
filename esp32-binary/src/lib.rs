use dotenv_codegen::dotenv;
use embedded_svc::{
    http::client::Client as HttpClient,
    io::Write,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};
use esp_idf_hal::delay::Delay;
use esp_idf_hal::gpio::{AnyInputPin, AnyOutputPin, PinDriver};
use esp_idf_hal::io::asynch::Write as asynch_Write;
use esp_idf_hal::uart::AsyncUartDriver;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::hal::uart;
use esp_idf_svc::hal::uart::UartDriver;
use esp_idf_svc::http::client::{Configuration as HttpConfiguration, EspHttpConnection};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sys::esp_crt_bundle_attach;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use std::fmt::Write as std_Write;

const SSID: &str = dotenv!("WIFI_SSID");
const PASSWORD: &str = dotenv!("WIFI_PASSWORD");
const API_URL: &str = dotenv!("API_URL");

pub fn run() -> anyhow::Result<()> {
    let mut local_executor = LocalPool::new();

    local_executor
        .spawner()
        .spawn_local(async move { run_async().await.unwrap() })?;

    Ok(local_executor.run())
}

async fn run_async() -> anyhow::Result<()> {
    let p = Peripherals::take()?;
    let delay: Delay = Default::default();
    // let event_loop = EspSystemEventLoop::take()?;
    // let nvs = EspDefaultNvsPartition::take()?;
    //
    // let mut wifi = BlockingWifi::wrap(
    //     EspWifi::new(peripherals.modem, event_loop.clone(), Some(nvs))?,
    //     event_loop,
    // )?;
    //
    // connect_wifi(&mut wifi)?;
    //
    // let mut client = HttpClient::wrap(EspHttpConnection::new(&HttpConfiguration {
    //     crt_bundle_attach: Some(esp_crt_bundle_attach),
    //     ..Default::default()
    // })?);

    let mut uart = UartDriver::new(
        p.uart0,
        p.pins.gpio5,                 // TX pin
        p.pins.gpio16,                // RX pin
        Option::<AnyInputPin>::None,  // RTS pin, not used
        Option::<AnyOutputPin>::None, // CTS pin, not used
        &uart::config::Config::default().baudrate(115_200.Hz()),
    )?;

    let to_send: u8 = 100;

    uart.write_all(&[to_send])?;

    let mut a0 = PinDriver::output(p.pins.gpio18)?;
    let mut a1 = PinDriver::output(p.pins.gpio17)?;
    let mut a2 = PinDriver::output(p.pins.gpio9)?;
    a0.set_low()?;
    a1.set_low()?;
    a2.set_low()?;

    let mut buf = [0u8; 100];
    uart.read(&mut buf, 1_000_000)?;
    if buf[0] == to_send {
        a0.set_high()?;
    } else {
        a1.set_high()?;
        delay.delay_ms(100);
        a1.set_low()?;
        delay.delay_ms(100);
        a1.set_high()?;
        delay.delay_ms(100);
        a1.set_low()?;
        delay.delay_ms(100);
        for i in 0..u8::BITS {
            if buf[0] & (1 << i) != 0 {
                a1.set_high()?;
            } else {
                a1.set_low()?;
            }
            delay.delay_ms(1_000);
        }
        a1.set_low()?;
        delay.delay_ms(100);
        a1.set_high()?;
        delay.delay_ms(100);
        a1.set_low()?;
        delay.delay_ms(100);
        a1.set_high()?;
        delay.delay_ms(100);
        a1.set_low()?;
    }

    // Do something in the infinite loop to avoid the program resetting
    loop {
        a2.toggle()?;
        delay.delay_ms(1_000);
    }

    // POST
    // post_request(&mut client)?;

    Ok(())
}

/// Send an HTTP POST request.
fn post_request(client: &mut HttpClient<EspHttpConnection>) -> anyhow::Result<()> {
    // Prepare payload
    let payload = b"{\"temperature\":99999}";

    // Prepare headers and URL
    let content_length_header = format!("{}", payload.len());
    let headers = [
        ("Content-Type", "application/json"),
        ("Content-Length", &*content_length_header),
    ];
    let url = &*(API_URL.to_owned() + "/sensor_data");

    // Send request
    let mut request = client.post(url, &headers)?;
    request.write_all(payload)?;
    request.flush()?;
    let mut response = request.submit()?;

    // Process response
    // let status = response.status();
    // let mut buf = [0u8; 1024];
    // let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
    // match std::str::from_utf8(&buf[0..bytes_read]) {
    //     Ok(body_string) => info!(
    //         "Response body (truncated to {} bytes): {:?}",
    //         buf.len(),
    //         body_string
    //     ),
    //     Err(e) => error!("Error decoding response body: {}", e),
    // };

    Ok(())
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;

    Ok(())
}
