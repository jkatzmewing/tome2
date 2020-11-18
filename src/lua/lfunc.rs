use ::libc;
extern "C" {
    pub type TM;
    pub type lua_longjmp;
    #[no_mangle]
    fn luaM_realloc(L: *mut lua_State, oldblock: *mut libc::c_void,
                    size: lint32) -> *mut libc::c_void;
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
#[no_mangle]
pub unsafe extern "C" fn luaF_newclosure(mut L: *mut lua_State,
                                         mut nelems: libc::c_int)
 -> *mut Closure {
    let mut size: libc::c_int =
        ::std::mem::size_of::<Closure>() as libc::c_ulong as libc::c_int +
            ::std::mem::size_of::<TObject>() as libc::c_ulong as libc::c_int *
                (nelems - 1 as libc::c_int); /* chain in list of protos */
    let mut c: *mut Closure =
        luaM_realloc(L, 0 as *mut libc::c_void, size as lint32) as
            *mut Closure; /* signal that proto was properly created */
    (*c).next = (*L).rootcl;
    (*L).rootcl = c;
    (*c).mark = c;
    (*c).nupvalues = nelems as libc::c_short;
    (*L).nblocks = (*L).nblocks.wrapping_add(size as libc::c_ulong);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newproto(mut L: *mut lua_State) -> *mut Proto {
    let mut f: *mut Proto =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     ::std::mem::size_of::<Proto>() as libc::c_ulong) as
            *mut Proto;
    (*f).knum = 0 as *mut Number;
    (*f).nknum = 0 as libc::c_int;
    (*f).kstr = 0 as *mut *mut TString;
    (*f).nkstr = 0 as libc::c_int;
    (*f).kproto = 0 as *mut *mut Proto;
    (*f).nkproto = 0 as libc::c_int;
    (*f).code = 0 as *mut Instruction;
    (*f).ncode = 0 as libc::c_int;
    (*f).numparams = 0 as libc::c_int as libc::c_short;
    (*f).is_vararg = 0 as libc::c_int as libc::c_short;
    (*f).maxstacksize = 0 as libc::c_int as libc::c_short;
    (*f).marked = 0 as libc::c_int as libc::c_short;
    (*f).lineinfo = 0 as *mut libc::c_int;
    (*f).nlineinfo = 0 as libc::c_int;
    (*f).nlocvars = 0 as libc::c_int;
    (*f).locvars = 0 as *mut LocVar;
    (*f).lineDefined = 0 as libc::c_int;
    (*f).source = 0 as *mut TString;
    (*f).next = (*L).rootproto;
    (*L).rootproto = f;
    return f;
}
unsafe extern "C" fn protosize(mut f: *mut Proto) -> size_t {
    return (::std::mem::size_of::<Proto>() as
                libc::c_ulong).wrapping_add(((*f).nknum as
                                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<Number>()
                                                                                 as
                                                                                 libc::c_ulong)).wrapping_add(((*f).nkstr
                                                                                                                   as
                                                                                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                                                                                                                   as
                                                                                                                                                   libc::c_ulong)).wrapping_add(((*f).nkproto
                                                                                                                                                                                     as
                                                                                                                                                                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>()
                                                                                                                                                                                                                     as
                                                                                                                                                                                                                     libc::c_ulong)).wrapping_add(((*f).ncode
                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>()
                                                                                                                                                                                                                                                                                       as
                                                                                                                                                                                                                                                                                       libc::c_ulong)).wrapping_add(((*f).nlocvars
                                                                                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<LocVar>()
                                                                                                                                                                                                                                                                                                                                                         as
                                                                                                                                                                                                                                                                                                                                                         libc::c_ulong)).wrapping_add(((*f).nlineinfo
                                                                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                                                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                                                                                                                                                                                                                                                                                                                                                                           as
                                                                                                                                                                                                                                                                                                                                                                                                                           libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn luaF_protook(mut L: *mut lua_State,
                                      mut f: *mut Proto,
                                      mut pc: libc::c_int) {
    (*f).ncode = pc;
    (*L).nblocks = (*L).nblocks.wrapping_add(protosize(f));
}
#[no_mangle]
pub unsafe extern "C" fn luaF_freeproto(mut L: *mut lua_State,
                                        mut f: *mut Proto) {
    if (*f).ncode > 0 as libc::c_int {
        /* function was properly created? */
        (*L).nblocks = (*L).nblocks.wrapping_sub(protosize(f))
    }
    luaM_realloc(L, (*f).code as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    luaM_realloc(L, (*f).locvars as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    luaM_realloc(L, (*f).kstr as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    luaM_realloc(L, (*f).knum as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    luaM_realloc(L, (*f).kproto as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    luaM_realloc(L, (*f).lineinfo as *mut libc::c_void,
                 0 as libc::c_int as lint32);
    luaM_realloc(L, f as *mut libc::c_void, 0 as libc::c_int as lint32);
}
#[no_mangle]
pub unsafe extern "C" fn luaF_freeclosure(mut L: *mut lua_State,
                                          mut c: *mut Closure) {
    (*L).nblocks =
        (*L).nblocks.wrapping_sub((::std::mem::size_of::<Closure>() as
                                       libc::c_ulong as libc::c_int +
                                       ::std::mem::size_of::<TObject>() as
                                           libc::c_ulong as libc::c_int *
                                           ((*c).nupvalues as libc::c_int -
                                                1 as libc::c_int)) as
                                      libc::c_ulong);
    luaM_realloc(L, c as *mut libc::c_void, 0 as libc::c_int as lint32);
}
/*
** Look for n-th local variable at line `line' in function `func'.
** Returns NULL if not found.
*/
#[no_mangle]
pub unsafe extern "C" fn luaF_getlocalname(mut f: *const Proto,
                                           mut local_number: libc::c_int,
                                           mut pc: libc::c_int)
 -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*f).nlocvars &&
              (*(*f).locvars.offset(i as isize)).startpc <= pc {
        if pc < (*(*f).locvars.offset(i as isize)).endpc {
            /* is variable active? */
            local_number -= 1;
            if local_number == 0 as libc::c_int {
                return (*(*(*f).locvars.offset(i as
                                                   isize)).varname).str_0.as_mut_ptr()
            }
        }
        i += 1
    }
    return 0 as *const libc::c_char;
    /* not found */
}
