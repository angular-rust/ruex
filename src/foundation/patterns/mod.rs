//! Catalog of patterns

#![allow(
    clippy::module_inception,
    clippy::new_without_default,
    clippy::type_complexity
)]

pub mod command;

pub mod default;

pub mod facade;

pub mod fsm;

pub mod mediator;

pub mod observer;

pub mod proxy;

pub mod builder;
