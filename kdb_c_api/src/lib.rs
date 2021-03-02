//! Rust FFI header file mirroring C API header file (`k.h`) for kdb+ interface. The objective usage is to build a
//!  shared library in Rust. For a client library, see [rustkdb](https://github.com/KxSystems/rustkdb).
//! 
//! for details of functions, structures and macros, see [C API for kdb+](https://code.kx.com/q/interfaces/capiref/).
//! # Note
//! - This file is for kdb+ version 3.0+.
//! - Meangless C macros are excluded.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Load Libraries                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

use std::str;
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_float, c_int, c_longlong, c_short, c_schar, c_uchar, c_void};

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          Global Variables                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/// Type indicator of q mixed list.
///  Access function: `kK`
pub const Q_MIXED: i8=0;

/// Type indicator of q bool.
///  Access fucntion: `kG`
pub const Q_BOOL: i8=1;

/// Type indicator of q GUID.
///  Access function: `kU`
pub const Q_GUID: i8=2;

/// Type indicator of q byte
///  Access function: `kG`
pub const Q_BYTE: i8=4;

/// Type indicator of q short.
///  Access function: `kH`
pub const Q_SHORT: i8=5;

/// Type indicator of q int.
///  Access function: `kI`
pub const Q_INT: i8=6;

/// Type indicator of q long.
///  Access function: `kJ`
pub const Q_LONG: i8=7;

/// Type indicator of q real.
///  Access function: `kE`
pub const Q_REAL: i8=8;

/// Type indicator of q float.
///  Access function: `kF`
pub const Q_FLOAT: i8=9;

/// Type indicator of q char.
///  Access function: `kC`
pub const Q_CHAR: i8=10;

/// Type indicator of q symbol.
///  Access function: `kS`
pub const Q_SYMBOL: i8=11;

/// Type indicator of q timestamp.
///  Access function: `kJ`
pub const Q_TIMESTAMP: i8=12;

/// Type indicator of q month.
///  Access function: `kI`
pub const Q_MONTH: i8=13;

/// Type indicator of q date.
///  Access function: `kI`
pub const Q_DATE: i8=14;

/// Type indicator of q datetime.
///  Access function: `kF`
pub const Q_DATETIME: i8=15;

/// Type indicator of q timespan.
///  Access function: `kJ`
pub const Q_TIMESPAN: i8=16;

/// Type indicator of q minute.
///  Access function: `kI`
pub const Q_MINUTE: i8=17;

/// Type indicator of q second.
///  Access function: `kI`
pub const Q_SECOND: i8=18;

/// Type indicator of q time.
///  Access function: `kI`
pub const Q_TIME: i8=19;

/// Type indicator of q table.
///  `*(qstruct).k` is q dictionary.
pub const Q_TABLE: i8=98;

/// Type indicator of q dictionary.
/// - `kK(x)[0]`: keys
/// - `kK(x)[1]`: values
pub const Q_DICTIONARY: i8=99;

/// Type indicator of q sorted dictionary
pub const Q_SORTED_DICTIONARY: i8=127;

/// Type indicator of q error
pub const Q_ERROR: i8=-128;

/// Type indicator of q general null
pub const Q_GENERAL_NULL: i8=101;

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                             Type Alias                               //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

