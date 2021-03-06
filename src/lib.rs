#![feature(optin_builtin_traits)]

extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate png;
extern crate rand;

#[macro_use] mod macros;

pub mod chance;
pub mod graphics;
pub mod input;
pub mod simulation;
pub mod timing;
pub mod world;
