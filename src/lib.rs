//! # Harmonica
//! 
//! `harmonica` is a Rust crate that provides tools for analyzing and transforming music theoretical objects.

#![allow(unused)]

/// Types Module
/// 
/// The 'types' module contains definitions for data types used throughout the library.
pub mod types;

/// Behaviors Module
/// 
/// The `behaviors` module contains definitions for behaviors used throughout the library.
pub mod behaviors;

/// Utility Module
/// 
/// The `utility` module provides general purpose tools used throughout the library.
pub mod utility;

/// API Module
/// 
/// The `api` module exposes items in the library to the public API.
mod api;
pub use api::*;
