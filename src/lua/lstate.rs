use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_longjmp;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn lua_tostring(L: *mut lua_State, index: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction,
                        n: libc::c_int);
    #[no_mangle]
    fn lua_newtable(L: *mut lua_State);
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char);
    #[no_mangle]
    fn lua_ref(L: *mut lua_State, lock: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaD_init(L: *mut lua_State, stacksize: libc::c_int);
    #[no_mangle]
    fn luaD_runprotected(L: *mut lua_State,
                         f:
                             Option<unsafe extern "C" fn(_: *mut lua_State,
                                                         _: *mut libc::c_void)
                                        -> ()>, ud: *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn luaC_collect(L: *mut lua_State, all: libc::c_int);
    #[no_mangle]
    fn luaX_init(L: *mut lua_State);
    #[no_mangle]
    fn luaM_realloc(L: *mut lua_State, oldblock: *mut libc::c_void,
                    size: lint32) -> *mut libc::c_void;
    #[no_mangle]
    fn luaS_init(L: *mut lua_State);
    #[no_mangle]
    fn luaS_freeall(L: *mut lua_State);
    #[no_mangle]
    fn luaH_new(L: *mut lua_State, nhash: libc::c_int) -> *mut Hash;
    #[no_mangle]
    fn luaT_init(L: *mut lua_State);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
/*
** $Id: lstate.c,v 1.3 2001/11/26 23:00:26 darkgod Exp $
** Global State
** See Copyright Notice in lua.h
*/
/*
** built-in implementation for ERRORMESSAGE. In a "correct" environment
** ERRORMESSAGE should have an external definition, and so this function
** would not be used.
*/
unsafe extern "C" fn errormessage(mut L: *mut lua_State) -> libc::c_int {
    let mut s: *const libc::c_char = lua_tostring(L, 1 as libc::c_int);
    if s.is_null() {
        s = b"(no message)\x00" as *const u8 as *const libc::c_char
    }
    fprintf(stderr, b"error: %s\n\x00" as *const u8 as *const libc::c_char,
            s);
    return 0 as libc::c_int;
}
/*
** open parts that may cause memory-allocation errors
*/
unsafe extern "C" fn f_luaopen(mut L: *mut lua_State,
                               mut ud: *mut libc::c_void) {
    let mut stacksize: libc::c_int =
        *(ud as *mut libc::c_int); /* table of globals */
    if stacksize == 0 as libc::c_int {
        stacksize = 1024 as libc::c_int
    } else { stacksize += 20 as libc::c_int } /* create registry */
    (*L).gt = luaH_new(L, 10 as libc::c_int); /* memory allocation error */
    luaD_init(L, stacksize); /* to avoid GC during pre-definitions */
    luaS_init(L);
    luaX_init(L);
    luaT_init(L);
    lua_newtable(L);
    lua_ref(L, 1 as libc::c_int);
    lua_pushcclosure(L,
                     Some(errormessage as
                              unsafe extern "C" fn(_: *mut lua_State)
                                  -> libc::c_int), 0 as libc::c_int);
    lua_setglobal(L,
                  b"_ERRORMESSAGE\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn lua_open(mut stacksize: libc::c_int)
 -> *mut lua_State {
    let mut L: *mut lua_State =
        luaM_realloc(0 as *mut lua_State, 0 as *mut libc::c_void,
                     ::std::mem::size_of::<lua_State>() as libc::c_ulong) as
            *mut lua_State;
    if L.is_null() { return 0 as *mut lua_State }
    (*L).stack = 0 as StkId;
    (*L).udt.size = 0 as libc::c_int;
    (*L).strt.size = (*L).udt.size;
    (*L).udt.nuse = 0 as libc::c_int as lint32;
    (*L).strt.nuse = (*L).udt.nuse;
    (*L).strt.hash = 0 as *mut *mut TString;
    (*L).udt.hash = 0 as *mut *mut TString;
    (*L).Mbuffer = 0 as *mut libc::c_char;
    (*L).Mbuffsize = 0 as libc::c_int as size_t;
    (*L).rootproto = 0 as *mut Proto;
    (*L).rootcl = 0 as *mut Closure;
    (*L).roottable = 0 as *mut Hash;
    (*L).TMtable = 0 as *mut TM;
    (*L).last_tag = -(1 as libc::c_int);
    (*L).refArray = 0 as *mut Ref;
    (*L).refSize = 0 as libc::c_int;
    (*L).refFree = -(1 as libc::c_int);
    (*L).nblocks = ::std::mem::size_of::<lua_State>() as libc::c_ulong;
    (*L).GCthreshold =
        (2147483647 as libc::c_int - 2 as libc::c_int) as libc::c_ulong;
    (*L).callhook = None;
    (*L).linehook = None;
    (*L).allowhooks = 1 as libc::c_int;
    (*L).errorJmp = 0 as *mut lua_longjmp;
    if luaD_runprotected(L,
                         Some(f_luaopen as
                                  unsafe extern "C" fn(_: *mut lua_State,
                                                       _: *mut libc::c_void)
                                      -> ()),
                         &mut stacksize as *mut libc::c_int as
                             *mut libc::c_void) != 0 as libc::c_int {
        /* memory allocation error: free partial state */
        lua_close(L); /* collect all elements */
        return 0 as *mut lua_State
    }
    (*L).GCthreshold =
        (2 as libc::c_int as libc::c_ulong).wrapping_mul((*L).nblocks);
    return L;
}
#[no_mangle]
pub unsafe extern "C" fn lua_close(mut L: *mut lua_State) {
    luaC_collect(L, 1 as libc::c_int);
    luaS_freeall(L);
    if !(*L).stack.is_null() {
        (*L).nblocks =
            (*L).nblocks.wrapping_sub((((*L).stack_last.wrapping_offset_from((*L).stack)
                                            as libc::c_long +
                                            1 as libc::c_int as libc::c_long)
                                           as
                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<TObject>()
                                                                           as
                                                                           libc::c_ulong))
    }
    luaM_realloc(L, (*L).stack as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    (*L).nblocks =
        (*L).nblocks.wrapping_sub((((*L).last_tag + 1 as libc::c_int) as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<TM>()
                                                                       as
                                                                       libc::c_ulong));
    luaM_realloc(L, (*L).TMtable as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    (*L).nblocks =
        (*L).nblocks.wrapping_sub(((*L).refSize as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<Ref>()
                                                                       as
                                                                       libc::c_ulong));
    luaM_realloc(L, (*L).refArray as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    (*L).nblocks =
        (*L).nblocks.wrapping_sub((*L).Mbuffsize.wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                                  as
                                                                  libc::c_ulong));
    luaM_realloc(L, (*L).Mbuffer as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    luaM_realloc(L, L as *mut libc::c_void, 0 as libc::c_int as lint32);
}
