/
* @file test.q
* @overview Tests of C API examples. The artefact of `c_api_examples` is loaded
* and functions are called from q side.
\

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                           Inital Setting     			                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Fetch shared object from build directory.
system "cp ../target/debug/libc_api_examples.so .";

// Load test helper functions.
\l test_helper_function.q

// Function to load shared library.
LIBPATH_: `libc_api_examples 2:

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	Load Libraries     			                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// These function list can be checked against `nm -D libc_api_examples.so | awk '$2 ~/T/ {print $3}'`.

// str_to_S
.capi.bigbang: LIBPATH_ (`bigbang; 1);
// ee
.capi.catchy: LIBPATH_ (`catchy; 2);
// js
.capi.create_symbol_list: LIBPATH_ (`create_symbol_list; 1);
// str_to_const_S
.capi.must_be_int: LIBPATH_ (`must_be_int; 1);
// S_to_str
.capi.print_symbol: LIBPATH_ (`print_symbol; 1);
// dot
.capi.rust_parse: LIBPATH_ (`rust_parse; 1);
// KNULL
.capi.vanity: LIBPATH_ (`vanity; 1);

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Tests    	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// str_to_S
.test.ASSERT_EQ[`$"str_to_S"; .capi.bigbang[]; `super_illusion]

// Show result.
.test.DISPLAY_RESULT[]
