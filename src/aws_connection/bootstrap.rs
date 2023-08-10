use crate::aws_connection::create_encorded_certificate;
use reqwest::Client as api_cli;
use serde_json::json;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::io::Write;

/*get device id using wifi mac addess*/

// pub fn getmac_address_todeviceid() -> Option<String> {
//     let net = Path::new("/sys/class/net");
//     let entry = fs::read_dir(net).expect("Error");
//     let ifaces = entry
//         .filter_map(|p| p.ok())
//         .map(|p| p.path().file_name().expect("Error").to_os_string())
//         .filter_map(|s| s.into_string().ok())
//         .collect::<Vec<String>>();
//     println!("Available interfaces: {:?}", ifaces);
//     let iface = net.join(ifaces[1].as_str()).join("address");
//     let mut f = fs::File::open(iface).expect("Failed");
//     let mut macaddr = String::new();
//     f.read_to_string(&mut macaddr).expect("Error");
//     let macaddr = macaddr.replace(":", "");
//     let reverse: String = macaddr.chars().rev().collect();
//     let keyword = "6cSs9".to_string();
//     let appendaddr = format!("{}{}{}", macaddr, keyword, reverse);
//     let prefix = "HACD00".to_string();
//     let deviceid = format!("{}{}", prefix, appendaddr);
//     Some(deviceid)
// }

/*get device id using eth+wifi mac addess*/

pub fn ethwifi_address_todeviceid() -> Option<String> {
    let net = Path::new("/sys/class/net");
    let entry = fs::read_dir(net).expect("Error");
    let ifaces = entry
        .filter_map(|p| p.ok())
        .map(|p| p.path().file_name().expect("Error").to_os_string())
        .filter_map(|s| s.into_string().ok())
        .collect::<Vec<String>>();
    println!("Available interfaces: {:?}", ifaces);
    let iface0 = net.join(ifaces[0].as_str()).join("address");
    let mut f = fs::File::open(iface0).expect("Failed");
    let mut macaddr0 = String::new();
    f.read_to_string(&mut macaddr0).expect("Error");
    let macaddr0 = macaddr0.replace(":", "");
    let iface1 = net.join(ifaces[1].as_str()).join("address");
    let mut f = fs::File::open(iface1).expect("Failed");
    let mut macaddr1 = String::new();
    f.read_to_string(&mut macaddr1).expect("Error");
    let macaddr1 = macaddr1.replace(":", "");
    let keyword = "6cSs9".to_string();
    let appendaddr = format!("{}{}{}", macaddr0, keyword, macaddr1);
    let prefix = "HDA960".to_string();
    let deviceid = format!("{}{}", prefix, appendaddr);
    Some(deviceid)
}

/*create iot things and downlaod it certificate file */
pub async fn auth_download_file(
    url: &str,
   
) {
    // let certificate= b"-----BEGIN CERTIFICATE-----MIIDWjCCAkKgAwIBAgIVAOrO8PbeseyqRkhtPoEijMMRPG+VMA0GCSqGSIb3DQEBCwUAME0xSzBJBgNVBAsMQkFtYXpvbiBXZWIgU2VydmljZXMgTz1BbWF6b24uY29tIEluYy4gTD1TZWF0dGxlIFNUPVdhc2hpbmd0b24gQz1VUzAeFw0xOTA1MDcwNTQ2MzBaFw00OTEyMzEyMzU5NTlaMB4xHDAaBgNVBAMME0FXUyBJb1QgQ2VydGlmaWNhdGUwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQDZzkUR4/+ekifuK9106x7xkxhxaGn925v9hRCUcBle8rmJibGhttEr3kFMi5Lx5Hfh3zQvM5fkPIDDw93bbCT0QnQA8l/uMrQNtS6vkX+SpPmf1Jpu/iVpA3kWcM/+m9Za4hFa/XM1c+yc0sQrAvkvYjrRgo2XLSFdPcrDuWUrJfrOCfgBnaTzvwPx+BI06RXgoUKss6Y8jy+d3o2EkPpF59GTaAsCmWxZvraZ9mHqgGX029u7P9lpL+7/MzqXX/YHqQZJOIndFDq810UTatazlr2rQ9b3B925fg9I1VQjaOx5C1nMUwgmiLqzuD1iD8/Stq+mEYsQZo+norPb3cMTAgMBAAGjYDBeMB8GA1UdIwQYMBaAFPDJZsV1LqiKBoFZwowi3tnaQggiMB0GA1UdDgQWBBTAuyQwvofCdKpzSIub7C0uW9ToDzAMBgNVHRMBAf8EAjAAMA4GA1UdDwEB/wQEAwIHgDANBgkqhkiG9w0BAQsFAAOCAQEAzGf/vVKegYqDyIj7cRRCzfuEPdQb8+LvTjlWPgVUmFzsN0Iu8/gLelQ00Gymwva4AGd+XZ6RKU12fCuN7ngQxi8+hD6mvTTAsuwyXW0M0EVRiKkOj8LolRe/qkAae0VMOPjTOj6Az7QB01y5Ix2J+MNDDugXw4Q/ufm4BHJ/CfKAdwL1QtFl8CoEYXdAQBuBldTsrSNgRIH7MiYm/zAl0gMlhsoZ6n4jH77TaCK8leqtBqVhek1a2u400Hu1Ht9bpfV4cKk3mtA9B/WMQ5ax560SNlY5B9g2pctpwCoWqH7CzhFI8BGqNeJ8cv0up7rtr0b1hPktpynn3j4Xj21U0A==-----END CERTIFICATE-----";
    // let common_key = b"12345678";
    // let iv = b"1234567890abcdef";
    let hardware_lot = "QWOP";
    let software_version = "1.2345";
    // let edge_version=format!("{}:{}",hardware_lot,software_version);
    let binding = ethwifi_address_todeviceid().unwrap_or_default();
    let dev= binding.replace("\n","");
    let device_id = dev.as_str();
    println!("{}", device_id);
    let encoded_certificate = create_encorded_certificate(device_id);
    let json_payload = json!({
        "device_id": device_id,
        "initial_certificate": encoded_certificate,
        "edge_version": format!("{}:{}", hardware_lot, software_version),
    });

    let api_cli = api_cli::new();
    let res = api_cli.post(url).json(&json_payload).send().await.unwrap();
    // println!("{:#?}", res.text().await.unwrap());
    let response_json: serde_json::Value = res.json().await.unwrap();
    let folder_path = "/home/desolated/Downloads/create_device_id/credentials";
    let certificate_file_path = format!("{}/certificate.txt", folder_path);
    let private_key_file_path = format!("{}/private_key.txt", folder_path);

    let certificate = response_json["certificate"].as_str().unwrap();
    let private_key = response_json["private_key"].as_str().unwrap();

    let mut certificate_file = File::create(certificate_file_path).unwrap();
    certificate_file.write_all(certificate.as_bytes()).unwrap();

    let mut private_key_file = File::create(private_key_file_path).unwrap();
    private_key_file.write_all(private_key.as_bytes()).unwrap();
    // println!("Response JSON: {:?}", response_json);
}
