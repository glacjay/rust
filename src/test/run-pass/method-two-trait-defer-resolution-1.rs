// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that we pick which version of `foo` to run based on the
// type that is (ultimately) inferred for `x`.

trait foo {
    fn foo(&self) -> int;
}

impl foo for Vec<uint> {
    fn foo(&self) -> int {1}
}

impl foo for Vec<int> {
    fn foo(&self) -> int {2}
}

fn call_foo_uint() -> int {
    let mut x = Vec::new();
    let y = x.foo();
    x.push(0u);
    y
}

fn call_foo_int() -> int {
    let mut x = Vec::new();
    let y = x.foo();
    x.push(0i);
    y
}

fn main() {
    assert_eq!(call_foo_uint(), 1);
    assert_eq!(call_foo_int(), 2);
}
