
mod bigtable_wrappers;

// #[macro_use]
// extern crate cpython;

// use cpython::{Python, PyResult};

// fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
//     let mut total = 0u64;

//     // There is an improved version later on this post
//     for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
//         if c1 == c2 {
//             total += 1;
//         }
//     }

//     Ok(total)
// }

// py_module_initializer!(libsolana_bigtable_decode, initlibsolana_bigtable_decode, PyInit_solana_bigtable_decode, |py, m | {
//     try!(m.add(py, "__doc__", "This module is implemented in Rust"));
//     try!(m.add(py, "count_doubles", py_fn!(py, count_doubles(val: &str))));
//     Ok(())
// });
