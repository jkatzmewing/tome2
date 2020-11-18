#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(main)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#![feature(const_transmute)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]

#[macro_use]
extern crate c2rust_asm_casts;
#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod src {
pub mod birth;
pub mod bldg;
pub mod cave;
pub mod cmd1;
pub mod cmd2;
pub mod cmd3;
pub mod cmd4;
pub mod cmd5;
pub mod cmd6;
pub mod cmd7;
pub mod cmovie;
pub mod dungeon;
pub mod files;
pub mod generate;
pub mod gen_evol;
pub mod gen_maze;
pub mod gods;
pub mod help;
pub mod init1;
pub mod init2;
pub mod levels;
pub mod loadsave;
pub mod lua_bind;
pub mod main_gcu;
pub mod main_gtk2;
pub mod main;
pub mod main_sdl;
pub mod main_x11;
pub mod main_xaw;
pub mod melee1;
pub mod melee2;
pub mod modules;
pub mod monster1;
pub mod monster2;
pub mod monster3;
pub mod notes;
pub mod object1;
pub mod object2;
pub mod plots;
pub mod powers;
pub mod randart;
pub mod script;
pub mod skills;
pub mod spells1;
pub mod spells2;
pub mod squeltch;
pub mod status;
pub mod store;
pub mod tables;
pub mod traps;
pub mod util;
pub mod variable;
pub mod wild;
pub mod wizard1;
pub mod wizard2;
pub mod xtra1;
pub mod xtra2;
pub mod z_form;
pub mod z_rand;
pub mod z_term;
pub mod z_util;
pub mod z_virt;
} // mod src
