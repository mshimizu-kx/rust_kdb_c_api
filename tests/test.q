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
// kb
.capi.create_bool: LIBPATH_ (`create_bool; 1);
// kg
.capi.create_byte: LIBPATH_ (`create_byte; 1);
// ku
.capi.create_guid: LIBPATH_ (`create_guid; 1);
// ki
.capi.create_int: LIBPATH_ (`create_int; 1);
// jv
.capi.concat_list: LIBPATH_ (`concat_list; 2);
// kj
.capi.create_long: LIBPATH_ (`create_long; 1);
// kc
.capi.create_char: LIBPATH_ (`create_char; 1);
// jk
.capi.create_compound_list: LIBPATH_ (`create_compound_list; 1);
// kd
.capi.create_date: LIBPATH_ (`create_date; 1);
// kz
.capi.create_datetime: LIBPATH_ (`create_datetime; 1);
// xD
.capi.create_dictionary: LIBPATH_ (`create_dictionary; 1);
// kf
.capi.create_float: LIBPATH_ (`create_float; 1);
// knt
.capi.create_keyed_table: LIBPATH_ (`create_keyed_table; 1);
// new_minute
.capi.create_minute: LIBPATH_ (`create_minute; 1);
// new_month
.capi.create_month: LIBPATH_ (`create_month; 1);
// ke
.capi.create_real: LIBPATH_ (`create_real; 1);
// kh
.capi.create_short: LIBPATH_ (`create_short; 1);
// new_second
.capi.create_second: LIBPATH_ (`create_second; 1);
// kp
.capi.create_string: LIBPATH_ (`create_string; 1);
// kpn
.capi.create_string2: LIBPATH_ (`create_string2; 1);
// ks
.capi.create_symbol: LIBPATH_ (`create_symbol; 1);
// js
.capi.create_symbol_list: LIBPATH_ (`create_symbol_list; 1);
// xT
.capi.create_table: LIBPATH_ (`create_table; 1);
// kt
.capi.create_time: LIBPATH_ (`create_time; 1);
// ktj
.capi.create_timespan: LIBPATH_ (`create_timespan; 1);
// ktj
.capi.create_timestamp: LIBPATH_ (`create_timestamp; 1);
// dj
.capi.days_to_date: LIBPATH_ (`days_to_date; 1);
// k
.capi.dictionary_list_to_table: LIBPATH_ (`dictionary_list_to_table; 1);
// r0
.capi.idle_man: LIBPATH_ (`idle_man; 1);
// ktd
.capi.keyed_to_simple_table: LIBPATH_ (`keyed_to_simple_table; 1);
// as_mut_slice
.capi.modify_long_list_a_bit: LIBPATH_ (`modify_long_list_a_bit; 1);
// str_to_const_S
.capi.must_be_int: LIBPATH_ (`must_be_int; 1);
// setm
.capi.parallel_sym_change: LIBPATH_ (`parallel_sym_change; 1);
// r1
.capi.pass_through_cave: LIBPATH_ (`pass_through_cave; 1);
// get_byte
.capi.print_byte: LIBPATH_ (`print_byte; 1);
// get_char
.capi.print_char: LIBPATH_ (`print_char; 1);
// get_float
.capi.print_float: LIBPATH_ (`print_float; 1);
// get_int
.capi.print_int: LIBPATH_ (`print_int; 1);
// get_long
.capi.print_long: LIBPATH_ (`print_long; 1);
// get_real
.capi.print_real: LIBPATH_ (`print_real; 1);
// get_short
.capi.print_short: LIBPATH_ (`print_short; 1);
// get_string
.capi.print_string: LIBPATH_ (`print_string; 1);
// S_to_str
.capi.print_symbol: LIBPATH_ (`print_symbol; 1);
// get_symbol
.capi.print_symbol2: LIBPATH_ (`print_symbol2; 1);
// dot
.capi.rust_parse: LIBPATH_ (`rust_parse; 2);
// krr
.capi.thai_kick: LIBPATH_ (`thai_kick; 1);
// KNULL
.capi.vanity: LIBPATH_ (`vanity; 1);
// ymd
.capi.ymd_to_days: LIBPATH_ (`ymd_to_days; 1);

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

// get_byte
.test.ASSERT_EQ["get_byte"; .capi.print_byte[0xc4]; (::)]
// get_byte - failure
.test.ASSERT_ERROR["get_byte - failure"; .capi.print_byte; enlist "c"; "not a byte"]

