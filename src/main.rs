#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(myos::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[allow(unused_imports)]
use myos;