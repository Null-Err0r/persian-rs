// src/lib.rs

//!This library is an lightweight,exhaustive and practical
//! toolkit for Rust programers that needs to work with presian
//! and Iraninan tools.
//!
//! This library doesn't have any external dependency and all algoritms
//! are implmented locally
//!
//!## Features:
//! This library currently provaides 2 main modules:
//! * **Date conversion module([`jalali`])**
//! * **Validateion module([`validation`])**

///Contains function and types for date conversion
pub mod jalali;
///Contains function for Iranian id number and card number validation
pub mod validation;
