// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(step_by)]
#![feature(test)]
#![feature(core)]
#![feature(rand)]

#[cfg(test)] extern crate test;

mod cryptoutil;
mod md5;

pub use md5::Md5;
