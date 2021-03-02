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
// null_terminated_str_to_S
.capi.bigbang2: LIBPATH_ (`bigbang2; 1);
// ee
.capi.catchy: LIBPATH_ (`catchy; 2);
// js
.capi.create_symbol_list: LIBPATH_ (`create_symbol_list; 1);
// str_to_const_S
.capi.must_be_int: LIBPATH_ (`must_be_int; 1);
// S_to_str
.capi.print_symbol: LIBPATH_ (`print_symbol; 1);
// dot
.capi.rust_parse: LIBPATH_ (`rust_parse; 2);
// KNULL
.capi.vanity: LIBPATH_ (`vanity; 1);

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Tests    	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Macros %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// KNULL
.test.ASSERT_EQ["KNULL"; .capi.vanity[]; (::)]

// str_to_S
.test.ASSERT_EQ["str_to_S"; .capi.bigbang[]; `super_illusion]

//%% Cinstructors %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// ee
.test.ASSERT_EQ["ee - success"; .capi.catchy[$; ("S"; "rust")]; `rust]
// ee (print error to stdout)
.test.ASSERT_EQ["ee - failure"; .capi.catchy[+; (2; "rust")]; (::)]

// js
.test.ASSERT_EQ["js"; .capi.create_symbol_list[]; `Abraham`Isaac`Jacob]

//%% Miscellaneous %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// dot
.test.ASSERT_EQ["dot"; .capi.rust_parse[$; ("J"; "42")]; 42]

//%% Utility Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// S_to_str (print value to stdout)
.test.ASSERT_EQ["S_to_str"; .capi.print_symbol[`rust]; (::)]

// null_terminated_str_to_S
.test.ASSERT_EQ["null_terminated_str_to_S"; .capi.bigbang2[]; `super_illusion]

// null_terminated_str_to_const_S
.test.ASSERT_ERROR["str_to_const_S"; .capi.must_be_int; enlist 10000; "not an int"]

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Result   	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Show result.
.test.DISPLAY_RESULT[]
