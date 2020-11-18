use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[no_mangle]
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn freopen(__filename: *const libc::c_char, __modes: *const libc::c_char,
               __stream: *mut FILE) -> *mut FILE;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, index: libc::c_int);
    #[no_mangle]
    fn lua_tostring(L: *mut lua_State, index: libc::c_int)
     -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_concat(L: *mut lua_State, n: libc::c_int);
    #[no_mangle]
    fn luaG_typeerror(L: *mut lua_State, o: StkId, op: *const libc::c_char);
    #[no_mangle]
    fn luaC_checkGC(L: *mut lua_State);
    #[no_mangle]
    fn luaM_realloc(L: *mut lua_State, oldblock: *mut libc::c_void,
                    size: lint32) -> *mut libc::c_void;
    #[no_mangle]
    fn luaZ_Fopen(z: *mut ZIO, f: *mut FILE, name: *const libc::c_char)
     -> *mut ZIO;
    #[no_mangle]
    fn luaZ_mopen(z: *mut ZIO, b: *const libc::c_char, size: size_t,
                  name: *const libc::c_char) -> *mut ZIO;
    #[no_mangle]
    fn luaY_parser(L: *mut lua_State, z: *mut ZIO) -> *mut Proto;
    #[no_mangle]
    fn luaH_getglobal(L: *mut lua_State, name: *const libc::c_char)
     -> *const TObject;
    #[no_mangle]
    fn luaT_tag(o: *const TObject) -> libc::c_int;
    #[no_mangle]
    fn luaU_undump(L: *mut lua_State, Z: *mut ZIO) -> *mut Proto;
    #[no_mangle]
    fn luaV_execute(L: *mut lua_State, cl: *const Closure, base: StkId)
     -> StkId;
    #[no_mangle]
    fn luaV_Lclosure(L: *mut lua_State, l: *mut Proto, nelems: libc::c_int);
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
/*
** {======================================================
** Error-recover functions (based on long jumps)
** =======================================================
*/
/* chain list of long jump buffers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_longjmp {
    pub b: jmp_buf,
    pub previous: *mut lua_longjmp,
    pub status: libc::c_int,
}
pub type StkId = *mut TObject;
/*
** Execute a protected call.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallS {
    pub func: StkId,
    pub nresults: libc::c_int,
}
pub const TM_FUNCTION: C2RustUnnamed_3 = 14;
pub type ZIO = zio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zio {
    pub n: size_t,
    pub p: *const libc::c_uchar,
    pub filbuf: Option<unsafe extern "C" fn(_: *mut ZIO) -> libc::c_int>,
    pub u: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub buffer: [libc::c_uchar; 256],
}
/*
** Execute a protected parser.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParserS {
    pub z: *mut ZIO,
    pub bin: libc::c_int,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const TM_N: C2RustUnnamed_3 = 15;
pub const TM_GC: C2RustUnnamed_3 = 13;
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
#[no_mangle]
pub unsafe extern "C" fn luaD_init(mut L: *mut lua_State,
                                   mut stacksize: libc::c_int) {
    (*L).stack =
        luaM_realloc(L, 0 as *mut libc::c_void,
                     ((stacksize + 2 as libc::c_int * 20 as libc::c_int) as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<TObject>()
                                                          as libc::c_ulong))
            as *mut TObject;
    (*L).nblocks =
        (*L).nblocks.wrapping_add((stacksize as
                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<TObject>()
                                                                       as
                                                                       libc::c_ulong));
    (*L).stack_last =
        (*L).stack.offset((stacksize - 1 as libc::c_int) as isize);
    (*L).stacksize = stacksize;
    (*L).top = (*L).stack;
    (*L).Cbase = (*L).top;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_checkstack(mut L: *mut lua_State,
                                         mut n: libc::c_int) {
    if (*L).stack_last.wrapping_offset_from((*L).top) as libc::c_long <=
           n as libc::c_long {
        /* stack overflow? */
        if (*L).stack_last.wrapping_offset_from((*L).stack) as libc::c_long >
               ((*L).stacksize - 1 as libc::c_int) as libc::c_long {
            /* overflow while handling overflow */
            luaD_breakrun(L, 5 as libc::c_int);
            /* break run without error message */
        } else {
            (*L).stack_last =
                (*L).stack_last.offset((2 as libc::c_int * 20 as libc::c_int)
                                           as
                                           isize); /* to be used by error message */
            lua_error(L,
                      b"stack overflow\x00" as *const u8 as
                          *const libc::c_char);
        }
    };
}
unsafe extern "C" fn restore_stack_limit(mut L: *mut lua_State) {
    if ((*L).top.wrapping_offset_from((*L).stack) as libc::c_long) <
           ((*L).stacksize - 1 as libc::c_int) as libc::c_long {
        (*L).stack_last =
            (*L).stack.offset(((*L).stacksize - 1 as libc::c_int) as isize)
    };
}
/*
** Adjust stack. Set top to base+extra, pushing NILs if needed.
** (we cannot add base+extra unless we are sure it fits in the stack;
**  otherwise the result of such operation on pointers is undefined)
*/
#[no_mangle]
pub unsafe extern "C" fn luaD_adjusttop(mut L: *mut lua_State,
                                        mut base: StkId,
                                        mut extra: libc::c_int) {
    let mut diff: libc::c_int =
        (extra as libc::c_long -
             (*L).top.wrapping_offset_from(base) as libc::c_long) as
            libc::c_int;
    if diff <= 0 as libc::c_int {
        (*L).top = base.offset(extra as isize)
    } else {
        luaD_checkstack(L, diff);
        loop  {
            let fresh0 = diff;
            diff = diff - 1;
            if !(fresh0 != 0) { break ; }
            let fresh1 = (*L).top;
            (*L).top = (*L).top.offset(1);
            (*fresh1).ttype = 1 as libc::c_int
        }
    };
}
/*
** Open a hole inside the stack at `pos'
*/
unsafe extern "C" fn luaD_openstack(mut L: *mut lua_State, mut pos: StkId) {
    let mut i: libc::c_int =
        (*L).top.wrapping_offset_from(pos) as libc::c_long as
            libc::c_int; /* ensure minimum stack size */
    loop  {
        let fresh2 = i; /* cannot call hooks inside a hook */
        i = i - 1; /* function is not active */
        if !(fresh2 != 0) {
            break ; /* number of upvalues */
        } /* new base for C function */
        *pos.offset((i + 1 as libc::c_int) as isize) = *pos.offset(i as isize)
    } /* ensure minimum stack size */
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
unsafe extern "C" fn dohook(mut L: *mut lua_State, mut ar: *mut lua_Debug,
                            mut hook: lua_Hook) {
    let mut old_Cbase: StkId = (*L).Cbase;
    (*L).Cbase = (*L).top;
    let mut old_top: StkId = (*L).Cbase;
    luaD_checkstack(L, 20 as libc::c_int);
    (*L).allowhooks = 0 as libc::c_int;
    Some(hook.expect("non-null function pointer")).expect("non-null function pointer")(L,
                                                                                       ar);
    (*L).allowhooks = 1 as libc::c_int;
    (*L).top = old_top;
    (*L).Cbase = old_Cbase;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_lineHook(mut L: *mut lua_State, mut func: StkId,
                                       mut line: libc::c_int,
                                       mut linehook: lua_Hook) {
    if (*L).allowhooks != 0 {
        let mut ar: lua_Debug =
            lua_Debug{event: 0 as *const libc::c_char,
                      currentline: 0,
                      name: 0 as *const libc::c_char,
                      namewhat: 0 as *const libc::c_char,
                      nups: 0,
                      linedefined: 0,
                      what: 0 as *const libc::c_char,
                      source: 0 as *const libc::c_char,
                      short_src: [0; 60],
                      _func: 0 as *mut lua_TObject,};
        ar._func = func;
        ar.event = b"line\x00" as *const u8 as *const libc::c_char;
        ar.currentline = line;
        dohook(L, &mut ar, linehook);
    };
}
unsafe extern "C" fn luaD_callHook(mut L: *mut lua_State, mut func: StkId,
                                   mut callhook: lua_Hook,
                                   mut event: *const libc::c_char) {
    if (*L).allowhooks != 0 {
        let mut ar: lua_Debug =
            lua_Debug{event: 0 as *const libc::c_char,
                      currentline: 0,
                      name: 0 as *const libc::c_char,
                      namewhat: 0 as *const libc::c_char,
                      nups: 0,
                      linedefined: 0,
                      what: 0 as *const libc::c_char,
                      source: 0 as *const libc::c_char,
                      short_src: [0; 60],
                      _func: 0 as *mut lua_TObject,};
        ar._func = func;
        ar.event = event;
        (*(*func).value.i).pc = 0 as *mut *const Instruction;
        dohook(L, &mut ar, callhook);
    };
}
unsafe extern "C" fn callCclosure(mut L: *mut lua_State,
                                  mut cl: *const Closure, mut base: StkId)
 -> StkId {
    let mut nup: libc::c_int = (*cl).nupvalues as libc::c_int;
    let mut old_Cbase: StkId = (*L).Cbase;
    let mut n: libc::c_int = 0;
    (*L).Cbase = base;
    luaD_checkstack(L, nup + 20 as libc::c_int);
    n = 0 as libc::c_int;
    while n < nup {
        /* copy upvalues as extra arguments */
        let fresh3 = (*L).top; /* do the actual call */
        (*L).top = (*L).top.offset(1); /* restore old C base */
        *fresh3 = *(*cl).upvalue.as_ptr().offset(n as isize);
        n += 1
    }
    n =
        Some((*cl).f.c.expect("non-null function pointer")).expect("non-null function pointer")(L);
    (*L).Cbase = old_Cbase;
    return (*L).top.offset(-(n as isize));
    /* return index of first result */
}
#[no_mangle]
pub unsafe extern "C" fn luaD_callTM(mut L: *mut lua_State,
                                     mut f: *mut Closure,
                                     mut nParams: libc::c_int,
                                     mut nResults: libc::c_int) {
    let mut base: StkId = (*L).top.offset(-(nParams as isize));
    luaD_openstack(L, base);
    (*base).value.cl = f;
    (*base).ttype = 5 as libc::c_int;
    luaD_call(L, base, nResults);
}
/*
** Call a function (C or Lua). The function to be called is at *func.
** The arguments are on the stack, right after the function.
** When returns, the results are on the stack, starting at the original
** function position.
** The number of results is nResults, unless nResults=LUA_MULTRET.
*/
#[no_mangle]
pub unsafe extern "C" fn luaD_call(mut L: *mut lua_State, mut func: StkId,
                                   mut nResults: libc::c_int) {
    let mut callhook: lua_Hook = None;
    let mut firstResult: StkId = 0 as *mut TObject;
    let mut ci: CallInfo =
        CallInfo{func: 0 as *mut Closure,
                 pc: 0 as *mut *const Instruction,
                 lastpc: 0,
                 line: 0,
                 refi: 0,};
    let mut cl: *mut Closure = 0 as *mut Closure;
    if (*func).ttype != 5 as libc::c_int {
        /* `func' is not a function; check the `function' tag method */
        let mut tm: *mut Closure =
            (*(*L).TMtable.offset(luaT_tag(func as *const TObject) as
                                      isize)).method[TM_FUNCTION as
                                                         libc::c_int as
                                                         usize]; /* tag method is the new function to be called */
        if tm.is_null() {
            luaG_typeerror(L, func,
                           b"call\x00" as *const u8 as *const libc::c_char);
        }
        luaD_openstack(L, func);
        (*func).value.cl = tm;
        (*func).ttype = 5 as libc::c_int
    }
    cl = (*func).value.cl;
    ci.func = cl;
    (*func).value.i = &mut ci;
    (*func).ttype = 6 as libc::c_int;
    callhook = (*L).callhook;
    if callhook.is_some() {
        luaD_callHook(L, func, callhook,
                      b"call\x00" as *const u8 as *const libc::c_char);
    }
    firstResult =
        if (*cl).isC as libc::c_int != 0 {
            callCclosure(L, cl, func.offset(1 as libc::c_int as isize))
        } else {
            luaV_execute(L, cl, func.offset(1 as libc::c_int as isize))
        };
    if callhook.is_some() {
        /* same hook that was active at entry */
        luaD_callHook(L, func, callhook,
                      b"return\x00" as *const u8 as *const libc::c_char);
    }
    /* move results to `func' (to erase parameters and function) */
    if nResults == -(1 as libc::c_int) {
        while firstResult < (*L).top {
            /* copy all results */
            let fresh4 = firstResult;
            firstResult = firstResult.offset(1);
            let fresh5 = func;
            func = func.offset(1);
            *fresh5 = *fresh4
        }
        (*L).top = func
    } else {
        /* copy at most `nResults' */
        while nResults > 0 as libc::c_int && firstResult < (*L).top {
            let fresh6 = firstResult;
            firstResult = firstResult.offset(1);
            let fresh7 = func;
            func = func.offset(1);
            *fresh7 = *fresh6;
            nResults -= 1
        }
        (*L).top = func;
        while nResults > 0 as libc::c_int {
            /* if there are not enough results */
            (*(*L).top).ttype = 1 as libc::c_int;
            if (*L).top == (*L).stack_last {
                luaD_checkstack(L, 1 as libc::c_int); /* adjust the stack */
            }
            (*L).top = (*L).top.offset(1);
            nResults -= 1
            /* must check stack space */
        }
    } /* function to be called */
    luaC_checkGC(L); /* remove parameters from the stack */
}
unsafe extern "C" fn f_call(mut L: *mut lua_State,
                            mut ud: *mut libc::c_void) {
    let mut c: *mut CallS = ud as *mut CallS;
    luaD_call(L, (*c).func, (*c).nresults);
}
#[no_mangle]
pub unsafe extern "C" fn lua_call(mut L: *mut lua_State,
                                  mut nargs: libc::c_int,
                                  mut nresults: libc::c_int) -> libc::c_int {
    let mut func: StkId =
        (*L).top.offset(-((nargs + 1 as libc::c_int) as isize));
    let mut c: CallS = CallS{func: 0 as *mut TObject, nresults: 0,};
    let mut status: libc::c_int = 0;
    c.func = func;
    c.nresults = nresults;
    status =
        luaD_runprotected(L,
                          Some(f_call as
                                   unsafe extern "C" fn(_: *mut lua_State,
                                                        _: *mut libc::c_void)
                                       -> ()),
                          &mut c as *mut CallS as *mut libc::c_void);
    if status != 0 as libc::c_int {
        /* an error occurred? */
        (*L).top = func
    }
    return status;
}
unsafe extern "C" fn f_parser(mut L: *mut lua_State,
                              mut ud: *mut libc::c_void) {
    let mut p: *mut ParserS = ud as *mut ParserS;
    let mut tf: *mut Proto =
        if (*p).bin != 0 {
            luaU_undump(L, (*p).z)
        } else { luaY_parser(L, (*p).z) };
    luaV_Lclosure(L, tf, 0 as libc::c_int);
}
unsafe extern "C" fn protectedparser(mut L: *mut lua_State, mut z: *mut ZIO,
                                     mut bin: libc::c_int) -> libc::c_int {
    let mut p: ParserS = ParserS{z: 0 as *mut ZIO, bin: 0,};
    let mut old_blocks: libc::c_ulong = 0;
    let mut status: libc::c_int = 0;
    p.z = z;
    p.bin = bin;
    luaC_checkGC(L);
    old_blocks = (*L).nblocks;
    status =
        luaD_runprotected(L,
                          Some(f_parser as
                                   unsafe extern "C" fn(_: *mut lua_State,
                                                        _: *mut libc::c_void)
                                       -> ()),
                          &mut p as *mut ParserS as *mut libc::c_void);
    if status == 0 as libc::c_int {
        /* add new memory to threshold (as it probably will stay) */
        (*L).GCthreshold =
            (*L).GCthreshold.wrapping_add((*L).nblocks.wrapping_sub(old_blocks))
    } else if status == 1 as libc::c_int {
        /* an error occurred: correct error code */
        status = 3 as libc::c_int
    } /* flag for file mode */
    return status; /* look ahead char */
}
unsafe extern "C" fn parse_file(mut L: *mut lua_State,
                                mut filename: *const libc::c_char)
 -> libc::c_int {
    let mut z: ZIO =
        ZIO{n: 0,
            p: 0 as *const libc::c_uchar,
            filbuf: None,
            u: 0 as *mut libc::c_void,
            name: 0 as *const libc::c_char,
            buffer: [0; 256],}; /* unable to open file */
    let mut status: libc::c_int = 0; /* set binary mode */
    let mut bin: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut f: *mut FILE =
        if filename.is_null() {
            stdin
        } else {
            fopen(filename, b"r\x00" as *const u8 as *const libc::c_char)
        };
    if f.is_null() { return 2 as libc::c_int }
    c = fgetc(f);
    ungetc(c, f);
    bin = (c == 27 as libc::c_int) as libc::c_int;
    if bin != 0 && f != stdin {
        f =
            freopen(filename, b"rb\x00" as *const u8 as *const libc::c_char,
                    f);
        if f.is_null() { return 2 as libc::c_int }
        /* unable to reopen file */
    } /* filename = '@'..filename */
    lua_pushstring(L,
                   b"@\x00" as *const u8 as
                       *const libc::c_char); /* OK: there is no GC during parser */
    lua_pushstring(L,
                   if filename.is_null() {
                       b"(stdin)\x00" as *const u8 as *const libc::c_char
                   } else { filename }); /* call main */
    lua_concat(L, 2 as libc::c_int);
    filename = lua_tostring(L, -(1 as libc::c_int));
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    luaZ_Fopen(&mut z, f, filename);
    status = protectedparser(L, &mut z, bin);
    if f != stdin { fclose(f); }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_dofile(mut L: *mut lua_State,
                                    mut filename: *const libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = parse_file(L, filename);
    if status == 0 as libc::c_int {
        /* parse OK? */
        status = lua_call(L, 0 as libc::c_int, -(1 as libc::c_int))
    } /* call main */
    return status;
}
unsafe extern "C" fn parse_buffer(mut L: *mut lua_State,
                                  mut buff: *const libc::c_char,
                                  mut size: size_t,
                                  mut name: *const libc::c_char)
 -> libc::c_int {
    let mut z: ZIO =
        ZIO{n: 0,
            p: 0 as *const libc::c_uchar,
            filbuf: None,
            u: 0 as *mut libc::c_void,
            name: 0 as *const libc::c_char,
            buffer: [0; 256],};
    if name.is_null() { name = b"?\x00" as *const u8 as *const libc::c_char }
    luaZ_mopen(&mut z, buff, size, name);
    return protectedparser(L, &mut z,
                           (*buff.offset(0 as libc::c_int as isize) as
                                libc::c_int == 27 as libc::c_int) as
                               libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lua_dobuffer(mut L: *mut lua_State,
                                      mut buff: *const libc::c_char,
                                      mut size: size_t,
                                      mut name: *const libc::c_char)
 -> libc::c_int {
    let mut status: libc::c_int = parse_buffer(L, buff, size, name);
    if status == 0 as libc::c_int {
        /* parse OK? */
        status = lua_call(L, 0 as libc::c_int, -(1 as libc::c_int))
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_dostring(mut L: *mut lua_State,
                                      mut str: *const libc::c_char)
 -> libc::c_int {
    return lua_dobuffer(L, str, strlen(str), str);
}
/* error code */
unsafe extern "C" fn message(mut L: *mut lua_State,
                             mut s: *const libc::c_char) {
    let mut em: *const TObject =
        luaH_getglobal(L,
                       b"_ERRORMESSAGE\x00" as *const u8 as
                           *const libc::c_char);
    if (*em).ttype == 5 as libc::c_int {
        *(*L).top = *em;
        if (*L).top == (*L).stack_last {
            luaD_checkstack(L, 1 as libc::c_int);
        }
        (*L).top = (*L).top.offset(1);
        lua_pushstring(L, s);
        luaD_call(L, (*L).top.offset(-(2 as libc::c_int as isize)),
                  0 as libc::c_int);
    };
}
/*
** Reports an error, and jumps up to the available recovery label
*/
#[no_mangle]
pub unsafe extern "C" fn lua_error(mut L: *mut lua_State,
                                   mut s: *const libc::c_char) {
    if !s.is_null() {
        message(L, s); /* chain new error handler */
    }
    luaD_breakrun(L, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_breakrun(mut L: *mut lua_State,
                                       mut errcode: libc::c_int) {
    if !(*L).errorJmp.is_null() {
        ::std::ptr::write_volatile(&mut (*(*L).errorJmp).status as
                                       *mut libc::c_int, errcode);
        longjmp((*(*L).errorJmp).b.as_mut_ptr(), 1 as libc::c_int);
    } else {
        if errcode != 4 as libc::c_int {
            message(L,
                    b"unable to recover; exiting\n\x00" as *const u8 as
                        *const libc::c_char);
        }
        exit(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_runprotected(mut L: *mut lua_State,
                                           mut f:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut lua_State,
                                                                           _:
                                                                               *mut libc::c_void)
                                                          -> ()>,
                                           mut ud: *mut libc::c_void)
 -> libc::c_int {
    let mut oldCbase: StkId = (*L).Cbase;
    let mut oldtop: StkId = (*L).top;
    let mut lj: lua_longjmp =
        lua_longjmp{b:
                        [__jmp_buf_tag{__jmpbuf: [0; 8],
                                       __mask_was_saved: 0,
                                       __saved_mask:
                                           __sigset_t{__val: [0; 16],},}; 1],
                    previous: 0 as *mut lua_longjmp,
                    status: 0,};
    let mut allowhooks: libc::c_int = (*L).allowhooks;
    ::std::ptr::write_volatile(&mut lj.status as *mut libc::c_int,
                               0 as libc::c_int);
    lj.previous = (*L).errorJmp;
    (*L).errorJmp = &mut lj;
    if _setjmp(lj.b.as_mut_ptr()) == 0 as libc::c_int {
        Some(f.expect("non-null function pointer")).expect("non-null function pointer")(L,
                                                                                        ud);
    } else {
        /* an error occurred: restore the state */
        (*L).allowhooks = allowhooks; /* restore old error handler */
        (*L).Cbase = oldCbase;
        (*L).top = oldtop;
        restore_stack_limit(L);
    }
    (*L).errorJmp = lj.previous;
    return lj.status;
}
/* }====================================================== */
