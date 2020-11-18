use ::libc;
extern "C" {
    pub type TM;
    pub type lua_longjmp;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn luaO_power2(n: lint32) -> lint32;
    #[no_mangle]
    fn luaD_breakrun(L: *mut lua_State, errcode: libc::c_int);
}
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
    pub f: C2RustUnnamed,
    pub next: *mut Closure,
    pub mark: *mut Closure,
    pub isC: libc::c_short,
    pub nupvalues: libc::c_short,
    pub upvalue: [TObject; 1],
}
pub type TObject = lua_TObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
    pub u: C2RustUnnamed_0,
    pub len: size_t,
    pub nexthash: *mut TString,
    pub marked: libc::c_int,
    pub str_0: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: C2RustUnnamed_2,
    pub d: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tag: libc::c_int,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
** $Id: lmem.c,v 1.3 2001/11/26 23:00:24 darkgod Exp $
** Interface to Memory Manager
** See Copyright Notice in lua.h
*/
/*
** Real ISO (ANSI) systems do not need these tests;
** but some systems (Sun OS) are not that ISO...
*/
#[no_mangle]
pub unsafe extern "C" fn luaM_growaux(mut L: *mut lua_State,
                                      mut block: *mut libc::c_void,
                                      mut nelems: size_t,
                                      mut inc: libc::c_int, mut size: size_t,
                                      mut errormsg: *const libc::c_char,
                                      mut limit: size_t)
 -> *mut libc::c_void {
    let mut newn: size_t = nelems.wrapping_add(inc as libc::c_ulong);
    if nelems >= limit.wrapping_sub(inc as libc::c_ulong) {
        lua_error(L, errormsg);
    }
    if newn ^ nelems <= nelems ||
           nelems > 0 as libc::c_int as libc::c_ulong &&
               newn < 4 as libc::c_int as libc::c_ulong {
        /* or block already is MINPOWER2? */
        return block
    } else { /* do not need to reallocate */
        /* it crossed a power-of-2 boundary; grow to next power */
        return luaM_realloc(L, block, luaO_power2(newn).wrapping_mul(size))
    };
}
/*
** generic allocation routine.
*/
#[no_mangle]
pub unsafe extern "C" fn luaM_realloc(mut L: *mut lua_State,
                                      mut block: *mut libc::c_void,
                                      mut size: lint32) -> *mut libc::c_void {
    if size == 0 as libc::c_int as libc::c_ulong {
        free(block); /* block may be NULL; that is OK for free */
        return 0 as *mut libc::c_void
    } else {
        if size >=
               (!(0 as libc::c_int as
                      size_t)).wrapping_sub(2 as libc::c_int as libc::c_ulong)
           {
            lua_error(L,
                      b"memory allocation error: block too big\x00" as
                          *const u8 as
                          *const libc::c_char); /* break run without error message */
        }
    }
    block = realloc(block, size);
    if block.is_null() {
        if !L.is_null() {
            luaD_breakrun(L, 4 as libc::c_int);
        } else { return 0 as *mut libc::c_void }
        /* error before creating state! */
    }
    return block;
}
