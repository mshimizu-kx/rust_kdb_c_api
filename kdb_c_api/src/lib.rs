//! Rust crate mirroring the C API header file (`k.h`) for kdb+. The expected usage is to build a
//!  shared library for kdb+ in Rust. For a client library, see [rustkdb](https://github.com/KxSystems/rustkdb).
//! 
//! # Note
//! - This library is for kdb+ version 3.0+.
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

pub mod qtype{
  //! This module provides a list of q types. The motivation to contain them in a module is to 
  //!  tie them up as related items rather than scattered values. Hence user should use these
  //!  indicators with `qtype::` prefix, e.g., `qtype::BOOL`.
  
  /// Type indicator of q mixed list.
  ///  Access function: `kK`
  pub const COMPOUND: i8=0;
  /// Type indicator of q bool.
  ///  Access fucntion: `kG`
  pub const BOOL: i8=1;
  /// Type indicator of q GUID.
  ///  Access function: `kU`
  pub const GUID: i8=2;
  /// Type indicator of q byte
  ///  Access function: `kG`
  pub const BYTE: i8=4;
  /// Type indicator of q short.
  ///  Access function: `kH`
  pub const SHORT: i8=5;
  /// Type indicator of q int.
  ///  Access function: `kI`
  pub const INT: i8=6;
  /// Type indicator of q long.
  ///  Access function: `kJ`
  pub const LONG: i8=7;
  /// Type indicator of q real.
  ///  Access function: `kE`
  pub const REAL: i8=8;
  /// Type indicator of q float.
  ///  Access function: `kF`
  pub const FLOAT: i8=9;
  /// Type indicator of q char.
  ///  Access function: `kC`
  pub const CHAR: i8=10;
  /// Type indicator of q symbol.
  ///  Access function: `kS`
  pub const SYMBOL: i8=11;
  /// Type indicator of q timestamp.
  ///  Access function: `kJ`
  pub const TIMESTAMP: i8=12;
  /// Type indicator of q month.
  ///  Access function: `kI`
  pub const MONTH: i8=13;
  /// Type indicator of q date.
  ///  Access function: `kI`
  pub const DATE: i8=14;
  /// Type indicator of q datetime.
  ///  Access function: `kF`
  pub const DATETIME: i8=15;
  /// Type indicator of q timespan.
  ///  Access function: `kJ`
  pub const TIMESPAN: i8=16;
  /// Type indicator of q minute.
  ///  Access function: `kI`
  pub const MINUTE: i8=17;
  /// Type indicator of q second.
  ///  Access function: `kI`
  pub const SECOND: i8=18;
  /// Type indicator of q time.
  ///  Access function: `kI`
  pub const TIME: i8=19;
  /// Type indicator of q table.
  ///  `*(qstruct).k` is q dictionary.
  pub const Table: i8=98;
  /// Type indicator of q dictionary.
  /// - `kK(x)[0]`: keys
  /// - `kK(x)[1]`: values
  pub const DICTIONARY: i8=99;
  /// Type indicator of q sorted dictionary
  pub const SORTED_DICTIONARY: i8=127;
  /// Type indicator of q error
  pub const ERROR: i8=-128;
  /// Type indicator of q general null
  pub const NULL: i8=101;

}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                                Macros                                //
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

/// Convert `&str` to `S` (null-terminated character array).
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

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                               Structs                                //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Alias %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// `char*` in C. Also used to access symbol of q.
pub type S = *mut c_char;
/// `const char*` in C.
pub type const_S = *const c_char; 
/// `char` in C. Also used to access char of q.
pub type C = c_char;
/// `unsigned char` in C. Also used to access byte of q.
pub type G = c_uchar;
/// `i16` in C. Also used to access short of q.
pub type H = c_short;
/// `i32` in C. Also used to access int and compatible types (month, date, minute, second and time) of q.
pub type I = c_int;
/// `i64` in C. Also used to access long and compatible types (timestamp and timespan) of q.
pub type J = c_longlong;
/// `f32` in C. Also used to access real of q.
pub type E = c_float;
/// `f64` in C. Also used to access float and datetime of q.
pub type F = c_double;
/// `void` in C.
pub type V = c_void;

//%% U %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Struct representing 16-bytes GUID.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct U{
  guid: [G; 16]
}

//%% K %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Underlying list value of q object.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct k0_list_info{
  /// Length of the list.
  pub n: J,
  /// Pointer referring to the head of the list. This pointer will be interpreted
  ///  as various types when accessing `K` object to edit the list.
  pub G0: [G; 1]
}

/// Underlying atom value of q object.
#[derive(Clone, Copy)]
#[repr(C)]
pub union k0_inner{
  /// Byte type holder.
  pub byte: G,
  /// Short type holder.
  pub short: H,
  /// Int type holder.
  pub int: I,
  /// Long type older.
  pub long: J,
  /// Real type holder.
  pub real: E,
  /// Float type holder.
  pub float: F,
  /// Symbol type holder.
  pub symbol: S,
  /// Table type holder.
  pub table: *mut k0,
  /// List type holder.
  pub list: k0_list_info
}

/// Underlying struct of `K` object.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct k0{
  /// For internal usage. 
  pub m: c_schar,
  /// For internal usage.
  pub a: c_schar,
  /// Type indicator.
  pub qtype: c_schar,
  /// Attribute of list.
  pub attribute: C,
  /// Reference count of the object.
  pub refcount: I,
  /// Underlying value.
  pub value: k0_inner
}

/// Struct representing q object.
pub type K=*mut k0;

//%% KList %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

