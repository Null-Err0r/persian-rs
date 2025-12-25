// src/lib.rs

//! # Presin-rs: a library for programers who needs to work with presian and Iranian tools
//!This library is an lightweight,exhaustive and practical
//! toolkit for Rust programers who needs to work with presian
//! and Iraninan tools.
//!
//! This library doesn't have any external dependency and all algoritms
//! are implmented locally
//!
//!## Features:
//! This library currently provaides 2 main modules:
//! * **Date conversion module([`jalali`])** for date conversion between jalali/shamsi and gregorian/miladi
//! * **Validateion module([`validation`])** for validating Iranian national id and card number

///Contains function and types for date conversion
pub mod jalali;
///Contains function for Iranian id number and card number validation
pub mod validation;
