use ::libc;
extern "C" {
    pub type TM;
    pub type lua_longjmp;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn luaK_goiftrue(fs: *mut FuncState, v: *mut expdesc,
                     keepvalue: libc::c_int);
    #[no_mangle]
    fn luaK_patchlist(fs: *mut FuncState, list: libc::c_int,
                      target: libc::c_int);
    #[no_mangle]
    fn luaK_lastisopen(fs: *mut FuncState) -> libc::c_int;
    #[no_mangle]
    fn luaK_setcallreturns(fs: *mut FuncState, nresults: libc::c_int);
    #[no_mangle]
    fn luaK_storevar(ls: *mut LexState, var: *const expdesc);
    #[no_mangle]
    fn luaK_prefix(ls: *mut LexState, op: UnOpr, v: *mut expdesc);
    #[no_mangle]
    fn luaK_number(fs: *mut FuncState, f: Number);
    #[no_mangle]
    fn luaK_deltastack(fs: *mut FuncState, delta: libc::c_int);
    #[no_mangle]
    fn luaX_setinput(L: *mut lua_State, LS: *mut LexState, z: *mut ZIO,
                     source: *mut TString);
    #[no_mangle]
    fn luaX_lex(LS: *mut LexState, seminfo: *mut SemInfo) -> libc::c_int;
    #[no_mangle]
    fn luaX_checklimit(ls: *mut LexState, val: libc::c_int,
                       limit: libc::c_int, msg: *const libc::c_char);
    #[no_mangle]
    fn luaX_syntaxerror(ls: *mut LexState, s: *const libc::c_char,
                        token: *const libc::c_char);
    #[no_mangle]
    fn luaX_token2str(token: libc::c_int, s: *mut libc::c_char);
    #[no_mangle]
    fn luaK_kstr(ls: *mut LexState, c: libc::c_int);
    #[no_mangle]
    fn luaK_code2(fs: *mut FuncState, o: OpCode, arg1: libc::c_int,
                  arg2: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaK_getlabel(fs: *mut FuncState) -> libc::c_int;
    #[no_mangle]
    fn luaK_code0(fs: *mut FuncState, o: OpCode) -> libc::c_int;
    #[no_mangle]
    fn luaK_infix(ls: *mut LexState, op: BinOpr, v: *mut expdesc);
    #[no_mangle]
    fn luaK_error(ls: *mut LexState, msg: *const libc::c_char);
    #[no_mangle]
    fn luaK_posfix(ls: *mut LexState, op: BinOpr, v1: *mut expdesc,
                   v2: *mut expdesc);
    #[no_mangle]
    fn luaK_tostack(ls: *mut LexState, v: *mut expdesc, onlyone: libc::c_int);
    #[no_mangle]
    fn luaK_code1(fs: *mut FuncState, o: OpCode, arg1: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn luaK_concat(fs: *mut FuncState, l1: *mut libc::c_int, l2: libc::c_int);
    #[no_mangle]
    fn luaK_adjuststack(fs: *mut FuncState, n: libc::c_int);
    #[no_mangle]
    fn luaK_jump(fs: *mut FuncState) -> libc::c_int;
    #[no_mangle]
    fn luaF_newproto(L: *mut lua_State) -> *mut Proto;
    #[no_mangle]
    fn luaF_protook(L: *mut lua_State, f: *mut Proto, pc: libc::c_int);
    #[no_mangle]
    fn luaM_realloc(L: *mut lua_State, oldblock: *mut libc::c_void,
                    size: lint32) -> *mut libc::c_void;
    #[no_mangle]
    fn luaM_growaux(L: *mut lua_State, block_0: *mut libc::c_void,
                    nelems: size_t, inc: libc::c_int, size: size_t,
                    errormsg: *const libc::c_char, limit: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    #[no_mangle]
    fn luaS_newfixed(L: *mut lua_State, str: *const libc::c_char)
     -> *mut TString;
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
pub type RESERVED = libc::c_uint;
pub const TK_EOS: RESERVED = 284;
pub const TK_STRING: RESERVED = 283;
pub const TK_NUMBER: RESERVED = 282;
pub const TK_NE: RESERVED = 281;
pub const TK_LE: RESERVED = 280;
pub const TK_GE: RESERVED = 279;
pub const TK_EQ: RESERVED = 278;
pub const TK_DOTS: RESERVED = 277;
pub const TK_CONCAT: RESERVED = 276;
pub const TK_NAME: RESERVED = 275;
pub const TK_WHILE: RESERVED = 274;
pub const TK_UNTIL: RESERVED = 273;
pub const TK_THEN: RESERVED = 272;
pub const TK_RETURN: RESERVED = 271;
pub const TK_REPEAT: RESERVED = 270;
pub const TK_OR: RESERVED = 269;
pub const TK_NOT: RESERVED = 268;
pub const TK_NIL: RESERVED = 267;
pub const TK_LOCAL: RESERVED = 266;
pub const TK_IF: RESERVED = 265;
pub const TK_FUNCTION: RESERVED = 264;
pub const TK_FOR: RESERVED = 263;
pub const TK_END: RESERVED = 262;
pub const TK_ELSEIF: RESERVED = 261;
pub const TK_ELSE: RESERVED = 260;
pub const TK_DO: RESERVED = 259;
pub const TK_BREAK: RESERVED = 258;
pub const TK_AND: RESERVED = 257;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Breaklabel {
    pub previous: *mut Breaklabel,
    pub breaklist: libc::c_int,
    pub stacklevel: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub left: libc::c_char,
    pub right: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Constdesc {
    pub n: libc::c_int,
    pub k: libc::c_int,
}
pub type UnOpr = libc::c_uint;
pub const OPR_NOUNOPR: UnOpr = 2;
pub const OPR_NOT: UnOpr = 1;
pub const OPR_MINUS: UnOpr = 0;
unsafe extern "C" fn next(mut ls: *mut LexState) {
    (*ls).lastline = (*ls).linenumber;
    if (*ls).lookahead.token != TK_EOS as libc::c_int {
        /* is there a look-ahead token? */
        (*ls).t = (*ls).lookahead;
        (*ls).lookahead.token = TK_EOS as libc::c_int /* use this one */
        /* and discharge it */
    } else { (*ls).t.token = luaX_lex(ls, &mut (*ls).t.seminfo) };
    /* read next token */
}
unsafe extern "C" fn lookahead(mut ls: *mut LexState) {
    (*ls).lookahead.token = luaX_lex(ls, &mut (*ls).lookahead.seminfo);
}
unsafe extern "C" fn error_expected(mut ls: *mut LexState,
                                    mut token: libc::c_int) {
    let mut buff: [libc::c_char; 100] = [0; 100];
    let mut t: [libc::c_char; 15] = [0; 15];
    luaX_token2str(token, t.as_mut_ptr());
    sprintf(buff.as_mut_ptr(),
            b"`%.20s\' expected\x00" as *const u8 as *const libc::c_char,
            t.as_mut_ptr());
    luaK_error(ls, buff.as_mut_ptr());
}
unsafe extern "C" fn check(mut ls: *mut LexState, mut c: libc::c_int) {
    if (*ls).t.token != c { error_expected(ls, c); }
    next(ls);
}
unsafe extern "C" fn check_condition(mut ls: *mut LexState,
                                     mut c: libc::c_int,
                                     mut msg: *const libc::c_char) {
    if c == 0 { luaK_error(ls, msg); };
}
unsafe extern "C" fn optional(mut ls: *mut LexState, mut c: libc::c_int)
 -> libc::c_int {
    if (*ls).t.token == c {
        next(ls);
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
unsafe extern "C" fn check_match(mut ls: *mut LexState, mut what: libc::c_int,
                                 mut who: libc::c_int,
                                 mut where_0: libc::c_int) {
    if (*ls).t.token != what {
        if where_0 == (*ls).linenumber {
            error_expected(ls, what);
        } else {
            let mut buff: [libc::c_char; 100] = [0; 100];
            let mut t_what: [libc::c_char; 15] = [0; 15];
            let mut t_who: [libc::c_char; 15] = [0; 15];
            luaX_token2str(what, t_what.as_mut_ptr());
            luaX_token2str(who, t_who.as_mut_ptr());
            sprintf(buff.as_mut_ptr(),
                    b"`%.20s\' expected (to close `%.20s\' at line %d)\x00" as
                        *const u8 as *const libc::c_char, t_what.as_mut_ptr(),
                    t_who.as_mut_ptr(), where_0);
            luaK_error(ls, buff.as_mut_ptr());
        }
    }
    next(ls);
}
unsafe extern "C" fn string_constant(mut fs: *mut FuncState,
                                     mut s: *mut TString) -> libc::c_int {
    let mut f: *mut Proto = (*fs).f;
    let mut c: libc::c_int = (*s).u.s.constindex;
    if c >= (*f).nkstr || *(*f).kstr.offset(c as isize) != s {
        (*f).kstr =
            luaM_growaux((*fs).L, (*f).kstr as *mut libc::c_void,
                         (*f).nkstr as size_t, 1 as libc::c_int,
                         ::std::mem::size_of::<*mut TString>() as
                             libc::c_ulong,
                         b"constant table overflow\x00" as *const u8 as
                             *const libc::c_char,
                         (((1 as libc::c_int) <<
                               32 as libc::c_int - 6 as libc::c_int) -
                              1 as libc::c_int) as size_t) as
                *mut *mut TString;
        let fresh0 = (*f).nkstr;
        (*f).nkstr = (*f).nkstr + 1;
        c = fresh0;
        let ref mut fresh1 = *(*f).kstr.offset(c as isize);
        *fresh1 = s;
        (*s).u.s.constindex = c
        /* hint for next time */
    }
    return c;
}
unsafe extern "C" fn code_string(mut ls: *mut LexState, mut s: *mut TString) {
    luaK_kstr(ls, string_constant((*ls).fs, s));
}
unsafe extern "C" fn str_checkname(mut ls: *mut LexState) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    check_condition(ls,
                    ((*ls).t.token == TK_NAME as libc::c_int) as libc::c_int,
                    b"<name> expected\x00" as *const u8 as
                        *const libc::c_char);
    ts = (*ls).t.seminfo.ts;
    next(ls);
    return ts;
}
unsafe extern "C" fn checkname(mut ls: *mut LexState) -> libc::c_int {
    return string_constant((*ls).fs, str_checkname(ls));
}
unsafe extern "C" fn luaI_registerlocalvar(mut ls: *mut LexState,
                                           mut varname: *mut TString)
 -> libc::c_int {
    let mut f: *mut Proto = (*(*ls).fs).f;
    (*f).locvars =
        luaM_growaux((*ls).L, (*f).locvars as *mut libc::c_void,
                     (*f).nlocvars as size_t, 1 as libc::c_int,
                     ::std::mem::size_of::<LocVar>() as libc::c_ulong,
                     b"\x00" as *const u8 as *const libc::c_char,
                     (2147483647 as libc::c_int - 2 as libc::c_int) as size_t)
            as *mut LocVar;
    let ref mut fresh2 =
        (*(*f).locvars.offset((*f).nlocvars as isize)).varname;
    *fresh2 = varname;
    let fresh3 = (*f).nlocvars;
    (*f).nlocvars = (*f).nlocvars + 1;
    return fresh3;
}
unsafe extern "C" fn new_localvar(mut ls: *mut LexState,
                                  mut name: *mut TString,
                                  mut n: libc::c_int) {
    let mut fs: *mut FuncState = (*ls).fs;
    luaX_checklimit(ls, (*fs).nactloc as libc::c_int + n + 1 as libc::c_int,
                    200 as libc::c_int,
                    b"local variables\x00" as *const u8 as
                        *const libc::c_char);
    (*fs).actloc[((*fs).nactloc as libc::c_int + n) as usize] =
        luaI_registerlocalvar(ls, name);
}
unsafe extern "C" fn adjustlocalvars(mut ls: *mut LexState,
                                     mut nvars: libc::c_int) {
    let mut fs: *mut FuncState = (*ls).fs;
    loop  {
        let fresh4 = nvars;
        nvars = nvars - 1;
        if !(fresh4 != 0) { break ; }
        let fresh5 = (*fs).nactloc;
        (*fs).nactloc = (*fs).nactloc + 1;
        (*(*(*fs).f).locvars.offset((*fs).actloc[fresh5 as usize] as
                                        isize)).startpc = (*fs).pc
    };
}
unsafe extern "C" fn removelocalvars(mut ls: *mut LexState,
                                     mut nvars: libc::c_int) {
    let mut fs: *mut FuncState = (*ls).fs;
    loop  {
        let fresh6 = nvars;
        nvars = nvars - 1;
        if !(fresh6 != 0) { break ; }
        (*fs).nactloc -= 1;
        (*(*(*fs).f).locvars.offset((*fs).actloc[(*fs).nactloc as usize] as
                                        isize)).endpc = (*fs).pc
    };
}
unsafe extern "C" fn new_localvarstr(mut ls: *mut LexState,
                                     mut name: *const libc::c_char,
                                     mut n: libc::c_int) {
    new_localvar(ls, luaS_newfixed((*ls).L, name), n);
}
unsafe extern "C" fn search_local(mut ls: *mut LexState, mut n: *mut TString,
                                  mut var: *mut expdesc) -> libc::c_int {
    let mut fs: *mut FuncState = 0 as *mut FuncState;
    let mut level: libc::c_int = 0 as libc::c_int;
    fs = (*ls).fs;
    while !fs.is_null() {
        let mut i: libc::c_int = 0;
        i = (*fs).nactloc as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            if n ==
                   (*(*(*fs).f).locvars.offset((*fs).actloc[i as usize] as
                                                   isize)).varname {
                (*var).k = VLOCAL;
                (*var).u.index = i;
                return level
            }
            i -= 1
        }
        level += 1;
        fs = (*fs).prev
        /* `var' not found; check outer level */
    } /* not found in any level; must be global */
    (*var).k = VGLOBAL;
    return -(1 as libc::c_int);
}
unsafe extern "C" fn singlevar(mut ls: *mut LexState, mut n: *mut TString,
                               mut var: *mut expdesc) {
    let mut level: libc::c_int = search_local(ls, n, var);
    if level >= 1 as libc::c_int {
        /* neither local (0) nor global (-1)? */
        luaX_syntaxerror(ls,
                         b"cannot access a variable in outer scope\x00" as
                             *const u8 as *const libc::c_char,
                         (*n).str_0.as_mut_ptr());
    } else if level == -(1 as libc::c_int) {
        /* global? */
        (*var).u.index = string_constant((*ls).fs, n)
    };
}
unsafe extern "C" fn indexupvalue(mut ls: *mut LexState, mut v: *mut expdesc)
 -> libc::c_int {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*fs).nupvalues as libc::c_int {
        if (*fs).upvalues[i as usize].k as libc::c_uint ==
               (*v).k as libc::c_uint &&
               (*fs).upvalues[i as usize].u.index == (*v).u.index {
            return i
        }
        i += 1
    }
    /* new one */
    luaX_checklimit(ls, (*fs).nupvalues as libc::c_int + 1 as libc::c_int,
                    32 as libc::c_int,
                    b"upvalues\x00" as *const u8 as *const libc::c_char);
    (*fs).upvalues[(*fs).nupvalues as usize] = *v;
    let fresh7 = (*fs).nupvalues;
    (*fs).nupvalues = (*fs).nupvalues + 1;
    return fresh7 as libc::c_int;
}
unsafe extern "C" fn pushupvalue(mut ls: *mut LexState, mut n: *mut TString) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut v: expdesc = expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
    let mut level: libc::c_int = search_local(ls, n, &mut v);
    if level == -(1 as libc::c_int) {
        /* global? */
        if (*fs).prev.is_null() {
            luaX_syntaxerror(ls,
                             b"cannot access upvalue in main\x00" as *const u8
                                 as *const libc::c_char,
                             (*n).str_0.as_mut_ptr());
        }
        v.u.index = string_constant((*fs).prev, n)
    } else if level != 1 as libc::c_int {
        luaX_syntaxerror(ls,
                         b"upvalue must be global or local to immediately outer scope\x00"
                             as *const u8 as *const libc::c_char,
                         (*n).str_0.as_mut_ptr());
    }
    luaK_code1(fs, OP_PUSHUPVALUE, indexupvalue(ls, &mut v));
}
unsafe extern "C" fn adjust_mult_assign(mut ls: *mut LexState,
                                        mut nvars: libc::c_int,
                                        mut nexps: libc::c_int) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut diff: libc::c_int = nexps - nvars;
    if nexps > 0 as libc::c_int && luaK_lastisopen(fs) != 0 {
        /* list ends in a function call */
        diff -= 1;
        if diff <= 0 as libc::c_int { /* do not count function call itself */
            /* call should provide no value */
            /* more variables than values? */
            luaK_setcallreturns(fs, -diff);
            diff = 0 as libc::c_int /* function call provide extra values */
            /* no more difference */
        } else {
            /* more values than variables */
            luaK_setcallreturns(fs, 0 as libc::c_int);
        }
    }
    /* push or pop eventual difference between list lengths */
    luaK_adjuststack(fs, diff); /* `self' could be there already */
}
unsafe extern "C" fn code_params(mut ls: *mut LexState,
                                 mut nparams: libc::c_int,
                                 mut dots: libc::c_int) {
    let mut fs: *mut FuncState = (*ls).fs;
    adjustlocalvars(ls, nparams);
    luaX_checklimit(ls, (*fs).nactloc as libc::c_int, 100 as libc::c_int,
                    b"parameters\x00" as *const u8 as *const libc::c_char);
    (*(*fs).f).numparams = (*fs).nactloc;
    (*(*fs).f).is_vararg = dots as libc::c_short;
    if dots != 0 {
        new_localvarstr(ls, b"arg\x00" as *const u8 as *const libc::c_char,
                        0 as libc::c_int);
        adjustlocalvars(ls, 1 as libc::c_int);
    }
    luaK_deltastack(fs, (*fs).nactloc as libc::c_int);
    /* count parameters in the stack */
}
unsafe extern "C" fn enterbreak(mut fs: *mut FuncState,
                                mut bl: *mut Breaklabel) {
    (*bl).stacklevel =
        (*fs).stacklevel as libc::c_int; /* linked list of funcstates */
    (*bl).breaklist = -(1 as libc::c_int); /* default for main chunk */
    (*bl).previous = (*fs).bl;
    (*fs).bl = bl;
}
unsafe extern "C" fn leavebreak(mut fs: *mut FuncState,
                                mut bl: *mut Breaklabel) {
    (*fs).bl = (*bl).previous;
    luaK_patchlist(fs, (*bl).breaklist, luaK_getlabel(fs));
}
unsafe extern "C" fn pushclosure(mut ls: *mut LexState,
                                 mut func: *mut FuncState) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut f: *mut Proto = (*fs).f;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*func).nupvalues as libc::c_int {
        luaK_tostack(ls,
                     &mut *(*func).upvalues.as_mut_ptr().offset(i as isize),
                     1 as libc::c_int);
        i += 1
    }
    (*f).kproto =
        luaM_growaux((*ls).L, (*f).kproto as *mut libc::c_void,
                     (*f).nkproto as size_t, 1 as libc::c_int,
                     ::std::mem::size_of::<*mut Proto>() as libc::c_ulong,
                     b"constant table overflow\x00" as *const u8 as
                         *const libc::c_char,
                     (((1 as libc::c_int) <<
                           32 as libc::c_int -
                               (6 as libc::c_int + 9 as libc::c_int)) -
                          1 as libc::c_int) as size_t) as *mut *mut Proto;
    let fresh8 = (*f).nkproto;
    (*f).nkproto = (*f).nkproto + 1;
    let ref mut fresh9 = *(*f).kproto.offset(fresh8 as isize);
    *fresh9 = (*func).f;
    luaK_code2(fs, OP_CLOSURE, (*f).nkproto - 1 as libc::c_int,
               (*func).nupvalues as libc::c_int);
}
unsafe extern "C" fn open_func(mut ls: *mut LexState,
                               mut fs: *mut FuncState) {
    let mut f: *mut Proto = luaF_newproto((*ls).L);
    (*fs).prev = (*ls).fs;
    (*fs).ls = ls;
    (*fs).L = (*ls).L;
    (*ls).fs = fs;
    (*fs).stacklevel = 0 as libc::c_int as libc::c_short;
    (*fs).nactloc = 0 as libc::c_int as libc::c_short;
    (*fs).nupvalues = 0 as libc::c_int as libc::c_short;
    (*fs).bl = 0 as *mut Breaklabel;
    (*fs).f = f;
    (*f).source = (*ls).source;
    (*fs).pc = 0 as libc::c_int;
    (*fs).lasttarget = 0 as libc::c_int;
    (*fs).lastline = 0 as libc::c_int;
    (*fs).jlt = -(1 as libc::c_int);
    (*f).code = 0 as *mut Instruction;
    (*f).maxstacksize = 0 as libc::c_int as libc::c_short;
    (*f).numparams = 0 as libc::c_int as libc::c_short;
    (*f).is_vararg = 0 as libc::c_int as libc::c_short;
    /* default for main chunk */
}
unsafe extern "C" fn close_func(mut ls: *mut LexState) {
    let mut L: *mut lua_State =
        (*ls).L; /* close eventual list of pending jumps */
    let mut fs: *mut FuncState = (*ls).fs; /* end flag */
    let mut f: *mut Proto = (*fs).f; /* proto is ok now */
    luaK_code0(fs, OP_END); /* read first token */
    luaK_getlabel(fs);
    (*f).code =
        luaM_realloc(L, (*f).code as *mut libc::c_void,
                     ((*fs).pc as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>()
                                                          as libc::c_ulong))
            as *mut Instruction;
    (*f).kstr =
        luaM_realloc(L, (*f).kstr as *mut libc::c_void,
                     ((*f).nkstr as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>()
                                                          as libc::c_ulong))
            as *mut *mut TString;
    (*f).knum =
        luaM_realloc(L, (*f).knum as *mut libc::c_void,
                     ((*f).nknum as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<Number>()
                                                          as libc::c_ulong))
            as *mut Number;
    (*f).kproto =
        luaM_realloc(L, (*f).kproto as *mut libc::c_void,
                     ((*f).nkproto as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>()
                                                          as libc::c_ulong))
            as *mut *mut Proto;
    removelocalvars(ls, (*fs).nactloc as libc::c_int);
    (*f).locvars =
        luaM_realloc(L, (*f).locvars as *mut libc::c_void,
                     ((*f).nlocvars as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<LocVar>()
                                                          as libc::c_ulong))
            as *mut LocVar;
    (*f).lineinfo =
        luaM_realloc(L, (*f).lineinfo as *mut libc::c_void,
                     (((*f).nlineinfo + 1 as libc::c_int) as
                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                          as libc::c_ulong))
            as *mut libc::c_int;
    let fresh10 = (*f).nlineinfo;
    (*f).nlineinfo = (*f).nlineinfo + 1;
    *(*f).lineinfo.offset(fresh10 as isize) =
        2147483647 as libc::c_int - 2 as libc::c_int;
    luaF_protook(L, f, (*fs).pc);
    (*ls).fs = (*fs).prev;
}
#[no_mangle]
pub unsafe extern "C" fn luaY_parser(mut L: *mut lua_State, mut z: *mut ZIO)
 -> *mut Proto {
    let mut lexstate: LexState =
        LexState{current: 0,
                 t: Token{token: 0, seminfo: SemInfo{r: 0,},},
                 lookahead: Token{token: 0, seminfo: SemInfo{r: 0,},},
                 fs: 0 as *mut FuncState,
                 L: 0 as *mut lua_State,
                 z: 0 as *mut zio,
                 linenumber: 0,
                 lastline: 0,
                 source: 0 as *mut TString,};
    let mut funcstate: FuncState =
        FuncState{f: 0 as *mut Proto,
                  prev: 0 as *mut FuncState,
                  ls: 0 as *mut LexState,
                  L: 0 as *mut lua_State,
                  pc: 0,
                  lasttarget: 0,
                  jlt: 0,
                  stacklevel: 0,
                  nactloc: 0,
                  nupvalues: 0,
                  lastline: 0,
                  bl: 0 as *mut Breaklabel,
                  upvalues:
                      [expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
                          32],
                  actloc: [0; 200],};
    luaX_setinput(L, &mut lexstate, z, luaS_new(L, (*z).name));
    open_func(&mut lexstate, &mut funcstate);
    next(&mut lexstate);
    chunk(&mut lexstate);
    check_condition(&mut lexstate,
                    (lexstate.t.token == TK_EOS as libc::c_int) as
                        libc::c_int,
                    b"<eof> expected\x00" as *const u8 as
                        *const libc::c_char);
    close_func(&mut lexstate);
    return funcstate.f;
}
/*============================================================*/
/* GRAMMAR RULES */
/*============================================================*/
unsafe extern "C" fn explist1(mut ls: *mut LexState) -> libc::c_int {
    /* explist1 -> expr { ',' expr } */
    let mut n: libc::c_int = 1 as libc::c_int; /* at least one expression */
    let mut v: expdesc =
        expdesc{k: VGLOBAL,
                u:
                    C2RustUnnamed_3{index:
                                        0,},}; /* gets only 1 value from previous expression */
    expr(ls, &mut v); /* skip comma */
    while (*ls).t.token == ',' as i32 {
        luaK_tostack(ls, &mut v,
                     1 as
                         libc::c_int); /* keep open number of values of last expression */
        next(ls); /* where is func in the stack */
        expr(ls, &mut v);
        n += 1
    }
    luaK_tostack(ls, &mut v, 0 as libc::c_int);
    return n;
}
unsafe extern "C" fn funcargs(mut ls: *mut LexState, mut slf: libc::c_int) {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut slevel: libc::c_int =
        (*fs).stacklevel as libc::c_int - slf - 1 as libc::c_int;
    match (*ls).t.token {
        40 => {
            /* funcargs -> '(' [ explist1 ] ')' */
            let mut line: libc::c_int = (*ls).linenumber;
            let mut nargs: libc::c_int = 0 as libc::c_int;
            next(ls);
            if (*ls).t.token != ')' as i32 {
                /* arg list not empty? */
                nargs = explist1(ls)
            }
            check_match(ls, ')' as i32, '(' as i32, line);
        }
        123 => {
            /* funcargs -> constructor */
            constructor(ls);
        }
        283 => {
            /* funcargs -> STRING */
            code_string(ls,
                        (*ls).t.seminfo.ts); /* must use `seminfo' before `next' */
            next(ls); /* call will remove function and arguments */
        }
        _ => {
            luaK_error(ls,
                       b"function arguments expected\x00" as *const u8 as
                           *const libc::c_char);
        }
    }
    (*fs).stacklevel = slevel as libc::c_short;
    luaK_code2(fs, OP_CALL, slevel, 255 as libc::c_int);
}
unsafe extern "C" fn var_or_func_tail(mut ls: *mut LexState,
                                      mut v: *mut expdesc) {
    loop  {
        match (*ls).t.token {
            46 => {
                /* var_or_func_tail -> '.' NAME */
                next(ls);
                /* should be follow... */
                luaK_tostack(ls, v,
                             1 as libc::c_int); /* `v' must be on stack */
                luaK_kstr(ls, checkname(ls));
                (*v).k = VINDEXED
            }
            91 => {
                /* var_or_func_tail -> '[' exp1 ']' */
                next(ls); /* `v' must be on stack */
                luaK_tostack(ls, v, 1 as libc::c_int);
                (*v).k = VINDEXED;
                exp1(ls);
                check(ls, ']' as i32);
            }
            58 => {
                /* var_or_func_tail -> ':' NAME funcargs */
                let mut name: libc::c_int = 0; /* `v' must be on stack */
                next(ls);
                name = checkname(ls);
                luaK_tostack(ls, v, 1 as libc::c_int);
                luaK_code1((*ls).fs, OP_PUSHSELF, name);
                funcargs(ls, 1 as libc::c_int);
                (*v).k = VEXP;
                (*v).u.l.f = -(1 as libc::c_int);
                (*v).u.l.t = (*v).u.l.f
            }
            40 | 283 | 123 => {
                /* var_or_func_tail -> funcargs */
                luaK_tostack(ls, v,
                             1 as libc::c_int); /* `v' must be on stack */
                funcargs(ls, 0 as libc::c_int);
                (*v).k = VEXP;
                (*v).u.l.f = -(1 as libc::c_int);
                (*v).u.l.t = (*v).u.l.f
            }
            _ => { return }
        }
    };
}
unsafe extern "C" fn var_or_func(mut ls: *mut LexState, mut v: *mut expdesc) {
    /* var_or_func -> ['%'] NAME var_or_func_tail */
    if optional(ls, '%' as i32) != 0 {
        /* upvalue? */
        pushupvalue(ls, str_checkname(ls));
        (*v).k = VEXP;
        (*v).u.l.f = -(1 as libc::c_int);
        (*v).u.l.t = (*v).u.l.f
    } else {
        /* variable name */
        singlevar(ls, str_checkname(ls), v);
    }
    var_or_func_tail(ls, v);
}
/*
** {======================================================================
** Rules for Constructors
** =======================================================================
*/
unsafe extern "C" fn recfield(mut ls: *mut LexState) {
    /* recfield -> (NAME | '['exp1']') = exp1 */
    match (*ls).t.token {
        275 => { luaK_kstr(ls, checkname(ls)); }
        91 => { next(ls); exp1(ls); check(ls, ']' as i32); }
        _ => {
            luaK_error(ls,
                       b"<name> or `[\' expected\x00" as *const u8 as
                           *const libc::c_char);
        }
    }
    check(ls, '=' as i32);
    exp1(ls);
}
unsafe extern "C" fn recfields(mut ls: *mut LexState) -> libc::c_int {
    /* recfields -> recfield { ',' recfield } [','] */
    let mut fs: *mut FuncState = (*ls).fs; /* at least one element */
    let mut n: libc::c_int = 1 as libc::c_int;
    recfield(ls);
    while (*ls).t.token == ',' as i32 {
        next(ls);
        if (*ls).t.token == ';' as i32 || (*ls).t.token == '}' as i32 {
            break ;
        }
        recfield(ls);
        n += 1;
        if n % (250 as libc::c_int / 4 as libc::c_int / 2 as libc::c_int) ==
               0 as libc::c_int {
            luaK_code1(fs, OP_SETMAP,
                       250 as libc::c_int / 4 as libc::c_int /
                           2 as libc::c_int);
        }
    }
    luaK_code1(fs, OP_SETMAP,
               n %
                   (250 as libc::c_int / 4 as libc::c_int /
                        2 as libc::c_int));
    return n;
}
unsafe extern "C" fn listfields(mut ls: *mut LexState) -> libc::c_int {
    /* listfields -> exp1 { ',' exp1 } [','] */
    let mut fs: *mut FuncState = (*ls).fs; /* at least one element */
    let mut n: libc::c_int = 1 as libc::c_int;
    exp1(ls);
    while (*ls).t.token == ',' as i32 {
        next(ls);
        if (*ls).t.token == ';' as i32 || (*ls).t.token == '}' as i32 {
            break ;
        }
        exp1(ls);
        n += 1;
        luaX_checklimit(ls, n / (250 as libc::c_int / 4 as libc::c_int),
                        ((1 as libc::c_int) <<
                             32 as libc::c_int -
                                 (6 as libc::c_int + 9 as libc::c_int)) -
                            1 as libc::c_int,
                        b"`item groups\' in a list initializer\x00" as
                            *const u8 as *const libc::c_char);
        if n % (250 as libc::c_int / 4 as libc::c_int) == 0 as libc::c_int {
            luaK_code2(fs, OP_SETLIST,
                       n / (250 as libc::c_int / 4 as libc::c_int) -
                           1 as libc::c_int,
                       250 as libc::c_int / 4 as libc::c_int);
        }
    }
    luaK_code2(fs, OP_SETLIST, n / (250 as libc::c_int / 4 as libc::c_int),
               n % (250 as libc::c_int / 4 as libc::c_int));
    return n;
}
unsafe extern "C" fn constructor_part(mut ls: *mut LexState,
                                      mut cd: *mut Constdesc) {
    let mut current_block_6: u64;
    match (*ls).t.token {
        59 | 125 => {
            /* constructor_part -> empty */
            (*cd).n = 0 as libc::c_int;
            (*cd).k = (*ls).t.token;
            current_block_6 = 13586036798005543211;
        }
        275 => {
            /* may be listfields or recfields */
            lookahead(ls);
            if (*ls).lookahead.token != '=' as i32 {
                /* else go through to recfields */
                /* expression? */
                current_block_6 = 373966280438382103;
            } else { current_block_6 = 11006700562992250127; }
        }
        91 => { current_block_6 = 11006700562992250127; }
        _ => { current_block_6 = 373966280438382103; }
    }
    match current_block_6 {
        373966280438382103 =>
        /* constructor_part -> listfields */
        {
            (*cd).n = listfields(ls); /* list */
            (*cd).k = 0 as libc::c_int
        }
        11006700562992250127 => {
            /* constructor_part -> recfields */
            (*cd).n = recfields(ls); /* record */
            (*cd).k = 1 as libc::c_int
        }
        _ => { }
    };
}
unsafe extern "C" fn constructor(mut ls: *mut LexState) {
    /* constructor -> '{' constructor_part [';' constructor_part] '}' */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut line: libc::c_int = (*ls).linenumber;
    let mut pc: libc::c_int =
        luaK_code1(fs, OP_CREATETABLE, 0 as libc::c_int);
    let mut nelems: libc::c_int = 0;
    let mut cd: Constdesc = Constdesc{n: 0, k: 0,};
    check(ls, '{' as i32);
    constructor_part(ls, &mut cd);
    nelems = cd.n;
    if optional(ls, ';' as i32) != 0 {
        let mut other_cd: Constdesc = Constdesc{n: 0, k: 0,};
        constructor_part(ls, &mut other_cd);
        check_condition(ls, (cd.k != other_cd.k) as libc::c_int,
                        b"invalid constructor syntax\x00" as *const u8 as
                            *const libc::c_char);
        nelems += other_cd.n
    }
    check_match(ls, '}' as i32, '{' as i32, line);
    luaX_checklimit(ls, nelems,
                    ((1 as libc::c_int) <<
                         32 as libc::c_int - 6 as libc::c_int) -
                        1 as libc::c_int,
                    b"elements in a table constructor\x00" as *const u8 as
                        *const libc::c_char);
    *(*(*fs).f).code.offset(pc as isize) =
        *(*(*fs).f).code.offset(pc as isize) &
            !((!((!(0 as libc::c_int as Instruction)) <<
                     32 as libc::c_int - 6 as libc::c_int)) <<
                  6 as libc::c_int) |
            (nelems as Instruction) << 6 as libc::c_int;
    /* set initial table size */
}
/* }====================================================================== */
/*
** {======================================================================
** Expression parsing
** =======================================================================
*/
unsafe extern "C" fn simpleexp(mut ls: *mut LexState, mut v: *mut expdesc) {
    let mut fs: *mut FuncState = (*ls).fs;
    match (*ls).t.token {
        282 => {
            /* simpleexp -> NUMBER */
            let mut r: Number = (*ls).t.seminfo.r;
            next(ls);
            luaK_number(fs, r);
        }
        283 => {
            /* simpleexp -> STRING */
            code_string(ls,
                        (*ls).t.seminfo.ts); /* must use `seminfo' before `next' */
            next(ls);
        }
        267 => {
            /* simpleexp -> NIL */
            luaK_adjuststack(fs, -(1 as libc::c_int));
            next(ls);
        }
        123 => {
            /* simpleexp -> constructor */
            constructor(ls);
        }
        264 => {
            /* simpleexp -> FUNCTION body */
            next(ls);
            body(ls, 0 as libc::c_int, (*ls).linenumber);
        }
        40 => {
            /* simpleexp -> '(' expr ')' */
            next(ls);
            expr(ls, v);
            check(ls, ')' as i32);
            return
        }
        275 | 37 => { var_or_func(ls, v); return }
        _ => {
            luaK_error(ls,
                       b"<expression> expected\x00" as *const u8 as
                           *const libc::c_char);
            return
        }
    }
    (*v).k = VEXP;
    (*v).u.l.f = -(1 as libc::c_int);
    (*v).u.l.t = (*v).u.l.f;
}
unsafe extern "C" fn exp1(mut ls: *mut LexState) {
    let mut v: expdesc = expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
    expr(ls, &mut v);
    luaK_tostack(ls, &mut v, 1 as libc::c_int);
}
unsafe extern "C" fn getunopr(mut op: libc::c_int) -> UnOpr {
    match op {
        268 => { return OPR_NOT }
        45 => { return OPR_MINUS }
        _ => { return OPR_NOUNOPR }
    };
}
unsafe extern "C" fn getbinopr(mut op: libc::c_int) -> BinOpr {
    match op {
        43 => { return OPR_ADD }
        45 => { return OPR_SUB }
        42 => { return OPR_MULT }
        47 => { return OPR_DIV }
        94 => { return OPR_POW }
        276 => { return OPR_CONCAT }
        281 => { return OPR_NE }
        278 => { return OPR_EQ }
        60 => { return OPR_LT }
        280 => { return OPR_LE }
        62 => { return OPR_GT }
        279 => { return OPR_GE }
        257 => { return OPR_AND }
        269 => { return OPR_OR }
        _ => { return OPR_NOBINOPR }
    };
}
/* right priority */
static mut priority: [C2RustUnnamed_5; 14] =
    [{
         let mut init =
             C2RustUnnamed_5{left: 5 as libc::c_int as libc::c_char,
                             right: 5 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 5 as libc::c_int as libc::c_char,
                             right: 5 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 6 as libc::c_int as libc::c_char,
                             right: 6 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 6 as libc::c_int as libc::c_char,
                             right: 6 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 9 as libc::c_int as libc::c_char,
                             right: 8 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 4 as libc::c_int as libc::c_char,
                             right: 3 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 2 as libc::c_int as libc::c_char,
                             right: 2 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 2 as libc::c_int as libc::c_char,
                             right: 2 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 2 as libc::c_int as libc::c_char,
                             right: 2 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 2 as libc::c_int as libc::c_char,
                             right: 2 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 2 as libc::c_int as libc::c_char,
                             right: 2 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 2 as libc::c_int as libc::c_char,
                             right: 2 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 1 as libc::c_int as libc::c_char,
                             right: 1 as libc::c_int as libc::c_char,};
         init
     },
     {
         let mut init =
             C2RustUnnamed_5{left: 1 as libc::c_int as libc::c_char,
                             right: 1 as libc::c_int as libc::c_char,};
         init
     }];