// get_short
.test.ASSERT_EQ["get_short"; .capi.print_short[10h]; (::)]
// get_short - failure
.test.ASSERT_ERROR["get_short - failure"; .capi.print_short; enlist 10; "not a short"]

// get_int
.test.ASSERT_EQ["get_int"; .capi.print_int[42i]; (::)]
// get_int - month
.test.ASSERT_EQ["get_int - month"; .capi.print_int[2010.03m]; (::)]
// get_int - date
.test.ASSERT_EQ["get_int - date"; .capi.print_int[2020.02.01]; (::)]
// get_int - minute
.test.ASSERT_EQ["get_int - minute"; .capi.print_int[12:03]; (::)]
// get_int - second
.test.ASSERT_EQ["get_int - second"; .capi.print_int[03:57:20]; (::)]
// get_int - time
.test.ASSERT_EQ["get_int - time"; .capi.print_int[00:34:16.636]; (::)]
// get_int - error
.test.ASSERT_ERROR["get_int - failure1"; .capi.print_int; enlist `error; "not an int"]
// get_int - error
.test.ASSERT_ERROR["get_int - failure2"; .capi.print_int; enlist 10000; "not an int"]

// get_long
.test.ASSERT_EQ["get_long"; .capi.print_long[-109210]; (::)]
// get_long - timestamp
.test.ASSERT_EQ["get_long - timestamp"; .capi.print_long[2000.01.01D12:00:00.123456789]; (::)]
// get_long - timespan
.test.ASSERT_EQ["get_long - timespan"; .capi.print_long[-3D18:23:09.000000021]; (::)]
// get_long - error
.test.ASSERT_ERROR["get_long - failure"; .capi.print_long; enlist 1b; "not a long"]

// get_real
.test.ASSERT_EQ["get_real"; .capi.print_real[193810.32e]; (::)]
// get_real - error
.test.ASSERT_ERROR["get_real - failure"; .capi.print_real; enlist 100f; "not a real"]

// get_float
.test.ASSERT_EQ["get_float"; .capi.print_float[-37017.0933]; (::)]
// get_float - datetime
.test.ASSERT_EQ["get_float - datetime"; .capi.print_float[2002.01.12T10:03:45.332]; (::)]
// get_float - error
.test.ASSERT_ERROR["get_float - failure"; .capi.print_float; enlist .z.p; "not a float"]

// get_char
.test.ASSERT_EQ["get_char"; .capi.print_char["k"]; (::)]
// get_char - error
.test.ASSERT_ERROR["get_char - failure1"; .capi.print_char; enlist "devour"; "not a char"]
// get_char - error
.test.ASSERT_ERROR["get_char - failure2"; .capi.print_char; enlist 1b; "not a char"]

// get_symbol
.test.ASSERT_EQ["get_symbol"; .capi.print_symbol2[`locust]; (::)]
// get_symool - error
.test.ASSERT_ERROR["get_symbol - failure"; .capi.print_symbol2; enlist "attack!"; "not a symbol"]

// get_string
.test.ASSERT_EQ["get_string"; .capi.print_string["grasshopper"]; (::)]
// get_string - error
.test.ASSERT_ERROR["get_string - failure"; .capi.print_string; enlist (1 2; `a`b); "not a string"]

//%% Constructors %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// kb
.test.ASSERT_EQ["kb"; .capi.create_bool[]; 1b]

// ku
.test.ASSERT_EQ["ku"; .capi.create_guid[]; "G"$"1e11170c-4224-252c-1c14-1e224d3d4624"]

// kg
.test.ASSERT_EQ["kg"; .capi.create_byte[]; 0x3c]

// kh
.test.ASSERT_EQ["kh"; .capi.create_short[]; -144h]

// ki
.test.ASSERT_EQ["ki"; .capi.create_int[]; 86400000i]

// kj
.test.ASSERT_EQ["kj"; .capi.create_long[]; -668541276001729000]

// ke
.test.ASSERT_EQ["ke"; .capi.create_real[]; 0.00324e]

// kf
.test.ASSERT_EQ["kf"; .capi.create_float[]; -6302.620]

// kc
.test.ASSERT_EQ["kc"; .capi.create_char[]; "q"]

