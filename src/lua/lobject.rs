use ::libc;
extern "C" {
    pub type TM;
    pub type lua_longjmp;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn vsprintf(_: *mut libc::c_char, _: *const libc::c_char,
                _: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char,
              _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn luaM_realloc(L: *mut lua_State, oldblock: *mut libc::c_void,
                    size: lint32) -> *mut libc::c_void;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_State {
    pub top: StkId,
    pub stack: StkId,
    pub stack_last: StkId,
    pub stacksize: libc::c_int,
    pub Cbase: StkId,
    pub errorJmp: *mut lua_longjmp,
    pub Mbuffer: *mut libc::c_char,
    pub Mbuffsize: size_t,
    pub rootproto: *mut Proto,
    pub rootcl: *mut Closure,
    pub roottable: *mut Hash,
    pub strt: stringtable,
    pub udt: stringtable,
    pub gt: *mut Hash,
    pub TMtable: *mut TM,
    pub last_tag: libc::c_int,
    pub refArray: *mut Ref,
    pub refSize: libc::c_int,
    pub refFree: libc::c_int,
    pub GCthreshold: libc::c_ulong,
    pub nblocks: libc::c_ulong,
    pub callhook: lua_Hook,
    pub linehook: lua_Hook,
    pub allowhooks: libc::c_int,
}
pub type lua_Hook
    =
    Option<unsafe extern "C" fn(_: *mut lua_State, _: *mut lua_Debug) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: *const libc::c_char,
    pub currentline: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub nups: libc::c_int,
    pub linedefined: libc::c_int,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub short_src: [libc::c_char; 60],
    pub _func: *mut lua_TObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_TObject {
    pub ttype: libc::c_int,
    pub value: Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub ts: *mut TString,
    pub cl: *mut Closure,
    pub a: *mut Hash,
    pub i: *mut CallInfo,
    pub n: Number,
}
pub type Number = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: *mut Closure,
    pub pc: *mut *const Instruction,
    pub lastpc: libc::c_int,
    pub line: libc::c_int,
    pub refi: libc::c_int,
}
pub type Instruction = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Closure {
    pub f: C2RustUnnamed_0,
    pub next: *mut Closure,
    pub mark: *mut Closure,
    pub isC: libc::c_short,
    pub nupvalues: libc::c_short,
    pub upvalue: [TObject; 1],
}
pub type TObject = lua_TObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub c: lua_CFunction,
    pub l: *mut Proto,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Proto {
    pub knum: *mut Number,
    pub nknum: libc::c_int,
    pub kstr: *mut *mut TString,
    pub nkstr: libc::c_int,
    pub kproto: *mut *mut Proto,
    pub nkproto: libc::c_int,
    pub code: *mut Instruction,
    pub ncode: libc::c_int,
    pub numparams: libc::c_short,
    pub is_vararg: libc::c_short,
    pub maxstacksize: libc::c_short,
    pub marked: libc::c_short,
    pub next: *mut Proto,
    pub lineinfo: *mut libc::c_int,
    pub nlineinfo: libc::c_int,
    pub nlocvars: libc::c_int,
    pub locvars: *mut LocVar,
    pub lineDefined: libc::c_int,
    pub source: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub u: C2RustUnnamed_1,
    pub len: size_t,
    pub nexthash: *mut TString,
    pub marked: libc::c_int,
    pub str_0: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: C2RustUnnamed_3,
    pub d: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tag: libc::c_int,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub hash: libc::c_ulong,
    pub constindex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
pub type lua_CFunction
    =
    Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub node: *mut Node,
    pub htag: libc::c_int,
    pub size: libc::c_int,
    pub firstfree: *mut Node,
    pub next: *mut Hash,
    pub mark: *mut Hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub key: TObject,
    pub val: TObject,
    pub next: *mut Node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ref {
    pub o: TObject,
    pub st: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable {
    pub size: libc::c_int,
    pub nuse: lint32,
    pub hash: *mut *mut TString,
}
pub type lint32 = libc::c_ulong;
pub type StkId = *mut TObject;
/*
** $Id: lobject.c,v 1.3 2001/11/26 23:00:24 darkgod Exp $
** Some generic functions over Lua objects
** See Copyright Notice in lua.h
*/
#[no_mangle]
pub static mut luaO_nilobject: TObject =
    {
        let mut init =
            lua_TObject{ttype: 1 as libc::c_int,
                        value:
                            Value{ts: 0 as *const TString as *mut TString,},};
        init
    };
#[no_mangle]
pub static mut luaO_typenames: [*const libc::c_char; 6] =
    [b"userdata\x00" as *const u8 as *const libc::c_char,
     b"nil\x00" as *const u8 as *const libc::c_char,
     b"number\x00" as *const u8 as *const libc::c_char,
     b"string\x00" as *const u8 as *const libc::c_char,
     b"table\x00" as *const u8 as *const libc::c_char,
     b"function\x00" as *const u8 as *const libc::c_char];
/*
** returns smaller power of 2 larger than `n' (minimum is MINPOWER2) 
*/
#[no_mangle]
pub unsafe extern "C" fn luaO_power2(mut n: lint32) -> lint32 {
    let mut p: lint32 = 4 as libc::c_int as lint32;
    while p <= n { p <<= 1 as libc::c_int }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_equalObj(mut t1: *const TObject,
                                       mut t2: *const TObject)
 -> libc::c_int {
    if (*t1).ttype != (*t2).ttype { return 0 as libc::c_int }
    match (*t1).ttype {
        2 => {
            return ((*t1).value.n == (*t2).value.n) as libc::c_int
            /* LUA_TNIL */
        }
        3 | 0 => { return ((*t1).value.ts == (*t2).value.ts) as libc::c_int }
        4 => { return ((*t1).value.a == (*t2).value.a) as libc::c_int }
        5 => { return ((*t1).value.cl == (*t2).value.cl) as libc::c_int }
        _ => { return 1 as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_openspace(mut L: *mut lua_State, mut n: size_t)
 -> *mut libc::c_char {
    if n > (*L).Mbuffsize {
        (*L).Mbuffer =
            luaM_realloc(L, (*L).Mbuffer as *mut libc::c_void,
                         n.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                            as libc::c_ulong)) as
                *mut libc::c_char;
        (*L).nblocks =
            (*L).nblocks.wrapping_add(n.wrapping_sub((*L).Mbuffsize).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                                      as
                                                                                      libc::c_ulong));
        (*L).Mbuffsize = n
    }
    return (*L).Mbuffer;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_str2d(mut s: *const libc::c_char,
                                    mut result: *mut Number) -> libc::c_int {
    /* LUA_NUMBER */
    let mut endptr: *mut libc::c_char =
        0 as *mut libc::c_char; /* no conversion */
    let mut res: Number =
        strtol(s, &mut endptr,
               10 as libc::c_int); /* invalid trailing characters? */
    if endptr == s as *mut libc::c_char { return 0 as libc::c_int }
    while *(*__ctype_b_loc()).offset(*endptr as libc::c_uchar as libc::c_int
                                         as isize) as libc::c_int &
              _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 {
        endptr = endptr.offset(1)
    }
    if *endptr as libc::c_int != '\u{0}' as i32 { return 0 as libc::c_int }
    *result = res;
    return 1 as libc::c_int;
}
/* maximum length of a string format for `luaO_verror' */
/* this function needs to handle only '%d' and '%.XXs' formats */
#[no_mangle]
pub unsafe extern "C" fn luaO_verror(mut L: *mut lua_State,
                                     mut fmt: *const libc::c_char,
                                     mut args: ...) {
    let mut argp: ::std::ffi::VaListImpl; /* to hold formatted message */
    let mut buff: [libc::c_char; 280] = [0; 280]; /* remove first char */
    argp = args.clone();
    vsprintf(buff.as_mut_ptr(), fmt, argp.as_va_list());
    lua_error(L, buff.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn luaO_chunkid(mut out: *mut libc::c_char,
                                      mut source: *const libc::c_char,
                                      mut bufflen: libc::c_int) {
    if *source as libc::c_int == '=' as i32 {
        strncpy(out, source.offset(1 as libc::c_int as isize),
                bufflen as libc::c_ulong);
        *out.offset((bufflen - 1 as libc::c_int) as isize) =
            '\u{0}' as i32 as libc::c_char
        /* ensures null termination */
    } else if *source as libc::c_int == '@' as i32 {
        let mut l: libc::c_int = 0; /* skip the `@' */
        source = source.offset(1); /* get last part of file name */
        bufflen =
            (bufflen as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_char; 13]>()
                                                 as libc::c_ulong) as
                libc::c_int as libc::c_int; /* stop at first newline */
        l = strlen(source) as libc::c_int;
        if l > bufflen {
            source = source.offset((l - bufflen) as isize);
            sprintf(out,
                    b"file `...%.99s\'\x00" as *const u8 as
                        *const libc::c_char, source);
        } else {
            sprintf(out,
                    b"file `%.99s\'\x00" as *const u8 as *const libc::c_char,
                    source);
        }
    } else {
        let mut len: libc::c_int =
            strcspn(source, b"\n\x00" as *const u8 as *const libc::c_char) as
                libc::c_int;
        bufflen =
            (bufflen as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<[libc::c_char; 17]>()
                                                 as libc::c_ulong) as
                libc::c_int as libc::c_int;
        if len > bufflen { len = bufflen }
        if *source.offset(len as isize) as libc::c_int != '\u{0}' as i32 {
            /* must truncate? */
            strcpy(out, b"string \"\x00" as *const u8 as *const libc::c_char);
            out = out.offset(strlen(out) as isize);
            strncpy(out, source, len as libc::c_ulong);
            strcpy(out.offset(len as isize),
                   b"...\"\x00" as *const u8 as *const libc::c_char);
        } else {
            sprintf(out,
                    b"string \"%.99s\"\x00" as *const u8 as
                        *const libc::c_char, source);
        }
    };
}
