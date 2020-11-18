use ::libc;
extern "C" {
    pub type TM;
    pub type lua_longjmp;
    pub type Breaklabel;
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaX_error(ls: *mut LexState, s: *const libc::c_char,
                  token: libc::c_int);
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
pub struct stringtable {
    pub size: libc::c_int,
    pub nuse: lint32,
    pub hash: *mut *mut TString,
}
pub type lint32 = libc::c_ulong;
pub type StkId = *mut TObject;
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
pub type ZIO = zio;
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    pub r: Number,
    pub ts: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: libc::c_int,
    pub seminfo: SemInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: libc::c_int,
    pub t: Token,
    pub lookahead: Token,
    pub fs: *mut FuncState,
    pub L: *mut lua_State,
    pub z: *mut zio,
    pub linenumber: libc::c_int,
    pub lastline: libc::c_int,
    pub source: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncState {
    pub f: *mut Proto,
    pub prev: *mut FuncState,
    pub ls: *mut LexState,
    pub L: *mut lua_State,
    pub pc: libc::c_int,
    pub lasttarget: libc::c_int,
    pub jlt: libc::c_int,
    pub stacklevel: libc::c_short,
    pub nactloc: libc::c_short,
    pub nupvalues: libc::c_short,
    pub lastline: libc::c_int,
    pub bl: *mut Breaklabel,
    pub upvalues: [expdesc; 32],
    pub actloc: [libc::c_int; 200],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expdesc {
    pub k: expkind,
    pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub index: libc::c_int,
    pub l: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub t: libc::c_int,
    pub f: libc::c_int,
}
pub type expkind = libc::c_uint;
pub const VEXP: expkind = 3;
pub const VINDEXED: expkind = 2;
pub const VLOCAL: expkind = 1;
pub const VGLOBAL: expkind = 0;
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
pub type BinOpr = libc::c_uint;
pub const OPR_NOBINOPR: BinOpr = 14;
pub const OPR_OR: BinOpr = 13;
pub const OPR_AND: BinOpr = 12;
pub const OPR_GE: BinOpr = 11;
pub const OPR_GT: BinOpr = 10;
pub const OPR_LE: BinOpr = 9;
pub const OPR_LT: BinOpr = 8;
pub const OPR_EQ: BinOpr = 7;
pub const OPR_NE: BinOpr = 6;
pub const OPR_CONCAT: BinOpr = 5;
pub const OPR_POW: BinOpr = 4;
pub const OPR_DIV: BinOpr = 3;
pub const OPR_MULT: BinOpr = 2;
pub const OPR_SUB: BinOpr = 1;
pub const OPR_ADD: BinOpr = 0;
pub type UnOpr = libc::c_uint;
pub const OPR_NOUNOPR: UnOpr = 2;
pub const OPR_NOT: UnOpr = 1;
pub const OPR_MINUS: UnOpr = 0;
pub type Mode = libc::c_uint;
pub const iAB: Mode = 3;
pub const iS: Mode = 2;
pub const iU: Mode = 1;
pub const iO: Mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpProperties {
    pub mode: libc::c_char,
    pub push: libc::c_uchar,
    pub pop: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub opcode: OpCode,
    pub arg: libc::c_int,
}
/*
** $Id: lcode.c,v 1.4 2004/06/04 13:42:10 neil Exp $
** Code generator for Lua
** See Copyright Notice in lua.h
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_error(mut ls: *mut LexState,
                                    mut msg: *const libc::c_char) {
    luaX_error(ls, msg, (*ls).t.token);
}
/*
** Returns the the previous instruction, for optimizations.
** If there is a jump target between this and the current instruction,
** returns a dummy instruction to avoid wrong optimizations.
*/
unsafe extern "C" fn previous_instruction(mut fs: *mut FuncState)
 -> Instruction {
    if (*fs).pc > (*fs).lasttarget {
        /* no jumps to current position? */
        return *(*(*fs).f).code.offset(((*fs).pc - 1 as libc::c_int) as isize)
    } else {
        return OP_END as libc::c_int as Instruction
    }; /* returns previous instruction */
    /* no optimizations after an `END' */
}
#[no_mangle]
pub unsafe extern "C" fn luaK_jump(mut fs: *mut FuncState) -> libc::c_int {
    let mut j: libc::c_int = luaK_code1(fs, OP_JMP, -(1 as libc::c_int));
    if j == (*fs).lasttarget {
        /* possible jumps to this jump? */
        luaK_concat(fs, &mut j, (*fs).jlt); /* keep them on hold */
        (*fs).jlt = -(1 as libc::c_int)
    } /* point to itself to represent end of list */
    return j;
}
unsafe extern "C" fn luaK_fixjump(mut fs: *mut FuncState, mut pc: libc::c_int,
                                  mut dest: libc::c_int) {
    let mut jmp: *mut Instruction =
        &mut *(*(*fs).f).code.offset(pc as isize) as *mut Instruction;
    if dest == -(1 as libc::c_int) {
        *jmp =
            *jmp &
                !((!((!(0 as libc::c_int as Instruction)) <<
                         32 as libc::c_int - 6 as libc::c_int)) <<
                      6 as libc::c_int) |
                ((-(1 as libc::c_int) +
                      (((1 as libc::c_int) <<
                            32 as libc::c_int - 6 as libc::c_int) -
                           1 as libc::c_int >> 1 as libc::c_int)) as
                     Instruction) << 6 as libc::c_int
    } else {
        /* jump is relative to position following jump instruction */
        let mut offset: libc::c_int = dest - (pc + 1 as libc::c_int);
        if abs(offset) >
               ((1 as libc::c_int) << 32 as libc::c_int - 6 as libc::c_int) -
                   1 as libc::c_int >> 1 as libc::c_int {
            luaK_error((*fs).ls,
                       b"control structure too long\x00" as *const u8 as
                           *const libc::c_char);
        }
        *jmp =
            *jmp &
                !((!((!(0 as libc::c_int as Instruction)) <<
                         32 as libc::c_int - 6 as libc::c_int)) <<
                      6 as libc::c_int) |
                ((offset +
                      (((1 as libc::c_int) <<
                            32 as libc::c_int - 6 as libc::c_int) -
                           1 as libc::c_int >> 1 as libc::c_int)) as
                     Instruction) << 6 as libc::c_int
    };
}
unsafe extern "C" fn luaK_getjump(mut fs: *mut FuncState, mut pc: libc::c_int)
 -> libc::c_int {
    let mut offset: libc::c_int =
        (*(*(*fs).f).code.offset(pc as isize) >> 6 as libc::c_int) as
            libc::c_int -
            (((1 as libc::c_int) << 32 as libc::c_int - 6 as libc::c_int) -
                 1 as libc::c_int >> 1 as libc::c_int);
    if offset == -(1 as libc::c_int) {
        /* point to itself represents end of list */
        return -(1 as libc::c_int)
    } else { return pc + 1 as libc::c_int + offset }; /* end of list */
    /* turn offset into absolute position */
}
/*
** returns current `pc' and marks it as a jump target (to avoid wrong
** optimizations with consecutive instructions not in the same basic block).
** discharge list of jumps to last target.
*/
#[no_mangle]
pub unsafe extern "C" fn luaK_getlabel(mut fs: *mut FuncState)
 -> libc::c_int {
    if (*fs).pc != (*fs).lasttarget {
        let mut lasttarget: libc::c_int = (*fs).lasttarget;
        (*fs).lasttarget = (*fs).pc;
        /* nobody jumps to this new label (yet) */
        luaK_patchlist(fs, (*fs).jlt,
                       lasttarget); /* discharge old list `jlt' */
        (*fs).jlt = -(1 as libc::c_int)
    }
    return (*fs).pc;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_deltastack(mut fs: *mut FuncState,
                                         mut delta: libc::c_int) {
    (*fs).stacklevel =
        ((*fs).stacklevel as libc::c_int + delta) as libc::c_short;
    if (*fs).stacklevel as libc::c_int >
           (*(*fs).f).maxstacksize as libc::c_int {
        if (*fs).stacklevel as libc::c_int > 250 as libc::c_int {
            luaK_error((*fs).ls,
                       b"function or expression too complex\x00" as *const u8
                           as *const libc::c_char);
        }
        (*(*fs).f).maxstacksize = (*fs).stacklevel
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_kstr(mut ls: *mut LexState,
                                   mut c: libc::c_int) {
    luaK_code1((*ls).fs, OP_PUSHSTRING, c);
}
unsafe extern "C" fn number_constant(mut fs: *mut FuncState, mut r: Number)
 -> libc::c_int {
    /* check whether `r' has appeared within the last LOOKBACKNUMS entries */
    let mut f: *mut Proto = (*fs).f;
    let mut c: libc::c_int = (*f).nknum;
    let mut lim: libc::c_int =
        if c < 20 as libc::c_int {
            0 as libc::c_int
        } else { (c) - 20 as libc::c_int };
    loop  {
        c -= 1;
        if !(c >= lim) { break ; }
        if *(*f).knum.offset(c as isize) == r { return c }
    }
    /* not found; create a new entry */
    (*f).knum =
        luaM_growaux((*fs).L, (*f).knum as *mut libc::c_void,
                     (*f).nknum as size_t, 1 as libc::c_int,
                     ::std::mem::size_of::<Number>() as libc::c_ulong,
                     b"constant table overflow\x00" as *const u8 as
                         *const libc::c_char,
                     (((1 as libc::c_int) <<
                           32 as libc::c_int - 6 as libc::c_int) -
                          1 as libc::c_int) as size_t) as
            *mut Number; /* f has a short integer value */
    let fresh0 = (*f).nknum;
    (*f).nknum = (*f).nknum + 1;
    c = fresh0;
    *(*f).knum.offset(c as isize) = r;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaK_number(mut fs: *mut FuncState, mut f: Number) {
    if f <=
           (((1 as libc::c_int) << 32 as libc::c_int - 6 as libc::c_int) -
                1 as libc::c_int >> 1 as libc::c_int) as Number &&
           f as libc::c_int as Number == f {
        luaK_code1(fs, OP_PUSHINT, f as libc::c_int);
    } else { luaK_code1(fs, OP_PUSHNUM, number_constant(fs, f)); };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_adjuststack(mut fs: *mut FuncState,
                                          mut n: libc::c_int) {
    if n > 0 as libc::c_int {
        luaK_code1(fs, OP_POP, n);
    } else { luaK_code1(fs, OP_PUSHNIL, -n); };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_lastisopen(mut fs: *mut FuncState)
 -> libc::c_int {
    /* check whether last instruction is an open function call */
    let mut i: Instruction = previous_instruction(fs);
    if (i &
            (!((!(0 as libc::c_int as Instruction)) << 6 as libc::c_int)) <<
                0 as libc::c_int) as OpCode as libc::c_uint ==
           OP_CALL as libc::c_int as libc::c_uint &&
           (i >> 6 as libc::c_int &
                (!((!(0 as libc::c_int as Instruction)) << 9 as libc::c_int))
                    << 0 as libc::c_int) as libc::c_int == 255 as libc::c_int
       {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_setcallreturns(mut fs: *mut FuncState,
                                             mut nresults: libc::c_int) {
    if luaK_lastisopen(fs) != 0 {
        /* expression is an open function call? */
        *(*(*fs).f).code.offset(((*fs).pc - 1 as libc::c_int) as isize) =
            *(*(*fs).f).code.offset(((*fs).pc - 1 as libc::c_int) as isize) &
                !((!((!(0 as libc::c_int as Instruction)) <<
                         9 as libc::c_int)) << 6 as libc::c_int) |
                (nresults as Instruction) << 6 as libc::c_int;
        luaK_deltastack(fs, nresults); /* set number of results */
        /* push results */
    };
}
unsafe extern "C" fn discharge(mut fs: *mut FuncState, mut var: *mut expdesc)
 -> libc::c_int {
    match (*var).k as libc::c_uint {
        1 => {
            luaK_code1(fs, OP_GETLOCAL, (*var).u.index);
            /* nothing to do */
        }
        0 => { luaK_code1(fs, OP_GETGLOBAL, (*var).u.index); }
        2 => { luaK_code0(fs, OP_GETTABLE); }
        3 => { return 0 as libc::c_int }
        _ => { }
    }
    (*var).k = VEXP;
    (*var).u.l.f = -(1 as libc::c_int);
    (*var).u.l.t = (*var).u.l.f;
    return 1 as libc::c_int;
}
unsafe extern "C" fn discharge1(mut fs: *mut FuncState,
                                mut var: *mut expdesc) {
    discharge(fs, var);
    /* if it has jumps then it is already discharged */
    if (*var).u.l.t == -(1 as libc::c_int) &&
           (*var).u.l.f == -(1 as libc::c_int) {
        luaK_setcallreturns(fs, 1 as libc::c_int);
    };
    /* call must return 1 value */
}
#[no_mangle]
pub unsafe extern "C" fn luaK_storevar(mut ls: *mut LexState,
                                       mut var: *const expdesc) {
    let mut fs: *mut FuncState = (*ls).fs;
    match (*var).k as libc::c_uint {
        1 => { luaK_code1(fs, OP_SETLOCAL, (*var).u.index); }
        0 => { luaK_code1(fs, OP_SETGLOBAL, (*var).u.index); }
        2 => {
            /* table is at top-3; pop 3 elements after operation */
            luaK_code2(fs, OP_SETTABLE, 3 as libc::c_int, 3 as libc::c_int);
        }
        _ => { }
    };
}
unsafe extern "C" fn invertjump(mut op: OpCode) -> OpCode {
    match op as libc::c_uint {
        32 => {
            return OP_JMPEQ
            /* to avoid warnings */
        }
        33 => { return OP_JMPNE }
        34 => { return OP_JMPGE }
        35 => { return OP_JMPGT }
        36 => { return OP_JMPLE }
        37 => { return OP_JMPLT }
        38 | 40 => { return OP_JMPF }
        39 | 41 => { return OP_JMPT }
        _ => { return OP_END }
    };
}
unsafe extern "C" fn luaK_patchlistaux(mut fs: *mut FuncState,
                                       mut list: libc::c_int,
                                       mut target: libc::c_int,
                                       mut special: OpCode,
                                       mut special_target: libc::c_int) {
    let mut code: *mut Instruction = (*(*fs).f).code;
    while list != -(1 as libc::c_int) {
        let mut next: libc::c_int = luaK_getjump(fs, list);
        let mut i: *mut Instruction =
            &mut *code.offset(list as isize) as *mut Instruction;
        let mut op: OpCode =
            (*i &
                 (!((!(0 as libc::c_int as Instruction)) << 6 as libc::c_int))
                     << 0 as libc::c_int) as OpCode;
        if op as libc::c_uint == special as libc::c_uint {
            /* this `op' already has a value */
            luaK_fixjump(fs, list, special_target); /* do the patch */
        } else {
            luaK_fixjump(fs, list, target);
            if op as libc::c_uint == OP_JMPONT as libc::c_int as libc::c_uint
               {
                /* remove eventual values */
                *i =
                    *i &
                        !((!((!(0 as libc::c_int as Instruction)) <<
                                 6 as libc::c_int)) << 0 as libc::c_int) |
                        OP_JMPT as libc::c_int as Instruction
            } else if op as libc::c_uint ==
                          OP_JMPONF as libc::c_int as libc::c_uint {
                *i =
                    *i &
                        !((!((!(0 as libc::c_int as Instruction)) <<
                                 6 as libc::c_int)) << 0 as libc::c_int) |
                        OP_JMPF as libc::c_int as Instruction
            }
        }
        list = next
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_patchlist(mut fs: *mut FuncState,
                                        mut list: libc::c_int,
                                        mut target: libc::c_int) {
    if target == (*fs).lasttarget {
        /* same target that list `jlt'? */
        luaK_concat(fs, &mut (*fs).jlt, list); /* delay fixing */
    } else { luaK_patchlistaux(fs, list, target, OP_END, 0 as libc::c_int); };
}
unsafe extern "C" fn need_value(mut fs: *mut FuncState, mut list: libc::c_int,
                                mut hasvalue: OpCode) -> libc::c_int {
    /* check whether list has a jump without a value */
    while list != -(1 as libc::c_int) {
        if (*(*(*fs).f).code.offset(list as isize) &
                (!((!(0 as libc::c_int as Instruction)) << 6 as libc::c_int))
                    << 0 as libc::c_int) as OpCode as libc::c_uint !=
               hasvalue as libc::c_uint {
            return 1 as libc::c_int
        }
        list = luaK_getjump(fs, list)
    }
    return 0 as libc::c_int;
    /* not found */
}
#[no_mangle]
pub unsafe extern "C" fn luaK_concat(mut fs: *mut FuncState,
                                     mut l1: *mut libc::c_int,
                                     mut l2: libc::c_int) {
    if *l1 == -(1 as libc::c_int) {
        *l1 = l2
    } else {
        let mut list: libc::c_int = *l1;
        loop  {
            /* traverse `l1' */
            let mut next: libc::c_int = luaK_getjump(fs, list);
            if next == -(1 as libc::c_int) {
                /* end of list? */
                luaK_fixjump(fs, list, l2); /* position of last instruction */
                return
            } /* go if false */
            list = next
        }
    };
}
unsafe extern "C" fn luaK_testgo(mut fs: *mut FuncState, mut v: *mut expdesc,
                                 mut invert: libc::c_int, mut jump: OpCode) {
    let mut prevpos: libc::c_int = 0;
    let mut previous: *mut Instruction = 0 as *mut Instruction;
    let mut golist: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut exitlist: *mut libc::c_int = 0 as *mut libc::c_int;
    if invert == 0 {
        golist = &mut (*v).u.l.f;
        exitlist = &mut (*v).u.l.t
        /* exit if true */
    } else {
        golist = &mut (*v).u.l.t;
        exitlist = &mut (*v).u.l.f /* go if true */
        /* exit if false */
    }
    discharge1(fs, v);
    prevpos = (*fs).pc - 1 as libc::c_int;
    previous =
        &mut *(*(*fs).f).code.offset(prevpos as isize) as *mut Instruction;
    if !(OP_JMPNE as libc::c_int as libc::c_uint <=
             (*previous &
                  (!((!(0 as libc::c_int as Instruction)) <<
                         6 as libc::c_int)) << 0 as libc::c_int) as OpCode as
                 libc::c_uint &&
             (*previous &
                  (!((!(0 as libc::c_int as Instruction)) <<
                         6 as libc::c_int)) << 0 as libc::c_int) as OpCode as
                 libc::c_uint <= OP_JMP as libc::c_int as libc::c_uint) {
        prevpos = luaK_code1(fs, jump, -(1 as libc::c_int))
    } else if invert != 0 {
        *previous =
            *previous &
                !((!((!(0 as libc::c_int as Instruction)) <<
                         6 as libc::c_int)) << 0 as libc::c_int) |
                invertjump((*previous &
                                (!((!(0 as libc::c_int as Instruction)) <<
                                       6 as libc::c_int)) << 0 as libc::c_int)
                               as OpCode) as Instruction
    }
    /* last instruction is already a jump */
    luaK_concat(fs, exitlist, prevpos); /* insert last jump in `exitlist' */
    luaK_patchlist(fs, *golist,
                   luaK_getlabel(fs)); /* those instructions may be jump targets */
    *golist = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_goiftrue(mut fs: *mut FuncState,
                                       mut v: *mut expdesc,
                                       mut keepvalue: libc::c_int) {
    luaK_testgo(fs, v, 1 as libc::c_int,
                if keepvalue != 0 {
                    OP_JMPONF as libc::c_int
                } else { OP_JMPF as libc::c_int } as OpCode);
}
unsafe extern "C" fn luaK_goiffalse(mut fs: *mut FuncState,
                                    mut v: *mut expdesc,
                                    mut keepvalue: libc::c_int) {
    luaK_testgo(fs, v, 0 as libc::c_int,
                if keepvalue != 0 {
                    OP_JMPONT as libc::c_int
                } else { OP_JMPT as libc::c_int } as OpCode);
}
unsafe extern "C" fn code_label(mut fs: *mut FuncState, mut op: OpCode,
                                mut arg: libc::c_int) -> libc::c_int {
    luaK_getlabel(fs);
    return luaK_code1(fs, op, arg);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_tostack(mut ls: *mut LexState,
                                      mut v: *mut expdesc,
                                      mut onlyone: libc::c_int) {
    let mut fs: *mut FuncState = (*ls).fs;
    if discharge(fs, v) == 0 {
        /* `v' is an expression? */
        let mut previous: OpCode =
            (*(*(*fs).f).code.offset(((*fs).pc - 1 as libc::c_int) as isize) &
                 (!((!(0 as libc::c_int as Instruction)) << 6 as libc::c_int))
                     << 0 as libc::c_int) as OpCode;
        if !(OP_JMPNE as libc::c_int as libc::c_uint <=
                 previous as libc::c_uint &&
                 previous as libc::c_uint <=
                     OP_JMP as libc::c_int as libc::c_uint) &&
               (*v).u.l.f == -(1 as libc::c_int) &&
               (*v).u.l.t == -(1 as libc::c_int) {
            /* expression has no jumps */
            if onlyone != 0 { luaK_setcallreturns(fs, 1 as libc::c_int); }
            /* call must return 1 value */
        } else {
            /* expression has jumps */
            let mut final_0: libc::c_int =
                0; /* position after whole expression */
            let mut j: libc::c_int =
                -(1 as libc::c_int); /*  eventual  jump over values */
            let mut p_nil: libc::c_int =
                -(1 as libc::c_int); /* position of an eventual PUSHNIL */
            let mut p_1: libc::c_int =
                -(1 as libc::c_int); /* position of an eventual PUSHINT */
            if OP_JMPNE as libc::c_int as libc::c_uint <=
                   previous as libc::c_uint &&
                   previous as libc::c_uint <=
                       OP_JMP as libc::c_int as libc::c_uint ||
                   need_value(fs, (*v).u.l.f, OP_JMPONF) != 0 ||
                   need_value(fs, (*v).u.l.t, OP_JMPONT) != 0 {
                /* expression needs values */
                if OP_JMPNE as libc::c_int as libc::c_uint <=
                       previous as libc::c_uint &&
                       previous as libc::c_uint <=
                           OP_JMP as libc::c_int as libc::c_uint {
                    luaK_concat(fs, &mut (*v).u.l.t,
                                (*fs).pc -
                                    1 as
                                        libc::c_int); /* put `previous' in t. list */
                } else {
                    j =
                        code_label(fs, OP_JMP,
                                   -(1 as
                                         libc::c_int)); /* to jump over both pushes */
                    /* correct stack for compiler and symbolic execution */
                    luaK_adjuststack(fs, 1 as libc::c_int);
                }
                p_nil = code_label(fs, OP_PUSHNILJMP, 0 as libc::c_int);
                p_1 = code_label(fs, OP_PUSHINT, 1 as libc::c_int);
                luaK_patchlist(fs, j, luaK_getlabel(fs));
            }
            final_0 = luaK_getlabel(fs);
            luaK_patchlistaux(fs, (*v).u.l.f, p_nil, OP_JMPONF, final_0);
            luaK_patchlistaux(fs, (*v).u.l.t, p_1, OP_JMPONT, final_0);
            (*v).u.l.t = -(1 as libc::c_int);
            (*v).u.l.f = (*v).u.l.t
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_prefix(mut ls: *mut LexState, mut op: UnOpr,
                                     mut v: *mut expdesc) {
    let mut fs: *mut FuncState = (*ls).fs;
    if op as libc::c_uint == OPR_MINUS as libc::c_int as libc::c_uint {
        luaK_tostack(ls, v, 1 as libc::c_int);
        luaK_code0(fs, OP_MINUS);
    } else {
        /* op == NOT */
        let mut previous: *mut Instruction = 0 as *mut Instruction;
        discharge1(fs, v);
        previous =
            &mut *(*(*fs).f).code.offset(((*fs).pc - 1 as libc::c_int) as
                                             isize) as *mut Instruction;
        if OP_JMPNE as libc::c_int as libc::c_uint <=
               (*previous &
                    (!((!(0 as libc::c_int as Instruction)) <<
                           6 as libc::c_int)) << 0 as libc::c_int) as OpCode
                   as libc::c_uint &&
               (*previous &
                    (!((!(0 as libc::c_int as Instruction)) <<
                           6 as libc::c_int)) << 0 as libc::c_int) as OpCode
                   as libc::c_uint <= OP_JMP as libc::c_int as libc::c_uint {
            *previous =
                *previous &
                    !((!((!(0 as libc::c_int as Instruction)) <<
                             6 as libc::c_int)) << 0 as libc::c_int) |
                    invertjump((*previous &
                                    (!((!(0 as libc::c_int as Instruction)) <<
                                           6 as libc::c_int)) <<
                                        0 as libc::c_int) as OpCode) as
                        Instruction
        } else { luaK_code0(fs, OP_NOT); }
        /* interchange true and false lists */
        let mut temp: libc::c_int = (*v).u.l.f;
        (*v).u.l.f = (*v).u.l.t;
        (*v).u.l.t = temp
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_infix(mut ls: *mut LexState, mut op: BinOpr,
                                    mut v: *mut expdesc) {
    let mut fs: *mut FuncState = (*ls).fs;
    match op as libc::c_uint {
        12 => {
            luaK_goiftrue(fs, v, 1 as libc::c_int);
            /* all other binary operators need a value */
        }
        13 => { luaK_goiffalse(fs, v, 1 as libc::c_int); }
        _ => { luaK_tostack(ls, v, 1 as libc::c_int); }
    };
}
/* default argument for the opcode */
static mut codes: [C2RustUnnamed_5; 12] =
    [{
         let mut init =
             C2RustUnnamed_5{opcode: OP_ADD,
                             arg:
                                 0 as
                                     libc::c_int,}; /* `v2' must be a value */
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_SUB,
                             arg:
                                 0 as
                                     libc::c_int,}; /* 1 when there is an optimization */
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_MULT,
                             arg: 0 as libc::c_int,}; /* nothing to do */
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_DIV,
                             arg: 0 as libc::c_int,}; /* nothing to do */
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_POW,
                             arg: 0 as libc::c_int,}; /* nothing to do */
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_CONCAT,
                             arg: 2 as libc::c_int,}; /* nothing to do */
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_JMPNE, arg: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_JMPEQ, arg: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_JMPLT, arg: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_JMPLE, arg: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_JMPGT, arg: -(1 as libc::c_int),};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{opcode: OP_JMPGE, arg: -(1 as libc::c_int),};
         init
     }];
#[no_mangle]
pub unsafe extern "C" fn luaK_posfix(mut ls: *mut LexState, mut op: BinOpr,
                                     mut v1: *mut expdesc,
                                     mut v2: *mut expdesc) {
    let mut fs: *mut FuncState = (*ls).fs;
    match op as libc::c_uint {
        12 => {
            discharge1(fs, v2);
            (*v1).u.l.t = (*v2).u.l.t;
            luaK_concat(fs, &mut (*v1).u.l.f, (*v2).u.l.f);
        }
        13 => {
            discharge1(fs, v2);
            (*v1).u.l.f = (*v2).u.l.f;
            luaK_concat(fs, &mut (*v1).u.l.t, (*v2).u.l.t);
        }
        _ => {
            luaK_tostack(ls, v2, 1 as libc::c_int);
            luaK_code1(fs, codes[op as usize].opcode, codes[op as usize].arg);
        }
    };
}
unsafe extern "C" fn codelineinfo(mut fs: *mut FuncState) {
    let mut f: *mut Proto = (*fs).f;
    let mut ls: *mut LexState = (*fs).ls;
    if (*ls).lastline > (*fs).lastline {
        (*f).lineinfo =
            luaM_growaux((*fs).L, (*f).lineinfo as *mut libc::c_void,
                         (*f).nlineinfo as size_t, 2 as libc::c_int,
                         ::std::mem::size_of::<libc::c_int>() as
                             libc::c_ulong,
                         b"line info overflow\x00" as *const u8 as
                             *const libc::c_char,
                         (2147483647 as libc::c_int - 2 as libc::c_int) as
                             size_t) as *mut libc::c_int;
        if (*ls).lastline > (*fs).lastline + 1 as libc::c_int {
            let fresh1 = (*f).nlineinfo;
            (*f).nlineinfo = (*f).nlineinfo + 1;
            *(*f).lineinfo.offset(fresh1 as isize) =
                -((*ls).lastline - ((*fs).lastline + 1 as libc::c_int))
        }
        let fresh2 = (*f).nlineinfo;
        (*f).nlineinfo = (*f).nlineinfo + 1;
        *(*f).lineinfo.offset(fresh2 as isize) = (*fs).pc;
        (*fs).lastline = (*ls).lastline
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaK_code0(mut fs: *mut FuncState, mut o: OpCode)
 -> libc::c_int {
    return luaK_code2(fs, o, 0 as libc::c_int, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_code1(mut fs: *mut FuncState, mut o: OpCode,
                                    mut arg1: libc::c_int) -> libc::c_int {
    return luaK_code2(fs, o, arg1, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn luaK_code2(mut fs: *mut FuncState, mut o: OpCode,
                                    mut arg1: libc::c_int,
                                    mut arg2: libc::c_int) -> libc::c_int {
    let mut i: Instruction = previous_instruction(fs);
    let mut delta: libc::c_int =
        luaK_opproperties[o as usize].push as libc::c_int -
            luaK_opproperties[o as usize].pop as libc::c_int;
    let mut optm: libc::c_int = 0 as libc::c_int;
    match o as libc::c_uint {
        48 => { delta = -arg2 + 1 as libc::c_int }
        20 => { delta = -arg2 }
        21 => {
            if arg2 == 0 as libc::c_int { return -(1 as libc::c_int) }
            delta = -arg2
        }
        22 => {
            if arg1 == 0 as libc::c_int { return -(1 as libc::c_int) }
            delta = -(2 as libc::c_int) * arg1
        }
        1 => {
            if (i &
                    (!((!(0 as libc::c_int as Instruction)) <<
                           6 as libc::c_int)) << 0 as libc::c_int) as OpCode
                   as libc::c_uint == OP_CALL as libc::c_int as libc::c_uint
                   &&
                   (i >> 6 as libc::c_int &
                        (!((!(0 as libc::c_int as Instruction)) <<
                               9 as libc::c_int)) << 0 as libc::c_int) as
                       libc::c_int == 255 as libc::c_int {
                i =
                    i &
                        !((!((!(0 as libc::c_int as Instruction)) <<
                                 6 as libc::c_int)) << 0 as libc::c_int) |
                        OP_TAILCALL as libc::c_int as Instruction;
                i =
                    i &
                        !((!((!(0 as libc::c_int as Instruction)) <<
                                 9 as libc::c_int)) << 6 as libc::c_int) |
                        (arg1 as Instruction) << 6 as libc::c_int;
                optm = 1 as libc::c_int
            }
        }
        4 => {
            if arg1 == 0 as libc::c_int { return -(1 as libc::c_int) }
            delta = arg1;
            match (i &
                       (!((!(0 as libc::c_int as Instruction)) <<
                              6 as libc::c_int)) << 0 as libc::c_int) as
                      OpCode as libc::c_uint {
                4 => {
                    i =
                        i &
                            !((!((!(0 as libc::c_int as Instruction)) <<
                                     32 as libc::c_int - 6 as libc::c_int)) <<
                                  6 as libc::c_int) |
                            (((i >> 6 as libc::c_int) as libc::c_int + arg1)
                                 as Instruction) << 6 as libc::c_int;
                    optm = 1 as libc::c_int
                }
                _ => { }
            }
        }
        5 => {
            if arg1 == 0 as libc::c_int { return -(1 as libc::c_int) }
            delta = -arg1;
            match (i &
                       (!((!(0 as libc::c_int as Instruction)) <<
                              6 as libc::c_int)) << 0 as libc::c_int) as
                      OpCode as libc::c_uint {
                20 => {
                    i =
                        i &
                            !((!((!(0 as libc::c_int as Instruction)) <<
                                     9 as libc::c_int)) << 6 as libc::c_int) |
                            (((i >> 6 as libc::c_int &
                                   (!((!(0 as libc::c_int as Instruction)) <<
                                          9 as libc::c_int)) <<
                                       0 as libc::c_int) as libc::c_int +
                                  arg1) as Instruction) << 6 as libc::c_int;
                    optm = 1 as libc::c_int
                }
                _ => { }
            }
        }
        13 => {
            match (i &
                       (!((!(0 as libc::c_int as Instruction)) <<
                              6 as libc::c_int)) << 0 as libc::c_int) as
                      OpCode as libc::c_uint {
                7 => {
                    /* `t.x' */
                    i =
                        i &
                            !((!((!(0 as libc::c_int as Instruction)) <<
                                     6 as libc::c_int)) << 0 as libc::c_int) |
                            OP_GETDOTTED as libc::c_int as Instruction;
                    optm = 1 as libc::c_int
                }
                11 => {
                    /* `t[i]' */
                    i =
                        i &
                            !((!((!(0 as libc::c_int as Instruction)) <<
                                     6 as libc::c_int)) << 0 as libc::c_int) |
                            OP_GETINDEXED as libc::c_int as Instruction;
                    optm = 1 as libc::c_int
                }
                _ => { }
            }
        }
        23 => {
            match (i &
                       (!((!(0 as libc::c_int as Instruction)) <<
                              6 as libc::c_int)) << 0 as libc::c_int) as
                      OpCode as libc::c_uint {
                6 => {
                    i =
                        i &
                            !((!((!(0 as libc::c_int as Instruction)) <<
                                     6 as libc::c_int)) << 0 as libc::c_int) |
                            OP_ADDI as libc::c_int as Instruction;
                    optm = 1 as libc::c_int
                }
                _ => { }
            }
        }
        25 => {
            match (i &
                       (!((!(0 as libc::c_int as Instruction)) <<
                              6 as libc::c_int)) << 0 as libc::c_int) as
                      OpCode as libc::c_uint {
                6 => {
                    /* `a-k' */
                    i =
                        OP_ADDI as libc::c_int as Instruction |
                            ((-((i >> 6 as libc::c_int) as libc::c_int -
                                    (((1 as libc::c_int) <<
                                          32 as libc::c_int -
                                              6 as libc::c_int) -
                                         1 as libc::c_int >>
                                         1 as libc::c_int)) +
                                  (((1 as libc::c_int) <<
                                        32 as libc::c_int - 6 as libc::c_int)
                                       - 1 as libc::c_int >>
                                       1 as libc::c_int)) as Instruction) <<
                                6 as libc::c_int;
                    optm = 1 as libc::c_int
                }
                _ => { }
            }
        }
        29 => {
            delta = -arg1 + 1 as libc::c_int;
            match (i &
                       (!((!(0 as libc::c_int as Instruction)) <<
                              6 as libc::c_int)) << 0 as libc::c_int) as
                      OpCode as libc::c_uint {
                29 => {
                    /* `a..b..c' */
                    i =
                        i &
                            !((!((!(0 as libc::c_int as Instruction)) <<
                                     32 as libc::c_int - 6 as libc::c_int)) <<
                                  6 as libc::c_int) |
                            (((i >> 6 as libc::c_int) as libc::c_int +
                                  1 as libc::c_int) as Instruction) <<
                                6 as libc::c_int;
                    optm = 1 as libc::c_int
                }
                _ => { }
            }
        }
        30 => {
            match (i &
                       (!((!(0 as libc::c_int as Instruction)) <<
                              6 as libc::c_int)) << 0 as libc::c_int) as
                      OpCode as libc::c_uint {
                6 => {
                    /* `-k' */
                    i =
                        i &
                            !((!((!(0 as libc::c_int as Instruction)) <<
                                     32 as libc::c_int - 6 as libc::c_int)) <<
                                  6 as libc::c_int) |
                            ((-((i >> 6 as libc::c_int) as libc::c_int -
                                    (((1 as libc::c_int) <<
                                          32 as libc::c_int -
                                              6 as libc::c_int) -
                                         1 as libc::c_int >>
                                         1 as libc::c_int)) +
                                  (((1 as libc::c_int) <<
                                        32 as libc::c_int - 6 as libc::c_int)
                                       - 1 as libc::c_int >>
                                       1 as libc::c_int)) as Instruction) <<
                                6 as libc::c_int;
                    optm = 1 as libc::c_int
                }
                8 => {
                    /* `-k' */
                    i =
                        i &
                            !((!((!(0 as libc::c_int as Instruction)) <<
                                     6 as libc::c_int)) << 0 as libc::c_int) |
                            OP_PUSHNEGNUM as libc::c_int as Instruction;
                    optm = 1 as libc::c_int
                }
                _ => { }
            }
        }
        32 => {
            if i ==
                   OP_PUSHNIL as libc::c_int as Instruction |
                       (1 as libc::c_int as Instruction) << 6 as libc::c_int {
                /* `a~=nil' */
                i =
                    OP_JMPT as libc::c_int as Instruction |
                        ((-(1 as libc::c_int) +
                              (((1 as libc::c_int) <<
                                    32 as libc::c_int - 6 as libc::c_int) -
                                   1 as libc::c_int >> 1 as libc::c_int)) as
                             Instruction) << 6 as libc::c_int;
                optm = 1 as libc::c_int
            }
        }
        33 => {
            if i ==
                   OP_PUSHNIL as libc::c_int as Instruction |
                       (1 as libc::c_int as Instruction) << 6 as libc::c_int {
                /* `a==nil' */
                i =
                    OP_NOT as libc::c_int as
                        Instruction; /* just undo effect of previous PUSHNIL */
                delta = -(1 as libc::c_int);
                optm = 1 as libc::c_int
            }
        }
        38 | 40 => {
            match (i &
                       (!((!(0 as libc::c_int as Instruction)) <<
                              6 as libc::c_int)) << 0 as libc::c_int) as
                      OpCode as libc::c_uint {
                31 => {
                    i =
                        OP_JMPF as libc::c_int as Instruction |
                            ((-(1 as libc::c_int) +
                                  (((1 as libc::c_int) <<
                                        32 as libc::c_int - 6 as libc::c_int)
                                       - 1 as libc::c_int >>
                                       1 as libc::c_int)) as Instruction) <<
                                6 as libc::c_int;
                    optm = 1 as libc::c_int
                }
                6 => {
                    if o as libc::c_uint ==
                           OP_JMPT as libc::c_int as libc::c_uint {
                        /* JMPONT must keep original integer value */
                        i =
                            OP_JMP as libc::c_int as Instruction |
                                ((-(1 as libc::c_int) +
                                      (((1 as libc::c_int) <<
                                            32 as libc::c_int -
                                                6 as libc::c_int) -
                                           1 as libc::c_int >>
                                           1 as libc::c_int)) as Instruction)
                                    <<
                                    6 as
                                        libc::c_int; /* erase previous instruction */
                        optm = 1 as libc::c_int
                    }
                }
                4 => {
                    if (i >> 6 as libc::c_int) as libc::c_int ==
                           1 as libc::c_int {
                        (*fs).pc -= 1; /* correct stack */
                        luaK_deltastack(fs, -(1 as libc::c_int));
                        return -(1 as libc::c_int)
                    }
                }
                _ => { }
            }
        }
        39 | 41 => {
            match (i &
                       (!((!(0 as libc::c_int as Instruction)) <<
                              6 as libc::c_int)) << 0 as libc::c_int) as
                      OpCode as libc::c_uint {
                31 => {
                    i =
                        OP_JMPT as libc::c_int as Instruction |
                            ((-(1 as libc::c_int) +
                                  (((1 as libc::c_int) <<
                                        32 as libc::c_int - 6 as libc::c_int)
                                       - 1 as libc::c_int >>
                                       1 as libc::c_int)) as Instruction) <<
                                6 as libc::c_int;
                    optm = 1 as libc::c_int
                }
                6 => {
                    /* `while 1 do ...' */
                    (*fs).pc -= 1; /* erase previous instruction */
                    luaK_deltastack(fs,
                                    -(1 as libc::c_int)); /* correct stack */
                    return -(1 as libc::c_int)
                }
                4 => {
                    /* `repeat ... until nil' */
                    if (i >> 6 as libc::c_int) as libc::c_int ==
                           1 as libc::c_int {
                        i =
                            OP_JMP as libc::c_int as Instruction |
                                ((-(1 as libc::c_int) +
                                      (((1 as libc::c_int) <<
                                            32 as libc::c_int -
                                                6 as libc::c_int) -
                                           1 as libc::c_int >>
                                           1 as libc::c_int)) as Instruction)
                                    << 6 as libc::c_int;
                        optm = 1 as libc::c_int
                    }
                }
                _ => { }
            }
        }
        14 | 15 | 3 | 24 | _ => { }
    }
    luaK_deltastack(fs, delta);
    if optm != 0 {
        /* optimize: put instruction in place of last one */
        *(*(*fs).f).code.offset(((*fs).pc - 1 as libc::c_int) as isize) = i;
        return (*fs).pc - 1 as libc::c_int /* change previous instruction */
        /* do not generate new instruction */
    }
    /* else build new instruction */
    match luaK_opproperties[o as usize].mode as Mode as libc::c_uint {
        0 => { i = o as Instruction }
        1 => {
            i = o as Instruction | (arg1 as Instruction) << 6 as libc::c_int
        }
        2 => {
            i =
                o as Instruction |
                    ((arg1 +
                          (((1 as libc::c_int) <<
                                32 as libc::c_int - 6 as libc::c_int) -
                               1 as libc::c_int >> 1 as libc::c_int)) as
                         Instruction) << 6 as libc::c_int
        }
        3 => {
            i =
                o as Instruction |
                    (arg1 as Instruction) <<
                        6 as libc::c_int + 9 as libc::c_int |
                    (arg2 as Instruction) << 6 as libc::c_int
        }
        _ => { }
    }
    codelineinfo(fs);
    /* put new instruction in code array */
    (*(*fs).f).code =
        luaM_growaux((*fs).L, (*(*fs).f).code as *mut libc::c_void,
                     (*fs).pc as size_t, 1 as libc::c_int,
                     ::std::mem::size_of::<Instruction>() as libc::c_ulong,
                     b"code size overflow\x00" as *const u8 as
                         *const libc::c_char,
                     (2147483647 as libc::c_int - 2 as libc::c_int) as size_t)
            as *mut Instruction;
    *(*(*fs).f).code.offset((*fs).pc as isize) = i;
    let fresh3 = (*fs).pc;
    (*fs).pc = (*fs).pc + 1;
    return fresh3;
}
#[no_mangle]
pub static mut luaK_opproperties: [OpProperties; 49] =
    [{
         let mut init =
             OpProperties{mode: iO as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iAB as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iAB as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 100 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 100 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iO as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 2 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iAB as libc::c_int as libc::c_char,
                          push: 100 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iAB as libc::c_int as libc::c_char,
                          push: 100 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 100 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iO as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iO as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iO as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iO as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iO as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iU as libc::c_int as libc::c_char,
                          push: 100 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iO as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iO as libc::c_int as libc::c_char,
                          push: 1 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 2 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 1 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iO as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 3 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 2 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iS as libc::c_int as libc::c_char,
                          push: 0 as libc::c_int as libc::c_uchar,
                          pop: 3 as libc::c_int as libc::c_uchar,};
         init
     },
     {
         let mut init =
             OpProperties{mode: iAB as libc::c_int as libc::c_char,
                          push: 100 as libc::c_int as libc::c_uchar,
                          pop: 0 as libc::c_int as libc::c_uchar,};
         init
     }];
