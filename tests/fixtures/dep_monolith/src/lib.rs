// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use std::net::TcpStream;
use std::time::Instant;

pub struct Monolith;

impl Monolith {
    pub fn network_call(&self) -> String {
        let mut conn = TcpStream::connect("192.168.1.1:8080").unwrap();
        let _ = conn.write_all(b"ping");
        "ok".to_string()
    }

    pub fn write_report(&self, data: &str) {
        let _ = fs::write("/var/log/report.txt", data);
    }

    pub fn timed_snapshot(&self) -> String {
        let _now = Instant::now();
        let _ = fs::write("/tmp/snapshot.txt", b"snap");
        "done".to_string()
    }

    pub fn execute(&self) -> i32 {
        let _path = std::env::var("PATH");
        std::process::exit(0);
    }

    pub fn probe(&self) -> u32 {
        let _conn = TcpStream::connect("10.0.0.1:80").unwrap();
        let _rng = rand::thread_rng();
        42
    }
}
