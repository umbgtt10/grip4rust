pub trait PaymentGateway {
    fn authorize(&self, card: &str, amount: f64) -> Result<String, String>;
    fn capture(&self, amount: f64) -> Result<String, String>;
    fn reverse(&self, tx_id: &str) -> Result<String, String>;
}

pub trait ReceiptLogger {
    fn write(&self, receipt: &str) -> Result<(), String>;
}

pub struct SeamedHandler {
    gateway: Box<dyn PaymentGateway>,
    logger: Box<dyn ReceiptLogger>,
}

impl SeamedHandler {
    pub fn new(gateway: Box<dyn PaymentGateway>, logger: Box<dyn ReceiptLogger>) -> Self {
        Self { gateway, logger }
    }

    pub fn validate(&self, amount: f64) -> bool {
        amount > 0.0
    }

    pub fn charge(&self, card: &str, amount: f64) -> Result<String, String> {
        self.gateway.authorize(card, amount)?;
        self.gateway.capture(amount)
    }

    pub fn refund(&self, tx_id: &str) -> Result<String, String> {
        self.gateway.reverse(tx_id)
    }

    pub fn log_receipt(&self, receipt: &str) -> Result<(), String> {
        self.logger.write(receipt)
    }
}
