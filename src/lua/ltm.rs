use ::libc;
extern "C" {
    pub type lua_longjmp;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn luaD_checkstack(L: *mut lua_State, n: libc::c_int);
    #[no_mangle]
    fn luaO_verror(L: *mut lua_State, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static luaO_typenames: [*const libc::c_char; 0];
    #[no_mangle]
    fn luaM_growaux(L: *mut lua_State, block: *mut libc::c_void,
                    nelems: size_t, inc: libc::c_int, size: size_t,
                    errormsg: *const libc::c_char, limit: size_t)
     -> *mut libc::c_void;
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
pub struct TM {
    pub method: [*mut Closure; 15],
    pub collected: *mut TString,
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
pub const TM_GC: C2RustUnnamed_3 = 13;
pub const TM_N: C2RustUnnamed_3 = 15;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const TM_FUNCTION: C2RustUnnamed_3 = 14;
pub const TM_CONCAT: C2RustUnnamed_3 = 12;
pub const TM_LT: C2RustUnnamed_3 = 11;
pub const TM_UNM: C2RustUnnamed_3 = 10;
pub const TM_POW: C2RustUnnamed_3 = 9;
pub const TM_DIV: C2RustUnnamed_3 = 8;
pub const TM_MUL: C2RustUnnamed_3 = 7;
pub const TM_SUB: C2RustUnnamed_3 = 6;
pub const TM_ADD: C2RustUnnamed_3 = 5;
pub const TM_SETGLOBAL: C2RustUnnamed_3 = 4;
pub const TM_GETGLOBAL: C2RustUnnamed_3 = 3;
pub const TM_INDEX: C2RustUnnamed_3 = 2;
pub const TM_SETTABLE: C2RustUnnamed_3 = 1;
pub const TM_GETTABLE: C2RustUnnamed_3 = 0;
/*
** $Id: ltm.c,v 1.3 2001/11/26 23:00:26 darkgod Exp $
** Tag methods
** See Copyright Notice in lua.h
*/
#[no_mangle]
pub static mut luaT_eventname: [*const libc::c_char; 19] =
    [b"gettable\x00" as *const u8 as *const libc::c_char,
     b"settable\x00" as *const u8 as *const libc::c_char,
     b"index\x00" as *const u8 as *const libc::c_char,
     b"getglobal\x00" as *const u8 as *const libc::c_char,
     b"setglobal\x00" as *const u8 as *const libc::c_char,
     b"add\x00" as *const u8 as *const libc::c_char,
     b"sub\x00" as *const u8 as *const libc::c_char,
     b"mul\x00" as *const u8 as *const libc::c_char,
     b"div\x00" as *const u8 as *const libc::c_char,
     b"pow\x00" as *const u8 as *const libc::c_char,
     b"unm\x00" as *const u8 as *const libc::c_char,
     b"lt\x00" as *const u8 as *const libc::c_char,
     b"concat\x00" as *const u8 as *const libc::c_char,
     b"gc\x00" as *const u8 as *const libc::c_char,
     b"function\x00" as *const u8 as *const libc::c_char,
     b"le\x00" as *const u8 as *const libc::c_char,
     b"gt\x00" as *const u8 as *const libc::c_char,
     b"ge\x00" as *const u8 as *const libc::c_char, 0 as *const libc::c_char];
unsafe extern "C" fn findevent(mut name: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !luaT_eventname[i as usize].is_null() {
        if strcmp(luaT_eventname[i as usize], name) == 0 as libc::c_int {
            return i
        }
        i += 1
    }
    return -(1 as libc::c_int);
    /* name not found */
}
unsafe extern "C" fn luaI_checkevent(mut L: *mut lua_State,
                                     mut name: *const libc::c_char,
                                     mut t: libc::c_int) -> libc::c_int {
    let mut e: libc::c_int = findevent(name);
    if e >= TM_N as libc::c_int {
        luaO_verror(L,
                    b"event `%.50s\' is deprecated\x00" as *const u8 as
                        *const libc::c_char, name);
    }
    if e == TM_GC as libc::c_int && t == 4 as libc::c_int {
        luaO_verror(L,
                    b"event `gc\' for tables is deprecated\x00" as *const u8
                        as *const libc::c_char);
    }
    if e < 0 as libc::c_int {
        luaO_verror(L,
                    b"`%.50s\' is not a valid event name\x00" as *const u8 as
                        *const libc::c_char, name);
    }
    return e;
}
/* events in LUA_TNIL are all allowed, since this is used as a
*  'placeholder' for "default" fallbacks
*/
/* ORDER LUA_T, ORDER TM */
static mut luaT_validevents: [[libc::c_char; 15]; 6] =
    [[1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char],
     [1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char],
     [1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char],
     [1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char],
     [0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char],
     [1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 1 as libc::c_int as libc::c_char,
      1 as libc::c_int as libc::c_char, 0 as libc::c_int as libc::c_char,
      0 as libc::c_int as libc::c_char]];
#[no_mangle]
pub unsafe extern "C" fn luaT_validevent(mut t: libc::c_int,
                                         mut e: libc::c_int) -> libc::c_int {
    /* ORDER LUA_T */
    return if t >= 6 as libc::c_int {
               1 as libc::c_int
           } else { luaT_validevents[t as usize][e as usize] as libc::c_int };
}
unsafe extern "C" fn init_entry(mut L: *mut lua_State, mut tag: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < TM_N as libc::c_int {
        let ref mut fresh0 =
            (*(*L).TMtable.offset(tag as isize)).method[i as usize];
        *fresh0 = 0 as *mut Closure;
        i += 1
    }
    let ref mut fresh1 = (*(*L).TMtable.offset(tag as isize)).collected;
    *fresh1 = 0 as *mut TString;
}
#[no_mangle]
pub unsafe extern "C" fn luaT_init(mut L: *mut lua_State) {
    let mut t: libc::c_int = 0;
    (*L).TMtable =
        luaM_growaux(L, (*L).TMtable as *mut libc::c_void,
                     0 as libc::c_int as size_t, 6 as libc::c_int,
                     ::std::mem::size_of::<TM>() as libc::c_ulong,
                     b"\x00" as *const u8 as *const libc::c_char,
                     (2147483647 as libc::c_int - 2 as libc::c_int) as size_t)
            as *mut TM;
    (*L).nblocks =
        (*L).nblocks.wrapping_add((6 as libc::c_int as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<TM>()
                                                                       as
                                                                       libc::c_ulong));
    (*L).last_tag = 6 as libc::c_int - 1 as libc::c_int;
    t = 0 as libc::c_int;
    while t <= (*L).last_tag { init_entry(L, t); t += 1 };
}
#[no_mangle]
pub unsafe extern "C" fn lua_newtag(mut L: *mut lua_State) -> libc::c_int {
    (*L).TMtable =
        luaM_growaux(L, (*L).TMtable as *mut libc::c_void,
                     (*L).last_tag as size_t, 1 as libc::c_int,
                     ::std::mem::size_of::<TM>() as libc::c_ulong,
                     b"tag table overflow\x00" as *const u8 as
                         *const libc::c_char,
                     (2147483647 as libc::c_int - 2 as libc::c_int) as size_t)
            as *mut TM;
    (*L).nblocks =
        (*L).nblocks.wrapping_add(::std::mem::size_of::<TM>() as
                                      libc::c_ulong);
    (*L).last_tag += 1;
    init_entry(L, (*L).last_tag);
    return (*L).last_tag;
}
unsafe extern "C" fn checktag(mut L: *mut lua_State, mut tag: libc::c_int) {
    if !(0 as libc::c_int <= tag && tag <= (*L).last_tag) {
        luaO_verror(L,
                    b"%d is not a valid tag\x00" as *const u8 as
                        *const libc::c_char, tag);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_realtag(mut L: *mut lua_State,
                                      mut tag: libc::c_int) {
    if !(6 as libc::c_int <= tag && tag <= (*L).last_tag) {
        luaO_verror(L,
                    b"tag %d was not created by `newtag\'\x00" as *const u8 as
                        *const libc::c_char, tag);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_copytagmethods(mut L: *mut lua_State,
                                            mut tagto: libc::c_int,
                                            mut tagfrom: libc::c_int)
 -> libc::c_int {
    let mut e: libc::c_int = 0;
    checktag(L, tagto);
    checktag(L, tagfrom);
    e = 0 as libc::c_int;
    while e < TM_N as libc::c_int {
        if luaT_validevent(tagto, e) != 0 {
            let ref mut fresh2 =
                (*(*L).TMtable.offset(tagto as isize)).method[e as usize];
            *fresh2 =
                (*(*L).TMtable.offset(tagfrom as isize)).method[e as usize]
        }
        e += 1
    }
    return tagto;
}
#[no_mangle]
pub unsafe extern "C" fn luaT_tag(mut o: *const TObject) -> libc::c_int {
    let mut t: libc::c_int = (*o).ttype;
    match t {
        0 => { return (*(*o).value.ts).u.d.tag }
        4 => { return (*(*o).value.a).htag }
        _ => { return t }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettagmethod(mut L: *mut lua_State,
                                          mut t: libc::c_int,
                                          mut event: *const libc::c_char) {
    let mut e: libc::c_int = 0;
    e = luaI_checkevent(L, event, t);
    checktag(L, t);
    if luaT_validevent(t, e) != 0 &&
           !(*(*L).TMtable.offset(t as isize)).method[e as usize].is_null() {
        (*(*L).top).value.cl =
            (*(*L).TMtable.offset(t as isize)).method[e as usize];
        (*(*L).top).ttype = 5 as libc::c_int
    } else { (*(*L).top).ttype = 1 as libc::c_int }
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_settagmethod(mut L: *mut lua_State,
                                          mut t: libc::c_int,
                                          mut event: *const libc::c_char) {
    let mut e: libc::c_int = luaI_checkevent(L, event, t);
    checktag(L, t);
    if luaT_validevent(t, e) == 0 {
        luaO_verror(L,
                    b"cannot change `%.20s\' tag method for type `%.20s\'%.20s\x00"
                        as *const u8 as *const libc::c_char,
                    luaT_eventname[e as usize],
                    *luaO_typenames.as_ptr().offset(t as isize),
                    if t == 4 as libc::c_int || t == 0 as libc::c_int {
                        b" with default tag\x00" as *const u8 as
                            *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char });
    }
    match (*(*L).top.offset(-(1 as libc::c_int as isize))).ttype {
        1 => {
            let ref mut fresh3 =
                (*(*L).TMtable.offset(t as isize)).method[e as usize];
            *fresh3 = 0 as *mut Closure
        }
        5 => {
            let ref mut fresh4 =
                (*(*L).TMtable.offset(t as isize)).method[e as usize];
            *fresh4 =
                (*(*L).top.offset(-(1 as libc::c_int as isize))).value.cl
        }
        _ => {
            lua_error(L,
                      b"tag method must be a function (or nil)\x00" as
                          *const u8 as *const libc::c_char);
        }
    }
    (*L).top = (*L).top.offset(-1);
}
