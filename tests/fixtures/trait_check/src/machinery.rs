use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;

pub struct PaymentHandler {
    config: PaymentConfig,
    log_path: String,
}

pub struct PaymentConfig {
    pub api_key: String,
    pub endpoint: String,
}

impl PaymentHandler {
    pub fn new(config: PaymentConfig, log_path: String) -> Self {
        Self { config, log_path }
    }

    pub fn validate(&self, amount: f64) -> bool {
        amount > 0.0
    }

    pub fn charge(&mut self, card: &str, amount: f64) -> Result<String, String> {
        let mut stream = TcpStream::connect(&self.config.endpoint)
            .map_err(|e| format!("connect: {e}"))?;
        let body = format!("charge {} {}", card, amount);
        stream.write_all(body.as_bytes()).map_err(|e| format!("write: {e}"))?;
        Ok("ok".into())
    }

    pub fn refund(&mut self, tx_id: &str) -> Result<String, String> {
        let mut stream = TcpStream::connect(&self.config.endpoint)
            .map_err(|e| format!("connect: {e}"))?;
        let body = format!("refund {}", tx_id);
        stream.write_all(body.as_bytes()).map_err(|e| format!("write: {e}"))?;
        Ok("refunded".into())
    }

    pub fn log_receipt(&self, receipt: &str) -> Result<(), String> {
        let path = Path::new(&self.log_path);
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("open: {e}"))?;
        writeln!(file, "{receipt}").map_err(|e| format!("write: {e}"))
    }
}
