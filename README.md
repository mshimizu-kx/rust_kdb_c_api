# Rust Wrapper of kdb+ C API

Programming language q (kdb+ is a database written in q) are providing only C API but it should not stop Rust developers to write a shared library for q. This library is a wrapper of the C API of which shared library related functions are extracted. For example, connection functions to kdb+ like `khpu` or q function generator like `dl` are not included.

## Examples

The examples of using C API wrapper are included in `c_api_examples` folder. The examples are mirroring the examples in the document of `kdb_c_api` library and the functions are also used for simple tests of the library. The test is conducted in the `test.q` under `tests/` by loading the functions defined in a shared library built from the examples.

Here are parts of the examples:

```rust

use kdb_c_api::*;

#[no_mangle]
pub extern "C" fn create_symbol_list(_: K) -> K{
	unsafe{
		let mut list=ktn(Q_SYMBOL as i32, 0);
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
		if (*result).qtype == -128{
			println!("error: {}", S_to_str((*result).value.symbol));
			KNULL!()
		}
		else{
			result
		}
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

```
