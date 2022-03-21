#![cfg(unix)]
use jortestkit::prelude::EasyRsa;

#[test]
pub fn easy_rsa() {
    let easy_rsa = EasyRsa::default();
    easy_rsa.download_app();
    easy_rsa.prepare_var_file().unwrap();
    easy_rsa.init_pki().unwrap();
    let (server_crt, server_key, ca_crt) = easy_rsa.get_certificate().unwrap();

    println!("{:?}", server_crt);
    println!("{:?}", server_key);
    println!("{:?}", ca_crt);
}