/* priority for unary operators */
/*
** subexpr -> (simplexep | unop subexpr) { binop subexpr }
** where `binop' is any binary operator with a priority higher than `limit'
*/
unsafe extern "C" fn subexpr(mut ls: *mut LexState, mut v: *mut expdesc,
                             mut limit: libc::c_int) -> BinOpr {
    let mut op: BinOpr = OPR_ADD;
    let mut uop: UnOpr = getunopr((*ls).t.token);
    if uop as libc::c_uint != OPR_NOUNOPR as libc::c_int as libc::c_uint {
        next(ls);
        subexpr(ls, v, 7 as libc::c_int);
        luaK_prefix(ls, uop, v);
    } else { simpleexp(ls, v); }
    /* expand while operators have priorities higher than `limit' */
    op = getbinopr((*ls).t.token);
    while op as libc::c_uint != OPR_NOBINOPR as libc::c_int as libc::c_uint &&
              priority[op as usize].left as libc::c_int > limit {
        let mut v2: expdesc =
            expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
        let mut nextop: BinOpr = OPR_ADD;
        next(ls);
        luaK_infix(ls, op, v);
        /* read sub-expression with higher priority */
        nextop =
            subexpr(ls, &mut v2, priority[op as usize].right as libc::c_int);
        luaK_posfix(ls, op, v, &mut v2);
        op = nextop
    }
    return op;
    /* return first untreated operator */
}
unsafe extern "C" fn expr(mut ls: *mut LexState, mut v: *mut expdesc) {
    subexpr(ls, v, -(1 as libc::c_int));
}
/* }==================================================================== */
/*
** {======================================================================
** Rules for Statements
** =======================================================================
*/
unsafe extern "C" fn block_follow(mut token: libc::c_int) -> libc::c_int {
    match token {
        260 | 261 | 262 | 273 | 284 => { return 1 as libc::c_int }
        _ => { return 0 as libc::c_int }
    };
}
unsafe extern "C" fn block(mut ls: *mut LexState) {
    /* block -> chunk */
    let mut fs: *mut FuncState = (*ls).fs; /* remove local variables */
    let mut nactloc: libc::c_int =
        (*fs).nactloc as
            libc::c_int; /* number of values left in the stack after assignment */
    chunk(ls);
    luaK_adjuststack(fs, (*fs).nactloc as libc::c_int - nactloc);
    removelocalvars(ls, (*fs).nactloc as libc::c_int - nactloc);
}
unsafe extern "C" fn assignment(mut ls: *mut LexState, mut v: *mut expdesc,
                                mut nvars: libc::c_int) -> libc::c_int {
    let mut left: libc::c_int = 0 as libc::c_int;
    luaX_checklimit(ls, nvars, 255 as libc::c_int - 1 as libc::c_int,
                    b"variables in a multiple assignment\x00" as *const u8 as
                        *const libc::c_char);
    if (*ls).t.token == ',' as i32 {
        /* assignment -> ',' NAME assignment */
        let mut nv: expdesc =
            expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
        next(ls);
        var_or_func(ls, &mut nv);
        check_condition(ls,
                        (nv.k as libc::c_uint !=
                             VEXP as libc::c_int as libc::c_uint) as
                            libc::c_int,
                        b"syntax error\x00" as *const u8 as
                            *const libc::c_char);
        left = assignment(ls, &mut nv, nvars + 1 as libc::c_int)
    } else {
        /* assignment -> '=' explist1 */
        let mut nexps: libc::c_int = 0;
        check(ls, '=' as i32);
        nexps = explist1(ls);
        adjust_mult_assign(ls, nvars, nexps);
    }
    if (*v).k as libc::c_uint != VINDEXED as libc::c_int as libc::c_uint {
        luaK_storevar(ls, v);
    } else {
        /* there may be garbage between table-index and value */
        luaK_code2((*ls).fs, OP_SETTABLE, left + nvars + 2 as libc::c_int,
                   1 as libc::c_int);
        left += 2 as libc::c_int
    }
    return left;
}
unsafe extern "C" fn cond(mut ls: *mut LexState, mut v: *mut expdesc) {
    /* cond -> exp */
    expr(ls, v); /* read condition */
    luaK_goiftrue((*ls).fs, v, 0 as libc::c_int);
}
unsafe extern "C" fn whilestat(mut ls: *mut LexState, mut line: libc::c_int) {
    /* whilestat -> WHILE cond DO block END */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut while_init: libc::c_int = luaK_getlabel(fs);
    let mut v: expdesc = expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
    let mut bl: Breaklabel =
        Breaklabel{previous: 0 as *mut Breaklabel,
                   breaklist: 0,
                   stacklevel: 0,};
    enterbreak(fs, &mut bl);
    next(ls);
    cond(ls, &mut v);
    check(ls, TK_DO as libc::c_int);
    block(ls);
    luaK_patchlist(fs, luaK_jump(fs), while_init);
    luaK_patchlist(fs, v.u.l.f, luaK_getlabel(fs));
    check_match(ls, TK_END as libc::c_int, TK_WHILE as libc::c_int, line);
    leavebreak(fs, &mut bl);
}
unsafe extern "C" fn repeatstat(mut ls: *mut LexState,
                                mut line: libc::c_int) {
    /* repeatstat -> REPEAT block UNTIL cond */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut repeat_init: libc::c_int = luaK_getlabel(fs);
    let mut v: expdesc = expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
    let mut bl: Breaklabel =
        Breaklabel{previous: 0 as *mut Breaklabel,
                   breaklist: 0,
                   stacklevel: 0,};
    enterbreak(fs, &mut bl);
    next(ls);
    block(ls);
    check_match(ls, TK_UNTIL as libc::c_int, TK_REPEAT as libc::c_int, line);
    cond(ls, &mut v);
    luaK_patchlist(fs, v.u.l.f, repeat_init);
    leavebreak(fs, &mut bl);
}
unsafe extern "C" fn forbody(mut ls: *mut LexState, mut nvar: libc::c_int,
                             mut prepfor: OpCode, mut loopfor: OpCode) {
    /* forbody -> DO block END */
    let mut fs: *mut FuncState = (*ls).fs; /* scope for control variables */
    let mut prep: libc::c_int = luaK_code1(fs, prepfor, -(1 as libc::c_int));
    let mut blockinit: libc::c_int = luaK_getlabel(fs);
    check(ls, TK_DO as libc::c_int);
    adjustlocalvars(ls, nvar);
    block(ls);
    luaK_patchlist(fs, luaK_code1(fs, loopfor, -(1 as libc::c_int)),
                   blockinit);
    luaK_patchlist(fs, prep, luaK_getlabel(fs));
    removelocalvars(ls, nvar);
}
unsafe extern "C" fn fornum(mut ls: *mut LexState,
                            mut varname: *mut TString) {
    /* fornum -> NAME = exp1,exp1[,exp1] forbody */
    let mut fs: *mut FuncState = (*ls).fs; /* initial value */
    check(ls, '=' as i32); /* limit */
    exp1(ls); /* default step */
    check(ls, ',' as i32); /* optional step */
    exp1(ls);
    if optional(ls, ',' as i32) != 0 {
        exp1(ls);
    } else { luaK_code1(fs, OP_PUSHINT, 1 as libc::c_int); }
    new_localvar(ls, varname, 0 as libc::c_int);
    new_localvarstr(ls, b"(limit)\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int);
    new_localvarstr(ls, b"(step)\x00" as *const u8 as *const libc::c_char,
                    2 as libc::c_int);
    forbody(ls, 3 as libc::c_int, OP_FORPREP, OP_FORLOOP);
}
unsafe extern "C" fn forlist(mut ls: *mut LexState,
                             mut indexname: *mut TString) {
    /* forlist -> NAME,NAME IN exp1 forbody */
    let mut valname: *mut TString = 0 as *mut TString;
    check(ls, ',' as i32);
    valname = str_checkname(ls);
    /* next test is dirty, but avoids `in' being a reserved word */
    check_condition(ls,
                    ((*ls).t.token == TK_NAME as libc::c_int &&
                         (*ls).t.seminfo.ts ==
                             luaS_new((*ls).L,
                                      b"in\x00" as *const u8 as
                                          *const libc::c_char)) as
                        libc::c_int,
                    b"`in\' expected\x00" as *const u8 as
                        *const libc::c_char); /* skip `in' */
    next(ls); /* table */
    exp1(ls);
    new_localvarstr(ls, b"(table)\x00" as *const u8 as *const libc::c_char,
                    0 as libc::c_int);
    new_localvar(ls, indexname, 1 as libc::c_int);
    new_localvar(ls, valname, 2 as libc::c_int);
    forbody(ls, 3 as libc::c_int, OP_LFORPREP, OP_LFORLOOP);
}
unsafe extern "C" fn forstat(mut ls: *mut LexState, mut line: libc::c_int) {
    /* forstat -> fornum | forlist */
    let mut fs: *mut FuncState = (*ls).fs; /* skip `for' */
    let mut varname: *mut TString =
        0 as *mut TString; /* first variable name */
    let mut bl: Breaklabel =
        Breaklabel{previous: 0 as *mut Breaklabel,
                   breaklist: 0,
                   stacklevel: 0,};
    enterbreak(fs, &mut bl);
    next(ls);
    varname = str_checkname(ls);
    match (*ls).t.token {
        61 => { fornum(ls, varname); }
        44 => { forlist(ls, varname); }
        _ => {
            luaK_error(ls,
                       b"`=\' or `,\' expected\x00" as *const u8 as
                           *const libc::c_char);
        }
    }
    check_match(ls, TK_END as libc::c_int, TK_FOR as libc::c_int, line);
    leavebreak(fs, &mut bl);
}
unsafe extern "C" fn test_then_block(mut ls: *mut LexState,
                                     mut v: *mut expdesc) {
    /* test_then_block -> [IF | ELSEIF] cond THEN block */
    next(ls); /* skip IF or ELSEIF */
    cond(ls, v);
    check(ls, TK_THEN as libc::c_int);
    block(ls);
    /* `then' part */
}
unsafe extern "C" fn ifstat(mut ls: *mut LexState, mut line: libc::c_int) {
    /* ifstat -> IF cond THEN block {ELSEIF cond THEN block} [ELSE block] END */
    let mut fs: *mut FuncState = (*ls).fs; /* IF cond THEN block */
    let mut v: expdesc = expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
    let mut escapelist: libc::c_int = -(1 as libc::c_int);
    test_then_block(ls, &mut v);
    while (*ls).t.token == TK_ELSEIF as libc::c_int {
        luaK_concat(fs, &mut escapelist, luaK_jump(fs));
        luaK_patchlist(fs, v.u.l.f, luaK_getlabel(fs));
        test_then_block(ls, &mut v);
        /* ELSEIF cond THEN block */
    }
    if (*ls).t.token == TK_ELSE as libc::c_int {
        luaK_concat(fs, &mut escapelist, luaK_jump(fs));
        luaK_patchlist(fs, v.u.l.f, luaK_getlabel(fs));
        /* `else' part */
        next(ls); /* skip ELSE */
        block(ls);
    } else { luaK_concat(fs, &mut escapelist, v.u.l.f); }
    luaK_patchlist(fs, escapelist, luaK_getlabel(fs));
    check_match(ls, TK_END as libc::c_int, TK_IF as libc::c_int, line);
}
unsafe extern "C" fn localstat(mut ls: *mut LexState) {
    /* stat -> LOCAL NAME {',' NAME} ['=' explist1] */
    let mut nvars: libc::c_int = 0 as libc::c_int; /* skip LOCAL or ',' */
    let mut nexps: libc::c_int = 0;
    loop  {
        next(ls);
        let fresh11 = nvars;
        nvars = nvars + 1;
        new_localvar(ls, str_checkname(ls), fresh11);
        if !((*ls).t.token == ',' as i32) { break ; }
    }
    if optional(ls, '=' as i32) != 0 {
        nexps = explist1(ls)
    } else { nexps = 0 as libc::c_int }
    adjust_mult_assign(ls, nvars, nexps);
    adjustlocalvars(ls, nvars);
}
unsafe extern "C" fn funcname(mut ls: *mut LexState, mut v: *mut expdesc)
 -> libc::c_int {
    /* funcname -> NAME [':' NAME | '.' NAME] */
    let mut needself: libc::c_int = 0 as libc::c_int;
    singlevar(ls, str_checkname(ls), v);
    if (*ls).t.token == ':' as i32 || (*ls).t.token == '.' as i32 {
        needself = ((*ls).t.token == ':' as i32) as libc::c_int;
        next(ls);
        luaK_tostack(ls, v, 1 as libc::c_int);
        luaK_kstr(ls, checkname(ls));
        (*v).k = VINDEXED
    }
    return needself;
}
unsafe extern "C" fn funcstat(mut ls: *mut LexState, mut line: libc::c_int) {
    /* funcstat -> FUNCTION funcname body */
    let mut needself: libc::c_int = 0; /* skip FUNCTION */
    let mut v: expdesc = expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
    next(ls);
    needself = funcname(ls, &mut v);
    body(ls, needself, line);
    luaK_storevar(ls, &mut v);
}
unsafe extern "C" fn namestat(mut ls: *mut LexState) {
    /* stat -> func | ['%'] NAME assignment */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut v: expdesc = expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
    var_or_func(ls, &mut v);
    if v.k as libc::c_uint == VEXP as libc::c_int as libc::c_uint {
        /* stat -> func */
        check_condition(ls, luaK_lastisopen(fs),
                        b"syntax error\x00" as *const u8 as
                            *const libc::c_char);
        luaK_setcallreturns(fs, 0 as libc::c_int); /* an upvalue? */
        /* call statement uses no results */
    } else {
        /* stat -> ['%'] NAME assignment */
        let mut left: libc::c_int = assignment(ls, &mut v, 1 as libc::c_int);
        luaK_adjuststack(fs, left);
    };
}
unsafe extern "C" fn retstat(mut ls: *mut LexState) {
    /* stat -> RETURN explist */
    let mut fs: *mut FuncState = (*ls).fs; /* skip RETURN */
    next(ls); /* optional return values */
    if block_follow((*ls).t.token) == 0 { explist1(ls); }
    luaK_code1(fs, OP_RETURN, (*(*ls).fs).nactloc as libc::c_int);
    (*fs).stacklevel = (*fs).nactloc;
    /* removes all temp values */
}
unsafe extern "C" fn breakstat(mut ls: *mut LexState) {
    /* stat -> BREAK [NAME] */
    let mut fs: *mut FuncState = (*ls).fs; /* skip BREAK */
    let mut currentlevel: libc::c_int = (*fs).stacklevel as libc::c_int;
    let mut bl: *mut Breaklabel = (*fs).bl;
    if bl.is_null() {
        luaK_error(ls,
                   b"no loop to break\x00" as *const u8 as
                       *const libc::c_char);
    }
    next(ls);
    luaK_adjuststack(fs, currentlevel - (*bl).stacklevel);
    luaK_concat(fs, &mut (*bl).breaklist, luaK_jump(fs));
    /* correct stack for compiler and symbolic execution */
    luaK_adjuststack(fs,
                     (*bl).stacklevel -
                         currentlevel); /* may be needed for error messages */
}
unsafe extern "C" fn stat(mut ls: *mut LexState) -> libc::c_int {
    let mut line: libc::c_int = (*ls).linenumber;
    match (*ls).t.token {
        265 => {
            /* stat -> ifstat */
            ifstat(ls, line);
            return 0 as libc::c_int
        }
        274 => {
            /* stat -> whilestat */
            whilestat(ls, line);
            return 0 as libc::c_int
        }
        259 => {
            /* stat -> DO block END */
            next(ls); /* skip DO */
            block(ls);
            check_match(ls, TK_END as libc::c_int, TK_DO as libc::c_int,
                        line);
            return 0 as libc::c_int
        }
        263 => {
            /* stat -> forstat */
            forstat(ls, line);
            return 0 as libc::c_int
        }
        270 => {
            /* stat -> repeatstat */
            repeatstat(ls, line);
            return 0 as libc::c_int
        }
        264 => {
            /* stat -> funcstat */
            funcstat(ls, line);
            return 0 as libc::c_int
        }
        266 => {
            /* stat -> localstat */
            localstat(ls);
            return 0 as libc::c_int
        }
        275 | 37 => {
            /* stat -> namestat */
            namestat(ls);
            return 0 as libc::c_int
        }
        271 => {
            /* stat -> retstat */
            retstat(ls);
            return 1 as libc::c_int
            /* must be last statement */
        }
        258 => {
            /* stat -> breakstat */
            breakstat(ls);
            return 1 as libc::c_int
            /* must be last statement */
        }
        _ => {
            luaK_error(ls,
                       b"<statement> expected\x00" as *const u8 as
                           *const libc::c_char);
            return 0 as libc::c_int
            /* to avoid warnings */
        }
    };
}
unsafe extern "C" fn parlist(mut ls: *mut LexState) {
    /* parlist -> [ param { ',' param } ] */
    let mut nparams: libc::c_int = 0 as libc::c_int;
    let mut dots: libc::c_int = 0 as libc::c_int;
    if (*ls).t.token != ')' as i32 {
        loop 
             /* is `parlist' not empty? */
             {
            match (*ls).t.token {
                277 => { next(ls); dots = 1 as libc::c_int }
                275 => {
                    let fresh12 = nparams;
                    nparams = nparams + 1;
                    new_localvar(ls, str_checkname(ls), fresh12);
                }
                _ => {
                    luaK_error(ls,
                               b"<name> or `...\' expected\x00" as *const u8
                                   as *const libc::c_char);
                }
            }
            if !(dots == 0 && optional(ls, ',' as i32) != 0) { break ; }
        }
    }
    code_params(ls, nparams, dots);
}
/*
** prototypes for recursive non-terminal functions
*/
unsafe extern "C" fn body(mut ls: *mut LexState, mut needself: libc::c_int,
                          mut line: libc::c_int) {
    /* body ->  '(' parlist ')' chunk END */
    let mut new_fs: FuncState =
        FuncState{f: 0 as *mut Proto,
                  prev: 0 as *mut FuncState,
                  ls: 0 as *mut LexState,
                  L: 0 as *mut lua_State,
                  pc: 0,
                  lasttarget: 0,
                  jlt: 0,
                  stacklevel: 0,
                  nactloc: 0,
                  nupvalues: 0,
                  lastline: 0,
                  bl: 0 as *mut Breaklabel,
                  upvalues:
                      [expdesc{k: VGLOBAL, u: C2RustUnnamed_3{index: 0,},};
                          32],
                  actloc: [0; 200],};
    open_func(ls, &mut new_fs);
    (*new_fs.f).lineDefined = line;
    check(ls, '(' as i32);
    if needself != 0 {
        new_localvarstr(ls, b"self\x00" as *const u8 as *const libc::c_char,
                        0 as libc::c_int);
        adjustlocalvars(ls, 1 as libc::c_int);
    }
    parlist(ls);
    check(ls, ')' as i32);
    chunk(ls);
    check_match(ls, TK_END as libc::c_int, TK_FUNCTION as libc::c_int, line);
    close_func(ls);
    pushclosure(ls, &mut new_fs);
}
/* }====================================================================== */
unsafe extern "C" fn chunk(mut ls: *mut LexState) {
    /* chunk -> { stat [';'] } */
    let mut islast: libc::c_int = 0 as libc::c_int;
    while islast == 0 && block_follow((*ls).t.token) == 0 {
        islast = stat(ls);
        optional(ls, ';' as i32);
    };
}