pub trait KUtility{
  /// Derefer `K` as a mutable slice of the specified type. The supported types are:
  /// - `G`: Equivalent to C API macro `kG`.
  /// - `H`: Equivalent to C API macro `kH`.
  /// - `I`: Equivalent to C API macro `kI`.
  /// - `J`: Equivalent to C API macro `kJ`.
  /// - `E`: Equivalent to C API macro `kE`.
  /// - `F`: Equivalent to C API macro `kF`.
  /// - `C`: Equivalent to C API macro `kC`.
  /// - `S`: Equivalent to C API macro `kS`.
  /// - `K`: Equivalent to C API macro `kK`.
  /// # Example
  /// ```
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn modify_long_list_a_bit(long_list: K) -> K{
  ///   if long_list.len() >= 2{
  ///     // Derefer as a mutable i64 slice.
  ///     long_list.as_mut_slice::<J>()[1]=30000_i64;
  ///     // Increment the counter for reuse on q side.
  ///     r1(long_list)
  ///   }
  ///   else{
  ///     krr(null_terminated_str_to_const_S("this list is not long enough. how ironic...\0"))
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)ironic: `libc_api_examples 2: (`modify_long_list_a_bit; 1);
  /// q)list:1 2 3;
  /// q)ironic list
  /// 1 30000 3
  /// q)ironic enlist 1
  /// ```
  /// # Note
  /// Intuitively the parameter should be `&mut self` but it restricts a manipulating
  ///  `K` objects in the form of slice simultaneously. As copying a pointer is not
  ///  an expensive operation, using `self` should be fine.
  fn as_mut_slice<'a, T>(self) -> &'a mut[T];

  /// Get an underlying q byte.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn print_byte(atom: K) -> K{
  ///   match atom.get_byte(){
  ///     Ok(byte) => {
  ///       println!("byte: {:#4x}", byte);
  ///       KNULL!()
  ///     },
  ///     Err(error) => unsafe{krr(null_terminated_str_to_const_S(error))}
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)print_byte: LIBPATH_ (`print_byte; 1);
  /// q)print_byte[0xc4]
  /// byte: 0xc4
  /// ```
  fn get_byte(&self) -> Result<u8, &'static str>;

  /// Get an underlying q short.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn print_short(atom: K) -> K{
  ///   match atom.get_short(){
  ///     Ok(short) => {
  ///       println!("short: {}", short);
  ///       KNULL!()
  ///     },
  ///     Err(error) => unsafe{krr(null_terminated_str_to_const_S(error))}
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)print_short: LIBPATH_ (`print_short; 1);
  /// q)print_short[10h]
  /// short: 10
  /// ```
  fn get_short(&self) -> Result<i16, &'static str>;

  /// Get an underlying q int.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn print_int(atom: K) -> K{
  ///   match atom.get_int(){
  ///     Ok(int) => {
  ///       println!("int: {}", int);
  ///       KNULL!()
  ///     },
  ///     Err(error) => unsafe{krr(null_terminated_str_to_const_S(error))}
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)print_int: LIBPATH_ (`print_int; 1);
  /// q)print_int[03:57:20]
  /// int: 14240
  /// ```
  fn get_int(&self) -> Result<i32, &'static str>;

  /// Get an underlying q long.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn print_long(atom: K) -> K{
  ///   match atom.get_long(){
  ///     Ok(int) => {
  ///       println!("long: {}", long);
  ///       KNULL!()
  ///     },
  ///     Err(error) => unsafe{krr(null_terminated_str_to_const_S(error))}
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)print_long: LIBPATH_ (`print_long; 1);
  /// q)print_long[2000.01.01D12:00:00.123456789]
  /// long: 43200123456789
  /// ```
  fn get_long(&self) -> Result<i64, &'static str>;

  /// Get an underlying q real.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn print_real(atom: K) -> K{
  ///   match atom.get_real(){
  ///     Ok(real) => {
  ///       println!("real: {}", real);
  ///       KNULL!()
  ///     },
  ///     Err(error) => unsafe{krr(null_terminated_str_to_const_S(error))}
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)print_real: LIBPATH_ (`print_real; 1);
  /// q)print_real[193810.32e]
  /// real: 193810.31
  /// ```
  fn get_real(&self) -> Result<f32, &'static str>;

  /// Get an underlying q float.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn print_float(atom: K) -> K{
  ///   match atom.get_float(){
  ///     Ok(float) => {
  ///       println!("float: {:.8}", float);
  ///       KNULL!()
  ///     },
  ///     Err(error) => unsafe{krr(null_terminated_str_to_const_S(error))}
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)print_float: LIBPATH_ (`print_float; 1);
  /// q)print_float[2002.01.12T10:03:45.332]
  /// float: 742.41927468
  /// ```
  fn get_float(&self) -> Result<f64, &'static str>;

  /// Get an underlying q char.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn print_char(atom: K) -> K{
  ///   match atom.get_char(){
  ///     Ok(character) => {
  ///       println!("char: \"{}\"", character);
  ///       KNULL!()
  ///     },
  ///     Err(error) => unsafe{krr(null_terminated_str_to_const_S(error))}
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)print_char: LIBPATH_ (`print_char; 1);
  /// q)print_char["k"]
  /// char: "k"
  /// ```
  fn get_char(&self) -> Result<char, &'static str>;

  /// Get an underlying q symbol.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn print_symbol2(atom: K) -> K{
  ///   match atom.get_symbol(){
  ///     Ok(symbol) => {
  ///       println!("symbol: `{}", symbol);
  ///       KNULL!()
  ///     },
  ///     Err(error) => unsafe{krr(null_terminated_str_to_const_S(error))}
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)print_symbol2: LIBPATH_ (`print_symbol2; 1);
  /// q)print_symbol2[`locust]
  /// symbol: `locust
  /// ```
  fn get_symbol(&self) -> Result<&str, &'static str>;

  /// Get an underlying q string.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn print_string(string: K) -> K{
  ///   match atom.get_string(){
  ///     Ok(string_) => {
  ///       println!("string: \"{}\"", string_);
  ///       KNULL!()
  ///     },
  ///     Err(error) => unsafe{krr(null_terminated_str_to_const_S(error))}
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)print_string: LIBPATH_ (`print_string; 1);
  /// q)print_string["grasshopper"]
  /// string: "grasshopper"
  /// ```
  fn get_string(&self) -> Result<&str, &'static str>;

  /// Get a length of the list. More specifically, a value of `k0.value.list.n` for list types.
  ///  Otherwise 2 for table and 1 for atom and null.
  /// # Example
  /// See the example of [`as_mut_slice`](trait.KUtility.html#tymethod.as_mut_slice).
  fn len(&self) -> i64;

  /// Get a type of `K` object.
  fn get_type(&self) -> i8;
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                            Implementation                            //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% U %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

impl U{
  /// Create 16-byte GUID object.
  pub fn new(guid: [u8; 16]) -> Self{
    U{guid:guid}
  }
}

//%% K %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

unsafe impl Send for k0_inner{}
unsafe impl Send for k0{}

impl KUtility for K{
  fn as_mut_slice<'a, T>(self) -> &'a mut[T]{
    unsafe{
      std::slice::from_raw_parts_mut((*self).value.list.G0.as_mut_ptr() as *mut T, (*self).value.list.n as usize)
    }
  }

  fn get_byte(&self) -> Result<u8, &'static str>{
    unsafe{
      match -(**self).qtype{
        qtype::BYTE => Ok((**self).value.byte),
        _ => Err("not a byte\0")
      }
    }
  }

  fn get_short(&self) -> Result<i16, &'static str>{
    unsafe{
      match -(**self).qtype{
        qtype::SHORT => Ok((**self).value.short),
        _ => Err("not a short\0")
      }
    }
  }

  fn get_int(&self) -> Result<i32, &'static str>{
    unsafe{
      match -(**self).qtype{
        qtype::INT | qtype::MONTH | qtype::DATE | qtype::MINUTE | qtype::SECOND | qtype::TIME => Ok((**self).value.int),
        _ => Err("not an int\0")
      }
    }
  }

  fn get_long(&self) -> Result<i64, &'static str>{
    unsafe{
      match -(**self).qtype{
        qtype::LONG | qtype::TIMESTAMP | qtype::TIMESPAN => Ok((**self).value.long),
        _ => Err("not a long\0")
      }
    }
  }

  fn get_real(&self) -> Result<f32, &'static str>{
    unsafe{
      match -(**self).qtype{
        qtype::REAL => Ok((**self).value.real),
        _ => Err("not a real\0")
      }
    }
  }

  fn get_float(&self) -> Result<f64, &'static str>{
    unsafe{
      match -(**self).qtype{
        qtype::FLOAT | qtype::DATETIME => Ok((**self).value.float),
        _ => Err("not a float\0")
      }
    }
  }

  fn get_char(&self) -> Result<char, &'static str>{
    unsafe{
      match -(**self).qtype{
        qtype::CHAR => Ok((**self).value.byte as char),
        _ => Err("not a char\0")
      }
    }
  }

  fn get_symbol(&self) -> Result<&str, &'static str>{
    unsafe{
      match -(**self).qtype{
        qtype::SYMBOL => {
          Ok(S_to_str((**self).value.symbol))
        },
        _ => Err("not a symbol\0")
      }
    }
  }

  fn get_string(&self) -> Result<&str, &'static str>{
    unsafe{
      match (**self).qtype{
        qtype::CHAR => {
          Ok(str::from_utf8_unchecked_mut(self.as_mut_slice::<G>()))
        },
        _ => Err("not a string\0")
      }
    }
  }

  fn len(&self) -> i64{
    unsafe{
      if (**self).qtype < 0 || (**self).qtype == qtype::NULL{
        // Atom or (::)
        1
      }
      else if (**self).qtype == qtype::Table{
        // In case of table it has K must access `table` (K) and it is a dictionary
        //  whose `value.list.n` is 2
        2
      }
      else{
        // List or dictionary
        (**self).value.list.n
      }
    }
  }

  fn get_type(&self) -> i8{
    unsafe{(**self).qtype}
  }
}


