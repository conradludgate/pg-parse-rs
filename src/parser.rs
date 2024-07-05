#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
// #![feature(extern_types, linkage)]
// extern "C" {
//     pub type SelectLimit;
//     pub type ImportQual;
//     pub type PrivTarget;
//     fn abort() -> !;
//     fn strlen(_: *const libc::c_char) -> libc::c_ulong;
//     static mut _DefaultRuneLocale: _RuneLocale;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errcode(sqlerrcode: libc::c_int) -> libc::c_int;
//     fn errmsg(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn errhint(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn palloc(size: usize) -> *mut libc::c_void;
//     fn repalloc(pointer: *mut libc::c_void, size: usize) -> *mut libc::c_void;
//     fn pg_unicode_to_server(c: pg_wchar, s: *mut libc::c_uchar);
//     fn base_yyparse(yyscanner: core_yyscan_t) -> libc::c_int;
//     fn parser_init(yyext: *mut base_yy_extra_type);
//     fn scanner_yyerror(message: *const libc::c_char, yyscanner: core_yyscan_t) -> !;
//     fn cancel_scanner_errposition_callback(scbstate: *mut ScannerCallbackState);
//     fn setup_scanner_errposition_callback(
//         scbstate: *mut ScannerCallbackState,
//         yyscanner: core_yyscan_t,
//         location: libc::c_int,
//     );
//     fn scanner_errposition(
//         location: libc::c_int,
//         yyscanner: core_yyscan_t,
//     ) -> libc::c_int;
//     fn core_yylex(
//         lvalp: *mut core_YYSTYPE,
//         llocp: *mut libc::c_int,
//         yyscanner: core_yyscan_t,
//     ) -> libc::c_int;
//     fn scanner_finish(yyscanner: core_yyscan_t);
//     fn scanner_init(
//         str: *const libc::c_char,
//         yyext: *mut core_yy_extra_type,
//         keywordlist: *const ScanKeywordList,
//         keyword_tokens: *const u16,
//     ) -> core_yyscan_t;
//     static ScanKeywordTokens: [u16; 0];
//     static ScanKeywords: ScanKeywordList;
//     fn truncate_identifier(ident: *mut libc::c_char, len: libc::c_int, warn: bool);
//     fn scanner_isspace(ch: libc::c_char) -> bool;
// }
use super::*;
// pub type Oid = libc::c_uint;
pub type __u32_t = libc::c_uint;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
// pub type isize = __darwin_size_t;
// pub type bool = libc::c_uchar;
// pub type i32 = libc::c_int;
// pub type u16 = libc::c_ushort;
// pub type usize = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __u32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __u32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [libc::c_char; 8],
    pub __encoding: [libc::c_char; 32],
    pub __sgetrune: Option<
        unsafe extern "C" fn(
            *const libc::c_char,
            __darwin_size_t,
            *mut *const libc::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut libc::c_char,
            __darwin_size_t,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__u32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: libc::c_int,
    pub __ncharclasses: libc::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ErrorContextCallback {
    pub previous: *mut ErrorContextCallback,
    pub callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
pub type pg_wchar = libc::c_uint;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct List {
//     pub type_0: NodeTag,
//     pub length: libc::c_int,
//     pub max_length: libc::c_int,
//     pub elements: *mut ListCell,
//     pub initial_elements: [ListCell; 0],
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union ListCell {
//     pub ptr_value: *mut libc::c_void,
//     pub int_value: libc::c_int,
//     pub oid_value: Oid,
// }

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Node {
//     pub type_0: NodeTag,
// }
// pub type JoinType = libc::c_uint;
pub const JOIN_UNIQUE_INNER: JoinType = 7;
pub const JOIN_UNIQUE_OUTER: JoinType = 6;
pub const JOIN_ANTI: JoinType = 5;
pub const JOIN_SEMI: JoinType = 4;
pub const JOIN_RIGHT: JoinType = 3;
pub const JOIN_FULL: JoinType = 2;
pub const JOIN_LEFT: JoinType = 1;
pub const JOIN_INNER: JoinType = 0;
pub type OnConflictAction = libc::c_uint;
pub const ONCONFLICT_UPDATE: OnConflictAction = 2;
pub const ONCONFLICT_NOTHING: OnConflictAction = 1;
pub const ONCONFLICT_NONE: OnConflictAction = 0;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Alias {
//     pub type_0: NodeTag,
//     pub aliasname: *mut libc::c_char,
//     pub colnames: *mut List,
// }
pub type OnCommitAction = libc::c_uint;
pub const ONCOMMIT_DROP: OnCommitAction = 3;
pub const ONCOMMIT_DELETE_ROWS: OnCommitAction = 2;
pub const ONCOMMIT_PRESERVE_ROWS: OnCommitAction = 1;
pub const ONCOMMIT_NOOP: OnCommitAction = 0;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct RangeVar {
//     pub type_0: NodeTag,
//     pub catalogname: *mut libc::c_char,
//     pub schemaname: *mut libc::c_char,
//     pub relname: *mut libc::c_char,
//     pub inh: bool,
//     pub relpersistence: libc::c_char,
//     pub alias: *mut Alias,
//     pub location: libc::c_int,
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IntoClause {
    pub type_0: NodeTag,
    pub rel: *mut RangeVar,
    pub colNames: *mut List,
    pub accessMethod: *mut libc::c_char,
    pub options: *mut List,
    pub onCommit: OnCommitAction,
    pub tableSpaceName: *mut libc::c_char,
    pub viewQuery: *mut Node,
    pub skipData: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JoinExpr {
    pub type_0: NodeTag,
    pub jointype: JoinType,
    pub isNatural: bool,
    pub larg: *mut Node,
    pub rarg: *mut Node,
    pub usingClause: *mut List,
    pub quals: *mut Node,
    pub alias: *mut Alias,
    pub rtindex: libc::c_int,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Value {
//     pub type_0: NodeTag,
//     pub val: ValUnion,
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub union ValUnion {
    pub ival: libc::c_int,
    pub str_0: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionBoundSpec {
    pub type_0: NodeTag,
    pub strategy: libc::c_char,
    pub is_default: bool,
    pub modulus: libc::c_int,
    pub remainder: libc::c_int,
    pub listdatums: *mut List,
    pub lowerdatums: *mut List,
    pub upperdatums: *mut List,
    pub location: libc::c_int,
}
pub type OverridingKind = libc::c_uint;
pub const OVERRIDING_SYSTEM_VALUE: OverridingKind = 2;
pub const OVERRIDING_USER_VALUE: OverridingKind = 1;
pub const OVERRIDING_NOT_SET: OverridingKind = 0;
pub type SortByDir = libc::c_uint;
pub const SORTBY_USING: SortByDir = 3;
pub const SORTBY_DESC: SortByDir = 2;
pub const SORTBY_ASC: SortByDir = 1;
pub const SORTBY_DEFAULT: SortByDir = 0;
pub type SortByNulls = libc::c_uint;
pub const SORTBY_NULLS_LAST: SortByNulls = 2;
pub const SORTBY_NULLS_FIRST: SortByNulls = 1;
pub const SORTBY_NULLS_DEFAULT: SortByNulls = 0;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct TypeName {
//     pub type_0: NodeTag,
//     pub names: *mut List,
//     pub typeOid: Oid,
//     pub setof: bool,
//     pub pct_type: bool,
//     pub typmods: *mut List,
//     pub typemod: i32,
//     pub arrayBounds: *mut List,
//     pub location: libc::c_int,
// }
pub type RoleSpecType = libc::c_uint;
pub const ROLESPEC_PUBLIC: RoleSpecType = 4;
pub const ROLESPEC_SESSION_USER: RoleSpecType = 3;
pub const ROLESPEC_CURRENT_USER: RoleSpecType = 2;
pub const ROLESPEC_CURRENT_ROLE: RoleSpecType = 1;
pub const ROLESPEC_CSTRING: RoleSpecType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RoleSpec {
    pub type_0: NodeTag,
    pub roletype: RoleSpecType,
    pub rolename: *mut libc::c_char,
    pub location: libc::c_int,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct WindowDef {
//     pub type_0: NodeTag,
//     pub name: *mut libc::c_char,
//     pub refname: *mut libc::c_char,
//     pub partitionClause: *mut List,
//     pub orderClause: *mut List,
//     pub frameOptions: libc::c_int,
//     pub startOffset: *mut Node,
//     pub endOffset: *mut Node,
//     pub location: libc::c_int,
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct A_Indices {
    pub type_0: NodeTag,
    pub is_slice: bool,
    pub lidx: *mut Node,
    pub uidx: *mut Node,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct ResTarget {
//     pub type_0: NodeTag,
//     pub name: *mut libc::c_char,
//     pub indirection: *mut List,
//     pub val: *mut Node,
//     pub location: libc::c_int,
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SortBy {
    pub type_0: NodeTag,
    pub node: *mut Node,
    pub sortby_dir: SortByDir,
    pub sortby_nulls: SortByNulls,
    pub useOp: *mut List,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexElem {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub expr: *mut Node,
    pub indexcolname: *mut libc::c_char,
    pub collation: *mut List,
    pub opclass: *mut List,
    pub opclassopts: *mut List,
    pub ordering: SortByDir,
    pub nulls_ordering: SortByNulls,
}
pub type DefElemAction = libc::c_uint;
pub const DEFELEM_DROP: DefElemAction = 3;
pub const DEFELEM_ADD: DefElemAction = 2;
pub const DEFELEM_SET: DefElemAction = 1;
pub const DEFELEM_UNSPEC: DefElemAction = 0;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct DefElem {
//     pub type_0: NodeTag,
//     pub defnamespace: *mut libc::c_char,
//     pub defname: *mut libc::c_char,
//     pub arg: *mut Node,
//     pub defaction: DefElemAction,
//     pub location: libc::c_int,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct PartitionElem {
//     pub type_0: NodeTag,
//     pub name: *mut libc::c_char,
//     pub expr: *mut Node,
//     pub collation: *mut List,
//     pub opclass: *mut List,
//     pub location: libc::c_int,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct PartitionSpec {
//     pub type_0: NodeTag,
//     pub strategy: *mut libc::c_char,
//     pub partParams: *mut List,
//     pub location: libc::c_int,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct WithClause {
//     pub type_0: NodeTag,
//     pub ctes: *mut List,
//     pub recursive: bool,
//     pub location: libc::c_int,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct InferClause {
//     pub type_0: NodeTag,
//     pub indexElems: *mut List,
//     pub whereClause: *mut Node,
//     pub conname: *mut libc::c_char,
//     pub location: libc::c_int,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct OnConflictClause {
//     pub type_0: NodeTag,
//     pub action: OnConflictAction,
//     pub infer: *mut InferClause,
//     pub targetList: *mut List,
//     pub whereClause: *mut Node,
//     pub location: libc::c_int,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct InsertStmt {
//     pub type_0: NodeTag,
//     pub relation: *mut RangeVar,
//     pub cols: *mut List,
//     pub selectStmt: *mut Node,
//     pub onConflictClause: *mut OnConflictClause,
//     pub returningList: *mut List,
//     pub withClause: *mut WithClause,
//     pub override_0: OverridingKind,
// }
pub type ObjectType = libc::c_uint;
pub const OBJECT_VIEW: ObjectType = 49;
pub const OBJECT_USER_MAPPING: ObjectType = 48;
pub const OBJECT_TYPE: ObjectType = 47;
pub const OBJECT_TSTEMPLATE: ObjectType = 46;
pub const OBJECT_TSPARSER: ObjectType = 45;
pub const OBJECT_TSDICTIONARY: ObjectType = 44;
pub const OBJECT_TSCONFIGURATION: ObjectType = 43;
pub const OBJECT_TRIGGER: ObjectType = 42;
pub const OBJECT_TRANSFORM: ObjectType = 41;
pub const OBJECT_TABLESPACE: ObjectType = 40;
pub const OBJECT_TABLE: ObjectType = 39;
pub const OBJECT_TABCONSTRAINT: ObjectType = 38;
pub const OBJECT_STATISTIC_EXT: ObjectType = 37;
pub const OBJECT_SUBSCRIPTION: ObjectType = 36;
pub const OBJECT_SEQUENCE: ObjectType = 35;
pub const OBJECT_SCHEMA: ObjectType = 34;
pub const OBJECT_RULE: ObjectType = 33;
pub const OBJECT_ROUTINE: ObjectType = 32;
pub const OBJECT_ROLE: ObjectType = 31;
pub const OBJECT_PUBLICATION_REL: ObjectType = 30;
pub const OBJECT_PUBLICATION: ObjectType = 29;
pub const OBJECT_PROCEDURE: ObjectType = 28;
pub const OBJECT_POLICY: ObjectType = 27;
pub const OBJECT_OPFAMILY: ObjectType = 26;
pub const OBJECT_OPERATOR: ObjectType = 25;
pub const OBJECT_OPCLASS: ObjectType = 24;
pub const OBJECT_MATVIEW: ObjectType = 23;
pub const OBJECT_LARGEOBJECT: ObjectType = 22;
pub const OBJECT_LANGUAGE: ObjectType = 21;
pub const OBJECT_INDEX: ObjectType = 20;
pub const OBJECT_FUNCTION: ObjectType = 19;
pub const OBJECT_FOREIGN_TABLE: ObjectType = 18;
pub const OBJECT_FOREIGN_SERVER: ObjectType = 17;
pub const OBJECT_FDW: ObjectType = 16;
pub const OBJECT_EXTENSION: ObjectType = 15;
pub const OBJECT_EVENT_TRIGGER: ObjectType = 14;
pub const OBJECT_DOMCONSTRAINT: ObjectType = 13;
pub const OBJECT_DOMAIN: ObjectType = 12;
pub const OBJECT_DEFACL: ObjectType = 11;
pub const OBJECT_DEFAULT: ObjectType = 10;
pub const OBJECT_DATABASE: ObjectType = 9;
pub const OBJECT_CONVERSION: ObjectType = 8;
pub const OBJECT_COLLATION: ObjectType = 7;
pub const OBJECT_COLUMN: ObjectType = 6;
pub const OBJECT_CAST: ObjectType = 5;
pub const OBJECT_ATTRIBUTE: ObjectType = 4;
pub const OBJECT_AMPROC: ObjectType = 3;
pub const OBJECT_AMOP: ObjectType = 2;
pub const OBJECT_AGGREGATE: ObjectType = 1;
pub const OBJECT_ACCESS_METHOD: ObjectType = 0;
pub type DropBehavior = libc::c_uint;
pub const DROP_CASCADE: DropBehavior = 1;
pub const DROP_RESTRICT: DropBehavior = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectWithArgs {
    pub type_0: NodeTag,
    pub objname: *mut List,
    pub objargs: *mut List,
    pub args_unspecified: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccessPriv {
    pub type_0: NodeTag,
    pub priv_name: *mut libc::c_char,
    pub cols: *mut List,
}
pub type VariableSetKind = libc::c_uint;
pub const VAR_RESET_ALL: VariableSetKind = 5;
pub const VAR_RESET: VariableSetKind = 4;
pub const VAR_SET_MULTI: VariableSetKind = 3;
pub const VAR_SET_CURRENT: VariableSetKind = 2;
pub const VAR_SET_DEFAULT: VariableSetKind = 1;
pub const VAR_SET_VALUE: VariableSetKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSetStmt {
    pub type_0: NodeTag,
    pub kind: VariableSetKind,
    pub name: *mut libc::c_char,
    pub args: *mut List,
    pub is_local: bool,
}
pub type FunctionParameterMode = libc::c_uint;
pub const FUNC_PARAM_TABLE: FunctionParameterMode = 116;
pub const FUNC_PARAM_VARIADIC: FunctionParameterMode = 118;
pub const FUNC_PARAM_INOUT: FunctionParameterMode = 98;
pub const FUNC_PARAM_OUT: FunctionParameterMode = 111;
pub const FUNC_PARAM_IN: FunctionParameterMode = 105;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionParameter {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub argType: *mut TypeName,
    pub mode: FunctionParameterMode,
    pub defexpr: *mut Node,
}
pub type ScanKeywordHashFunc =
    Option<unsafe extern "C" fn(*const libc::c_void, isize) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScanKeywordList {
    pub kw_string: *const libc::c_char,
    pub kw_offsets: *const u16,
    pub hash: ScanKeywordHashFunc,
    pub num_keywords: libc::c_int,
    pub max_kw_len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union core_YYSTYPE {
    pub ival: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub keyword: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct core_yy_extra_type {
    pub scanbuf: *mut libc::c_char,
    pub scanbuflen: usize,
    pub keywordlist: *const ScanKeywordList,
    pub keyword_tokens: *const u16,
    pub backslash_quote: libc::c_int,
    pub escape_string_warning: bool,
    pub standard_conforming_strings: bool,
    pub literalbuf: *mut libc::c_char,
    pub literallen: libc::c_int,
    pub literalalloc: libc::c_int,
    pub state_before_str_stop: libc::c_int,
    pub xcdepth: libc::c_int,
    pub dolqstart: *mut libc::c_char,
    pub save_yylloc: libc::c_int,
    pub utf16_first_part: i32,
    pub warn_on_first_escape: bool,
    pub saw_non_ascii: bool,
}
pub type core_yyscan_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScannerCallbackState {
    pub yyscanner: core_yyscan_t,
    pub location: libc::c_int,
    pub errcallback: ErrorContextCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub core_yystype: core_YYSTYPE,
    pub ival: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub keyword: *const libc::c_char,
    pub chr: libc::c_char,
    pub boolean: bool,
    pub jtype: JoinType,
    pub dbehavior: DropBehavior,
    pub oncommit: OnCommitAction,
    pub list: *mut List,
    pub node: *mut Node,
    pub value: *mut Value,
    pub objtype: ObjectType,
    pub typnam: *mut TypeName,
    pub fun_param: *mut FunctionParameter,
    pub fun_param_mode: FunctionParameterMode,
    pub objwithargs: *mut ObjectWithArgs,
    pub defelt: *mut DefElem,
    pub sortby: *mut SortBy,
    pub windef: *mut WindowDef,
    pub jexpr: *mut JoinExpr,
    pub ielem: *mut IndexElem,
    pub alias: *mut Alias,
    pub range: *mut RangeVar,
    pub into: *mut IntoClause,
    pub with: *mut WithClause,
    pub infer: *mut InferClause,
    pub onconflict: *mut OnConflictClause,
    pub aind: *mut A_Indices,
    pub target: *mut ResTarget,
    pub privtarget: *mut PrivTarget,
    pub accesspriv: *mut AccessPriv,
    pub importqual: *mut ImportQual,
    pub istmt: *mut InsertStmt,
    pub vsetstmt: *mut VariableSetStmt,
    pub partelem: *mut PartitionElem,
    pub partspec: *mut PartitionSpec,
    pub partboundspec: *mut PartitionBoundSpec,
    pub rolespec: *mut RoleSpec,
    pub selectlimit: *mut SelectLimit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base_yy_extra_type {
    pub core_yy_extra: core_yy_extra_type,
    pub have_lookahead: bool,
    pub lookahead_token: libc::c_int,
    pub lookahead_yylval: core_YYSTYPE,
    pub lookahead_yylloc: libc::c_int,
    pub lookahead_end: *mut libc::c_char,
    pub lookahead_hold_char: libc::c_char,
    pub parsetree: *mut List,
}
pub type RawParseMode = libc::c_uint;
pub const RAW_PARSE_PLPGSQL_ASSIGN3: RawParseMode = 5;
pub const RAW_PARSE_PLPGSQL_ASSIGN2: RawParseMode = 4;
pub const RAW_PARSE_PLPGSQL_ASSIGN1: RawParseMode = 3;
pub const RAW_PARSE_PLPGSQL_EXPR: RawParseMode = 2;
pub const RAW_PARSE_TYPE_NAME: RawParseMode = 1;
pub const RAW_PARSE_DEFAULT: RawParseMode = 0;
#[inline]
unsafe extern "C" fn __isctype(
    mut _c: __darwin_ct_rune_t,
    mut _f: libc::c_ulong,
) -> __darwin_ct_rune_t {
    return if _c < 0 as libc::c_int || _c >= (1 as libc::c_int) << 8 as libc::c_int {
        0 as libc::c_int
    } else {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0) as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isxdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x10000 as libc::c_long as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn is_valid_unicode_codepoint(mut c: pg_wchar) -> bool {
    (c > 0 as libc::c_int as pg_wchar && c <= 0x10ffff as libc::c_int as pg_wchar)
}
#[inline]
unsafe extern "C" fn is_utf16_surrogate_first(mut c: pg_wchar) -> bool {
    (c >= 0xd800 as libc::c_int as pg_wchar && c <= 0xdbff as libc::c_int as pg_wchar)
}
#[inline]
unsafe extern "C" fn is_utf16_surrogate_second(mut c: pg_wchar) -> bool {
    (c >= 0xdc00 as libc::c_int as pg_wchar && c <= 0xdfff as libc::c_int as pg_wchar)
}
#[inline]
unsafe extern "C" fn surrogate_pair_to_codepoint(
    mut first: pg_wchar,
    mut second: pg_wchar,
) -> pg_wchar {
    return ((first & 0x3ff as libc::c_int as pg_wchar) << 10 as libc::c_int)
        .wrapping_add(0x10000 as libc::c_int as pg_wchar)
        .wrapping_add(second & 0x3ff as libc::c_int as pg_wchar);
}
#[no_mangle]
pub unsafe extern "C" fn raw_parser(
    mut str: *const libc::c_char,
    mut mode: RawParseMode,
) -> *mut List {
    let mut yyscanner: core_yyscan_t = 0 as *mut libc::c_void;
    let mut yyextra: base_yy_extra_type = base_yy_extra_type {
        core_yy_extra: core_yy_extra_type {
            scanbuf: 0 as *mut libc::c_char,
            scanbuflen: 0,
            keywordlist: 0 as *const ScanKeywordList,
            keyword_tokens: 0 as *const u16,
            backslash_quote: 0,
            escape_string_warning: 0,
            standard_conforming_strings: 0,
            literalbuf: 0 as *mut libc::c_char,
            literallen: 0,
            literalalloc: 0,
            state_before_str_stop: 0,
            xcdepth: 0,
            dolqstart: 0 as *mut libc::c_char,
            save_yylloc: 0,
            utf16_first_part: 0,
            warn_on_first_escape: 0,
            saw_non_ascii: 0,
        },
        have_lookahead: 0,
        lookahead_token: 0,
        lookahead_yylval: core_YYSTYPE { ival: 0 },
        lookahead_yylloc: 0,
        lookahead_end: 0 as *mut libc::c_char,
        lookahead_hold_char: 0,
        parsetree: 0 as *mut List,
    };
    let mut yyresult: libc::c_int = 0;
    yyscanner = scanner_init(
        str,
        &mut yyextra.core_yy_extra,
        &ScanKeywords,
        ScanKeywordTokens.as_ptr(),
    );
    if mode as libc::c_uint == RAW_PARSE_DEFAULT as libc::c_int as libc::c_uint {
        yyextra.have_lookahead = false;
    } else {
        static mut mode_token: [libc::c_int; 6] = [
            0 as libc::c_int,
            730 as libc::c_int,
            731 as libc::c_int,
            732 as libc::c_int,
            733 as libc::c_int,
            734 as libc::c_int,
        ];
        yyextra.have_lookahead = true;
        yyextra.lookahead_token = mode_token[mode as usize];
        yyextra.lookahead_yylloc = 0 as libc::c_int;
        yyextra.lookahead_end = 0 as *mut libc::c_char;
    }
    parser_init(&mut yyextra);
    yyresult = base_yyparse(yyscanner);
    scanner_finish(yyscanner);
    if yyresult != 0 {
        return 0 as *mut libc::c_void as *mut List;
    }
    return yyextra.parsetree;
}
#[no_mangle]
pub unsafe extern "C" fn base_yylex(
    mut lvalp: *mut YYSTYPE,
    mut llocp: *mut libc::c_int,
    mut yyscanner: core_yyscan_t,
) -> libc::c_int {
    let mut yyextra: *mut base_yy_extra_type = *(yyscanner as *mut *mut base_yy_extra_type);
    let mut cur_token: libc::c_int = 0;
    let mut next_token: libc::c_int = 0;
    let mut cur_token_length: libc::c_int = 0;
    let mut cur_yylloc: libc::c_int = 0;
    if (*yyextra).have_lookahead != 0 {
        cur_token = (*yyextra).lookahead_token;
        (*lvalp).core_yystype = (*yyextra).lookahead_yylval;
        *llocp = (*yyextra).lookahead_yylloc;
        if !((*yyextra).lookahead_end).is_null() {
            *(*yyextra).lookahead_end = (*yyextra).lookahead_hold_char;
        }
        (*yyextra).have_lookahead = false;
    } else {
        cur_token = core_yylex(&mut (*lvalp).core_yystype, llocp, yyscanner);
    }
    match cur_token {
        520 => {
            cur_token_length = 3 as libc::c_int;
        }
        527 => {
            cur_token_length = 5 as libc::c_int;
        }
        706 => {
            cur_token_length = 4 as libc::c_int;
        }
        259 | 262 => {
            cur_token_length =
                strlen(((*yyextra).core_yy_extra.scanbuf).offset(*llocp as isize)) as libc::c_int;
        }
        _ => return cur_token,
    }
    (*yyextra).lookahead_end = ((*yyextra).core_yy_extra.scanbuf)
        .offset(*llocp as isize)
        .offset(cur_token_length as isize);
    cur_yylloc = *llocp;
    next_token = core_yylex(&mut (*yyextra).lookahead_yylval, llocp, yyscanner);
    (*yyextra).lookahead_token = next_token;
    (*yyextra).lookahead_yylloc = *llocp;
    *llocp = cur_yylloc;
    (*yyextra).lookahead_hold_char = *(*yyextra).lookahead_end;
    *(*yyextra).lookahead_end = '\0' as i32 as libc::c_char;
    (*yyextra).have_lookahead = true;
    match cur_token {
        520 => match next_token {
            304 | 447 | 484 | 442 | 627 => {
                cur_token = 727 as libc::c_int;
            }
            _ => {}
        },
        527 => match next_token {
            415 | 477 => {
                cur_token = 728 as libc::c_int;
            }
            _ => {}
        },
        706 => match next_token {
            661 | 542 => {
                cur_token = 729 as libc::c_int;
            }
            _ => {}
        },
        259 | 262 => {
            if next_token == 675 as libc::c_int {
                let mut escstr: *const libc::c_char = 0 as *const libc::c_char;
                cur_yylloc = *llocp;
                *(*yyextra).lookahead_end = (*yyextra).lookahead_hold_char;
                next_token = core_yylex(&mut (*yyextra).lookahead_yylval, llocp, yyscanner);
                if next_token != 261 as libc::c_int {
                    scanner_yyerror(
                        b"UESCAPE must be followed by a simple string literal\0" as *const u8
                            as *const libc::c_char,
                        yyscanner,
                    );
                }
                escstr = (*yyextra).lookahead_yylval.str_0;
                if strlen(escstr) != 1 as libc::c_int as libc::c_ulong
                    || check_uescapechar(*escstr.offset(0 as libc::c_int as isize) as libc::c_uchar)
                        == 0
                {
                    scanner_yyerror(
                        b"invalid Unicode escape character\0" as *const u8 as *const libc::c_char,
                        yyscanner,
                    );
                }
                *llocp = cur_yylloc;
                (*lvalp).core_yystype.str_0 = str_udeescape(
                    (*lvalp).core_yystype.str_0,
                    *escstr.offset(0 as libc::c_int as isize),
                    *llocp,
                    yyscanner,
                );
                (*yyextra).have_lookahead = false;
            } else {
                (*lvalp).core_yystype.str_0 = str_udeescape(
                    (*lvalp).core_yystype.str_0,
                    '\\' as i32 as libc::c_char,
                    *llocp,
                    yyscanner,
                );
            }
            if cur_token == 259 as libc::c_int {
                truncate_identifier(
                    (*lvalp).core_yystype.str_0,
                    strlen((*lvalp).core_yystype.str_0) as libc::c_int,
                    true,
                );
                cur_token = 258 as libc::c_int;
            } else if cur_token == 262 as libc::c_int {
                cur_token = 261 as libc::c_int;
            }
        }
        _ => {}
    }
    return cur_token;
}
unsafe extern "C" fn hexval(mut c: libc::c_uchar) -> libc::c_uint {
    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
        return (c as libc::c_int - '0' as i32) as libc::c_uint;
    }
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
        return (c as libc::c_int - 'a' as i32 + 0xa as libc::c_int) as libc::c_uint;
    }
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
        return (c as libc::c_int - 'A' as i32 + 0xa as libc::c_int) as libc::c_uint;
    }
    let elevel_: libc::c_int = 21 as libc::c_int;
    let mut __error: libc::c_int = 0;
    if errstart(elevel_, 0 as *const libc::c_char) != 0 {
        errmsg_internal(b"invalid hexadecimal digit\0" as *const u8 as *const libc::c_char);
        errfinish(
            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parser.c\0" as *const u8
                as *const libc::c_char,
            310 as libc::c_int,
            0 as *const libc::c_char,
        );
    }
    if elevel_ >= 21 as libc::c_int {
        abort();
    }
    return 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn check_unicode_value(mut c: pg_wchar) {
    if is_valid_unicode_codepoint(c) == 0 {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errcode(ERRCODE_SYNTAX_ERROR);
            errmsg(b"invalid Unicode escape value\0" as *const u8 as *const libc::c_char);
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parser.c\0" as *const u8
                    as *const libc::c_char,
                321 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
}
unsafe extern "C" fn check_uescapechar(mut escape: libc::c_uchar) -> bool {
    if isxdigit(escape as libc::c_int) != 0
        || escape as libc::c_int == '+' as i32
        || escape as libc::c_int == '\'' as i32
        || escape as libc::c_int == '"' as i32
        || scanner_isspace(escape as libc::c_char) as libc::c_int != 0
    {
        return false;
    } else {
        return true;
    };
}
unsafe extern "C" fn str_udeescape(
    mut str: *const libc::c_char,
    mut escape: libc::c_char,
    mut position: libc::c_int,
    mut yyscanner: core_yyscan_t,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut in_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_len: isize = 0;
    let mut pair_first: pg_wchar = 0 as libc::c_int as pg_wchar;
    let mut scbstate: ScannerCallbackState = ScannerCallbackState {
        yyscanner: 0 as *mut libc::c_void,
        location: 0,
        errcallback: ErrorContextCallback {
            previous: 0 as *mut ErrorContextCallback,
            callback: None,
            arg: 0 as *mut libc::c_void,
        },
    };
    new_len = (strlen(str))
        .wrapping_add(16 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    new = palloc(new_len) as *mut libc::c_char;
    in_0 = str;
    out = new;
    loop {
        if !(*in_0 != 0) {
            current_block = 9627623479216730126;
            break;
        }
        let mut out_dist: isize = out.offset_from(new) as libc::c_long as isize;
        if out_dist > new_len.wrapping_sub((16 as libc::c_int + 1 as libc::c_int) as isize) {
            new_len = new_len * 2 as libc::c_int as isize;
            new = repalloc(new as *mut libc::c_void, new_len) as *mut libc::c_char;
            out = new.offset(out_dist as isize);
        }
        if *in_0.offset(0 as libc::c_int as isize) as libc::c_int == escape as libc::c_int {
            setup_scanner_errposition_callback(
                &mut scbstate,
                yyscanner,
                (in_0.offset_from(str) as libc::c_long
                    + position as libc::c_long
                    + 3 as libc::c_int as libc::c_long) as libc::c_int,
            );
            if *in_0.offset(1 as libc::c_int as isize) as libc::c_int == escape as libc::c_int {
                if pair_first != 0 {
                    current_block = 2947293050515664763;
                    break;
                }
                let fresh0 = out;
                out = out.offset(1);
                *fresh0 = escape;
                in_0 = in_0.offset(2 as libc::c_int as isize);
            } else if isxdigit(
                *in_0.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            ) != 0
                && isxdigit(*in_0.offset(2 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
                    != 0
                && isxdigit(*in_0.offset(3 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
                    != 0
                && isxdigit(*in_0.offset(4 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
                    != 0
            {
                let mut unicode: pg_wchar = 0;
                unicode = (hexval(*in_0.offset(1 as libc::c_int as isize) as libc::c_uchar)
                    << 12 as libc::c_int)
                    .wrapping_add(
                        hexval(*in_0.offset(2 as libc::c_int as isize) as libc::c_uchar)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        hexval(*in_0.offset(3 as libc::c_int as isize) as libc::c_uchar)
                            << 4 as libc::c_int,
                    )
                    .wrapping_add(hexval(
                        *in_0.offset(4 as libc::c_int as isize) as libc::c_uchar
                    ));
                check_unicode_value(unicode);
                if pair_first != 0 {
                    if !(is_utf16_surrogate_second(unicode) != 0) {
                        current_block = 2947293050515664763;
                        break;
                    }
                    unicode = surrogate_pair_to_codepoint(pair_first, unicode);
                    pair_first = 0 as libc::c_int as pg_wchar;
                } else if is_utf16_surrogate_second(unicode) != 0 {
                    current_block = 2947293050515664763;
                    break;
                }
                if is_utf16_surrogate_first(unicode) != 0 {
                    pair_first = unicode;
                } else {
                    pg_unicode_to_server(unicode, out as *mut libc::c_uchar);
                    out = out.offset(strlen(out) as isize);
                }
                in_0 = in_0.offset(5 as libc::c_int as isize);
            } else if *in_0.offset(1 as libc::c_int as isize) as libc::c_int == '+' as i32
                && isxdigit(*in_0.offset(2 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
                    != 0
                && isxdigit(*in_0.offset(3 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
                    != 0
                && isxdigit(*in_0.offset(4 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
                    != 0
                && isxdigit(*in_0.offset(5 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
                    != 0
                && isxdigit(*in_0.offset(6 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
                    != 0
                && isxdigit(*in_0.offset(7 as libc::c_int as isize) as libc::c_uchar as libc::c_int)
                    != 0
            {
                let mut unicode_0: pg_wchar = 0;
                unicode_0 = (hexval(*in_0.offset(2 as libc::c_int as isize) as libc::c_uchar)
                    << 20 as libc::c_int)
                    .wrapping_add(
                        hexval(*in_0.offset(3 as libc::c_int as isize) as libc::c_uchar)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        hexval(*in_0.offset(4 as libc::c_int as isize) as libc::c_uchar)
                            << 12 as libc::c_int,
                    )
                    .wrapping_add(
                        hexval(*in_0.offset(5 as libc::c_int as isize) as libc::c_uchar)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        hexval(*in_0.offset(6 as libc::c_int as isize) as libc::c_uchar)
                            << 4 as libc::c_int,
                    )
                    .wrapping_add(hexval(
                        *in_0.offset(7 as libc::c_int as isize) as libc::c_uchar
                    ));
                check_unicode_value(unicode_0);
                if pair_first != 0 {
                    if !(is_utf16_surrogate_second(unicode_0) != 0) {
                        current_block = 2947293050515664763;
                        break;
                    }
                    unicode_0 = surrogate_pair_to_codepoint(pair_first, unicode_0);
                    pair_first = 0 as libc::c_int as pg_wchar;
                } else if is_utf16_surrogate_second(unicode_0) != 0 {
                    current_block = 2947293050515664763;
                    break;
                }
                if is_utf16_surrogate_first(unicode_0) != 0 {
                    pair_first = unicode_0;
                } else {
                    pg_unicode_to_server(unicode_0, out as *mut libc::c_uchar);
                    out = out.offset(strlen(out) as isize);
                }
                in_0 = in_0.offset(8 as libc::c_int as isize);
            } else {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                    errcode(ERRCODE_SYNTAX_ERROR);
                    errmsg(b"invalid Unicode escape\0" as *const u8 as *const libc::c_char);
                    errhint(
                        b"Unicode escapes must be \\XXXX or \\+XXXXXX.\0" as *const u8
                            as *const libc::c_char,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parser.c\0"
                            as *const u8 as *const libc::c_char,
                        469 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            cancel_scanner_errposition_callback(&mut scbstate);
        } else {
            if pair_first != 0 {
                current_block = 2947293050515664763;
                break;
            }
            let fresh1 = in_0;
            in_0 = in_0.offset(1);
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 = *fresh1;
        }
    }
    match current_block {
        9627623479216730126 => {
            if !(pair_first != 0) {
                *out = '\0' as i32 as libc::c_char;
                return new;
            }
        }
        _ => {}
    }
    let elevel__0: libc::c_int = 21 as libc::c_int;
    let mut __error_0: libc::c_int = 0;
    if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
        errcode(ERRCODE_SYNTAX_ERROR);
        errmsg(b"invalid Unicode surrogate pair\0" as *const u8 as *const libc::c_char);
        scanner_errposition(
            (in_0.offset_from(str) as libc::c_long
                + position as libc::c_long
                + 3 as libc::c_int as libc::c_long) as libc::c_int,
            yyscanner,
        );
        errfinish(
            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parser.c\0" as *const u8
                as *const libc::c_char,
            499 as libc::c_int,
            0 as *const libc::c_char,
        );
    }
    if elevel__0 >= 21 as libc::c_int {
        abort();
    }
    return 0 as *mut libc::c_char;
}
