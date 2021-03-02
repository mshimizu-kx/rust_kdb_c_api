//! This module is provided as examples of `kdb_c_api` crate. The functions defined here will be
//!  used for simple tests.

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	Load Libraries     			                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

extern crate kdb_c_api;

use kdb_c_api::*;

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          		Macros		   				                    //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `KUNLL`.
#[no_mangle]
pub extern "C" fn vanity(_: K) -> K{
  println!("Initialized something, probably it is your mindset.");
  KNULL!()
}

/// Example of `str_to_S`.
#[no_mangle]
pub extern "C" fn bigbang(_: K) -> K{
  unsafe{
		ks(str_to_S!("super_illusion"))
	}
}

/// Example of `str_to_const_S`.
#[no_mangle]
pub extern "C" fn must_be_int(obj: K) -> K{
  unsafe{
    if (*obj).qtype != -Q_INT{
      krr(str_to_const_S("not an int"))
    }
    else{
      KNULL!()
    }
  }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	Constructors      			                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `js`.
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

/// Example of `ee`. 
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

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	Miscellaneous   			                    //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `dot`.
#[no_mangle]
pub extern "C" fn rust_parse(dollar: K, type_and_text: K) -> K{
  unsafe{
    dot(dollar, type_and_text)
  }
}

/*
#[no_mangle]
pub extern "C" fn parallel_stringify(list: K) -> K{
	unsafe{
		(*list).value.
	}
}
*/

/// Example of `S_to_str`.
#[no_mangle]
pub extern "C" fn print_symbol(symbol: K) -> K{
  unsafe{
    if (*symbol).qtype == -Q_SYMBOL{
      println!("symbol: `{}", S_to_str((*symbol).value.symbol));
    }
    // return null
    KNULL!()
  }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          		Functions   				                    //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `null_terminated_str_to_S`.
#[no_mangle]
pub extern "C" fn bigbang2(_: K) -> K{
  unsafe{
		ks(null_terminated_str_to_S("super_illusion\0"))
	}
}