impl k0{
  /// Derefer `k0` as a mutable slice. For supported types, see [`as_mut_slice`](trait.KUtility.html#tymethod.as_mut_slice)
  /// # Note
  /// Used if `K` needs to be sent to another thread. `K` cannot implement `Send` and therefore
  ///  its inner struct must besent instead.
  /// # Example
  /// See the example of [`setm`](fn.setm.html).
  pub fn as_mut_slice<'a, T>(&mut self) -> &'a mut[T]{
    unsafe{
      std::slice::from_raw_parts_mut(self.value.list.G0.as_mut_ptr() as *mut T, self.value.list.n as usize)
    }
  }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                          External C Functions                        //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

extern "C"{

  //%% Constructors %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

  /// Creates an atom of the specified type.
  pub fn ka(qtype: I) -> K;

  /// Constructor of q bool object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_bool(_: K) -> K{
  ///   unsafe{kb(1)}
  /// }
  /// ```
  /// ```q
  /// q)yes: libc_api_examples (`create_bool; 1);
  /// q)yes[]
  /// 1b
  /// ```
  pub fn kb(boolean: I) -> K;

  /// Constructor of q GUID object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_guid(_: K) -> K{
  ///   unsafe{ku(U::new([0x1e_u8, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]))}
  /// }
  /// ```
  /// ```q
  /// q)create_guid: libc_api_examples (`create_guid; 1);
  /// q)create_guid[]
  /// 1e11170c-4224-252c-1c14-1e224d3d4624
  /// ```
  pub fn ku(array: U) -> K;

  /// Constructor of q byte object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_byte(_: K) -> K{
  ///   unsafe{kg(0x3c)}
  /// }
  /// ```
  /// ```q
  /// q)create_byte: libc_api_examples (`create_byte; 1);
  /// q)create_byte[]
  /// 0x3c
  /// ```
  pub fn kg(byte: I) -> K;

  /// Constructor of q short object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_short(_: K) -> K{
  ///   unsafe{kh(-144)}
  /// }
  /// ```
  /// ```q
  /// q)shortage: libc_api_examples (`create_short; 1);
  /// q)shortage[]
  /// -144h
  /// ```
  pub fn kh(short: I) -> K;

  /// Constructor of q int object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_int(_: K) -> K{
  ///   unsafe{ki(86400000)}
  /// }
  /// ```
  /// ```q
  /// q)trvial: libc_api_examples (`create_int; 1);
  /// q)trivial[]
  /// 86400000i
  /// ```
  pub fn ki(int: I) -> K;

  /// Constructor of q long object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_long(_: K) -> K{
  ///   unsafe{kj(-668541276001729000)}
  /// }
  /// ```
  /// ```q
  /// q)lengthy: libc_api_examples (`create_long; 1);
  /// q)lengthy[]
  /// -668541276001729000
  /// ```
  pub fn kj(long: J) -> K;

  /// Constructor of q real object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_real(_: K) -> K{
  ///   unsafe{ke(0.00324)}
  /// }
  /// ```
  /// ```q
  /// q)reality: libc_api_examples (`create_real; 1);
  /// q)reality[]
  /// 0.00324e
  /// ```
  pub fn ke(real: F) -> K;

  /// Constructor of q float object.
  /// # Example
  /// ```
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_float(_: K) -> K{
  ///   unsafe{kf(-6302.620)}
  /// }
  /// ```
  /// ```q
  /// q)coffee_float: libc_api_examples (`create_float; 1);
  /// q)coffee_float[]
  /// -6302.62
  /// ```
  pub fn kf(float: F) -> K;

  ///  Constructor of q char object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_char(_: K) -> K{
  ///   unsafe{kc('q' as I)}
  /// }
  /// ```
  /// ```q
  /// q)quiz: libc_api_examples (`create_char; 1);
  /// q)quiz[]
  /// "q"
  /// ```
  pub fn kc(character: I) -> K;

  /// Constructor of q symbol object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_symbol(_: K) -> K{
  ///   unsafe{ks(str_to_S!("symbolism"))}
  /// }
  /// ```
  /// ```q
  /// q)formal: libc_api_examples (`create_symbol; 1);
  /// q)formal[]
  /// `symbolism
  /// q)`symbolism ~ formal[]
  /// 1b
  /// ```
  pub fn ks(symbol: S) -> K;

  /// Constructor of q timestamp or timespan object.
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_timestamp(_: K) -> K{
  ///   // 2015.03.16D00:00:00:00.000000000
  ///   unsafe{ktj(-qtype::TIMESTAMP as I, 479779200000000000)}
  /// }
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_timespan(_: K) -> K{
  ///   // -1D01:30:00.001234567
  ///   unsafe{ktj(-qtype::TIMESPAN as I, -91800001234567)}
  /// }
  /// ```
  /// ```q
  /// q)hanko: libc_api_examples (`create_timestamp; 1);
  /// q)hanko[]
  /// 2015.03.16D00:00:00.000000000
  /// q)duration: libc_api_examples (`create_timespan; 1);
  /// q)duration[]
  /// -1D01:30:00.001234567
  /// ```
  pub fn ktj(qtype: I, nanoseconds: J) -> K;

  /// Constructor of q date object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_date(_: K) -> K{
  ///   // 1999.12.25
  ///   unsafe{kd(-7)}
  /// }
  /// ```
  /// ```q
  /// q)christmas_at_the_END: libc_api_examples (`create_date; 1);
  /// q)christmas_at_the_END[]
  /// 1999.12.25
  /// ```
  pub fn kd(date: I) -> K;

  /// Constructor of q datetime object.
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_datetime(_: K) -> K{
  ///   // 2015.03.16T12:00:00:00.000
  ///   unsafe{kz(5553.5)}
  /// }
  /// ```
  /// ```q
  /// q)omega_date: libc_api_examples (`create_datetime; 1);
  /// q)omega_date[]
  /// 2015.03.16T12:00:00.000
  /// ```
  pub fn kz(datetime: F) -> K;

  /// Constructor of q time object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_time(_: K) -> K{
  ///   // -01:30:00.123
  ///   unsafe{kt(-5400123)}
  /// }
  /// ```
  /// ```q
  /// q)ancient: libc_api_examples (`create_time; 1);
  /// q)ancient[]
  /// -01:30:00.123
  /// ```
  pub fn kt(milliseconds: I) -> K;

  /// Constructor of q compound list.
  /// # Example
  /// See the example of [`xD`](fn.xD.html).
  pub fn knk(qtype: I, ...) -> K;
  
  /// Constructor of q simple list.
  /// # Example
  /// See the example of [`xD`](fn.xD.html).
  pub fn ktn(qtype: I, length: J) -> K;
  
  /// Constructor of q string object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_string(_: K) -> K{
  ///   unsafe{kp(str_to_S!("this is a text."))}
  /// }
  /// ```
  /// ```q
  /// q)text: libc_api_examples (`create_string; 1);
  /// q)text[]
  /// "this is a text."
  /// ```
  pub fn kp(chararray: S) -> K;

  /// Constructor if q string object with a fixed length.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_string2(_: K) -> K{
  ///   unsafe{kpn(str_to_S!("The meeting was too long and I felt it s..."), 24)}
  /// }
  /// ```
  /// ```q
  /// q)speak_inwardly: libc_api_examples (`create_string2; 1);
  /// q)speak_inwardly[]
  /// "The meeting was too long"
  /// ```
  pub fn kpn(chararray: S, length: J) -> K;

  /// Constructor of q table object from q dictionary object.
  /// # Note
  /// Basically this is a `flip` command of q. Hence the value of the dictionary must have
  ///  lists as its elements.
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_table(_: K) -> K{
  ///   unsafe{
  ///     let keys=ktn(qtype::SYMBOL as I, 2);
  ///     let keys_slice=keys.as_mut_slice::<S>();
  ///     keys_slice[0]=ss(str_to_S!("time"));
  ///     keys_slice[1]=ss(str_to_S!("temperature"));
  ///     let values=knk(2);
  ///     let time=ktn(qtype::TIMESTAMP as I, 3);
  ///     // 2003.10.10D02:24:19.167018272 2006.05.24D06:16:49.419710368 2008.08.12D23:12:24.018691392
  ///     time.as_mut_slice::<J>().copy_from_slice(&[119067859167018272_i64, 201766609419710368, 271897944018691392]);
  ///     let temperature=ktn(qtype::FLOAT as I, 3);
  ///     temperature.as_mut_slice::<F>().copy_from_slice(&[22.1_f64, 24.7, 30.5]);
  ///     values.as_mut_slice::<K>().copy_from_slice(&[time, temperature]);
  ///     xT(xD(keys, values))
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)climate_change: libc_api_examples (`create_table; 1);
  /// q)climate_change[]
  /// time                          temperature
  /// -----------------------------------------
  /// 2003.10.10D02:24:19.167018272 22.1       
  /// 2006.05.24D06:16:49.419710368 24.7       
  /// 2008.08.12D23:12:24.018691392 30.5    
  /// ```
  pub fn xT(dictionary: K) -> K;

  /// Constructor of simple q table object from q keyed table object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_keyed_table(dummy: K) -> K{
  ///   unsafe{knt(1, create_table(dummy))}
  /// }
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn keyed_to_simple_table(dummy: K) -> K{
  ///   unsafe{ktd(create_keyed_table(dummy))}
  /// }
  /// ```
  /// ```q
  /// q)unkey: libc_api_examples (`keyed_to_simple_table; 1);
  /// q)unkey[]
  /// time                          temperature
  /// -----------------------------------------
  /// 2003.10.10D02:24:19.167018272 22.1       
  /// 2006.05.24D06:16:49.419710368 24.7       
  /// 2008.08.12D23:12:24.018691392 30.5    
  /// ```
  pub fn ktd(keyedtable: K) -> K;

  /// Constructor of q keyed table object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_keyed_table(dummy: K) -> K{
  ///   unsafe{knt(1, create_table(dummy))}
  /// }
  /// ```
  /// ```q
  /// q)locker: libc_api_examples (`create_keyed_table; 1);
  /// q)locker[]
  /// time                         | temperature
  /// -----------------------------| -----------
  /// 2003.10.10D02:24:19.167018272| 22.1       
  /// 2006.05.24D06:16:49.419710368| 24.7       
  /// 2008.08.12D23:12:24.018691392| 30.5  
  /// ```
  pub fn knt(keynum: J, table: K) -> K;

  /// Constructor of q dictionary object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn create_dictionary() -> K{
  ///   unsafe{
  ///     let keys=ktn(qtype::INT as I, 2);
  ///     keys.as_mut_slice::<I>()[0..2].copy_from_slice(&[0, 1]);
  ///     let values=knk(2);
  ///     let date_list=ktn(qtype::DATE as I, 3);
  ///     // 2000.01.01 2000.01.02 2000.01.03
  ///     date_list.as_mut_slice::<I>()[0..3].copy_from_slice(&[0, 1, 2]);
  ///     let string=kp(str_to_S!("I'm afraid I would crash the application..."));
  ///     values.as_mut_slice::<K>()[0..2].copy_from_slice(&[date_list, string]);
  ///     xD(keys, values)
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)create_dictionary: `libc_api_examples 2: (`create_dictionary; 1);
  /// q)create_dictionary[]
  /// 0| 2000.01.01 2000.01.02 2000.01.03
  /// 1| "I'm afraid I would crash the application..."
  /// ```
  pub fn xD(keys: K, values: K) -> K;

  /// Constructor of q error.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// pub extern "C" fn thai_kick(_: K) -> K{
  ///   unsafe{
  ///    krr(null_terminated_str_to_const_S("Thai kick unconditionally!!\0"))
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)monstrous: `libc_api_examples 2: (`thai_kick; 1);
  /// q)monstrous[]
  /// 'Thai kick unconditionally!!
  /// [0]  monstrous[]
  ///      ^
  /// ```
  pub fn krr(message: const_S) -> K;

  /// Similar to krr but this function appends a system-error message to string S before passing it to `krr`.
  pub fn orr(message: const_S) -> K;

  /// Appends a raw value to a list.
  ///  `list` points to a `K` object, which may be reallocated during the function.
  ///  The contents of `list`, i.e. `*list`, will be updated in case of reallocation. 
  ///  Returns a pointer to the (potentially reallocated) `K` object.
  /// # Note
  /// Not sure how to use this...
  pub fn ja(list: *mut K, value: *const V) -> K;

  /// Appends a q list object to a q list.
  ///  Returns a pointer to the (potentially reallocated) `K` object.
  /// ```no_run
  /// #[no_mangle]
  /// pub extern "C" fn concat_list(mut list1: K, list2: K) -> K{
  ///   unsafe{
  ///     jv(&mut list1, list2);
  ///     r1(list1)
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)glue: `libc_api_examples 2: (`concat_list; 2);
  /// q)glue[(::; `metals; `fire); ("clay"; 316)]
  /// ::
  /// `metals
  /// `fire
  /// "clay"
  /// 316
  /// q)glue[1 2 3; 4 5]
  /// 1 2 3 4 5
  /// q)glue[`a`b`c; `d`e]
  /// `a`b`c`d`e
  /// ```
  pub fn jv(list1: *mut K, list2: K) -> K;

  /// Appends a q object to a q list.
  ///  Returns a pointer to the (potentially reallocated) `K` object.
  /// # Example
  /// ```
  /// #[no_mangle]
  /// pub extern "C" fn create_compound_list(_: K) -> K{
  ///   unsafe{
  ///     let mut list=knk(0);
  ///     jk(&mut list, ks(str_to_S!("1st")));
  ///     jk(&mut list, ki(2));
  ///     jk(&mut list, kpn(str_to_S!("3rd"), "3rd".chars().count() as i64));
  ///     list
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)ranks: `libc_api_examples 2: (`create_compound_list; 1);
  /// q)ranks[]
  /// `1st
  /// 2i
  /// "3rd"
  /// ```
  /// # Note
  /// In this example we intentionally not allocated an array by `knk(0)` to use `jk` to make it grow.
  ///  When using `jk`, it accesses current value of `n` in `K`, so preallocating memory with `knk` and
  ///  then using `jk` will crash because `knk` initializes `n` with its argument. If you want to allocate
  ///  a memory in advance, use `knk` and then substitute a value after converting the `K` into a slice
  ///  with `as_mut_K_slice`.
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
  ///     let mut list=ktn(qtype::SYMBOL as I, 0);
  ///     js(&mut list, ss(str_to_S!("Abraham")));
  ///     js(&mut list, ss(str_to_S!("Isaac")));
  ///     js(&mut list, ss(str_to_S!("Jacob")));
  ///     js(&mut list, sn(str_to_S!("Josephine"), 6));
  ///     list
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)summon:`libc_api_examples 2: (`create_symbol_list; 1)
  /// q)summon[]
  /// `Abraham`Isaac`Jacob`Joseph
  /// q)`Abraham`Isaac`Jacob`Joseph ~ summon[]
  /// 1b
  /// ```
  /// # Note
  /// In this example we intentionally not allocated an array by `ktn(qtype::SYMBOL as I, 0)` to use `js`
  ///  to make it grow. When using `js`, it accesses current value of `n` in `K`, so preallocating memory
  ///  with `ktn` and then using `js` will crash because `ktn` initializes `n` with its argument. If you want
  ///  to allocate a memory in advance, use `ktn` and then substitute a value after converting the `K` into a
  ///  slice with `as_mut_symbol_slice`.
  pub fn js(list: *mut K, symbol: S) -> K;

  /// Intern `n` chars from a char array.
  ///  Returns an interned char array and should be used to add char array to a symbol vector.
  /// # Example
  /// See the example of [`js`](fn.js.html).
  pub fn sn(string: S, n: I) -> S;

  /// Intern a null-terminated char array.
  ///  Returns an interned char array and should be used to add char array to a symbol vector.
  /// # Example
  /// See the example of [`js`](fn.js.html).
  pub fn ss(string: S) -> S;

  /// Capture (and reset) error string into usual error object.
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// extern "C"{
  ///   fn catchy(func: K, args: K) -> K{
  ///     let result=unsafe{ee(dot(func, args))};
  ///     if (*result).qtype == qtype::ERROR{
  ///       println!("error: {}", (*result).symbol);
  ///       // Decrement reference count of the error object
  ///       r0(result);
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
  /// #[no_mangle]
  /// pub extern "C" fn dictionary_list_to_table() -> K{
  ///   unsafe{
  ///     let dicts=knk(3);
  ///     let dicts_slice=dicts.as_mut_slice::<K>();
  ///     for i in 0..3{
  ///       let keys=ktn(qtype::SYMBOL as I, 2);
  ///       let keys_slice=keys.as_mut_slice::<S>();
  ///       keys_slice[0]=ss(str_to_S!("a"));
  ///       keys_slice[1]=ss(str_to_S!("b"));
  ///       let values=ktn(qtype::INT as I, 4);
  ///       values.as_mut_slice::<I>()[0..2].copy_from_slice(&[i*10, i*100]);
  ///       dicts_slice[i as usize]=xD(keys, values);
  ///     }
  ///     // Format list of dictionary as a table. 
  ///     // ([] a: 0 10 20i; b: 0 100 200i)
  ///     k(0, str_to_S!("{[dicts] -1 _ dicts, (::)}"), dicts, KNULL!())
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)unfortunate_fact: `libc_api_examples 2: (`dictionary_list_to_table; 1);
  /// q)unfortunate_fact[]
  /// a  b  
  /// ------
  /// 0  0  
  /// 10 100
  /// 20 200
  /// ```
  pub fn k(handle: I, query: const_S,...) -> K;

  /// Serialize q object and return serialized q byte list object on success: otherwise null. 
  ///  Mode is either of:
  /// - -1: Serialize within the same process.
  /// - 1: retain enumerations, allow serialization of timespan and timestamp: Useful for passing data between threads
  /// - 2: unenumerate, allow serialization of timespan and timestamp
  /// - 3: unenumerate, compress, allow serialization of timespan and timestamp
  /// # Note
  /// Probably not used.
  pub fn b9(mode: I, qobject: K) -> K;

  /// Deserialize a bytes into q object.
  /// # Note
  /// - On success, returns deserialized `K` object. On error, `(K) 0` is returned; use `ee` to retrieve the error string.
  /// - Probably not used.
  pub fn d9(bytes: K) -> K;

  /// Remove callback from the associated kdb+ handle and call `kclose`.
  ///  Return null if the handle is invalid or not the one which had been registered by `sd1`.
  /// # Note
  /// A function which calls this function must be executed at the exit of the process.
  pub fn sd0(handle: I) -> V;

  /// Remove callback from the associated kdb+ handle and call `kclose` if the given condition is satisfied.
  ///  Return null if the handle is invalid or not the one which had been registered by `sd1`.
  /// # Note
  /// A function which calls this function must be executed at the exit of the process.
  pub fn sd0x(handle: I, condition: I) -> V;

  /// Register callback to the associated kdb+ handle.
  /// ```no_run
  /// use kdb_c_api::*;
  /// use std::ffi::c_void;
  /// use libc::send;
  /// 
  /// // Send asynchronous query to the q process which sent a query to the caller of this function.
  /// extern "C" fn counter(socket: I) -> K{
  ///   let extra_query="show `$\"Counter_punch!!\"".as_bytes();
  ///   let query_length=extra_query.len();
  ///   // header (8) + list header (6) + data length
  ///   let total_length=8+6+query_length;
  ///   // Buffer
  ///   let mut message: Vec<u8>=Vec::with_capacity(total_length);
  ///   // Little endian, async, uncompress, reserved
  ///   message.extend_from_slice(&[1_u8, 0, 0, 0]);
  ///   // Total message length
  ///   message.extend_from_slice(&(total_length as i32).to_le_bytes());
  ///   // Data type, attribute
  ///   message.extend_from_slice(&[10_u8, 0]);
  ///   // Length of data
  ///   message.extend_from_slice(&(query_length as i32).to_le_bytes());
  ///   // Data
  ///   message.extend_from_slice(extra_query);
  ///   // Send
  ///   unsafe{send(socket, message.as_slice().as_ptr() as *const c_void, total_length, 0)};
  ///   KNULL!()
  /// }
  ///
  /// #[no_mangle]
  /// pub extern "C" fn enable_counter(socket: K) -> K{
  ///   unsafe{
  ///     let result=sd1(socket.get_int().expect("oh no"), counter);
  ///     if result.get_type()== qtype::NULL || result.get_type()== qtype::ERROR{
  ///       return krr(null_terminated_str_to_const_S("Failed to hook\0"));
  ///     }
  ///     else{
  ///       KNULL!()
  ///     }
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)// process1
  /// q)enable_counter: `libc_api_examples 2: (`enable_counter; 1)
  /// q)\p 5000
  /// ```
  /// ```q
  /// q)// process2
  /// q)h:hopen `:unix://5000
  /// ```
  /// ```q
  /// q)// process1
  /// q).z.W
  /// 5|
  /// q)enable_counter[5i]
  /// ```
  /// ```q
  /// q)// process2
  /// q)h "1+2"
  /// `Counter_punch!!
  /// 3
  /// q)neg[h] "1+2"
  /// `Counter_punch!!
  /// ```
  pub fn sd1(handle: I, function: extern fn(I) -> K) -> K;

  //%% Reference Count %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

  /// Decrement reference count of the q object. The decrement must be done when `k` function gets an error
  ///  object whose type is `qtype::ERROR` and when you created an object but do not intend to return it to
  ///  q side. See details on [the reference page](https://code.kx.com/q/interfaces/c-client-for-q/#managing-memory-and-reference-counting).
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub ectern "C" fn idle_man(_: K)->K{
  ///   unsafe{
  ///     // Creare an int object.
  ///     let int=ki(777);
  ///     // Changed the mind. Discard it.
  ///     r0(int);
  ///   }
  ///   // Return null.
  ///   KNULL!()
  /// }
  /// ```
  /// ```q
  /// q)idle_man: libc_api_examples (`idle_man; 1);
  /// q)idle_man[]
  /// q)
  /// ```
  pub fn r0(qobject: K) -> V;

  /// Increment reference count of the q object. Increment must be done when you passed arguments
  ///  to Rust function and intends to return it to q side or when you pass some `K` objects to `k`
  ///  function and intend to use the parameter after the call.
  ///  See details on [the reference page](https://code.kx.com/q/interfaces/c-client-for-q/#managing-memory-and-reference-counting).
  /// # Example
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn pass_through_cave(pedestrian: K) -> K{
  ///   unsafe{
  ///     let item=k(0, str_to_S!("get_item1"), r1(pedestrian), KNULL!());
  ///     println!("What do you see, son of man?: {}", item.get_string().expect("oh no"));
  ///     r0(item);
  ///     let item=k(0, str_to_S!("get_item2"), r1(pedestrian), KNULL!());
  ///     println!("What do you see, son of man?: {}", item.get_string().expect("oh no"));
  ///     r0(item);
  ///     r1(pedestrian)
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)get_item1:{[man] "a basket of summer fruit"};
  /// q)get_item2:{[man] "boiling pot, facing away from the north"}
  /// q).capi.pass_through_cave[`son_of_man]
  /// What do you see, son of man?: a basket of summer fruit
  /// What do you see, son of man?: boiling pot, facing away from the north
  /// `son_of_man
  /// ```
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
  /// q)rust_parse:`libc_api_examples 2: (`rust_parse; 2);
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
  /// ```no_run
  /// use kdb_c_api::*;
  /// 
  /// #[no_mangle]
  /// pub extern "C" fn parallel_sym_change(list: K) -> K{
  ///   unsafe{
  ///     // `K` cannot have `Send` because it is a pointer but `k0` does.
  ///     let mut inner=*list;
  ///     // Lock symbol before creating an internal symbol on another thread.
  ///     setm(1);
  ///     let task=std::thread::spawn(move || {
  ///        inner.as_mut_slice::<S>()[0]=ss(str_to_S!("replaced"));
  ///        inner
  ///     });
  ///     list.as_mut_slice::<S>()[1]=ss(str_to_S!("symbolbol"));
  ///     match task.join(){
  ///       Err(_) => {
  ///         // Unlock.
  ///         setm(0);
  ///         krr(null_terminated_str_to_const_S("oh no"))
  ///       },
  ///       Ok(l) => {
  ///         // Unlock.
  ///         setm(0);
  ///         (*list)=l;
  ///         // Increment reference count for copy.
  ///         r1(list)
  ///       }
  ///     }
  ///   }
  /// }
  /// ```
  /// ```q
  /// q)paradise: `libc_api_examples 2: (`parallel_sym_change; 2);
  /// q)paradise[`a`b];
  /// `replaced`symbolbol
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

  /// Connect with timeout (millisecond) and capability. The value of capability is:
  /// - 1: 1TB limit
  /// - 2: use TLS
  /// Return value is either of:
  /// - 0   Authentication error
  /// - -1   Connection error
  /// - -2   Timeout error
  /// - -3   OpenSSL initialization failed
  /// # Note
  /// Standalone application only. Not for a shared library.
  pub fn khpunc(host: S, port: I, credential: S, timeout_millis: I, capability: I) -> I;

  /// Connect with timeout (millisecond).
  ///  Return value is either of:
  /// - 0   Authentication error
  /// - -1   Connection error
  /// - -2   Timeout error
  /// # Note
  /// Standalone application only. Not for a shared library.
  pub fn khpun(host: const_S, port: I, credential: const_S, timeout_millis: I) -> I;

  /// Connect with no timeout.
  pub fn khpu(host: const_S, port: I, credential: const_S) -> I;

  /// Connect anonymously.
  pub fn khp(host: const_S, port: I) -> I;

  /// Close the handle to a q process.
  /// # Note
  /// Standalone application only. Not for a shared library.
  pub fn kclose(handle: I) -> V;

  /// Verify that the received bytes is a valid IPC message.
  ///  The message is not modified.
  ///  Returns `0` if not valid.
  /// # Note
  /// Decompressed data only.
  pub fn okx(bytes: K) -> I;

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

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                              Utility                                 //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Utility %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Convert `S` to `&str`. This function is intended to convert symbol type (null-terminated char-array) to `str`.
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
///     if (*symbol).qtype == -qtype::SYMBOL{
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
///   unsafe{ks(null_terminated_str_to_S("super_illusion\0"))}
/// }
/// ```
/// ```q
/// q)bigbang: `libc_api_examples 2: (`bigbang2; 1);
/// q)bigbang[]
/// `super_illusion
/// ```
pub fn null_terminated_str_to_S(string: &str) -> S {
  unsafe{
    CStr::from_bytes_with_nul_unchecked(string.as_bytes()).as_ptr() as S
  }
}

/// Convert null terminated `&str` into `const_S`. Expected usage is to build
///  a q error with `krr`.
/// # Example
/// ```
/// #[macro_use]
/// extern crate kdb_c_api;
/// 
/// use kdb_c_api::*;
/// 
/// pub extern "C" fn must_be_int2(obj: K) -> K{
///   unsafe{
///     if (*obj).qtype != -qtype::INT{
///       krr(null_terminated_str_to_const_S("not an int\0"))
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
pub fn null_terminated_str_to_const_S(string: &str) -> const_S {
  string.as_bytes().as_ptr() as const_S
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                              Re-export                               //
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Constructor %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Create a month object. This is a complememtal constructor of
///  missing month type.
/// # Example
/// ```no_run
/// use kdb_c_api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_month(_: K) -> K{
///   // 2010.07m
///   new_month(126)
/// }
/// ```
/// ```q
/// q)create_month: libc_api_examples (`create_month; 1);
/// q)create_month[]
/// 2010.07m
/// ```
pub extern "C" fn new_month(months: I) -> K{
  unsafe{
    let month=ka(-qtype::MONTH as I);
    (*month).value.int=months;
    month
  }
}

/// Create a month object. This is a complememtal constructor of
///  missing minute type.
/// # Example
/// ```no_run
/// use kdb_c_api::*;
/// 
/// #[no_mangle]
/// pub extern "C" fn create_minute(_: K) -> K{
///   // 10:40
///   new_minute(640)
/// }
/// ```
/// ```q
/// q)minty: libc_api_examples (`create_minute; 1);
/// q)minty[]
/// 10:40
/// ```
pub extern "C" fn new_minute(minutes: I) -> K{
  unsafe{
    let minute=ka(-qtype::MINUTE as I);
    (*minute).value.int=minutes;
    minute
  }
}

/// Create a month object. This is a complememtal constructor of
///  missing second type.
/// # Example
/// ```no_run
/// #[no_mangle]
/// pub extern "C" fn create_second(_: K) -> K{
///   // -02:00:00
///   new_second(-7200)
/// }
/// ```
/// ```q
/// q)third: libc_api_examples (`create_second; 1);
/// q)third[]
/// -02:00:00
/// ```
pub extern "C" fn new_second(seconds: I) -> K{
  unsafe{
    let second=ka(-qtype::SECOND as I);
    (*second).value.int=seconds;
    second
  }
}
