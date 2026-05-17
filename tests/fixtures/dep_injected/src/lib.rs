// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub trait Database {
    fn query(&self, sql: &str) -> String;
}

pub trait Logger {
    fn log(&self, msg: &str);
}

pub trait Gateway {
    fn charge(&self, amount: f64) -> String;
}

pub struct Service {
    db: Box<dyn Database>,
    logger: Box<dyn Logger>,
    gateway: Box<dyn Gateway>,
}

impl Service {
    pub fn new(db: Box<dyn Database>, logger: Box<dyn Logger>, gateway: Box<dyn Gateway>) -> Self {
        Self { db, logger, gateway }
    }

    pub fn process(&self, amount: f64) -> String {
        let result = self.gateway.charge(amount);
        self.logger.log(&result);
        self.db.query("INSERT INTO payments VALUES (1)");
        result
    }
}
