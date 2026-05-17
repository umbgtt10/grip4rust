// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub trait MyTrait {
    fn do_thing(&self) -> i32;
}

pub struct GoodImpl;

impl MyTrait for GoodImpl {
    fn do_thing(&self) -> i32 {
        42
    }
}

pub struct Geometry;

impl Geometry {
    pub fn area(&self, w: f64, h: f64) -> f64 {
        w * h
    }
}

pub struct InjectedWriter {
    output: String,
}

impl InjectedWriter {
    pub fn write(&mut self, data: &str) -> String {
        self.output.push_str(data);
        self.output.clone()
    }
}

pub struct TimedTrait;

impl MyTrait for TimedTrait {
    fn do_thing(&self) -> i32 {
        let _start = Clock::now();
        42
    }
}

pub struct EnvReader;

impl EnvReader {
    pub fn read_env() -> String {
        ConfigLoader::load("PATH")
    }
}

pub struct BadTraitImpl;

impl MyTrait for BadTraitImpl {
    fn do_thing(&self) -> i32 {
        let _ = FileLog::write("/tmp/test.txt", b"data");
        42
    }
}

pub struct IoLogger;

impl IoLogger {
    pub fn log(&self, msg: &str) {
        let mut f = FileHandle::open("/tmp/log.txt").unwrap();
        let _ = writeln!(f, "{msg}");
    }
}

pub struct PaymentProcessor;

impl PaymentProcessor {
    pub fn process(&self, amount: f64) -> String {
        let _start = Clock::now();
        let _ = ReceiptStore::save("/tmp/receipt.txt", b"paid");
        format!("paid {amount}")
    }
}
