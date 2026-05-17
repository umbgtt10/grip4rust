// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub trait Processor {
    fn process(&self) -> i32;
}

pub struct GoodProcessor;

impl Processor for GoodProcessor {
    fn process(&self) -> i32 {
        42
    }
}

pub struct PureCalculator;

impl PureCalculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

pub struct InjectedHandler {
    processor: Box<dyn Processor>,
}

impl InjectedHandler {
    pub fn new(processor: Box<dyn Processor>) -> Self {
        Self { processor }
    }

    pub fn run(&mut self) -> i32 {
        self.processor.process()
    }
}
