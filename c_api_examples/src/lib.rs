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
		if (*obj).qtype != -qtype::INT{
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
		let mut list=ktn(qtype::SYMBOL as i32, 0);
		js(&mut list, ss(str_to_S!("Abraham")));
		js(&mut list, ss(str_to_S!("Isaac")));
		js(&mut list, ss(str_to_S!("Jacob")));
		list
	}
}

/// Example of `jk`.
#[no_mangle]
pub extern "C" fn create_compound_list(_: K) -> K{
	unsafe{
		let mut list=knk(0);
		jk(&mut list, ks(str_to_S!("1st")));
		jk(&mut list, ki(2));
		jk(&mut list, kpn(str_to_S!("3rd"), "3rd".chars().count() as i64));
		list
	}
}

/// Example of `xD`.
#[no_mangle]
pub extern "C" fn create_dictionary() -> K{
	unsafe{
		let keys=ktn(qtype::INT as i32, 2);
		as_mut_int_slice(keys).copy_from_slice(&[0, 1]);
		let values=knk(2);
		let date_list=ktn(qtype::DATE as i32, 3);
		// 2000.01.01 2000.01.02 2000.01.03
		as_mut_int_slice(date_list)[0..3].copy_from_slice(&[0, 1, 2]);
		let string=kp(str_to_S!("I'm afraid I would crash the application..."));
		as_mut_K_slice(values)[0..2].copy_from_slice(&[date_list, string]);
		// 0 1i!(2000.01.01 2000.01.02 2000.01.03; "I'm afraid I would crash the application...")
		xD(keys, values)
	}
}

/// Example of `ee`. 
#[no_mangle]
pub extern "C" fn catchy(func: K, args: K) -> K{
	unsafe{
		let result=ee(dot(func, args));
		if (*result).qtype == qtype::ERROR{
			println!("error: {}", S_to_str((*result).value.symbol));
			KNULL!()
		}
		else{
			result
		}
	}
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	IPC Functions   			                    //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `k`.
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
		if (*symbol).qtype == -qtype::SYMBOL{
			println!("symbol: `{}", S_to_str((*symbol).value.symbol));
		}
		// return null
		KNULL!()
	}
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//															Functions																//
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Example of `null_terminated_str_to_S`.
#[no_mangle]
pub extern "C" fn bigbang2(_: K) -> K{
	unsafe{
		ks(null_terminated_str_to_S("super_illusion\0"))
	}
}