pub type S = *mut c_char;
pub type const_S = *const c_char; 
pub type C = c_char;
pub type G = c_uchar;
pub type H = c_short;
pub type I = c_int;
pub type J = c_longlong;
pub type E = c_float;
pub type F = c_double;
pub type V = c_void;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct U{
  guid: [G; 16]
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct k0_list_info{
  pub n: J,
  pub G0: [G; 1]
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union k0_inner{
  pub byte: G,
  pub short: H,
  pub int: I,
  pub long: J,
  pub real: E,
  pub float: F,
  pub symbol: S,
  pub table: *mut k0,
  pub list: k0_list_info
}

unsafe impl Send for k0_inner{}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct k0{
  pub m: c_schar,
  pub a: c_schar,
  pub qtype: c_schar,
  pub attribute: C,
  pub refcount: I,
  pub value: k0_inner
}

unsafe impl Send for k0{}

/// Struct representing q object.
pub type K = *mut k0;

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                             Define Macros                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Utility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Create `K` nullptr. This value is used as general null returned value (`(::)`).
/// # Example
/// ```
/// use kdb_c_api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn vanity(_: K) -> K{
///   println!("Initialized something, probably it is your mindset.");
///   KNULL!()
/// }
/// ```
#[macro_export]
macro_rules! KNULL {
  () => {
    std::ptr::null_mut::<k0>()
  };
}

/// Convert `&str` to `S`.
/// # Example
/// ```no_run
/// use kdb_c_api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn bigbang(_: K) -> K{
///   unsafe{ks(str_to_S!("super_illusion"))}
/// }
/// ```
/// ```q
/// q)bigbang: `libc_api_examples 2: (`bigbang; 1);
/// q)bigbang[]
/// `super_illusion
/// ```
/// # Note
/// This macro cannot be created as a function due to freeing resource of Rust (not sure).
#[macro_export]
macro_rules! str_to_S {
  ($string: expr) => {
    [$string.as_bytes(), &[b'\0']].concat().as_mut_ptr() as S
  };
}

/*
#[macro_export]
macro_rules! kG {
  ($qstruct: expr) => {
    *((*$qstruct).value.list.G0)
  };
}

#[macro_export]
macro_rules! kU {
  ($qstruct: expr) => {
    std::mem::transmute::<[G; (*$qstruct).list.n], [U; (*$qstruct).list.n]>(kG!($qstruct))
  };
}

#[macro_export]
macro_rules! kH {
  ($qstruct: expr) => {
    std::mem::transmute::<[G; (*$qstruct).list.n], [H; (*$qstruct).list.n]>(kG!($qstruct))  
  };
}

#[macro_export]
macro_rules! kI {
  ($qstruct: expr) => {
    std::mem::transmute::<[G; (*$qstruct).list.n], [I; (*$qstruct).list.n]>(kG!($qstruct))
  };
}

#[macro_export]
macro_rules! kJ {
  ($qstruct: expr) => {
    std::mem::transmute::<[G; (*$qstruct).list.n], [J; (*$qstruct).list.n]>(kG!($qstruct))
  };
}

#[macro_export]
macro_rules! kE {
  ($qstruct: expr) => {
    std::mem::transmute::<[G; (*$qstruct).list.n], [E; (*$qstruct).list.n]>(kG!($qstruct))
  };
}

#[macro_export]
macro_rules! kF {
  ($qstruct: expr) => {
    std::mem::transmute::<[G; (*$qstruct).list.n], [F; (*$qstruct).list.n]>(kG!($qstruct))
  };
}

#[macro_export]
macro_rules! kS {
  ($qstruct: expr) => {
    std::mem::transmute::<[G; (*$qstruct).list.n], [S; (*$qstruct).list.n]>(kG!($qstruct))
  };
}

#[macro_export]
macro_rules! kC {
  ($qstruct: expr) => {
    std::mem::transmute::<[G; (*$qstruct).list.n], [C; (*$qstruct).list.n]>(kG!($qstruct))
  };
}

#[macro_export]
macro_rules! kK {
  ($qstruct: expr) => {
    std::mem::transmute::<[G; (*$qstruct).list.n], [K; (*$qstruct).list.n]>(kG!($qstruct))
  };
}
*/

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          External C Functions                        //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

extern "C"{

  //%% Constructors %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

  /// Creates an atom of the specified type.
  /// # Note
  /// Not sure how to use this... 
  pub fn ka(qtype: I) -> K;

  /// Constructor of q bool object.
  pub fn kb(boolean: I) -> K;

  /// Constructor of q GUID object.
  pub fn ku(array: U) -> K;

  /// Constructor of q byte object.
  pub fn kg(byte: I) -> K;

  /// Constructor of q short object.
  pub fn kh(short: I) -> K;

  /// Constructor of q int object.
  pub fn ki(int: I) -> K;

  /// Constructor of q long object.
  pub fn kj(long: J) -> K;

  /// Constructor of q real object.
  pub fn ke(real: F) -> K;

  /// Constructor of q float object.
  pub fn kf(float: F) -> K;

  ///  Constructor of q char object.
  pub fn kc(character: I) -> K;

  /// Constructor of q symbol object.
  pub fn ks(symbol: S) -> K;

  /// Constructor of q timestamp or timespan object.
  pub fn ktj(qtype: I, nanoseconds: J) -> K;

  /// Constructor of q date object.
  pub fn kd(date: I) -> K;

  /// Constructor of q datetime object.
  pub fn kz(datetime: F) -> K;

  /// Constructor of q time object.
  pub fn kt(time: I) -> K;

  /// Constructor of q keyed table object.
  pub fn knt(keynum: J, table: K) -> K;

  /// Constructor of simple q table object from q keyed table object.
  pub fn ktd(keyedtable: K) -> K;

  /// Constructor of q compound list.
  pub fn knk(qtype: I, ...) -> K;
  
  /// Constructor of q simple list.
  pub fn ktn(qtype: I, length: J) -> K;
  
  /// Constructor of q string object.
  pub fn kp(chararray: S) -> K;

  /// Constructor if q string object with a fixed length.
  pub fn kpn(chararray: S, length: J) -> K;

  /// Constructor of q table object from q dictionary object.
  pub fn xT(dictionary: K) -> K;

  /// Constructor of q dictionary object.
  pub fn xD(keys: K, values: K) -> K;

  /// Constructor of q error.
  pub fn krr(message: const_S) -> K;

  /// Appends a system-error message to string S before passing it to `krr`.
  pub fn orr(message: const_S) -> K;

  /// Appends a raw value to a list.
  ///  `list` points to a `K` object, which may be reallocated during the function.
  ///  The contents of `list`, i.e. *x, will be updated in case of reallocation. 
  ///  Returns a pointer to the (potentially reallocated) `K` object.
  pub fn ja(list: *mut K, value: *const V) -> K;

  /// Appends a q list object to a q list.
  ///  Returns a pointer to the (potentially reallocated) `K` object.
  pub fn jv(list1: *mut K, list2: K) -> K;

  /// Appends a q object to a q list.
  ///  Returns a pointer to the (potentially reallocated) `K` object.
  pub fn jk(list: *mut K, value: K) -> K;

  /// Appends an interned char array to symbol list.
  ///  Returns a pointer to the (potentially reallocated) `K` object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_symbol_list(_: K) -> K{
	///   unsafe{
	///	    let mut list=ktn(Q_SYMBOL as i32, 0);
	///	    js(&mut list, ss(str_to_S!("Abraham")));
	///   	js(&mut list, ss(str_to_S!("Isaac")));
	///   	js(&mut list, ss(str_to_S!("Jacob")));
	///   	list
	///   }
  /// }
  /// ```
  /// ```q
  /// q)summon_symbol_list:`libc_api_examples 2: (`create_symbol_list; 1)
  /// q)summon_symbol_list[]
  /// `Abraham`Isaac`Jacob
  /// q)`Abraham`Isaac`Jacob ~ summon_symbol_list[]
  /// ```
  pub fn js(list: *mut K, symbol: S) -> K;

  /// Intern `n` chars from a char array.
  ///  Returns an interned char array and should be used to add char array to a symbol vector.
  pub fn sn(string: S, n: I) -> S;

  /// Intern a null-terminated char array.
  ///  Returns an interned char array and should be used to add char array to a symbol vector.
  /// # Example
  /// See the example of [`js`](function.js).
  pub fn ss(string: S) -> S;

  /// Capture (and reset) error string into usual error object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// extern "C"{
  ///   fn catchy(func: K, args: K) -> K{
  ///     let result=unsafe{ee(dot(func, args))};
  ///     if (*result).qtype == -128{
  ///       println!("error: {}", (*result).symbol);
  ///       return KNULL;
  ///     }
  ///     else{
  ///       result
  ///     }
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)catchy: `libc_api_examples 2: (`catchy; 2);
  /// q)catchy[$; ("J"; "42")]
  /// 42
  /// q)catchy[+; (1; `a)]
  /// error: type
  /// ```
  pub fn ee(result: K) -> K;

  //%% IPC Functions %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

  /// Send a text query or evaluate the text query in a process which are loading the shared library.
  ///  As this library is purposed to build shared object, the only choice of `handle` is `0`. This
  ///  executes against the kdb+ process in which it is loaded.
  /// ```
  /// use kdb_c_api::*;
  /// 
  /// pub extern "C" fn to_table() -> K{
  ///   let mut dicts=knk(3);
  ///   (*dicts).list.n=0;
  ///   for _ in 0..3{
  ///     let keys=ktn(Q_SYMBOL, 2);
  ///     
  ///   }
  ///   jk(&mut dicts, 
  /// }
  /// ```
  pub fn k(handle: I, query: const_S,...) -> K;

  /// Serialize q object and return serialized q byte list object on success: otherwise null. 
  ///  Mode is either of:
  /// - -1: Serialize within the same process.
  /// - 1: retain enumerations, allow serialization of timespan and timestamp: Useful for passing data between threads
  /// - 2: unenumerate, allow serialization of timespan and timestamp
  /// - 3: unenumerate, compress, allow serialization of timespan and timestamp
  pub fn b9(mode: I, qobject: K) -> K;

  /// Deserialize a bytes into q object.
  /// # Note
  /// On success, returns deserialized K object. On error, NULL is returned; use `ee` to retrieve the error string.
  pub fn d9(bytes: K) -> K;

  /// Verify that the received bytes is a valid IPC message.
  ///  The message is not modified.
  ///  Returns `0` if not valid.
  /// # Note
  /// Decompressed data only.
  pub fn okx(bytes: K) -> I;

  //%% Callback %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

  /// Remove callback from the associated kdb+ handle and call `kclose`.
  ///  Return null if the handle is invalid or not the one which had been registered by `sd1`.
  pub fn sd0(handle: I) -> V;

  /// Remove callback from the associated kdb+ handle and call `kclose` if the given condition is satisfied.
  ///  Return null if the handle is invalid or not the one which had been registered by `sd1`.
  pub fn sd0x(handle: I, condition: I) -> V;

  /// Register callback to the associated kdb+ handle.
  pub fn sd1(handle: I, function: extern fn(I) -> K) -> K;

  //%% Reference Count %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

  /// Decrement reference count of the q object.
  pub fn r0(qobject: K) -> V;

  /// Increment reference count of the q object.
  pub fn r1(qobject: K) -> K;

  //%% Miscellaneous %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

  /// Apply a function to q list object `.[func; args]`.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn rust_parse(dollar: K, type_and_text: K) -> K{
  ///   unsafe{
  ///     dot(dollar, type_and_text)
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)rust_parse:`somelib 2: (`rust_parse; 2);
  /// q)rust_parse[$; ("S"; "text")]
  /// `text
  /// ```
  pub fn dot(func: K, args: K) -> K;

  /// Release the memory allocated for the thread's pool.
  ///  Call when the thread is about to complete, releasing the memory allocated for that thread's pool.
  pub fn m9() -> V;

  /// Set whether interning symbols uses a lock: `lock` is either 0 or 1.
  ///  Returns the previously set value.
  /// # Example
  /// ```
  ///  // Not
  /// ```
  pub fn setm(lock: I) -> I;

  /// Convert ymd to days from `2000.01.01`.
  /// # Example
  /// ```
  /// use kdb_c_api::*;
  /// 
  /// fn main(){
  /// 
  ///   let days=unsafe{ymd(2020, 4, 1)};
  ///   assert_eq!(days, 7396);
  /// 
  /// }
  /// ```
  pub fn ymd(year: I, month: I, date:I) -> I;

  /// Convert days from `2000.01.01` to a number expressed as `yyyymmdd`.
  /// # Example
  /// ```
  /// use kdb_c_api::*;
  /// 
  /// fn main(){
  /// 
  ///   let number=unsafe{dj(7396)};
  ///   assert_eq!(number, 20200401);
  /// 
  /// }
  /// ```
  pub fn dj(days: I) -> I;

  /* Unsupported

  /// Return a dictionary of TLS setting. See `-26!`.
  /// # Note
  /// As this library is purposed to build shared object, this function will not add a value.
  pub fn sslInfo(_: K) -> K;

  /// Return kdb+ release date.
  /// # Note
  /// This function seems not exist (`undefined symbol`).
  pub fn ver() -> I;

  /// Function takes a C function that would take `n` `K` objects as arguments and returns a `K` object.
  ///  Returns a q function.
  /// # Note
  /// As this library is purposed to build shared object, this function will not add a value. User can
  ///  use `2:` instead.
  pub fn dl(func: *const V, n: J) -> K;
  
  /// Variadic version of `knk`.
  fn vaknk(qtype: I, args: va_list) -> K;

  /// Variadic version of `k`.
  fn vak(qtype: I, query: const_S, args: va_list) -> K;
  
  */
}

