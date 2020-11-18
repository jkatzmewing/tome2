use ::libc;
extern "C" {
    pub type lua_longjmp;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn luaO_equalObj(t1: *const TObject, t2: *const TObject) -> libc::c_int;
    #[no_mangle]
    fn luaO_verror(L: *mut lua_State, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    static luaO_typenames: [*const libc::c_char; 0];
    #[no_mangle]
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    #[no_mangle]
    fn luaD_adjusttop(L: *mut lua_State, base: StkId, extra: libc::c_int);
    #[no_mangle]
    fn luaD_checkstack(L: *mut lua_State, n: libc::c_int);
    #[no_mangle]
    fn luaC_checkGC(L: *mut lua_State);
    #[no_mangle]
    fn luaM_growaux(L: *mut lua_State, block: *mut libc::c_void,
                    nelems: size_t, inc: libc::c_int, size: size_t,
                    errormsg: *const libc::c_char, limit: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn luaS_newudata(L: *mut lua_State, s: size_t, udata: *mut libc::c_void)
     -> *mut TString;
    #[no_mangle]
    fn luaS_createudata(L: *mut lua_State, udata: *mut libc::c_void,
                        tag: libc::c_int) -> *mut TString;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: size_t)
     -> *mut TString;
    #[no_mangle]
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    #[no_mangle]
    fn luaH_new(L: *mut lua_State, nhash: libc::c_int) -> *mut Hash;
    #[no_mangle]
    fn luaH_get(L: *mut lua_State, t: *const Hash, key: *const TObject)
     -> *const TObject;
    #[no_mangle]
    fn luaH_getnum(t: *const Hash, key: Number) -> *const TObject;
    #[no_mangle]
    fn luaH_getstr(t: *const Hash, key: *mut TString) -> *const TObject;
    #[no_mangle]
    fn luaH_set(L: *mut lua_State, t: *mut Hash, key: *const TObject)
     -> *mut TObject;
    #[no_mangle]
    fn luaH_next(L: *mut lua_State, t: *const Hash, r: *const TObject)
     -> *mut Node;
    #[no_mangle]
    fn luaH_setint(L: *mut lua_State, t: *mut Hash, key: libc::c_int)
     -> *mut TObject;
    #[no_mangle]
    fn luaT_realtag(L: *mut lua_State, tag: libc::c_int);
    #[no_mangle]
    fn luaT_tag(o: *const TObject) -> libc::c_int;
    #[no_mangle]
    fn luaV_tonumber(obj: *mut TObject) -> libc::c_int;
    #[no_mangle]
    fn luaV_tostring(L: *mut lua_State, obj: *mut TObject) -> libc::c_int;
    #[no_mangle]
    fn luaV_gettable(L: *mut lua_State, t: StkId) -> *const TObject;
    #[no_mangle]
    fn luaV_settable(L: *mut lua_State, t: StkId, key: StkId);
    #[no_mangle]
    fn luaV_getglobal(L: *mut lua_State, s: *mut TString) -> *const TObject;
    #[no_mangle]
    fn luaV_setglobal(L: *mut lua_State, s: *mut TString);
    #[no_mangle]
    fn luaV_Cclosure(L: *mut lua_State, c: lua_CFunction,
                     nelems: libc::c_int);
    #[no_mangle]
    fn luaV_lessthan(L: *mut lua_State, l: *const TObject, r: *const TObject,
                     top: StkId) -> libc::c_int;
    #[no_mangle]
    fn luaV_strconc(L: *mut lua_State, total: libc::c_int, top: StkId);
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
/*
** $Id: lapi.c,v 1.3 2001/11/26 23:00:23 darkgod Exp $
** Lua API
** See Copyright Notice in lua.h
*/
#[no_mangle]
pub static mut lua_ident: [libc::c_char; 118] =
    unsafe {
        *::std::mem::transmute::<&[u8; 118],
                                 &[libc::c_char; 118]>(b"$Lua: Lua 4.0 Copyright (C) 1994-2000 TeCGraf, PUC-Rio $\n$Authors: W. Celes, R. Ierusalimschy & L. H. de Figueiredo $\x00")
    };
#[no_mangle]
pub unsafe extern "C" fn luaA_index(mut L: *mut lua_State,
                                    mut index: libc::c_int) -> *mut TObject {
    return if index >= 0 as libc::c_int {
               (*L).Cbase.offset((index - 1 as libc::c_int) as isize)
           } else { (*L).top.offset(index as isize) };
}
unsafe extern "C" fn luaA_indexAcceptable(mut L: *mut lua_State,
                                          mut index: libc::c_int)
 -> *mut TObject {
    if index == 0 as libc::c_int {
        return 0 as *mut TObject
    } else if index > 0 as libc::c_int {
        let mut o: *mut TObject =
            (*L).Cbase.offset((index - 1 as libc::c_int) as isize);
        if o >= (*L).top { return 0 as *mut TObject } else { return o }
    } else {
        let mut o_0: *mut TObject = (*L).top.offset(index as isize);
        if o_0 < (*L).Cbase { return 0 as *mut TObject } else { return o_0 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaA_pushobject(mut L: *mut lua_State,
                                         mut o: *const TObject) {
    *(*L).top = *o;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_stackspace(mut L: *mut lua_State)
 -> libc::c_int {
    return (*L).stack_last.wrapping_offset_from((*L).top) as libc::c_long as
               libc::c_int;
}
/*
** basic stack manipulation
*/
#[no_mangle]
pub unsafe extern "C" fn lua_gettop(mut L: *mut lua_State) -> libc::c_int {
    return (*L).top.wrapping_offset_from((*L).Cbase) as libc::c_long as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_settop(mut L: *mut lua_State,
                                    mut index: libc::c_int) {
    if index >= 0 as libc::c_int {
        luaD_adjusttop(L, (*L).Cbase, index);
    } else {
        (*L).top =
            (*L).top.offset(index as isize).offset(1 as libc::c_int as isize)
    };
    /* index is negative */
}
#[no_mangle]
pub unsafe extern "C" fn lua_remove(mut L: *mut lua_State,
                                    mut index: libc::c_int) {
    let mut p: StkId = luaA_index(L, index);
    loop  {
        p = p.offset(1);
        if !(p < (*L).top) { break ; }
        *p.offset(-(1 as libc::c_int as isize)) = *p
    }
    (*L).top = (*L).top.offset(-1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_insert(mut L: *mut lua_State,
                                    mut index: libc::c_int) {
    let mut p: StkId = luaA_index(L, index);
    let mut q: StkId = 0 as *mut TObject;
    q = (*L).top;
    while q > p {
        *q = *q.offset(-(1 as libc::c_int as isize));
        q = q.offset(-1)
    }
    *p = *(*L).top;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvalue(mut L: *mut lua_State,
                                       mut index: libc::c_int) {
    *(*L).top = *luaA_index(L, index);
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
/*
** access functions (stack -> C)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_type(mut L: *mut lua_State,
                                  mut index: libc::c_int) -> libc::c_int {
    let mut o: StkId =
        luaA_indexAcceptable(L, index); /* index out-of-range */
    return if o.is_null() {
               -(1 as libc::c_int)
           } else { (*o).ttype }; /* index out-of-range */
}
#[no_mangle]
pub unsafe extern "C" fn lua_typename(mut L: *mut lua_State,
                                      mut t: libc::c_int)
 -> *const libc::c_char {
    return if t == -(1 as libc::c_int) {
               b"no value\x00" as *const u8 as *const libc::c_char
           } else { *luaO_typenames.as_ptr().offset(t as isize) };
}
#[no_mangle]
pub unsafe extern "C" fn lua_iscfunction(mut L: *mut lua_State,
                                         mut index: libc::c_int)
 -> libc::c_int {
    let mut o: StkId = luaA_indexAcceptable(L, index);
    return if o.is_null() {
               0 as libc::c_int
           } else {
               ((*o).ttype == 5 as libc::c_int &&
                    (*(*o).value.cl).isC as libc::c_int != 0) as libc::c_int
           };
}
#[no_mangle]
pub unsafe extern "C" fn lua_isnumber(mut L: *mut lua_State,
                                      mut index: libc::c_int) -> libc::c_int {
    let mut o: *mut TObject = luaA_indexAcceptable(L, index);
    return if o.is_null() {
               0 as libc::c_int
           } else {
               (((*o).ttype != 2 as libc::c_int &&
                     luaV_tonumber(o) != 0 as libc::c_int) as libc::c_int ==
                    0 as libc::c_int) as libc::c_int
           };
}
#[no_mangle]
pub unsafe extern "C" fn lua_isstring(mut L: *mut lua_State,
                                      mut index: libc::c_int) -> libc::c_int {
    let mut t: libc::c_int = lua_type(L, index);
    return (t == 3 as libc::c_int || t == 2 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tag(mut L: *mut lua_State,
                                 mut index: libc::c_int) -> libc::c_int {
    let mut o: StkId = luaA_indexAcceptable(L, index);
    return if o.is_null() {
               -(2 as libc::c_int)
           } else { luaT_tag(o as *const TObject) };
}
#[no_mangle]
pub unsafe extern "C" fn lua_equal(mut L: *mut lua_State,
                                   mut index1: libc::c_int,
                                   mut index2: libc::c_int) -> libc::c_int {
    let mut o1: StkId = luaA_indexAcceptable(L, index1);
    let mut o2: StkId = luaA_indexAcceptable(L, index2);
    if o1.is_null() || o2.is_null() {
        return 0 as libc::c_int
    } else {
        return luaO_equalObj(o1 as *const TObject, o2 as *const TObject)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_lessthan(mut L: *mut lua_State,
                                      mut index1: libc::c_int,
                                      mut index2: libc::c_int)
 -> libc::c_int {
    let mut o1: StkId = luaA_indexAcceptable(L, index1);
    let mut o2: StkId = luaA_indexAcceptable(L, index2);
    if o1.is_null() || o2.is_null() {
        return 0 as libc::c_int
    } else {
        return luaV_lessthan(L, o1 as *const TObject, o2 as *const TObject,
                             (*L).top)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tonumber(mut L: *mut lua_State,
                                      mut index: libc::c_int)
 -> libc::c_long {
    let mut o: StkId = luaA_indexAcceptable(L, index);
    return if o.is_null() ||
                  (*o).ttype != 2 as libc::c_int &&
                      luaV_tonumber(o) != 0 as libc::c_int {
               0 as libc::c_int as libc::c_long
           } else { (*o).value.n };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tostring(mut L: *mut lua_State,
                                      mut index: libc::c_int)
 -> *const libc::c_char {
    let mut o: StkId = luaA_indexAcceptable(L, index);
    return if o.is_null() ||
                  (*o).ttype != 3 as libc::c_int &&
                      luaV_tostring(L, o) != 0 as libc::c_int {
               0 as *mut libc::c_char
           } else { (*(*o).value.ts).str_0.as_mut_ptr() };
}
#[no_mangle]
pub unsafe extern "C" fn lua_strlen(mut L: *mut lua_State,
                                    mut index: libc::c_int) -> size_t {
    let mut o: StkId = luaA_indexAcceptable(L, index);
    return if o.is_null() ||
                  (*o).ttype != 3 as libc::c_int &&
                      luaV_tostring(L, o) != 0 as libc::c_int {
               0 as libc::c_int as libc::c_ulong
           } else { (*(*o).value.ts).len };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tocfunction(mut L: *mut lua_State,
                                         mut index: libc::c_int)
 -> lua_CFunction {
    let mut o: StkId = luaA_indexAcceptable(L, index);
    return if o.is_null() ||
                  !((*o).ttype == 5 as libc::c_int &&
                        (*(*o).value.cl).isC as libc::c_int != 0) {
               None
           } else { (*(*o).value.cl).f.c };
}
#[no_mangle]
pub unsafe extern "C" fn lua_touserdata(mut L: *mut lua_State,
                                        mut index: libc::c_int)
 -> *mut libc::c_void {
    let mut o: StkId = luaA_indexAcceptable(L, index);
    return if o.is_null() || (*o).ttype != 0 as libc::c_int {
               0 as *mut libc::c_void
           } else { (*(*o).value.ts).u.d.value };
}
#[no_mangle]
pub unsafe extern "C" fn lua_topointer(mut L: *mut lua_State,
                                       mut index: libc::c_int)
 -> *const libc::c_void {
    let mut o: StkId = luaA_indexAcceptable(L, index);
    if o.is_null() { return 0 as *const libc::c_void }
    match (*o).ttype {
        4 => { return (*o).value.a as *const libc::c_void }
        5 => { return (*o).value.cl as *const libc::c_void }
        _ => { return 0 as *const libc::c_void }
    };
}
/*
** push functions (C -> stack)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_pushnil(mut L: *mut lua_State) {
    (*(*L).top).ttype = 1 as libc::c_int;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushnumber(mut L: *mut lua_State,
                                        mut n: libc::c_long) {
    (*(*L).top).value.n = n;
    (*(*L).top).ttype = 2 as libc::c_int;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlstring(mut L: *mut lua_State,
                                         mut s: *const libc::c_char,
                                         mut len: size_t) {
    (*(*L).top).value.ts = luaS_newlstr(L, s, len);
    (*(*L).top).ttype = 3 as libc::c_int;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushstring(mut L: *mut lua_State,
                                        mut s: *const libc::c_char) {
    if s.is_null() {
        lua_pushnil(L);
    } else { lua_pushlstring(L, s, strlen(s)); };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushcclosure(mut L: *mut lua_State,
                                          mut fn_0: lua_CFunction,
                                          mut n: libc::c_int) {
    luaV_Cclosure(L, fn_0, n);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushusertag(mut L: *mut lua_State,
                                         mut u: *mut libc::c_void,
                                         mut tag: libc::c_int) {
    /* ORDER LUA_T */
    if !(tag == -(1 as libc::c_int) || tag == 0 as libc::c_int ||
             6 as libc::c_int <= tag && tag <= (*L).last_tag) {
        luaO_verror(L,
                    b"invalid tag for a userdata (%d)\x00" as *const u8 as
                        *const libc::c_char, tag);
    }
    (*(*L).top).value.ts = luaS_createudata(L, u, tag);
    (*(*L).top).ttype = 0 as libc::c_int;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
/*
** get functions (Lua -> stack)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_getglobal(mut L: *mut lua_State,
                                       mut name: *const libc::c_char) {
    let mut top: StkId = (*L).top;
    *top = *luaV_getglobal(L, luaS_new(L, name));
    (*L).top = top;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettable(mut L: *mut lua_State,
                                      mut index: libc::c_int) {
    let mut t: StkId =
        if index >= 0 as libc::c_int {
            (*L).Cbase.offset((index - 1 as libc::c_int) as isize)
        } else { (*L).top.offset(index as isize) };
    let mut top: StkId = (*L).top;
    *top.offset(-(1 as libc::c_int as isize)) = *luaV_gettable(L, t);
    (*L).top = top;
    /* tag method may change top */
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawget(mut L: *mut lua_State,
                                    mut index: libc::c_int) {
    let mut t: StkId =
        if index >= 0 as libc::c_int {
            (*L).Cbase.offset((index - 1 as libc::c_int) as isize)
        } else { (*L).top.offset(index as isize) };
    *(*L).top.offset(-(1 as libc::c_int as isize)) =
        *luaH_get(L, (*t).value.a,
                  (*L).top.offset(-(1 as libc::c_int as isize)) as
                      *const TObject);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgeti(mut L: *mut lua_State,
                                     mut index: libc::c_int,
                                     mut n: libc::c_int) {
    let mut o: StkId =
        if index >= 0 as libc::c_int {
            (*L).Cbase.offset((index - 1 as libc::c_int) as isize)
        } else { (*L).top.offset(index as isize) };
    *(*L).top = *luaH_getnum((*o).value.a, n as Number);
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_getglobals(mut L: *mut lua_State) {
    (*(*L).top).value.a = (*L).gt;
    (*(*L).top).ttype = 4 as libc::c_int;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_getref(mut L: *mut lua_State,
                                    mut ref_0: libc::c_int) -> libc::c_int {
    if ref_0 == -(1 as libc::c_int) {
        (*(*L).top).ttype = 1 as libc::c_int
    } else if 0 as libc::c_int <= ref_0 && ref_0 < (*L).refSize &&
                  ((*(*L).refArray.offset(ref_0 as isize)).st ==
                       -(4 as libc::c_int) ||
                       (*(*L).refArray.offset(ref_0 as isize)).st ==
                           -(2 as libc::c_int)) {
        *(*L).top = (*(*L).refArray.offset(ref_0 as isize)).o
    } else { return 0 as libc::c_int }
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_newtable(mut L: *mut lua_State) {
    (*(*L).top).value.a = luaH_new(L, 0 as libc::c_int);
    (*(*L).top).ttype = 4 as libc::c_int;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
/*
** set functions (stack -> Lua)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_setglobal(mut L: *mut lua_State,
                                       mut name: *const libc::c_char) {
    let mut top: StkId = (*L).top;
    luaV_setglobal(L, luaS_new(L, name));
    (*L).top = top.offset(-(1 as libc::c_int as isize));
    /* remove element from the top */
}
#[no_mangle]
pub unsafe extern "C" fn lua_settable(mut L: *mut lua_State,
                                      mut index: libc::c_int) {
    let mut t: StkId =
        if index >= 0 as libc::c_int {
            (*L).Cbase.offset((index - 1 as libc::c_int) as isize)
        } else { (*L).top.offset(index as isize) };
    let mut top: StkId = (*L).top;
    luaV_settable(L, t, top.offset(-(2 as libc::c_int as isize)));
    (*L).top = top.offset(-(2 as libc::c_int as isize));
    /* pop index and value */
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawset(mut L: *mut lua_State,
                                    mut index: libc::c_int) {
    let mut t: StkId =
        if index >= 0 as libc::c_int {
            (*L).Cbase.offset((index - 1 as libc::c_int) as isize)
        } else { (*L).top.offset(index as isize) };
    *luaH_set(L, (*t).value.a,
              (*L).top.offset(-(2 as libc::c_int as isize)) as *const TObject)
        = *(*L).top.offset(-(1 as libc::c_int as isize));
    (*L).top = (*L).top.offset(-(2 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawseti(mut L: *mut lua_State,
                                     mut index: libc::c_int,
                                     mut n: libc::c_int) {
    let mut o: StkId =
        if index >= 0 as libc::c_int {
            (*L).Cbase.offset((index - 1 as libc::c_int) as isize)
        } else { (*L).top.offset(index as isize) };
    *luaH_setint(L, (*o).value.a, n) =
        *(*L).top.offset(-(1 as libc::c_int as isize));
    (*L).top = (*L).top.offset(-1);
}
#[no_mangle]
pub unsafe extern "C" fn lua_setglobals(mut L: *mut lua_State) {
    (*L).top = (*L).top.offset(-1);
    let mut newtable: StkId = (*L).top;
    (*L).gt = (*newtable).value.a;
}
#[no_mangle]
pub unsafe extern "C" fn lua_ref(mut L: *mut lua_State, mut lock: libc::c_int)
 -> libc::c_int {
    let mut ref_0: libc::c_int = 0;
    if (*(*L).top.offset(-(1 as libc::c_int as isize))).ttype ==
           1 as libc::c_int {
        ref_0 = -(1 as libc::c_int)
    } else {
        if (*L).refFree != -(1 as libc::c_int) {
            /* is there a free place? */
            ref_0 = (*L).refFree;
            (*L).refFree = (*(*L).refArray.offset(ref_0 as isize)).st
        } else {
            /* no more free places */
            (*L).refArray =
                luaM_growaux(L, (*L).refArray as *mut libc::c_void,
                             (*L).refSize as size_t, 1 as libc::c_int,
                             ::std::mem::size_of::<Ref>() as libc::c_ulong,
                             b"reference table overflow\x00" as *const u8 as
                                 *const libc::c_char,
                             (2147483647 as libc::c_int - 2 as libc::c_int) as
                                 size_t) as *mut Ref;
            (*L).nblocks =
                (*L).nblocks.wrapping_add(::std::mem::size_of::<Ref>() as
                                              libc::c_ulong);
            let fresh0 = (*L).refSize;
            (*L).refSize = (*L).refSize + 1;
            ref_0 = fresh0
        }
        (*(*L).refArray.offset(ref_0 as isize)).o =
            *(*L).top.offset(-(1 as libc::c_int as isize));
        (*(*L).refArray.offset(ref_0 as isize)).st =
            if lock != 0 { -(4 as libc::c_int) } else { -(2 as libc::c_int) }
    }
    (*L).top = (*L).top.offset(-1);
    return ref_0;
}
/*
** "do" functions (run Lua code)
** (most of them are in ldo.c)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_rawcall(mut L: *mut lua_State,
                                     mut nargs: libc::c_int,
                                     mut nresults: libc::c_int) {
    luaD_call(L, (*L).top.offset(-((nargs + 1 as libc::c_int) as isize)),
              nresults);
}
#[no_mangle]
pub unsafe extern "C" fn lua_getgcthreshold(mut L: *mut lua_State)
 -> libc::c_int {
    return ((*L).GCthreshold >> 10 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getgccount(mut L: *mut lua_State)
 -> libc::c_int {
    return ((*L).nblocks >> 10 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setgcthreshold(mut L: *mut lua_State,
                                            mut newthreshold: libc::c_int) {
    if newthreshold >
           ((9223372036854775807 as libc::c_long as
                 libc::c_ulong).wrapping_mul(2 as
                                                 libc::c_ulong).wrapping_add(1
                                                                                 as
                                                                                 libc::c_ulong)
                >> 10 as libc::c_int) as libc::c_int {
        (*L).GCthreshold =
            (9223372036854775807 as libc::c_long as
                 libc::c_ulong).wrapping_mul(2 as
                                                 libc::c_ulong).wrapping_add(1
                                                                                 as
                                                                                 libc::c_ulong)
    } else {
        (*L).GCthreshold =
            (newthreshold as libc::c_ulong) << 10 as libc::c_int
    }
    luaC_checkGC(L);
}
/*
** miscellaneous functions
*/
#[no_mangle]
pub unsafe extern "C" fn lua_settag(mut L: *mut lua_State,
                                    mut tag: libc::c_int) {
    luaT_realtag(L, tag);
    match (*(*L).top.offset(-(1 as libc::c_int as isize))).ttype {
        4 => {
            (*(*(*L).top.offset(-(1 as libc::c_int as isize))).value.a).htag =
                tag
        }
        0 => {
            (*(*(*L).top.offset(-(1 as libc::c_int as
                                      isize))).value.ts).u.d.tag = tag
        }
        _ => {
            luaO_verror(L,
                        b"cannot change the tag of a %.20s\x00" as *const u8
                            as *const libc::c_char,
                        *luaO_typenames.as_ptr().offset((*(*L).top.offset(-(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize))).ttype
                                                            as isize));
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_unref(mut L: *mut lua_State,
                                   mut ref_0: libc::c_int) {
    if ref_0 >= 0 as libc::c_int {
        (*(*L).refArray.offset(ref_0 as isize)).st = (*L).refFree;
        (*L).refFree = ref_0
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_next(mut L: *mut lua_State,
                                  mut index: libc::c_int) -> libc::c_int {
    let mut t: StkId = luaA_index(L, index);
    let mut n: *mut Node = 0 as *mut Node;
    n = luaH_next(L, (*t).value.a, luaA_index(L, -(1 as libc::c_int)));
    if !n.is_null() {
        *(*L).top.offset(-(1 as libc::c_int as isize)) = (*n).key;
        *(*L).top = (*n).val;
        if (*L).top == (*L).stack_last {
            luaD_checkstack(L, 1 as libc::c_int);
        }
        (*L).top = (*L).top.offset(1);
        return 1 as libc::c_int
    } else {
        /* no more elements */
        (*L).top =
            (*L).top.offset(-(1 as libc::c_int as isize)); /* remove key */
        return 0 as libc::c_int
    }; /* value = h.n */
}
#[no_mangle]
pub unsafe extern "C" fn lua_getn(mut L: *mut lua_State,
                                  mut index: libc::c_int) -> libc::c_int {
    let mut h: *mut Hash = (*luaA_index(L, index)).value.a;
    let mut value: *const TObject =
        luaH_getstr(h,
                    luaS_new(L,
                             b"n\x00" as *const u8 as *const libc::c_char));
    if (*value).ttype == 2 as libc::c_int {
        return (*value).value.n as libc::c_int
    } else {
        let mut max: Number = 0 as libc::c_int as Number;
        let mut i: libc::c_int = (*h).size;
        let mut n: *mut Node = (*h).node;
        loop  {
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 != 0) { break ; }
            if (*n).key.ttype == 2 as libc::c_int &&
                   (*n).val.ttype != 1 as libc::c_int &&
                   (*n).key.value.n > max {
                max = (*n).key.value.n
            }
            n = n.offset(1)
        }
        return max as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_concat(mut L: *mut lua_State,
                                    mut n: libc::c_int) {
    let mut top: StkId = (*L).top;
    luaV_strconc(L, n, top);
    (*L).top = top.offset(-((n - 1 as libc::c_int) as isize));
    luaC_checkGC(L);
}
#[no_mangle]
pub unsafe extern "C" fn lua_newuserdata(mut L: *mut lua_State,
                                         mut size: size_t)
 -> *mut libc::c_void {
    let mut ts: *mut TString = luaS_newudata(L, size, 0 as *mut libc::c_void);
    (*(*L).top).value.ts = ts;
    (*(*L).top).ttype = 0 as libc::c_int;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
    return (*ts).u.d.value;
}
