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
// jk
.capi.create_compound_list: LIBPATH_ (`create_compound_list; 1);
// xD
.capi.create_dictionary: LIBPATH_ (`create_dictionary; 1);
// js
.capi.create_symbol_list: LIBPATH_ (`create_symbol_list; 1);
// k
.capi.dictionary_list_to_table: LIBPATH_ (`dictionary_list_to_table; 1);
// as_mut_slice
.capi.modify_long_list_a_bit: LIBPATH_ (`modify_long_list_a_bit; 1);
// str_to_const_S
.capi.must_be_int: LIBPATH_ (`must_be_int; 1);
// setm
.capi.parallel_sym_change: LIBPATH_ (`parallel_sym_change; 1);
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

//%% KUtility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// as_mut_slice
// Assign to a variable to keep the result.
.test.ASSERT_EQ["as_mut_slice - success"; .capi.modify_long_list_a_bit[list:1 2 3]; 1 30000 3]
// as_mut_slice (return error)
.test.ASSERT_ERROR["as_mut_slice - failure"; .capi.modify_long_list_a_bit; enlist enlist 1; "this list is not long enough"]

//%% Constructors %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// js
.test.ASSERT_EQ["jk"; .capi.create_compound_list[]; (`1st; 2i; "3rd")]

// js
.test.ASSERT_EQ["js"; .capi.create_symbol_list[]; `Abraham`Isaac`Jacob]

// xD
.test.ASSERT_EQ["xD"; .capi.create_dictionary[]; 0 1i!(2000.01.01 2000.01.02 2000.01.03; "I'm afraid I would crash the application...")]

// ee
.test.ASSERT_EQ["ee - success"; .capi.catchy[$; ("S"; "rust")]; `rust]
// ee (print error to stdout)
.test.ASSERT_EQ["ee - failure"; .capi.catchy[+; (2; "rust")]; (::)]

//%% IPC Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// k
.test.ASSERT_EQ[enlist "k"; .capi.dictionary_list_to_table[]; ([] a: 0 10 20i; b: 0 100 200i)]

//%% Miscellaneous %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// dot
.test.ASSERT_EQ["dot"; .capi.rust_parse[$; ("J"; "42")]; 42]

// setm
.test.ASSERT_EQ["dot"; .capi.parallel_sym_change[`a`b]; `replaced`symbolbol]

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