/// Convert `S` to `&str`.
/// # Extern
/// ```no_run
/// #[macro_use]
/// extern crate kdb_c_api;
/// 
/// use kdb_c_api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn print_symbol(symbol: K) -> K{
///   unsafe{
///     if (*symbol).qtype == -Q_SYMBOL{
///       println!("symbol: `{}", S_to_str((*symbol).value.symbol));
///     }
///     // return null
///     KNULL!()
///   }
/// }
/// ```
/// ```q
/// q)print_symbol:`libc_api_examples 2: (`print_symbol; 1)
/// q)a:`kx
/// q)print_symbol a
/// symbol: `kx
/// ```
pub fn S_to_str<'a>(cstring: S) -> &'a str{
  unsafe{
    CStr::from_ptr(cstring).to_str().unwrap()
  }
}

/// Convert `&str` to `S`.
/// # Example
/// ```no_run
/// use kdb_c_api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn bigbang2(_: K) -> K{
///   unsafe{ks(str_to_S("super_illusion\0"))}
/// }
/// ```
/// ```q
/// q)bigbang: `libc_api_examples 2: (`bigbang2; 1);
/// q)bigbang[]
/// `super_illusion
/// ```
pub extern "C" fn null_terminated_str_to_S(string: &str) -> S {
  unsafe{
    CStr::from_bytes_with_nul_unchecked(string.as_bytes()).as_ptr() as S
  }
}


/// Convert null terminated `&str` into `const_S`.
/// # Example
/// ```
/// #[macro_use]
/// extern crate kdb_c_api;
/// 
/// use kdb_c_api::*;
/// 
/// pub extern "C" fn must_be_int2(obj: K) -> K{
///   unsafe{
///     if (*obj).qtype != -Q_INT{
///       krr(str_to_const_S("not an int"))
///     }
///     else{
///       KNULL!()
///     }
///   }
/// }
/// ```
/// ```q
/// q)check:`libc_api_examples 2: (`must_be_int; 1)
/// q)a:100
/// q)check a
/// 'not an int
///   [0]  check a
///        ^
/// q)a:42i
/// q)check a
/// ```
pub extern "C" fn str_to_const_S(string: &str) -> const_S {
  string.as_bytes().as_ptr() as const_S
}
