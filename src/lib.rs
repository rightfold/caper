#![feature(optin_builtin_traits)]

extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate rand;

#[macro_use] mod macros;

pub mod chance;
pub mod graphics;
pub mod input;
pub mod world;
