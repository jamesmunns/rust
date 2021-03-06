// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Example cycle where a bound on `T` uses a shorthand for `T`. This
// creates a cycle because we have to know the bounds on `T` to figure
// out what trait defines `Item`, but we can't know the bounds on `T`
// without knowing how to handle `T::Item`.
//
// Note that in the future cases like this could perhaps become legal,
// if we got more fine-grained about our cycle detection or changed
// how we handle `T::Item` resolution.

use std::ops::Add;

// Preamble.
trait Trait { type Item; }

struct A<T>
    where T : Trait,
          T : Add<T::Item>
    //~^ ERROR cycle detected
    //~| ERROR associated type `Item` not found for `T`
{
    data: T
}

fn main() {
}
