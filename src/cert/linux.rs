use crate::archive::DecompressError;
use crate::prelude::decompress;
use crate::prelude::download_file_as;
use crate::web::WebError;
use assert_fs::fixture::PathChild;
use assert_fs::TempDir;
use reqwest::Url;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;
use thiserror::Error;

pub struct EasyRsa {
    temp_dir: TempDir,
}

impl Default for EasyRsa {
    fn default() -> Self {
        let temp_dir = TempDir::new().unwrap().into_persistent();
        Self { temp_dir }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Web(#[from] WebError),
    #[error(transparent)]
    Archive(#[from] DecompressError),
}

const EASY_RSA_BIN: &str =
    "https://github.com/OpenVPN/easy-rsa/releases/download/v3.0.8/EasyRSA-3.0.8.tgz";

impl EasyRsa {
    pub fn download_app(&self) -> Result<(), Error> {
        std::fs::create_dir_all(self.easy_dir())?;
        download_file_as(EASY_RSA_BIN, &self.easy_dir(), &self.easy_rsa_file())?;

        let mut compressed_app = self.easy_dir();
        compressed_app.push(self.easy_rsa_file());
        decompress(&compressed_app, &self.easy_dir()).map_err(Into::into)
    }

    fn easy_rsa_file(&self) -> String {
        Url::parse(EASY_RSA_BIN)
            .unwrap()
            .path_segments()
            .unwrap()
            .last()
            .unwrap()
            .to_string()
    }

    fn easy_rsa_file_name(&self) -> String {
        Url::parse(EASY_RSA_BIN)
            .unwrap()
            .to_file_path()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    fn easy_dir(&self) -> PathBuf {
        self.temp_dir.child("easy_rsa").path().to_path_buf()
    }

    fn extracted_easy_dir(&self) -> PathBuf {
        let mut source = self.easy_dir();
        source.push(self.easy_rsa_file_name());
        source
    }

    fn extracted_vars_example(&self) -> PathBuf {
        let mut source = self.extracted_easy_dir();
        source.push("vars.example");
        source
    }
    pub fn init_pki(&self) -> Result<Output, std::io::Error> {
        self.easy_rsa()
            .arg("init-pki")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .output()
    }

    pub fn prepare_var_file(&self) -> Result<(), std::io::Error> {
        let source = self.extracted_vars_example();

        let text = crate::file::read_file(&source)?
            .replace(
                "#set_var.EASYRSA_REQ_COUNTRY",
                "set_var.EASYRSA_REQ_COUNTRY",
            )
            .replace("#set_var.EASYRSA_REQ_CITY", "set_var.EASYRSA_REQ_CITY")
            .replace(
                "#set_var.EASYRSA_REQ_PROVINCE",
                "set_var.EASYRSA_REQ_PROVINCE",
            )
            .replace(
                "#set_var.EASYRSA_REQ_ORG----\"Copyleft.Certificate.Co\"",
                "set_var.EASYRSA_REQ_ORG----\"Iohk.Co\"",
            )
            .replace(
                "#set_var.EASYRSA_REQ_EMAIL--\"me@example.net\"",
                "set_var.EASYRSA_REQ_EMAIL--\"dariusz.kijania@iohk.io\"",
            )
            .replace(
                "#set_var.EASYRSA_REQ_OU-----\"My.Organizational.Unit\"",
                "set_var.EASYRSA_REQ_OU--\"Jormungandr\"",
            );

        let mut dest = self.easy_dir();
        dest.push("vars");
        std::fs::write(dest, text)
    }

    fn easy_rsa(&self) -> Command {
        let mut easy_rsa = self.extracted_easy_dir();
        easy_rsa.push("easyrsa");
        Command::new(easy_rsa)
    }

    pub fn get_certificate(&self) -> Result<(PathBuf, PathBuf, PathBuf), std::io::Error> {
        let mut process = self
            .easy_rsa()
            .arg("build-ca")
            .arg("nopass")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = process.stdin.take().unwrap();
        stdin.write_all(b"localhost")?;
        process.wait()?;

        let mut process = self
            .easy_rsa()
            .arg("gen-req")
            .arg("server")
            .arg("nopass")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = process.stdin.take().unwrap();

        stdin.write_all(b"server")?;
        process.wait()?;

        let mut process = self
            .easy_rsa()
            .arg("sign-req")
            .arg("server")
            .arg("server")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = process.stdin.take().unwrap();

        stdin.write_all(b"server")?;
        process.wait()?;

        let mut server_crt = self.easy_dir();
        server_crt.push("pki");
        server_crt.push("issued");
        server_crt.push("server.crt");

        let mut server_key = self.easy_dir();
        server_key.push("pki");
        server_key.push("private");
        server_key.push("server.key");

        let mut ca_crt = self.easy_dir();
        ca_crt.push("pki");
        ca_crt.push("ca.crt");

        Ok((server_crt, server_key, ca_crt))
    }
}
