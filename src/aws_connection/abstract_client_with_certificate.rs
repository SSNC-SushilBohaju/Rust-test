
use openssl::symm::{Cipher, encrypt};
use base64::{engine::general_purpose,Engine as _};

pub fn encrypt_data(key: &[u8], iv: &[u8], certificate: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_256_cfb8();
    encrypt(cipher, key, Some(iv), certificate).unwrap()
}

pub fn create_encorded_certificate(device_id: &str) -> String {
    let common_key="12345678";
    let end = if device_id.len() < 28 { device_id.len() } else { 27 };
    let secret_key = format!("{}{}", common_key, &device_id[3..end]);
    println!("{}",secret_key);
    let key = secret_key.as_bytes();
    let iv = b"1234567890abcdef";
    let certificate = b"-----BEGIN CERTIFICATE-----MIIDWjCCAkKgAwIBAgIVAOrO8PbeseyqRkhtPoEijMMRPG+VMA0GCSqGSIb3DQEBCwUAME0xSzBJBgNVBAsMQkFtYXpvbiBXZWIgU2VydmljZXMgTz1BbWF6b24uY29tIEluYy4gTD1TZWF0dGxlIFNUPVdhc2hpbmd0b24gQz1VUzAeFw0xOTA1MDcwNTQ2MzBaFw00OTEyMzEyMzU5NTlaMB4xHDAaBgNVBAMME0FXUyBJb1QgQ2VydGlmaWNhdGUwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQDZzkUR4/+ekifuK9106x7xkxhxaGn925v9hRCUcBle8rmJibGhttEr3kFMi5Lx5Hfh3zQvM5fkPIDDw93bbCT0QnQA8l/uMrQNtS6vkX+SpPmf1Jpu/iVpA3kWcM/+m9Za4hFa/XM1c+yc0sQrAvkvYjrRgo2XLSFdPcrDuWUrJfrOCfgBnaTzvwPx+BI06RXgoUKss6Y8jy+d3o2EkPpF59GTaAsCmWxZvraZ9mHqgGX029u7P9lpL+7/MzqXX/YHqQZJOIndFDq810UTatazlr2rQ9b3B925fg9I1VQjaOx5C1nMUwgmiLqzuD1iD8/Stq+mEYsQZo+norPb3cMTAgMBAAGjYDBeMB8GA1UdIwQYMBaAFPDJZsV1LqiKBoFZwowi3tnaQggiMB0GA1UdDgQWBBTAuyQwvofCdKpzSIub7C0uW9ToDzAMBgNVHRMBAf8EAjAAMA4GA1UdDwEB/wQEAwIHgDANBgkqhkiG9w0BAQsFAAOCAQEAzGf/vVKegYqDyIj7cRRCzfuEPdQb8+LvTjlWPgVUmFzsN0Iu8/gLelQ00Gymwva4AGd+XZ6RKU12fCuN7ngQxi8+hD6mvTTAsuwyXW0M0EVRiKkOj8LolRe/qkAae0VMOPjTOj6Az7QB01y5Ix2J+MNDDugXw4Q/ufm4BHJ/CfKAdwL1QtFl8CoEYXdAQBuBldTsrSNgRIH7MiYm/zAl0gMlhsoZ6n4jH77TaCK8leqtBqVhek1a2u400Hu1Ht9bpfV4cKk3mtA9B/WMQ5ax560SNlY5B9g2pctpwCoWqH7CzhFI8BGqNeJ8cv0up7rtr0b1hPktpynn3j4Xj21U0A==-----END CERTIFICATE-----";

    // Encrypt the plaintext
    let ciphertext = encrypt_data(key, iv, certificate);
    // println!("Ciphertext: {:?}", ciphertext);
    let encoded_ciphertext = general_purpose::STANDARD.encode(&ciphertext);
    println!("Encoded Ciphertext: {:?}", encoded_ciphertext);
    let encoded_certificate=String::from_utf8(encoded_ciphertext.into()).unwrap();
    encoded_certificate
}
