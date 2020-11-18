use ::libc;
extern "C" {
    pub type lua_longjmp;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    fn lua_error(L: *mut lua_State, s: *const libc::c_char);
    #[no_mangle]
    static luaO_nilobject: TObject;
    #[no_mangle]
    fn luaO_openspace(L: *mut lua_State, n: size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn luaO_equalObj(t1: *const TObject, t2: *const TObject) -> libc::c_int;
    #[no_mangle]
    fn luaO_str2d(s: *const libc::c_char, result: *mut Number) -> libc::c_int;
    #[no_mangle]
    fn luaG_ordererror(L: *mut lua_State, top: StkId);
    #[no_mangle]
    fn luaG_typeerror(L: *mut lua_State, o: StkId, op: *const libc::c_char);
    #[no_mangle]
    fn luaG_binerror(L: *mut lua_State, p1: StkId, t: libc::c_int,
                     op: *const libc::c_char);
    #[no_mangle]
    fn luaG_getline(lineinfo: *mut libc::c_int, pc: libc::c_int,
                    refline: libc::c_int, refi: *mut libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn luaD_adjusttop(L: *mut lua_State, base: StkId, extra: libc::c_int);
    #[no_mangle]
    fn luaD_lineHook(L: *mut lua_State, func: StkId, line: libc::c_int,
                     linehook: lua_Hook);
    #[no_mangle]
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: libc::c_int);
    #[no_mangle]
    fn luaD_callTM(L: *mut lua_State, f: *mut Closure, nParams: libc::c_int,
                   nResults: libc::c_int);
    #[no_mangle]
    fn luaD_checkstack(L: *mut lua_State, n: libc::c_int);
    #[no_mangle]
    fn luaF_newclosure(L: *mut lua_State, nelems: libc::c_int)
     -> *mut Closure;
    #[no_mangle]
    fn luaC_checkGC(L: *mut lua_State);
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
    fn luaH_setstrnum(L: *mut lua_State, t: *mut Hash, key: *mut TString,
                      val: Number);
    #[no_mangle]
    static luaT_eventname: [*const libc::c_char; 0];
    #[no_mangle]
    fn luaT_tag(o: *const TObject) -> libc::c_int;
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
pub type OpCode = libc::c_uint;
pub const OP_CLOSURE: OpCode = 48;
pub const OP_LFORLOOP: OpCode = 47;
pub const OP_LFORPREP: OpCode = 46;
pub const OP_FORLOOP: OpCode = 45;
pub const OP_FORPREP: OpCode = 44;
pub const OP_PUSHNILJMP: OpCode = 43;
pub const OP_JMP: OpCode = 42;
pub const OP_JMPONF: OpCode = 41;
pub const OP_JMPONT: OpCode = 40;
pub const OP_JMPF: OpCode = 39;
pub const OP_JMPT: OpCode = 38;
pub const OP_JMPGE: OpCode = 37;
pub const OP_JMPGT: OpCode = 36;
pub const OP_JMPLE: OpCode = 35;
pub const OP_JMPLT: OpCode = 34;
pub const OP_JMPEQ: OpCode = 33;
pub const OP_JMPNE: OpCode = 32;
pub const OP_NOT: OpCode = 31;
pub const OP_MINUS: OpCode = 30;
pub const OP_CONCAT: OpCode = 29;
pub const OP_POW: OpCode = 28;
pub const OP_DIV: OpCode = 27;
pub const OP_MULT: OpCode = 26;
pub const OP_SUB: OpCode = 25;
pub const OP_ADDI: OpCode = 24;
pub const OP_ADD: OpCode = 23;
pub const OP_SETMAP: OpCode = 22;
pub const OP_SETLIST: OpCode = 21;
pub const OP_SETTABLE: OpCode = 20;
pub const OP_SETGLOBAL: OpCode = 19;
pub const OP_SETLOCAL: OpCode = 18;
pub const OP_CREATETABLE: OpCode = 17;
pub const OP_PUSHSELF: OpCode = 16;
pub const OP_GETINDEXED: OpCode = 15;
pub const OP_GETDOTTED: OpCode = 14;
pub const OP_GETTABLE: OpCode = 13;
pub const OP_GETGLOBAL: OpCode = 12;
pub const OP_GETLOCAL: OpCode = 11;
pub const OP_PUSHUPVALUE: OpCode = 10;
pub const OP_PUSHNEGNUM: OpCode = 9;
pub const OP_PUSHNUM: OpCode = 8;
pub const OP_PUSHSTRING: OpCode = 7;
pub const OP_PUSHINT: OpCode = 6;
pub const OP_POP: OpCode = 5;
pub const OP_PUSHNIL: OpCode = 4;
pub const OP_TAILCALL: OpCode = 3;
pub const OP_CALL: OpCode = 2;
pub const OP_RETURN: OpCode = 1;
pub const OP_END: OpCode = 0;
pub type TMS = libc::c_uint;
pub const TM_N: TMS = 15;
pub const TM_FUNCTION: TMS = 14;
pub const TM_GC: TMS = 13;
pub const TM_CONCAT: TMS = 12;
pub const TM_LT: TMS = 11;
pub const TM_UNM: TMS = 10;
pub const TM_POW: TMS = 9;
pub const TM_DIV: TMS = 8;
pub const TM_MUL: TMS = 7;
pub const TM_SUB: TMS = 6;
pub const TM_ADD: TMS = 5;
pub const TM_SETGLOBAL: TMS = 4;
pub const TM_GETGLOBAL: TMS = 3;
pub const TM_INDEX: TMS = 2;
pub const TM_SETTABLE: TMS = 1;
pub const TM_GETTABLE: TMS = 0;
#[no_mangle]
pub unsafe extern "C" fn luaV_tonumber(mut obj: *mut TObject) -> libc::c_int {
    if (*obj).ttype != 3 as libc::c_int {
        return 1 as libc::c_int
    } else {
        if luaO_str2d((*(*obj).value.ts).str_0.as_mut_ptr(),
                      &mut (*obj).value.n) == 0 {
            return 2 as libc::c_int
        }
        (*obj).ttype = 2 as libc::c_int;
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_tostring(mut L: *mut lua_State,
                                       mut obj: *mut TObject) -> libc::c_int {
    /* LUA_NUMBER */
    if (*obj).ttype != 2 as libc::c_int {
        return 1 as libc::c_int
    } else {
        let mut s: [libc::c_char; 32] =
            [0; 32]; /* 16 digits, sign, point and \0  (+ some extra...) */
        sprintf(s.as_mut_ptr(),
                b"%ld\x00" as *const u8 as *const libc::c_char,
                (*obj).value.n); /* convert `s' to number */
        (*obj).value.ts = luaS_new(L, s.as_mut_ptr());
        (*obj).ttype = 3 as libc::c_int;
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn traceexec(mut L: *mut lua_State, mut base: StkId,
                               mut top: StkId, mut linehook: lua_Hook) {
    let mut ci: *mut CallInfo =
        (*base.offset(-(1 as libc::c_int as isize))).value.i;
    let mut lineinfo: *mut libc::c_int = (*(*(*ci).func).f.l).lineinfo;
    let mut pc: libc::c_int =
        ((*(*ci).pc).wrapping_offset_from((*(*(*ci).func).f.l).code) as
             libc::c_long - 1 as libc::c_int as libc::c_long) as libc::c_int;
    let mut newline: libc::c_int = 0;
    if pc == 0 as libc::c_int {
        /* may be first time? */
        (*ci).line = 1 as libc::c_int;
        (*ci).refi = 0 as libc::c_int;
        (*ci).lastpc = pc + 1 as libc::c_int
        /* make sure it will call linehook */
    }
    newline = luaG_getline(lineinfo, pc, (*ci).line, &mut (*ci).refi);
    /* calls linehook when enters a new line or jumps back (loop) */
    if newline != (*ci).line || pc <= (*ci).lastpc {
        (*ci).line = newline;
        (*L).top = top;
        luaD_lineHook(L, base.offset(-(2 as libc::c_int as isize)), newline,
                      linehook);
    }
    (*ci).lastpc = pc;
}
unsafe extern "C" fn luaV_closure(mut L: *mut lua_State,
                                  mut nelems: libc::c_int) -> *mut Closure {
    let mut c: *mut Closure = luaF_newclosure(L, nelems);
    (*L).top = (*L).top.offset(-(nelems as isize));
    loop  {
        let fresh0 = nelems;
        nelems = nelems - 1;
        if !(fresh0 != 0) { break ; }
        *(*c).upvalue.as_mut_ptr().offset(nelems as isize) =
            *(*L).top.offset(nelems as isize)
    }
    (*(*L).top).value.cl = c;
    (*(*L).top).ttype = 5 as libc::c_int;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaV_Cclosure(mut L: *mut lua_State,
                                       mut c: lua_CFunction,
                                       mut nelems: libc::c_int) {
    let mut cl: *mut Closure = luaV_closure(L, nelems);
    (*cl).f.c = c;
    (*cl).isC = 1 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn luaV_Lclosure(mut L: *mut lua_State,
                                       mut l: *mut Proto,
                                       mut nelems: libc::c_int) {
    let mut cl: *mut Closure = luaV_closure(L, nelems);
    (*cl).f.l = l;
    (*cl).isC = 0 as libc::c_int as libc::c_short;
}
/*
** Function to index a table.
** Receives the table at `t' and the key at top.
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_gettable(mut L: *mut lua_State, mut t: StkId)
 -> *const TObject {
    let mut tm: *mut Closure = 0 as *mut Closure;
    let mut tg: libc::c_int = 0;
    if (*t).ttype == 4 as libc::c_int &&
           {
               tg = (*(*t).value.a).htag;
               (tg == 4 as libc::c_int) ||
                   (*(*L).TMtable.offset(tg as
                                             isize)).method[TM_GETTABLE as
                                                                libc::c_int as
                                                                usize].is_null()
           } {
        /* or no TM? */
        /* do a primitive get */
        let mut h: *const TObject =
            luaH_get(L, (*t).value.a,
                     (*L).top.offset(-(1 as libc::c_int as isize)) as
                         *const TObject);
        /* result is no nil or there is no `index' tag method? */
        if (*h).ttype != 1 as libc::c_int ||
               {
                   tm =
                       (*(*L).TMtable.offset(tg as
                                                 isize)).method[TM_INDEX as
                                                                    libc::c_int
                                                                    as usize];
                   tm.is_null()
               } {
            return h
        }
    } else {
        /* try a `gettable' tag method */
        tm =
            (*(*L).TMtable.offset(luaT_tag(t as *const TObject) as
                                      isize)).method[TM_GETTABLE as
                                                         libc::c_int as usize]
    }
    if !tm.is_null() {
        /* is there a tag method? */
        luaD_checkstack(L, 2 as libc::c_int);
        /* call result */
        *(*L).top.offset(1 as libc::c_int as isize) =
            *(*L).top.offset(-(1 as libc::c_int as isize)); /* key */
        *(*L).top = *t; /* table */
        let ref mut fresh1 =
            (*(*L).top.offset(-(1 as libc::c_int as
                                    isize))).value.cl; /* tag method */
        *fresh1 = tm;
        (*(*L).top.offset(-(1 as libc::c_int as isize))).ttype =
            5 as libc::c_int;
        (*L).top = (*L).top.offset(2 as libc::c_int as isize);
        luaD_call(L, (*L).top.offset(-(3 as libc::c_int as isize)),
                  1 as libc::c_int);
        return (*L).top.offset(-(1 as libc::c_int as isize)) as *const TObject
    } else {
        /* no tag method */
        luaG_typeerror(L, t,
                       b"index\x00" as *const u8 as *const libc::c_char);
        return 0 as *const TObject
        /* to avoid warnings */
    };
}
/*
** Receives table at `t', key at `key' and value at top.
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_settable(mut L: *mut lua_State, mut t: StkId,
                                       mut key: StkId) {
    let mut tg: libc::c_int = 0;
    if (*t).ttype == 4 as libc::c_int &&
           {
               tg = (*(*t).value.a).htag;
               (tg == 4 as libc::c_int) ||
                   (*(*L).TMtable.offset(tg as
                                             isize)).method[TM_SETTABLE as
                                                                libc::c_int as
                                                                usize].is_null()
           } {
        /* or no TM? */
        *luaH_set(L, (*t).value.a, key as *const TObject) =
            *(*L).top.offset(-(1 as libc::c_int as isize))
    } else { /* do a primitive set */
        /* try a `settable' tag method */
        let mut tm: *mut Closure =
            (*(*L).TMtable.offset(luaT_tag(t as *const TObject) as
                                      isize)).method[TM_SETTABLE as
                                                         libc::c_int as
                                                         usize];
        if !tm.is_null() {
            luaD_checkstack(L, 3 as libc::c_int);
            *(*L).top.offset(2 as libc::c_int as isize) =
                *(*L).top.offset(-(1 as libc::c_int as isize));
            *(*L).top.offset(1 as libc::c_int as isize) = *key;
            *(*L).top = *t;
            let ref mut fresh2 =
                (*(*L).top.offset(-(1 as libc::c_int as isize))).value.cl;
            *fresh2 = tm;
            (*(*L).top.offset(-(1 as libc::c_int as isize))).ttype =
                5 as libc::c_int;
            (*L).top = (*L).top.offset(3 as libc::c_int as isize);
            luaD_call(L, (*L).top.offset(-(4 as libc::c_int as isize)),
                      0 as libc::c_int);
            /* call `settable' tag method */
        } else {
            /* no tag method... */
            luaG_typeerror(L, t,
                           b"index\x00" as *const u8 as *const libc::c_char);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_getglobal(mut L: *mut lua_State,
                                        mut s: *mut TString)
 -> *const TObject {
    let mut value: *const TObject = luaH_getstr((*L).gt, s);
    let mut tm: *mut Closure =
        (*(*L).TMtable.offset(luaT_tag(value) as
                                  isize)).method[TM_GETGLOBAL as libc::c_int
                                                     as usize];
    if tm.is_null() {
        /* is there a tag method? */
        return value
    } else { /* default behavior */
        /* tag method */
        luaD_checkstack(L, 3 as libc::c_int); /* global name */
        (*(*L).top).value.cl = tm;
        (*(*L).top).ttype = 5 as libc::c_int;
        let ref mut fresh3 =
            (*(*L).top.offset(1 as libc::c_int as isize)).value.ts;
        *fresh3 = s;
        (*(*L).top.offset(1 as libc::c_int as isize)).ttype =
            3 as libc::c_int;
        *(*L).top.offset(2 as libc::c_int as isize) = *value;
        (*L).top = (*L).top.offset(3 as libc::c_int as isize);
        luaD_call(L, (*L).top.offset(-(3 as libc::c_int as isize)),
                  1 as libc::c_int);
        return (*L).top.offset(-(1 as libc::c_int as isize)) as *const TObject
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_setglobal(mut L: *mut lua_State,
                                        mut s: *mut TString) {
    let mut oldvalue: *const TObject = luaH_getstr((*L).gt, s);
    let mut tm: *mut Closure =
        (*(*L).TMtable.offset(luaT_tag(oldvalue) as
                                  isize)).method[TM_SETGLOBAL as libc::c_int
                                                     as usize];
    if tm.is_null() {
        /* is there a tag method? */
        if oldvalue != &luaO_nilobject as *const TObject {
            /* cast to remove `const' is OK, because `oldvalue' != luaO_nilobject */
            *(oldvalue as *mut TObject) =
                *(*L).top.offset(-(1 as libc::c_int as isize))
        } else {
            let mut key: TObject =
                TObject{ttype: 0,
                        value:
                            Value{ts:
                                      0 as *const TString as
                                          *mut TString,},}; /* new value */
            key.ttype = 3 as libc::c_int;
            key.value.ts = s;
            *luaH_set(L, (*L).gt, &mut key) =
                *(*L).top.offset(-(1 as libc::c_int as isize))
        }
    } else {
        luaD_checkstack(L, 3 as libc::c_int);
        *(*L).top.offset(2 as libc::c_int as isize) =
            *(*L).top.offset(-(1 as libc::c_int as isize));
        *(*L).top.offset(1 as libc::c_int as isize) = *oldvalue;
        (*(*L).top).ttype = 3 as libc::c_int;
        (*(*L).top).value.ts = s;
        let ref mut fresh4 =
            (*(*L).top.offset(-(1 as libc::c_int as isize))).value.cl;
        *fresh4 = tm;
        (*(*L).top.offset(-(1 as libc::c_int as isize))).ttype =
            5 as libc::c_int;
        (*L).top = (*L).top.offset(3 as libc::c_int as isize);
        luaD_call(L, (*L).top.offset(-(4 as libc::c_int as isize)),
                  0 as libc::c_int);
    };
}
unsafe extern "C" fn call_binTM(mut L: *mut lua_State, mut top: StkId,
                                mut event: TMS) -> libc::c_int {
    /* try first operand */
    let mut tm: *mut Closure =
        (*(*L).TMtable.offset(luaT_tag(top.offset(-(2 as libc::c_int as
                                                        isize)) as
                                           *const TObject) as
                                  isize)).method[event as
                                                     usize]; /* try second operand */
    (*L).top = top; /* try a `global' method */
    if tm.is_null() {
        tm =
            (*(*L).TMtable.offset(luaT_tag(top.offset(-(1 as libc::c_int as
                                                            isize)) as
                                               *const TObject) as
                                      isize)).method[event as usize];
        if tm.is_null() {
            tm =
                (*(*L).TMtable.offset(0 as libc::c_int as
                                          isize)).method[event as usize];
            if tm.is_null() { return 0 as libc::c_int }
            /* error */
        }
    }
    lua_pushstring(L, *luaT_eventname.as_ptr().offset(event as isize));
    luaD_callTM(L, tm, 3 as libc::c_int, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn call_arith(mut L: *mut lua_State, mut top: StkId,
                                mut event: TMS) {
    if call_binTM(L, top, event) == 0 {
        luaG_binerror(L, top.offset(-(2 as libc::c_int as isize)),
                      2 as libc::c_int,
                      b"perform arithmetic on\x00" as *const u8 as
                          *const libc::c_char);
    };
}
unsafe extern "C" fn luaV_strcomp(mut ls: *const TString,
                                  mut rs: *const TString) -> libc::c_int {
    let mut l: *const libc::c_char = (*ls).str_0.as_ptr();
    let mut ll: size_t = (*ls).len;
    let mut r: *const libc::c_char = (*rs).str_0.as_ptr();
    let mut lr: size_t = (*rs).len;
    loop  {
        let mut temp: libc::c_int = strcoll(l, r);
        if temp != 0 as libc::c_int {
            return temp
        } else {
            /* strings are equal up to a '\0' */
            let mut len: size_t =
                strlen(l); /* index of first '\0' in both strings */
            if len == ll
               { /* l is greater than r (because l is not finished) */
                /* l is finished? */
                return if len == lr {
                           0 as libc::c_int
                       } else { -(1 as libc::c_int) }
            } else {
                if len == lr { /* l is equal or smaller than r */
                    /* r is finished? */
                    return 1 as libc::c_int
                }
            }
            /* both strings longer than `len'; go on comparing (after the '\0') */
            len = len.wrapping_add(1);
            l = l.offset(len as isize);
            ll = (ll as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
            r = r.offset(len as isize);
            lr = (lr as libc::c_ulong).wrapping_sub(len) as size_t as size_t
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_lessthan(mut L: *mut lua_State,
                                       mut l: *const TObject,
                                       mut r: *const TObject, mut top: StkId)
 -> libc::c_int {
    if (*l).ttype == 2 as libc::c_int && (*r).ttype == 2 as libc::c_int {
        return ((*l).value.n < (*r).value.n) as libc::c_int
    } else if (*l).ttype == 3 as libc::c_int && (*r).ttype == 3 as libc::c_int
     {
        return (luaV_strcomp((*l).value.ts, (*r).value.ts) < 0 as libc::c_int)
                   as libc::c_int
    } else {
        /* call TM */
        luaD_checkstack(L,
                        2 as
                            libc::c_int); /* number of elements handled in this pass (at least 2) */
        let fresh5 = top;
        top = top.offset(1);
        *fresh5 = *l;
        let fresh6 = top;
        top = top.offset(1);
        *fresh6 = *r;
        if call_binTM(L, top, TM_LT) == 0 {
            luaG_ordererror(L, top.offset(-(2 as libc::c_int as isize)));
        }
        (*L).top = (*L).top.offset(-1);
        return ((*(*L).top).ttype != 1 as libc::c_int) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_strconc(mut L: *mut lua_State,
                                      mut total: libc::c_int,
                                      mut top: StkId) {
    loop  {
        let mut n: libc::c_int = 2 as libc::c_int;
        if (*top.offset(-(2 as libc::c_int as isize))).ttype !=
               3 as libc::c_int &&
               luaV_tostring(L, top.offset(-(2 as libc::c_int as isize))) !=
                   0 as libc::c_int ||
               (*top.offset(-(1 as libc::c_int as isize))).ttype !=
                   3 as libc::c_int &&
                   luaV_tostring(L, top.offset(-(1 as libc::c_int as isize)))
                       != 0 as libc::c_int {
            if call_binTM(L, top, TM_CONCAT) == 0 {
                luaG_binerror(L, top.offset(-(2 as libc::c_int as isize)),
                              3 as libc::c_int,
                              b"concat\x00" as *const u8 as
                                  *const libc::c_char);
            }
        } else if (*(*top.offset(-(1 as libc::c_int as isize))).value.ts).len
                      > 0 as libc::c_int as libc::c_ulong {
            /* if len=0, do nothing */
            /* at least two string values; get as many as possible */
            let mut tl: lint32 =
                (*(*top.offset(-(1 as libc::c_int as
                                     isize))).value.ts).len.wrapping_add((*(*top.offset(-(2
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize))).value.ts).len);
            let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut i: libc::c_int = 0;
            while n < total &&
                      !((*top.offset(-(n as
                                           isize)).offset(-(1 as libc::c_int
                                                                as
                                                                isize))).ttype
                            != 3 as libc::c_int &&
                            luaV_tostring(L,
                                          top.offset(-(n as
                                                           isize)).offset(-(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize)))
                                != 0 as libc::c_int) {
                /* collect total length */
                tl =
                    (tl as
                         libc::c_ulong).wrapping_add((*(*top.offset(-(n as
                                                                          isize)).offset(-(1
                                                                                               as
                                                                                               libc::c_int
                                                                                               as
                                                                                               isize))).value.ts).len)
                        as lint32 as lint32;
                n += 1
            }
            if tl >
                   (!(0 as libc::c_int as
                          size_t)).wrapping_sub(2 as libc::c_int as
                                                    libc::c_ulong) {
                lua_error(L,
                          b"string size overflow\x00" as *const u8 as
                              *const libc::c_char);
            }
            buffer = luaO_openspace(L, tl);
            tl = 0 as libc::c_int as lint32;
            i = n;
            while i > 0 as libc::c_int {
                /* concat all strings */
                let mut l: size_t =
                    (*(*top.offset(-(i as
                                         isize))).value.ts).len; /* got `n' strings to create 1 new */
                memcpy(buffer.offset(tl as isize) as *mut libc::c_void,
                       (*(*top.offset(-(i as
                                            isize))).value.ts).str_0.as_mut_ptr()
                           as *const libc::c_void, l);
                tl =
                    (tl as libc::c_ulong).wrapping_add(l) as lint32 as lint32;
                i -= 1
            }
            let ref mut fresh7 = (*top.offset(-(n as isize))).value.ts;
            *fresh7 = luaS_newlstr(L, buffer, tl)
        }
        total -= n - 1 as libc::c_int;
        top = top.offset(-((n - 1 as libc::c_int) as isize));
        if !(total > 1 as libc::c_int) { break ; }
    };
    /* repeat until only 1 result left */
}
unsafe extern "C" fn luaV_pack(mut L: *mut lua_State, mut firstelem: StkId) {
    let mut i: libc::c_int = 0;
    let mut htab: *mut Hash = luaH_new(L, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while firstelem.offset(i as isize) < (*L).top {
        *luaH_setint(L, htab, i + 1 as libc::c_int) =
            *firstelem.offset(i as isize);
        i += 1
    }
    /* store counter in field `n' */
    luaH_setstrnum(L, htab,
                   luaS_new(L, b"n\x00" as *const u8 as *const libc::c_char),
                   i as Number); /* remove elements from the stack */
    (*L).top = firstelem;
    (*(*L).top).ttype = 4 as libc::c_int;
    (*(*L).top).value.a = htab;
    if (*L).top == (*L).stack_last { luaD_checkstack(L, 1 as libc::c_int); }
    (*L).top = (*L).top.offset(1);
}
unsafe extern "C" fn adjust_varargs(mut L: *mut lua_State, mut base: StkId,
                                    mut nfixargs: libc::c_int) {
    let mut nvararg: libc::c_int =
        ((*L).top.wrapping_offset_from(base) as libc::c_long -
             nfixargs as libc::c_long) as libc::c_int;
    if nvararg < 0 as libc::c_int { luaD_adjusttop(L, base, nfixargs); }
    luaV_pack(L, base.offset(nfixargs as isize));
}
/*
** Executes the given Lua function. Parameters are between [base,top).
** Returns n such that the the results are between [n,top).
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_execute(mut L: *mut lua_State,
                                      mut cl: *const Closure, mut base: StkId)
 -> StkId {
    let tf: *const Proto = (*cl).f.l; /* keep top local, for performance */
    let mut top: StkId = 0 as *mut TObject;
    let mut pc: *const Instruction = (*tf).code;
    let kstr: *mut *mut TString = (*tf).kstr;
    let linehook: lua_Hook = (*L).linehook;
    let ref mut fresh8 =
        (*(*base.offset(-(1 as libc::c_int as isize))).value.i).pc;
    *fresh8 = &mut pc;
    luaD_checkstack(L, (*tf).maxstacksize as libc::c_int + 8 as libc::c_int);
    if (*tf).is_vararg != 0 {
        /* varargs? */
        adjust_varargs(L, base, (*tf).numparams as libc::c_int);
    } else { luaD_adjusttop(L, base, (*tf).numparams as libc::c_int); }
    top = (*L).top;
    loop 
         /* main loop of interpreter */
         {
        let fresh9 = pc; /* pop values */
        pc = pc.offset(1); /* final value of `top' (in case of errors) */
        let i: Instruction =
            *fresh9; /* final value of `top' (in case of errors) */
        if linehook.is_some() { traceexec(L, base, top, linehook); }
        match (i &
                   (!((!(0 as libc::c_int as Instruction)) <<
                          6 as libc::c_int)) << 0 as libc::c_int) as OpCode as
                  libc::c_uint {
            0 => { (*L).top = top; return top }
            1 => {
                (*L).top = top;
                return base.offset((i >> 6 as libc::c_int) as libc::c_int as
                                       isize)
            }
            2 => {
                let mut nres: libc::c_int =
                    (i >> 6 as libc::c_int &
                         (!((!(0 as libc::c_int as Instruction)) <<
                                9 as libc::c_int)) << 0 as libc::c_int) as
                        libc::c_int;
                if nres == 255 as libc::c_int { nres = -(1 as libc::c_int) }
                (*L).top = top;
                luaD_call(L,
                          base.offset((i >>
                                           6 as libc::c_int +
                                               9 as libc::c_int) as
                                          libc::c_int as isize), nres);
                top = (*L).top
            }
            3 => {
                (*L).top = top;
                luaD_call(L,
                          base.offset((i >>
                                           6 as libc::c_int +
                                               9 as libc::c_int) as
                                          libc::c_int as isize),
                          -(1 as libc::c_int));
                return base.offset((i >> 6 as libc::c_int &
                                        (!((!(0 as libc::c_int as
                                                  Instruction)) <<
                                               9 as libc::c_int)) <<
                                            0 as libc::c_int) as libc::c_int
                                       as isize)
            }
            4 => {
                let mut n: libc::c_int =
                    (i >> 6 as libc::c_int) as libc::c_int;
                loop  {
                    let fresh10 = top;
                    top = top.offset(1);
                    (*fresh10).ttype = 1 as libc::c_int;
                    n -= 1;
                    if !(n > 0 as libc::c_int) { break ; }
                }
            }
            5 => {
                top =
                    top.offset(-((i >> 6 as libc::c_int) as libc::c_int as
                                     isize))
            }
            6 => {
                (*top).ttype = 2 as libc::c_int;
                (*top).value.n =
                    ((i >> 6 as libc::c_int) as libc::c_int -
                         (((1 as libc::c_int) <<
                               32 as libc::c_int - 6 as libc::c_int) -
                              1 as libc::c_int >> 1 as libc::c_int)) as
                        Number;
                top = top.offset(1)
            }
            7 => {
                (*top).ttype = 3 as libc::c_int;
                (*top).value.ts =
                    *kstr.offset((i >> 6 as libc::c_int) as libc::c_int as
                                     isize);
                top = top.offset(1)
            }
            8 => {
                (*top).ttype = 2 as libc::c_int;
                (*top).value.n =
                    *(*tf).knum.offset((i >> 6 as libc::c_int) as libc::c_int
                                           as isize);
                top = top.offset(1)
            }
            9 => {
                (*top).ttype = 2 as libc::c_int;
                (*top).value.n =
                    -*(*tf).knum.offset((i >> 6 as libc::c_int) as libc::c_int
                                            as isize);
                top = top.offset(1)
            }
            10 => {
                let fresh11 = top;
                top = top.offset(1);
                *fresh11 =
                    *(*cl).upvalue.as_ptr().offset((i >> 6 as libc::c_int) as
                                                       libc::c_int as isize)
            }
            11 => {
                let fresh12 = top;
                top = top.offset(1);
                *fresh12 =
                    *base.offset((i >> 6 as libc::c_int) as libc::c_int as
                                     isize)
            }
            12 => {
                (*L).top = top;
                *top =
                    *luaV_getglobal(L,
                                    *kstr.offset((i >> 6 as libc::c_int) as
                                                     libc::c_int as isize));
                top = top.offset(1)
            }
            13 => {
                (*L).top = top;
                top = top.offset(-1);
                *top.offset(-(1 as libc::c_int as isize)) =
                    *luaV_gettable(L,
                                   top.offset(-(1 as libc::c_int as isize)))
            }
            14 => {
                (*top).ttype = 3 as libc::c_int;
                (*top).value.ts =
                    *kstr.offset((i >> 6 as libc::c_int) as libc::c_int as
                                     isize);
                (*L).top = top.offset(1 as libc::c_int as isize);
                *top.offset(-(1 as libc::c_int as isize)) =
                    *luaV_gettable(L,
                                   top.offset(-(1 as libc::c_int as isize)))
            }
            15 => {
                *top =
                    *base.offset((i >> 6 as libc::c_int) as libc::c_int as
                                     isize);
                (*L).top = top.offset(1 as libc::c_int as isize);
                *top.offset(-(1 as libc::c_int as isize)) =
                    *luaV_gettable(L,
                                   top.offset(-(1 as libc::c_int as isize)))
            }
            16 => {
                let mut receiver: TObject =
                    TObject{ttype: 0,
                            value:
                                Value{ts:
                                          0 as *const TString as
                                              *mut TString,},};
                receiver = *top.offset(-(1 as libc::c_int as isize));
                (*top).ttype = 3 as libc::c_int;
                let fresh13 = top;
                top = top.offset(1);
                (*fresh13).value.ts =
                    *kstr.offset((i >> 6 as libc::c_int) as libc::c_int as
                                     isize);
                (*L).top = top;
                *top.offset(-(2 as libc::c_int as isize)) =
                    *luaV_gettable(L,
                                   top.offset(-(2 as libc::c_int as isize)));
                *top.offset(-(1 as libc::c_int as isize)) = receiver
            }
            17 => {
                (*L).top = top;
                luaC_checkGC(L);
                (*top).value.a =
                    luaH_new(L, (i >> 6 as libc::c_int) as libc::c_int);
                (*top).ttype = 4 as libc::c_int;
                top = top.offset(1)
            }
            18 => {
                top = top.offset(-1);
                *base.offset((i >> 6 as libc::c_int) as libc::c_int as isize)
                    = *top
            }
            19 => {
                (*L).top = top;
                luaV_setglobal(L,
                               *kstr.offset((i >> 6 as libc::c_int) as
                                                libc::c_int as isize));
                top = top.offset(-1)
            }
            20 => {
                let mut t: StkId =
                    top.offset(-((i >> 6 as libc::c_int + 9 as libc::c_int) as
                                     libc::c_int as isize));
                (*L).top = top;
                luaV_settable(L, t, t.offset(1 as libc::c_int as isize));
                top =
                    top.offset(-((i >> 6 as libc::c_int &
                                      (!((!(0 as libc::c_int as Instruction))
                                             << 9 as libc::c_int)) <<
                                          0 as libc::c_int) as libc::c_int as
                                     isize))
            }
            21 => {
                let mut aux: libc::c_int =
                    (i >> 6 as libc::c_int + 9 as libc::c_int) as libc::c_int
                        * (250 as libc::c_int / 4 as libc::c_int);
                let mut n_0: libc::c_int =
                    (i >> 6 as libc::c_int &
                         (!((!(0 as libc::c_int as Instruction)) <<
                                9 as libc::c_int)) << 0 as libc::c_int) as
                        libc::c_int;
                let mut arr: *mut Hash =
                    (*top.offset(-(n_0 as
                                       isize)).offset(-(1 as libc::c_int as
                                                            isize))).value.a;
                (*L).top = top.offset(-(n_0 as isize));
                while n_0 != 0 {
                    top = top.offset(-1);
                    *luaH_setint(L, arr, n_0 + aux) = *top;
                    n_0 -= 1
                }
            }
            22 => {
                let mut n_1: libc::c_int =
                    (i >> 6 as libc::c_int) as libc::c_int;
                let mut finaltop: StkId =
                    top.offset(-((2 as libc::c_int * n_1) as isize));
                let mut arr_0: *mut Hash =
                    (*finaltop.offset(-(1 as libc::c_int as isize))).value.a;
                (*L).top = finaltop;
                while n_1 != 0 {
                    top = top.offset(-(2 as libc::c_int as isize));
                    *luaH_set(L, arr_0, top as *const TObject) =
                        *top.offset(1 as libc::c_int as isize);
                    n_1 -= 1
                }
            }
            23 => {
                if (*top.offset(-(2 as libc::c_int as isize))).ttype !=
                       2 as libc::c_int &&
                       luaV_tonumber(top.offset(-(2 as libc::c_int as isize)))
                           != 0 as libc::c_int ||
                       (*top.offset(-(1 as libc::c_int as isize))).ttype !=
                           2 as libc::c_int &&
                           luaV_tonumber(top.offset(-(1 as libc::c_int as
                                                          isize))) !=
                               0 as libc::c_int {
                    call_arith(L, top, TM_ADD);
                } else {
                    let ref mut fresh14 =
                        (*top.offset(-(2 as libc::c_int as isize))).value.n;
                    *fresh14 +=
                        (*top.offset(-(1 as libc::c_int as isize))).value.n
                }
                top = top.offset(-1)
            }
            24 => {
                if (*top.offset(-(1 as libc::c_int as isize))).ttype !=
                       2 as libc::c_int &&
                       luaV_tonumber(top.offset(-(1 as libc::c_int as isize)))
                           != 0 as libc::c_int {
                    (*top).ttype = 2 as libc::c_int;
                    (*top).value.n =
                        ((i >> 6 as libc::c_int) as libc::c_int -
                             (((1 as libc::c_int) <<
                                   32 as libc::c_int - 6 as libc::c_int) -
                                  1 as libc::c_int >> 1 as libc::c_int)) as
                            Number;
                    call_arith(L, top.offset(1 as libc::c_int as isize),
                               TM_ADD);
                } else {
                    let ref mut fresh15 =
                        (*top.offset(-(1 as libc::c_int as isize))).value.n;
                    *fresh15 +=
                        ((i >> 6 as libc::c_int) as libc::c_int -
                             (((1 as libc::c_int) <<
                                   32 as libc::c_int - 6 as libc::c_int) -
                                  1 as libc::c_int >> 1 as libc::c_int)) as
                            Number
                }
            }
            25 => {
                if (*top.offset(-(2 as libc::c_int as isize))).ttype !=
                       2 as libc::c_int &&
                       luaV_tonumber(top.offset(-(2 as libc::c_int as isize)))
                           != 0 as libc::c_int ||
                       (*top.offset(-(1 as libc::c_int as isize))).ttype !=
                           2 as libc::c_int &&
                           luaV_tonumber(top.offset(-(1 as libc::c_int as
                                                          isize))) !=
                               0 as libc::c_int {
                    call_arith(L, top, TM_SUB);
                } else {
                    let ref mut fresh16 =
                        (*top.offset(-(2 as libc::c_int as isize))).value.n;
                    *fresh16 -=
                        (*top.offset(-(1 as libc::c_int as isize))).value.n
                }
                top = top.offset(-1)
            }
            26 => {
                if (*top.offset(-(2 as libc::c_int as isize))).ttype !=
                       2 as libc::c_int &&
                       luaV_tonumber(top.offset(-(2 as libc::c_int as isize)))
                           != 0 as libc::c_int ||
                       (*top.offset(-(1 as libc::c_int as isize))).ttype !=
                           2 as libc::c_int &&
                           luaV_tonumber(top.offset(-(1 as libc::c_int as
                                                          isize))) !=
                               0 as libc::c_int {
                    call_arith(L, top, TM_MUL);
                } else {
                    let ref mut fresh17 =
                        (*top.offset(-(2 as libc::c_int as isize))).value.n;
                    *fresh17 *=
                        (*top.offset(-(1 as libc::c_int as isize))).value.n
                }
                top = top.offset(-1)
            }
            27 => {
                if (*top.offset(-(2 as libc::c_int as isize))).ttype !=
                       2 as libc::c_int &&
                       luaV_tonumber(top.offset(-(2 as libc::c_int as isize)))
                           != 0 as libc::c_int ||
                       (*top.offset(-(1 as libc::c_int as isize))).ttype !=
                           2 as libc::c_int &&
                           luaV_tonumber(top.offset(-(1 as libc::c_int as
                                                          isize))) !=
                               0 as libc::c_int {
                    call_arith(L, top, TM_DIV);
                } else {
                    let ref mut fresh18 =
                        (*top.offset(-(2 as libc::c_int as isize))).value.n;
                    *fresh18 /=
                        (*top.offset(-(1 as libc::c_int as isize))).value.n
                }
                top = top.offset(-1)
            }
            28 => {
                if call_binTM(L, top, TM_POW) == 0 {
                    lua_error(L,
                              b"undefined operation\x00" as *const u8 as
                                  *const libc::c_char);
                }
                top = top.offset(-1)
            }
            29 => {
                let mut n_2: libc::c_int =
                    (i >> 6 as libc::c_int) as libc::c_int;
                luaV_strconc(L, n_2, top);
                top = top.offset(-((n_2 - 1 as libc::c_int) as isize));
                (*L).top = top;
                luaC_checkGC(L);
            }
            30 => {
                if (*top.offset(-(1 as libc::c_int as isize))).ttype !=
                       2 as libc::c_int &&
                       luaV_tonumber(top.offset(-(1 as libc::c_int as isize)))
                           != 0 as libc::c_int {
                    (*top).ttype = 1 as libc::c_int;
                    call_arith(L, top.offset(1 as libc::c_int as isize),
                               TM_UNM);
                } else {
                    (*top.offset(-(1 as libc::c_int as isize))).value.n =
                        -(*top.offset(-(1 as libc::c_int as isize))).value.n
                }
            }
            31 => {
                (*top.offset(-(1 as libc::c_int as isize))).ttype =
                    if (*top.offset(-(1 as libc::c_int as isize))).ttype ==
                           1 as libc::c_int {
                        2 as libc::c_int
                    } else { 1 as libc::c_int };
                (*top.offset(-(1 as libc::c_int as isize))).value.n =
                    1 as libc::c_int as Number
            }
            32 => {
                top = top.offset(-(2 as libc::c_int as isize));
                if luaO_equalObj(top as *const TObject,
                                 top.offset(1 as libc::c_int as isize) as
                                     *const TObject) == 0 {
                    let mut d: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d as isize)
                }
            }
            33 => {
                top = top.offset(-(2 as libc::c_int as isize));
                if luaO_equalObj(top as *const TObject,
                                 top.offset(1 as libc::c_int as isize) as
                                     *const TObject) != 0 {
                    let mut d_0: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_0 as isize)
                }
            }
            34 => {
                top = top.offset(-(2 as libc::c_int as isize));
                if luaV_lessthan(L, top as *const TObject,
                                 top.offset(1 as libc::c_int as isize) as
                                     *const TObject,
                                 top.offset(2 as libc::c_int as isize)) != 0 {
                    let mut d_1: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_1 as isize)
                }
            }
            35 => {
                /* a <= b  ===  !(b<a) */
                top = top.offset(-(2 as libc::c_int as isize));
                if luaV_lessthan(L,
                                 top.offset(1 as libc::c_int as isize) as
                                     *const TObject, top as *const TObject,
                                 top.offset(2 as libc::c_int as isize)) == 0 {
                    let mut d_2: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_2 as isize)
                }
            }
            36 => {
                /* a > b  ===  (b<a) */
                top = top.offset(-(2 as libc::c_int as isize));
                if luaV_lessthan(L,
                                 top.offset(1 as libc::c_int as isize) as
                                     *const TObject, top as *const TObject,
                                 top.offset(2 as libc::c_int as isize)) != 0 {
                    let mut d_3: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_3 as isize)
                }
            }
            37 => {
                /* a >= b  ===  !(a<b) */
                top = top.offset(-(2 as libc::c_int as isize));
                if luaV_lessthan(L, top as *const TObject,
                                 top.offset(1 as libc::c_int as isize) as
                                     *const TObject,
                                 top.offset(2 as libc::c_int as isize)) == 0 {
                    let mut d_4: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_4 as isize)
                }
            }
            38 => {
                top = top.offset(-1);
                if (*top).ttype != 1 as libc::c_int {
                    let mut d_5: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_5 as isize)
                }
            }
            39 => {
                top = top.offset(-1);
                if (*top).ttype == 1 as libc::c_int {
                    let mut d_6: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_6 as isize)
                }
            }
            40 => {
                if (*top.offset(-(1 as libc::c_int as isize))).ttype ==
                       1 as libc::c_int {
                    top = top.offset(-1)
                } else {
                    let mut d_7: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_7 as isize)
                }
            }
            41 => {
                if (*top.offset(-(1 as libc::c_int as isize))).ttype !=
                       1 as libc::c_int {
                    top = top.offset(-1)
                } else {
                    let mut d_8: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_8 as isize)
                }
            }
            42 => {
                let mut d_9: libc::c_int =
                    (i >> 6 as libc::c_int) as libc::c_int -
                        (((1 as libc::c_int) <<
                              32 as libc::c_int - 6 as libc::c_int) -
                             1 as libc::c_int >> 1 as libc::c_int);
                pc = pc.offset(d_9 as isize)
            }
            43 => {
                let fresh19 = top;
                top = top.offset(1);
                (*fresh19).ttype = 1 as libc::c_int;
                pc = pc.offset(1)
            }
            44 => {
                if (*top.offset(-(1 as libc::c_int as isize))).ttype !=
                       2 as libc::c_int &&
                       luaV_tonumber(top.offset(-(1 as libc::c_int as isize)))
                           != 0 as libc::c_int {
                    lua_error(L,
                              b"`for\' step must be a number\x00" as *const u8
                                  as *const libc::c_char);
                }
                if (*top.offset(-(2 as libc::c_int as isize))).ttype !=
                       2 as libc::c_int &&
                       luaV_tonumber(top.offset(-(2 as libc::c_int as isize)))
                           != 0 as libc::c_int {
                    lua_error(L,
                              b"`for\' limit must be a number\x00" as
                                  *const u8 as *const libc::c_char);
                }
                if (*top.offset(-(3 as libc::c_int as isize))).ttype !=
                       2 as libc::c_int &&
                       luaV_tonumber(top.offset(-(3 as libc::c_int as isize)))
                           != 0 as libc::c_int {
                    lua_error(L,
                              b"`for\' initial value must be a number\x00" as
                                  *const u8 as *const libc::c_char);
                }
                if if (*top.offset(-(1 as libc::c_int as isize))).value.n >
                          0 as libc::c_int as libc::c_long {
                       ((*top.offset(-(3 as libc::c_int as isize))).value.n >
                            (*top.offset(-(2 as libc::c_int as
                                               isize))).value.n) as
                           libc::c_int
                   } else {
                       ((*top.offset(-(3 as libc::c_int as isize))).value.n <
                            (*top.offset(-(2 as libc::c_int as
                                               isize))).value.n) as
                           libc::c_int
                   } != 0 {
                    /* `empty' loop? */
                    top = top.offset(-(3 as libc::c_int as isize));
                    let mut d_10: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc =
                        pc.offset(d_10 as
                                      isize) /* remove control variables */
                    /* jump to loop end */
                }
            }
            45 => {
                if (*top.offset(-(3 as libc::c_int as isize))).ttype !=
                       2 as libc::c_int {
                    lua_error(L,
                              b"`for\' index must be a number\x00" as
                                  *const u8 as
                                  *const libc::c_char); /* increment index */
                } /* end loop: remove control variables */
                let ref mut fresh20 =
                    (*top.offset(-(3 as libc::c_int as isize))).value.n;
                *fresh20 +=
                    (*top.offset(-(1 as libc::c_int as isize))).value.n;
                if if (*top.offset(-(1 as libc::c_int as isize))).value.n >
                          0 as libc::c_int as libc::c_long {
                       ((*top.offset(-(3 as libc::c_int as isize))).value.n >
                            (*top.offset(-(2 as libc::c_int as
                                               isize))).value.n) as
                           libc::c_int
                   } else {
                       ((*top.offset(-(3 as libc::c_int as isize))).value.n <
                            (*top.offset(-(2 as libc::c_int as
                                               isize))).value.n) as
                           libc::c_int
                   } != 0 {
                    top = top.offset(-(3 as libc::c_int as isize))
                } else {
                    let mut d_11: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_11 as isize)
                }
            }
            46 => {
                let mut node: *mut Node = 0 as *mut Node;
                if (*top.offset(-(1 as libc::c_int as isize))).ttype !=
                       4 as libc::c_int {
                    lua_error(L,
                              b"`for\' table must be a table\x00" as *const u8
                                  as *const libc::c_char);
                }
                node =
                    luaH_next(L,
                              (*top.offset(-(1 as libc::c_int as
                                                 isize))).value.a,
                              &luaO_nilobject);
                if node.is_null() {
                    /* `empty' loop? */
                    top = top.offset(-1);
                    let mut d_12: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc = pc.offset(d_12 as isize) /* remove table */
                    /* jump to loop end */
                } else {
                    top =
                        top.offset(2 as libc::c_int as
                                       isize); /* index,value */
                    *top.offset(-(2 as libc::c_int as isize)) = (*node).key;
                    *top.offset(-(1 as libc::c_int as isize)) = (*node).val
                }
            }
            47 => {
                let mut node_0: *mut Node = 0 as *mut Node;
                node_0 =
                    luaH_next(L,
                              (*top.offset(-(3 as libc::c_int as
                                                 isize))).value.a,
                              top.offset(-(2 as libc::c_int as isize)) as
                                  *const TObject);
                if node_0.is_null() {
                    /* end loop? */
                    top = top.offset(-(3 as libc::c_int as isize))
                } else {
                    *top.offset(-(2 as libc::c_int as isize)) = (*node_0).key;
                    *top.offset(-(1 as libc::c_int as isize)) = (*node_0).val;
                    let mut d_13: libc::c_int =
                        (i >> 6 as libc::c_int) as libc::c_int -
                            (((1 as libc::c_int) <<
                                  32 as libc::c_int - 6 as libc::c_int) -
                                 1 as libc::c_int >> 1 as libc::c_int);
                    pc =
                        pc.offset(d_13 as
                                      isize) /* remove table, key, and value */
                    /* repeat loop */
                }
            }
            48 => {
                (*L).top = top;
                luaV_Lclosure(L,
                              *(*tf).kproto.offset((i >>
                                                        6 as libc::c_int +
                                                            9 as libc::c_int)
                                                       as libc::c_int as
                                                       isize),
                              (i >> 6 as libc::c_int &
                                   (!((!(0 as libc::c_int as Instruction)) <<
                                          9 as libc::c_int)) <<
                                       0 as libc::c_int) as libc::c_int);
                top = (*L).top;
                luaC_checkGC(L);
            }
            _ => { }
        }
    };
}
