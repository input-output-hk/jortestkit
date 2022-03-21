#![cfg(unix)]

use assert_fs::TempDir;
use jortestkit::prelude::EasyRsa;

#[test]
pub fn easy_rsa() {
    let easy_rsa = EasyRsa::new();
    easy_rsa.download_app();
    easy_rsa.prepare_var_file();
    let (server_crt, server_key, ca_crt) = easy_rsa.get_certificate();

    println!("{:?}", server_crt);
    println!("{:?}", server_key);
    println!("{:?}", ca_crt);
}
