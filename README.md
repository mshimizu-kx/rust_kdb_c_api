# Rust Wrapper of kdb+ C API

Programming language q (kdb+ is a database written in q) is providing only C API but sometimes an external library provides Rust interface but not C/C++ interface. From the fame of its performance, Rust still should be feasible to build a shared library for kdb+. This library is provided to address such a natural demand (desire, if you will). Since there is no way for everyone but creating a wrapper like this to write a shared library for kdb+, it probably make sense for someone to provide the wrapper, and we did.

## Installation

Use `kdb_c_api` as a library name in `Cargo.toml`.

```toml

[dependencies]
kdb_c_api="^0.1"

```

## Examples

The examples of using C API wrapper are included in `c_api_examples` folder. The examples are mirroring the examples in the document of `kdb_c_api` library and the functions are also used for simple tests of the library. The test is conducted in the `test.q` under `tests/` by loading the functions defined in a shared library built from the examples.

Here are parts of the examples:

```rust

use kdb_c_api::*;

#[no_mangle]
pub extern "C" fn create_symbol_list(_: K) -> K{
  unsafe{
    let mut list=ktn(qtype::SYMBOL as i32, 0);
    js(&mut list, ss(str_to_S!("Abraham")));
    js(&mut list, ss(str_to_S!("Isaac")));
    js(&mut list, ss(str_to_S!("Jacob")));
    list
  }
}
 
#[no_mangle]
pub extern "C" fn catchy(func: K, args: K) -> K{
  unsafe{
    let result=ee(dot(func, args));
    if (*result).qtype == -qtype::ERROR{
      println!("error: {}", S_to_str((*result).value.symbol));
      KNULL!()
    }
    else{
      result
    }
  }
}

#[no_mangle]
pub extern "C" fn dictionary_list_to_table() -> K{
  unsafe{
    let dicts=knk(3);
    let dicts_slice=as_mut_K_slice(dicts);
    //(*dicts).value.list.n=0;
    for i in 0..3{
      let keys=ktn(qtype::SYMBOL as i32, 2);
      let keys_slice=as_mut_symbol_slice(keys);
      keys_slice[0]=ss(str_to_S!("a"));
      keys_slice[1]=ss(str_to_S!("b"));
      let values=ktn(qtype::INT as i32, 2);
      as_mut_int_slice(values)[0..2].copy_from_slice(&[i*10, i*100]);
      dicts_slice[i as usize]=xD(keys, values);
    }
    // Format list of dictionary as a table.
    // ([] a: 0 10 20i; b: 0 100 200i)
    k(0, str_to_S!("{[dicts] -1 _ dicts, (::)}"), dicts, KNULL!())
  } 
}

```

q can use these functions like this:

```q

q)summon_symbol_list:`libc_api_examples 2: (`create_symbol_list; 1)
q)summon_symbol_list[]
`Abraham`Isaac`Jacob
q)`Abraham`Isaac`Jacob ~ summon_symbol_list[]
q)catchy: `libc_api_examples 2: (`catchy; 2);
q)catchy[$; ("J"; "42")]
42
q)catchy[+; (1; `a)]
error: type
q)unfortunate_fact: `libc_api_examples 2: (`dictionary_list_to_table; 1);
q)unfortunate_fact[]
a  b  
------
0  0  
10 100
20 200
```

## Test

Tests are conducted with the example functions in `tests/test.q` by loading the functions into q process.

```bash

rust_kdb_c_api]$ cargo build
rust_kdb_c_api]$ cp target/debug.libc_api_examples.so tests/
rust_kdb_c_api]$ cd tests
tests]$ q test.q
Initialized something, probably it is your mindset.
error: type
symbol: `rust
test result: ok. 12 passed; 0 failed
q)

```

## Note

- This library is purposed to be used to build a sared library; therefore some unrelated functions are removed. For example, connection functions to kdb+ like `khpu` or q function generator like `dl` are not included.
- As it is destined to use C API in general, sometimes resource management of Rust can lead to a strange behavior. For example, some logic works if directly used as a part of function but not if it is encapsulated in a separate function and called in the same position of the code. Unfortunately we cannot offer helpful advice for these kind of behaviors.