// ks
.test.ASSERT_EQ["ks"; .capi.create_symbol[]; `symbolism]

// ktj - timestamp
.test.ASSERT_EQ["ktj - timestamp"; .capi.create_timestamp[]; 2015.03.16D00:00:00:00.000000000]

// ktj - timespan
.test.ASSERT_EQ["ktj - timespan"; .capi.create_timespan[]; -1D01:30:00.001234567]

// kd
.test.ASSERT_EQ["kd"; .capi.create_date[]; 1999.12.25]

// kz
.test.ASSERT_EQ["kz"; .capi.create_datetime[]; 2015.03.16T12:00:00:00]

// kt
.test.ASSERT_EQ["kz"; .capi.create_time[]; -01:30:00.123]

// kp
.test.ASSERT_EQ["kp"; .capi.create_string[]; "this is a text."]

// kpn
.test.ASSERT_EQ["kpn"; .capi.create_string2[]; "The meeting was too long"]

// xT
.test.ASSERT_EQ["xT"; .capi.create_table[]; table:([] time: 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392; temperature: 22.1, 24.7, 30.5)]

// ktd
.test.ASSERT_EQ["xT"; .capi.keyed_to_simple_table[]; table]

// xD
.test.ASSERT_EQ["xD"; .capi.create_dictionary[]; 0 1i!(2000.01.01 2000.01.02 2000.01.03; "I'm afraid I would crash the application...")]

// knt
.test.ASSERT_EQ["xT"; .capi.create_keyed_table[]; 1!table]

// krr
.test.ASSERT_ERROR["krr"; .capi.thai_kick; enlist (::); "Thai kick unconditionally!!"]

// jv
.test.ASSERT_EQ["jv - compound"; .capi.concat_list[(::; `metals; `fire); ("clay"; 316)]; (::; `metals; `fire; "clay"; 316)]
.test.ASSERT_EQ["jv - long"; .capi.concat_list[1 2 3; 4 5]; 1 2 3 4 5]
.test.ASSERT_EQ["jv - symbol"; .capi.concat_list[`a`b`c; `d`e]; `a`b`c`d`e]

// jk
.test.ASSERT_EQ["jk"; .capi.create_compound_list[]; (`1st; 2i; "3rd")]

// js
.test.ASSERT_EQ["js"; .capi.create_symbol_list[]; `Abraham`Isaac`Jacob`Joseph]

// ee
.test.ASSERT_EQ["ee - success"; .capi.catchy[$; ("S"; "rust")]; `rust]
// ee (print error to stdout)
.test.ASSERT_EQ["ee - failure"; .capi.catchy[+; (2; "rust")]; (::)]

//%% IPC Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// k
.test.ASSERT_EQ[enlist "k"; .capi.dictionary_list_to_table[]; ([] a: 0 10 20i; b: 0 100 200i)]

//%% Reference count %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// r0
.test.ASSERT_EQ["r0"; .capi.idle_man[]; (::)]

// r1
get_item1:{[man] "a basket of summer fruit"};
get_item2:{[man] "boiling pot, facing away from the north"}
.test.ASSERT_EQ["r1"; .capi.pass_through_cave[`son_of_man]; `son_of_man]

//%% Miscellaneous %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// dot
.test.ASSERT_EQ["dot"; .capi.rust_parse[$; ("J"; "42")]; 42]

// setm
.test.ASSERT_EQ["dot"; .capi.parallel_sym_change[`a`b]; `replaced`symbolbol]

// ymd
.test.ASSERT_EQ["ymd"; .capi.ymd_to_days[]; 7396i]

// dj
.test.ASSERT_EQ["dj"; .capi.days_to_date[7396i]; 20200401i]

//%% Utility Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// S_to_str (print value to stdout)
.test.ASSERT_EQ["S_to_str"; .capi.print_symbol[`rust]; (::)]

// null_terminated_str_to_S
.test.ASSERT_EQ["null_terminated_str_to_S"; .capi.bigbang2[]; `super_illusion]

// null_terminated_str_to_const_S
.test.ASSERT_ERROR["str_to_const_S"; .capi.must_be_int; enlist 10000; "not an int"]

// new_month
.test.ASSERT_EQ["new_month"; .capi.create_month[]; 2010.07m]

// new_minute
.test.ASSERT_EQ["new_minute"; .capi.create_minute[]; 10:40]

// new_second
.test.ASSERT_EQ["new_second"; .capi.create_second[]; -02:00:00]

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          	  Result   	        		                  //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Show result.
.test.DISPLAY_RESULT[]
