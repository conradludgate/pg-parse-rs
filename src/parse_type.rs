#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
// #![feature(extern_types)]
// extern "C" {
//     pub type RelationData;
//     pub type QueryEnvironment;
//     fn abort() -> !;
//     fn strlen(_: *const libc::c_char) -> libc::c_ulong;
//     fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn psprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
//     fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
//     fn palloc(size: usize) -> *mut libc::c_void;
//     fn pfree(pointer: *mut libc::c_void);
//     fn errcontext_msg(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn set_errcontext_domain(domain: *const libc::c_char) -> libc::c_int;
//     static mut error_context_stack: *mut ErrorContextCallback;
//     fn OidFunctionCall1Coll(functionId: Oid, collation: Oid, arg1: Datum) -> Datum;
//     fn OidInputFunctionCall(
//         functionId: Oid,
//         str: *mut libc::c_char,
//         typioparam: Oid,
//         typmod: i32,
//     ) -> Datum;
//     fn RangeVarGetRelidExtended(
//         relation: *const RangeVar,
//         lockmode: LOCKMODE,
//         flags: u32,
//         callback: RangeVarGetRelidCallback,
//         callback_arg: *mut libc::c_void,
//     ) -> Oid;
//     fn TypenameGetTypidExtended(typname: *const libc::c_char, temp_ok: bool) -> Oid;
//     fn DeconstructQualifiedName(
//         names: *mut List,
//         nspname_p: *mut *mut libc::c_char,
//         objname_p: *mut *mut libc::c_char,
//     );
//     fn get_collation_oid(collname: *mut List, missing_ok: bool) -> Oid;
//     fn LookupExplicitNamespace(nspname: *const libc::c_char, missing_ok: bool) -> Oid;
//     fn initStringInfo(str: StringInfo);
//     fn appendStringInfoString(str: StringInfo, s: *const libc::c_char);
//     fn appendStringInfoChar(str: StringInfo, ch: libc::c_char);
//     fn makeRangeVar(
//         schemaname: *mut libc::c_char,
//         relname: *mut libc::c_char,
//         location: libc::c_int,
//     ) -> *mut RangeVar;
//     fn setup_parser_errposition_callback(
//         pcbstate: *mut ParseCallbackState,
//         pstate: *mut ParseState,
//         location: libc::c_int,
//     );
//     fn cancel_parser_errposition_callback(pcbstate: *mut ParseCallbackState);
//     fn raw_parser(str: *const libc::c_char, mode: RawParseMode) -> *mut List;
//     fn format_type_be(type_oid: Oid) -> *mut libc::c_char;
//     fn get_attnum(relid: Oid, attname: *const libc::c_char) -> AttrNumber;
//     fn get_atttype(relid: Oid, attnum: AttrNumber) -> Oid;
//     fn getTypeIOParam(typeTuple: HeapTuple) -> Oid;
//     fn get_array_type(typid: Oid) -> Oid;
//     fn get_typcollation(typid: Oid) -> Oid;
//     fn SearchSysCache1(cacheId: libc::c_int, key1: Datum) -> HeapTuple;
//     fn ReleaseSysCache(tuple: HeapTuple);
// }
use super::*;
// pub type Oid = libc::c_uint;
pub type __darwin_size_t = libc::c_ulong;
// pub type usize = libc::c_ulong;
// pub type isize = __darwin_size_t;
// pub type bool = libc::c_uchar;
// pub type i16 = libc::c_short;
// pub type i32 = libc::c_int;
// pub type u8 = libc::c_uchar;
// pub type u16 = libc::c_ushort;
// pub type u32 = libc::c_uint;
pub type bits8 = u8;
// pub type uint64 = libc::c_ulong;
// pub type usize = isize;
// pub type Index = libc::c_uint;
pub type regproc = Oid;
pub type TransactionId = u32;
pub type CommandId = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameData {
    pub data: [libc::c_char; 64],
}
pub type NameData = nameData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ErrorContextCallback {
    pub previous: *mut ErrorContextCallback,
    pub callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
// pub type Datum = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockIdData {
    pub bi_hi: u16,
    pub bi_lo: u16,
}
pub type OffsetNumber = u16;
#[derive(Copy, Clone)]
#[repr(C, align(2))]
pub struct ItemPointerData(pub ItemPointerData_Inner);
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ItemPointerData_Inner {
    pub ip_blkid: BlockIdData,
    pub ip_posid: OffsetNumber,
}
#[allow(dead_code, non_upper_case_globals)]
const ItemPointerData_PADDING: usize =
    ::core::mem::size_of::<ItemPointerData>() - ::core::mem::size_of::<ItemPointerData_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleHeaderData {
    pub t_choice: C2RustUnnamed,
    pub t_ctid: ItemPointerData,
    pub t_infomask2: u16,
    pub t_infomask: u16,
    pub t_hoff: u8,
    pub t_bits: [bits8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub t_heap: HeapTupleFields,
    pub t_datum: DatumTupleFields,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DatumTupleFields {
    pub datum_len_: i32,
    pub datum_typmod: i32,
    pub datum_typeid: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleFields {
    pub t_xmin: TransactionId,
    pub t_xmax: TransactionId,
    pub t_field3: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub t_cid: CommandId,
    pub t_xvac: TransactionId,
}
pub type HeapTupleHeader = *mut HeapTupleHeaderData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleData {
    pub t_len: u32,
    pub t_self: ItemPointerData,
    pub t_tableOid: Oid,
    pub t_data: HeapTupleHeader,
}
pub type HeapTuple = *mut HeapTupleData;
// pub type AttrNumber = i16;

// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Node {
//     pub type_0: NodeTag,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Bitmapset {
//     pub nwords: libc::c_int,
//     pub words: [bitmapword; 0],
// }
pub type bitmapword = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringInfoData {
    pub data: *mut libc::c_char,
    pub len: libc::c_int,
    pub maxlen: libc::c_int,
    pub cursor: libc::c_int,
}
pub type CmdType = libc::c_uint;
pub const CMD_NOTHING: CmdType = 6;
pub const CMD_UTILITY: CmdType = 5;
pub const CMD_DELETE: CmdType = 4;
pub const CMD_INSERT: CmdType = 3;
pub const CMD_UPDATE: CmdType = 2;
pub const CMD_SELECT: CmdType = 1;
pub const CMD_UNKNOWN: CmdType = 0;
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
pub type LimitOption = libc::c_uint;
pub const LIMIT_OPTION_DEFAULT: LimitOption = 2;
pub const LIMIT_OPTION_WITH_TIES: LimitOption = 1;
pub const LIMIT_OPTION_COUNT: LimitOption = 0;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub union ListCell {
//     pub ptr_value: *mut libc::c_void,
//     pub int_value: libc::c_int,
//     pub oid_value: Oid,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct List {
//     pub type_0: NodeTag,
//     pub length: libc::c_int,
//     pub max_length: libc::c_int,
//     pub elements: *mut ListCell,
//     pub initial_elements: [ListCell; 0],
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ForEachState {
    pub l: *const List,
    pub i: libc::c_int,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Alias {
//     pub type_0: NodeTag,
//     pub aliasname: *mut libc::c_char,
//     pub colnames: *mut List,
// }
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
pub struct TableFunc {
    pub type_0: NodeTag,
    pub ns_uris: *mut List,
    pub ns_names: *mut List,
    pub docexpr: *mut Node,
    pub rowexpr: *mut Node,
    pub colnames: *mut List,
    pub coltypes: *mut List,
    pub coltypmods: *mut List,
    pub colcollations: *mut List,
    pub colexprs: *mut List,
    pub coldefexprs: *mut List,
    pub notnulls: *mut Bitmapset,
    pub ordinalitycol: libc::c_int,
    pub location: libc::c_int,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Expr {
//     pub type_0: NodeTag,
// }
pub type ParamKind = libc::c_uint;
pub const PARAM_MULTIEXPR: ParamKind = 3;
pub const PARAM_SUBLINK: ParamKind = 2;
pub const PARAM_EXEC: ParamKind = 1;
pub const PARAM_EXTERN: ParamKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Param {
    pub xpr: Expr,
    pub paramkind: ParamKind,
    pub paramid: libc::c_int,
    pub paramtype: Oid,
    pub paramtypmod: i32,
    pub paramcollid: Oid,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FromExpr {
    pub type_0: NodeTag,
    pub fromlist: *mut List,
    pub quals: *mut Node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnConflictExpr {
    pub type_0: NodeTag,
    pub action: OnConflictAction,
    pub arbiterElems: *mut List,
    pub arbiterWhere: *mut Node,
    pub constraint: Oid,
    pub onConflictSet: *mut List,
    pub onConflictWhere: *mut Node,
    pub exclRelIndex: libc::c_int,
    pub exclRelTlist: *mut List,
}
// pub type LOCKMODE = libc::c_int;
pub type RVROption = libc::c_uint;
pub const RVR_SKIP_LOCKED: RVROption = 4;
pub const RVR_NOWAIT: RVROption = 2;
pub const RVR_MISSING_OK: RVROption = 1;
pub type RangeVarGetRelidCallback =
    Option<unsafe extern "C" fn(*const RangeVar, Oid, Oid, *mut libc::c_void) -> ()>;
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
pub type OverridingKind = libc::c_uint;
pub const OVERRIDING_SYSTEM_VALUE: OverridingKind = 2;
pub const OVERRIDING_USER_VALUE: OverridingKind = 1;
pub const OVERRIDING_NOT_SET: OverridingKind = 0;
pub type QuerySource = libc::c_uint;
pub const QSRC_NON_INSTEAD_RULE: QuerySource = 4;
pub const QSRC_QUAL_INSTEAD_RULE: QuerySource = 3;
pub const QSRC_INSTEAD_RULE: QuerySource = 2;
pub const QSRC_PARSER: QuerySource = 1;
pub const QSRC_ORIGINAL: QuerySource = 0;
pub type AclMode = u32;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Query {
//     pub type_0: NodeTag,
//     pub commandType: CmdType,
//     pub querySource: QuerySource,
//     pub queryId: uint64,
//     pub canSetTag: bool,
//     pub utilityStmt: *mut Node,
//     pub resultRelation: libc::c_int,
//     pub hasAggs: bool,
//     pub hasWindowFuncs: bool,
//     pub hasTargetSRFs: bool,
//     pub hasSubLinks: bool,
//     pub hasDistinctOn: bool,
//     pub hasRecursive: bool,
//     pub hasModifyingCTE: bool,
//     pub hasForUpdate: bool,
//     pub hasRowSecurity: bool,
//     pub cteList: *mut List,
//     pub rtable: *mut List,
//     pub jointree: *mut FromExpr,
//     pub targetList: *mut List,
//     pub override_0: OverridingKind,
//     pub onConflict: *mut OnConflictExpr,
//     pub returningList: *mut List,
//     pub groupClause: *mut List,
//     pub groupingSets: *mut List,
//     pub havingQual: *mut Node,
//     pub windowClause: *mut List,
//     pub distinctClause: *mut List,
//     pub sortClause: *mut List,
//     pub limitOffset: *mut Node,
//     pub limitCount: *mut Node,
//     pub limitOption: LimitOption,
//     pub rowMarks: *mut List,
//     pub setOperations: *mut Node,
//     pub constraintDeps: *mut List,
//     pub withCheckOptions: *mut List,
//     pub stmt_location: libc::c_int,
//     pub stmt_len: libc::c_int,
// }
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColumnRef {
    pub type_0: NodeTag,
    pub fields: *mut List,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamRef {
    pub type_0: NodeTag,
    pub number: libc::c_int,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct A_Const {
    pub type_0: NodeTag,
    pub val: Value,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollateClause {
    pub type_0: NodeTag,
    pub arg: *mut Node,
    pub collname: *mut List,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColumnDef {
    pub type_0: NodeTag,
    pub colname: *mut libc::c_char,
    pub typeName: *mut TypeName,
    pub inhcount: libc::c_int,
    pub is_local: bool,
    pub is_not_null: bool,
    pub is_from_type: bool,
    pub storage: libc::c_char,
    pub raw_default: *mut Node,
    pub cooked_default: *mut Node,
    pub identity: libc::c_char,
    pub identitySequence: *mut RangeVar,
    pub generated: libc::c_char,
    pub collClause: *mut CollateClause,
    pub collOid: Oid,
    pub constraints: *mut List,
    pub fdwoptions: *mut List,
    pub location: libc::c_int,
}
pub type RTEKind = libc::c_uint;
pub const RTE_RESULT: RTEKind = 8;
pub const RTE_NAMEDTUPLESTORE: RTEKind = 7;
pub const RTE_CTE: RTEKind = 6;
pub const RTE_VALUES: RTEKind = 5;
pub const RTE_TABLEFUNC: RTEKind = 4;
pub const RTE_FUNCTION: RTEKind = 3;
pub const RTE_JOIN: RTEKind = 2;
pub const RTE_SUBQUERY: RTEKind = 1;
pub const RTE_RELATION: RTEKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RangeTblEntry {
    pub type_0: NodeTag,
    pub rtekind: RTEKind,
    pub relid: Oid,
    pub relkind: libc::c_char,
    pub rellockmode: libc::c_int,
    pub tablesample: *mut TableSampleClause,
    pub subquery: *mut Query,
    pub security_barrier: bool,
    pub jointype: JoinType,
    pub joinmergedcols: libc::c_int,
    pub joinaliasvars: *mut List,
    pub joinleftcols: *mut List,
    pub joinrightcols: *mut List,
    pub functions: *mut List,
    pub funcordinality: bool,
    pub tablefunc: *mut TableFunc,
    pub values_lists: *mut List,
    pub ctename: *mut libc::c_char,
    pub ctelevelsup: Index,
    pub self_reference: bool,
    pub coltypes: *mut List,
    pub coltypmods: *mut List,
    pub colcollations: *mut List,
    pub enrname: *mut libc::c_char,
    pub enrtuples: libc::c_double,
    pub alias: *mut Alias,
    pub eref: *mut Alias,
    pub lateral: bool,
    pub inh: bool,
    pub inFromCl: bool,
    pub requiredPerms: AclMode,
    pub checkAsUser: Oid,
    pub selectedCols: *mut Bitmapset,
    pub insertedCols: *mut Bitmapset,
    pub updatedCols: *mut Bitmapset,
    pub extraUpdatedCols: *mut Bitmapset,
    pub securityQuals: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableSampleClause {
    pub type_0: NodeTag,
    pub tsmhandler: Oid,
    pub args: *mut List,
    pub repeatable: *mut Expr,
}
pub type CTEMaterialize = libc::c_uint;
pub const CTEMaterializeNever: CTEMaterialize = 2;
pub const CTEMaterializeAlways: CTEMaterialize = 1;
pub const CTEMaterializeDefault: CTEMaterialize = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CTESearchClause {
    pub type_0: NodeTag,
    pub search_col_list: *mut List,
    pub search_breadth_first: bool,
    pub search_seq_column: *mut libc::c_char,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CTECycleClause {
    pub type_0: NodeTag,
    pub cycle_col_list: *mut List,
    pub cycle_mark_column: *mut libc::c_char,
    pub cycle_mark_value: *mut Node,
    pub cycle_mark_default: *mut Node,
    pub cycle_path_column: *mut libc::c_char,
    pub location: libc::c_int,
    pub cycle_mark_type: Oid,
    pub cycle_mark_typmod: libc::c_int,
    pub cycle_mark_collation: Oid,
    pub cycle_mark_neop: Oid,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct CommonTableExpr {
//     pub type_0: NodeTag,
//     pub ctename: *mut libc::c_char,
//     pub aliascolnames: *mut List,
//     pub ctematerialized: CTEMaterialize,
//     pub ctequery: *mut Node,
//     pub search_clause: *mut CTESearchClause,
//     pub cycle_clause: *mut CTECycleClause,
//     pub location: libc::c_int,
//     pub cterecursive: bool,
//     pub cterefcount: libc::c_int,
//     pub ctecolnames: *mut List,
//     pub ctecoltypes: *mut List,
//     pub ctecoltypmods: *mut List,
//     pub ctecolcollations: *mut List,
// }
// pub type Relation = *mut RelationData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArrayType {
    pub vl_len_: i32,
    pub ndim: libc::c_int,
    pub dataoffset: i32,
    pub elemtype: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_type {
    pub oid: Oid,
    pub typname: NameData,
    pub typnamespace: Oid,
    pub typowner: Oid,
    pub typlen: i16,
    pub typbyval: bool,
    pub typtype: libc::c_char,
    pub typcategory: libc::c_char,
    pub typispreferred: bool,
    pub typisdefined: bool,
    pub typdelim: libc::c_char,
    pub typrelid: Oid,
    pub typsubscript: regproc,
    pub typelem: Oid,
    pub typarray: Oid,
    pub typinput: regproc,
    pub typoutput: regproc,
    pub typreceive: regproc,
    pub typsend: regproc,
    pub typmodin: regproc,
    pub typmodout: regproc,
    pub typanalyze: regproc,
    pub typalign: libc::c_char,
    pub typstorage: libc::c_char,
    pub typnotnull: bool,
    pub typbasetype: Oid,
    pub typtypmod: i32,
    pub typndims: i32,
    pub typcollation: Oid,
}
pub type Form_pg_type = *mut FormData_pg_type;
pub type StringInfo = *mut StringInfoData;

pub type CoerceParamHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut Param, Oid, i32, libc::c_int) -> *mut Node>;
pub type ParseParamRefHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut ParamRef) -> *mut Node>;
pub type PostParseColumnRefHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut ColumnRef, *mut Node) -> *mut Node>;
pub type PreParseColumnRefHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut ColumnRef) -> *mut Node>;
// pub type ParseExprKind = libc::c_uint;
pub const EXPR_KIND_CYCLE_MARK: ParseExprKind = 41;
pub const EXPR_KIND_GENERATED_COLUMN: ParseExprKind = 40;
pub const EXPR_KIND_COPY_WHERE: ParseExprKind = 39;
pub const EXPR_KIND_CALL_ARGUMENT: ParseExprKind = 38;
pub const EXPR_KIND_PARTITION_EXPRESSION: ParseExprKind = 37;
pub const EXPR_KIND_PARTITION_BOUND: ParseExprKind = 36;
pub const EXPR_KIND_POLICY: ParseExprKind = 35;
pub const EXPR_KIND_TRIGGER_WHEN: ParseExprKind = 34;
pub const EXPR_KIND_EXECUTE_PARAMETER: ParseExprKind = 33;
pub const EXPR_KIND_ALTER_COL_TRANSFORM: ParseExprKind = 32;
pub const EXPR_KIND_INDEX_PREDICATE: ParseExprKind = 31;
pub const EXPR_KIND_INDEX_EXPRESSION: ParseExprKind = 30;
pub const EXPR_KIND_FUNCTION_DEFAULT: ParseExprKind = 29;
pub const EXPR_KIND_COLUMN_DEFAULT: ParseExprKind = 28;
pub const EXPR_KIND_DOMAIN_CHECK: ParseExprKind = 27;
pub const EXPR_KIND_CHECK_CONSTRAINT: ParseExprKind = 26;
pub const EXPR_KIND_VALUES_SINGLE: ParseExprKind = 25;
pub const EXPR_KIND_VALUES: ParseExprKind = 24;
pub const EXPR_KIND_RETURNING: ParseExprKind = 23;
pub const EXPR_KIND_OFFSET: ParseExprKind = 22;
pub const EXPR_KIND_LIMIT: ParseExprKind = 21;
pub const EXPR_KIND_DISTINCT_ON: ParseExprKind = 20;
pub const EXPR_KIND_ORDER_BY: ParseExprKind = 19;
pub const EXPR_KIND_GROUP_BY: ParseExprKind = 18;
pub const EXPR_KIND_UPDATE_TARGET: ParseExprKind = 17;
pub const EXPR_KIND_UPDATE_SOURCE: ParseExprKind = 16;
pub const EXPR_KIND_INSERT_TARGET: ParseExprKind = 15;
pub const EXPR_KIND_SELECT_TARGET: ParseExprKind = 14;
pub const EXPR_KIND_WINDOW_FRAME_GROUPS: ParseExprKind = 13;
pub const EXPR_KIND_WINDOW_FRAME_ROWS: ParseExprKind = 12;
pub const EXPR_KIND_WINDOW_FRAME_RANGE: ParseExprKind = 11;
pub const EXPR_KIND_WINDOW_ORDER: ParseExprKind = 10;
pub const EXPR_KIND_WINDOW_PARTITION: ParseExprKind = 9;
pub const EXPR_KIND_FILTER: ParseExprKind = 8;
pub const EXPR_KIND_HAVING: ParseExprKind = 7;
pub const EXPR_KIND_WHERE: ParseExprKind = 6;
pub const EXPR_KIND_FROM_FUNCTION: ParseExprKind = 5;
pub const EXPR_KIND_FROM_SUBSELECT: ParseExprKind = 4;
pub const EXPR_KIND_JOIN_USING: ParseExprKind = 3;
pub const EXPR_KIND_JOIN_ON: ParseExprKind = 2;
pub const EXPR_KIND_OTHER: ParseExprKind = 1;
pub const EXPR_KIND_NONE: ParseExprKind = 0;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct ParseNamespaceItem {
//     pub p_rte: *mut RangeTblEntry,
//     pub p_rtindex: libc::c_int,
//     pub p_nscolumns: *mut ParseNamespaceColumn,
//     pub p_rel_visible: bool,
//     pub p_cols_visible: bool,
//     pub p_lateral_only: bool,
//     pub p_lateral_ok: bool,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct ParseNamespaceColumn {
//     pub p_varno: Index,
//     pub p_varattno: AttrNumber,
//     pub p_vartype: Oid,
//     pub p_vartypmod: i32,
//     pub p_varcollid: Oid,
//     pub p_varnosyn: Index,
//     pub p_varattnosyn: AttrNumber,
//     pub p_dontexpand: bool,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct ParseCallbackState {
//     pub pstate: *mut ParseState,
//     pub location: libc::c_int,
//     pub errcallback: ErrorContextCallback,
// }
pub type Type = HeapTuple;
pub const TYPEOID: SysCacheIdentifier = 76;
pub type RawParseMode = libc::c_uint;
pub const RAW_PARSE_PLPGSQL_ASSIGN3: RawParseMode = 5;
pub const RAW_PARSE_PLPGSQL_ASSIGN2: RawParseMode = 4;
pub const RAW_PARSE_PLPGSQL_ASSIGN1: RawParseMode = 3;
pub const RAW_PARSE_PLPGSQL_EXPR: RawParseMode = 2;
pub const RAW_PARSE_TYPE_NAME: RawParseMode = 1;
pub const RAW_PARSE_DEFAULT: RawParseMode = 0;
pub type SysCacheIdentifier = libc::c_uint;
pub const USERMAPPINGUSERSERVER: SysCacheIdentifier = 78;
pub const USERMAPPINGOID: SysCacheIdentifier = 77;
pub const TYPENAMENSP: SysCacheIdentifier = 75;
pub const TSTEMPLATEOID: SysCacheIdentifier = 74;
pub const TSTEMPLATENAMENSP: SysCacheIdentifier = 73;
pub const TSPARSEROID: SysCacheIdentifier = 72;
pub const TSPARSERNAMENSP: SysCacheIdentifier = 71;
pub const TSDICTOID: SysCacheIdentifier = 70;
pub const TSDICTNAMENSP: SysCacheIdentifier = 69;
pub const TSCONFIGOID: SysCacheIdentifier = 68;
pub const TSCONFIGNAMENSP: SysCacheIdentifier = 67;
pub const TSCONFIGMAP: SysCacheIdentifier = 66;
pub const TRFTYPELANG: SysCacheIdentifier = 65;
pub const TRFOID: SysCacheIdentifier = 64;
pub const TABLESPACEOID: SysCacheIdentifier = 63;
pub const SUBSCRIPTIONRELMAP: SysCacheIdentifier = 62;
pub const SUBSCRIPTIONOID: SysCacheIdentifier = 61;
pub const SUBSCRIPTIONNAME: SysCacheIdentifier = 60;
pub const STATRELATTINH: SysCacheIdentifier = 59;
pub const STATEXTOID: SysCacheIdentifier = 58;
pub const STATEXTNAMENSP: SysCacheIdentifier = 57;
pub const STATEXTDATASTXOID: SysCacheIdentifier = 56;
pub const SEQRELID: SysCacheIdentifier = 55;
pub const RULERELNAME: SysCacheIdentifier = 54;
pub const REPLORIGNAME: SysCacheIdentifier = 53;
pub const REPLORIGIDENT: SysCacheIdentifier = 52;
pub const RELOID: SysCacheIdentifier = 51;
pub const RELNAMENSP: SysCacheIdentifier = 50;
pub const RANGETYPE: SysCacheIdentifier = 49;
pub const RANGEMULTIRANGE: SysCacheIdentifier = 48;
pub const PUBLICATIONRELMAP: SysCacheIdentifier = 47;
pub const PUBLICATIONREL: SysCacheIdentifier = 46;
pub const PUBLICATIONOID: SysCacheIdentifier = 45;
pub const PUBLICATIONNAME: SysCacheIdentifier = 44;
pub const PROCOID: SysCacheIdentifier = 43;
pub const PROCNAMEARGSNSP: SysCacheIdentifier = 42;
pub const PARTRELID: SysCacheIdentifier = 41;
pub const OPFAMILYOID: SysCacheIdentifier = 40;
pub const OPFAMILYAMNAMENSP: SysCacheIdentifier = 39;
pub const OPEROID: SysCacheIdentifier = 38;
pub const OPERNAMENSP: SysCacheIdentifier = 37;
pub const NAMESPACEOID: SysCacheIdentifier = 36;
pub const NAMESPACENAME: SysCacheIdentifier = 35;
pub const LANGOID: SysCacheIdentifier = 34;
pub const LANGNAME: SysCacheIdentifier = 33;
pub const INDEXRELID: SysCacheIdentifier = 32;
pub const FOREIGNTABLEREL: SysCacheIdentifier = 31;
pub const FOREIGNSERVEROID: SysCacheIdentifier = 30;
pub const FOREIGNSERVERNAME: SysCacheIdentifier = 29;
pub const FOREIGNDATAWRAPPEROID: SysCacheIdentifier = 28;
pub const FOREIGNDATAWRAPPERNAME: SysCacheIdentifier = 27;
pub const EVENTTRIGGEROID: SysCacheIdentifier = 26;
pub const EVENTTRIGGERNAME: SysCacheIdentifier = 25;
pub const ENUMTYPOIDNAME: SysCacheIdentifier = 24;
pub const ENUMOID: SysCacheIdentifier = 23;
pub const DEFACLROLENSPOBJ: SysCacheIdentifier = 22;
pub const DATABASEOID: SysCacheIdentifier = 21;
pub const CONVOID: SysCacheIdentifier = 20;
pub const CONSTROID: SysCacheIdentifier = 19;
pub const CONNAMENSP: SysCacheIdentifier = 18;
pub const CONDEFAULT: SysCacheIdentifier = 17;
pub const COLLOID: SysCacheIdentifier = 16;
pub const COLLNAMEENCNSP: SysCacheIdentifier = 15;
pub const CLAOID: SysCacheIdentifier = 14;
pub const CLAAMNAMENSP: SysCacheIdentifier = 13;
pub const CASTSOURCETARGET: SysCacheIdentifier = 12;
pub const AUTHOID: SysCacheIdentifier = 11;
pub const AUTHNAME: SysCacheIdentifier = 10;
pub const AUTHMEMROLEMEM: SysCacheIdentifier = 9;
pub const AUTHMEMMEMROLE: SysCacheIdentifier = 8;
pub const ATTNUM: SysCacheIdentifier = 7;
pub const ATTNAME: SysCacheIdentifier = 6;
pub const AMPROCNUM: SysCacheIdentifier = 5;
pub const AMOPSTRATEGY: SysCacheIdentifier = 4;
pub const AMOPOPID: SysCacheIdentifier = 3;
pub const AMOID: SysCacheIdentifier = 2;
pub const AMNAME: SysCacheIdentifier = 1;
pub const AGGFNOID: SysCacheIdentifier = 0;
#[inline]
unsafe extern "C" fn list_head(mut l: *const List) -> *mut ListCell {
    return if !l.is_null() {
        &mut *((*l).elements).offset(0 as libc::c_int as isize) as *mut ListCell
    } else {
        0 as *mut ListCell
    };
}
#[inline]
unsafe extern "C" fn list_length(mut l: *const List) -> libc::c_int {
    return if !l.is_null() {
        (*l).length
    } else {
        0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn list_nth_cell(mut list: *const List, mut n: libc::c_int) -> *mut ListCell {
    return &mut *((*list).elements).offset(n as isize) as *mut ListCell;
}
#[no_mangle]
pub unsafe extern "C" fn LookupTypeName(
    mut pstate: *mut ParseState,
    mut typeName: *const TypeName,
    mut typmod_p: *mut i32,
    mut missing_ok: bool,
) -> Type {
    return LookupTypeNameExtended(pstate, typeName, typmod_p, true, missing_ok);
}
#[no_mangle]
pub unsafe extern "C" fn LookupTypeNameExtended(
    mut pstate: *mut ParseState,
    mut typeName: *const TypeName,
    mut typmod_p: *mut i32,
    mut temp_ok: bool,
    mut missing_ok: bool,
) -> Type {
    let mut typoid: Oid = 0;
    let mut tup: HeapTuple = 0 as *mut HeapTupleData;
    let mut typmod: i32 = 0;
    if ((*typeName).names).is_null() {
        typoid = (*typeName).typeOid;
    } else if (*typeName).pct_type != 0 {
        let mut rel: *mut RangeVar = makeRangeVar(
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            (*typeName).location,
        );
        let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut relid: Oid = 0;
        let mut attnum: AttrNumber = 0;
        match list_length((*typeName).names) {
            1 => {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            2 => {
                (*rel).relname = (*((*list_nth_cell((*typeName).names, 0 as libc::c_int)).ptr_value
                    as *mut Value))
                    .val
                    .str_0;
                field = (*((*list_nth_cell((*typeName).names, 1 as libc::c_int)).ptr_value
                    as *mut Value))
                    .val
                    .str_0;
            }
            3 => {
                (*rel).schemaname = (*((*list_nth_cell((*typeName).names, 0 as libc::c_int))
                    .ptr_value as *mut Value))
                    .val
                    .str_0;
                (*rel).relname = (*((*list_nth_cell((*typeName).names, 1 as libc::c_int)).ptr_value
                    as *mut Value))
                    .val
                    .str_0;
                field = (*((*list_nth_cell((*typeName).names, 2 as libc::c_int)).ptr_value
                    as *mut Value))
                    .val
                    .str_0;
            }
            4 => {
                (*rel).catalogname = (*((*list_nth_cell((*typeName).names, 0 as libc::c_int))
                    .ptr_value as *mut Value))
                    .val
                    .str_0;
                (*rel).schemaname = (*((*list_nth_cell((*typeName).names, 1 as libc::c_int))
                    .ptr_value as *mut Value))
                    .val
                    .str_0;
                (*rel).relname = (*((*list_nth_cell((*typeName).names, 2 as libc::c_int)).ptr_value
                    as *mut Value))
                    .val
                    .str_0;
                field = (*((*list_nth_cell((*typeName).names, 3 as libc::c_int)).ptr_value
                    as *mut Value))
                    .val
                    .str_0;
            }
            _ => {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        relid = RangeVarGetRelidExtended(
            rel,
            0 as libc::c_int,
            (if missing_ok as libc::c_int != 0 {
                RVR_MISSING_OK as libc::c_int
            } else {
                0 as libc::c_int
            }) as u32,
            None,
            0 as *mut libc::c_void,
        );
        attnum = get_attnum(relid, field);
        if attnum as libc::c_int == 0 as libc::c_int {
            if missing_ok {
                typoid = 0 as libc::c_int as Oid;
            } else {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
        } else {
            typoid = get_atttype(relid, attnum);
            let elevel__2: libc::c_int = 18 as libc::c_int;
            let mut __error_2: libc::c_int = 0;
            if errstart(elevel__2, 0 as *const libc::c_char) != 0 {
                errmsg(
                    b"type reference %s converted to %s\0" as *const u8 as *const libc::c_char,
                    TypeNameToString(typeName),
                    format_type_be(typoid),
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_type.c\0"
                        as *const u8 as *const libc::c_char,
                    159 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel__2 >= 21 as libc::c_int {
                abort();
            }
        }
    } else {
        let mut schemaname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut typname: *mut libc::c_char = 0 as *mut libc::c_char;
        DeconstructQualifiedName((*typeName).names, &mut schemaname, &mut typname);
        if !schemaname.is_null() {
            let mut namespaceId: Oid = 0;
            let mut pcbstate: ParseCallbackState = ParseCallbackState {
                pstate: 0 as *mut ParseState,
                location: 0,
                errcallback: ErrorContextCallback {
                    previous: 0 as *mut ErrorContextCallback,
                    callback: None,
                    arg: 0 as *mut libc::c_void,
                },
            };
            setup_parser_errposition_callback(&mut pcbstate, pstate, (*typeName).location);
            namespaceId = LookupExplicitNamespace(schemaname, missing_ok);
            if !((namespaceId != 0 as libc::c_int as Oid) as libc::c_int as bool != 0) {
                typoid = 0 as libc::c_int as Oid;
            }
            cancel_parser_errposition_callback(&mut pcbstate);
        } else {
            typoid = TypenameGetTypidExtended(typname, temp_ok);
        }
        if !((*typeName).arrayBounds).is_null() {
            typoid = get_array_type(typoid);
        }
    }
    if (typoid != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
        if !typmod_p.is_null() {
            *typmod_p = -(1 as libc::c_int);
        }
        return 0 as Type;
    }
    tup = SearchSysCache1(TYPEOID as libc::c_int, typoid as Datum);
    if (tup as *const libc::c_void).is_null() {
        let elevel__3: libc::c_int = 21 as libc::c_int;
        let mut __error_3: libc::c_int = 0;
        if errstart(elevel__3, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"cache lookup failed for type %u\0" as *const u8 as *const libc::c_char,
                typoid,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_type.c\0"
                    as *const u8 as *const libc::c_char,
                209 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel__3 >= 21 as libc::c_int {
            abort();
        }
    }
    typmod = typenameTypeMod(pstate, typeName, tup);
    if !typmod_p.is_null() {
        *typmod_p = typmod;
    }
    return tup;
}
#[no_mangle]
pub unsafe extern "C" fn LookupTypeNameOid(
    mut pstate: *mut ParseState,
    mut typeName: *const TypeName,
    mut missing_ok: bool,
) -> Oid {
    let mut typoid: Oid = 0;
    let mut tup: Type = 0 as *mut HeapTupleData;
    tup = LookupTypeName(pstate, typeName, 0 as *mut i32, missing_ok);
    if tup.is_null() {
        if missing_ok == 0 {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        return 0 as libc::c_int as Oid;
    }
    typoid = (*(((*tup).t_data as *mut libc::c_char)
        .offset((*(*tup).t_data).t_hoff as libc::c_int as isize) as Form_pg_type))
        .oid;
    ReleaseSysCache(tup);
    return typoid;
}
#[no_mangle]
pub unsafe extern "C" fn typenameType(
    mut pstate: *mut ParseState,
    mut typeName: *const TypeName,
    mut typmod_p: *mut i32,
) -> Type {
    let mut tup: Type = 0 as *mut HeapTupleData;
    tup = LookupTypeName(pstate, typeName, typmod_p, false);
    if tup.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if (*(((*tup).t_data as *mut libc::c_char)
        .offset((*(*tup).t_data).t_hoff as libc::c_int as isize) as Form_pg_type))
        .typisdefined
        == 0
    {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    return tup;
}
#[no_mangle]
pub unsafe extern "C" fn typenameTypeId(
    mut pstate: *mut ParseState,
    mut typeName: *const TypeName,
) -> Oid {
    let mut typoid: Oid = 0;
    let mut tup: Type = 0 as *mut HeapTupleData;
    tup = typenameType(pstate, typeName, 0 as *mut i32);
    typoid = (*(((*tup).t_data as *mut libc::c_char)
        .offset((*(*tup).t_data).t_hoff as libc::c_int as isize) as Form_pg_type))
        .oid;
    ReleaseSysCache(tup);
    return typoid;
}
#[no_mangle]
pub unsafe extern "C" fn typenameTypeIdAndMod(
    mut pstate: *mut ParseState,
    mut typeName: *const TypeName,
    mut typeid_p: *mut Oid,
    mut typmod_p: *mut i32,
) {
    let mut tup: Type = 0 as *mut HeapTupleData;
    tup = typenameType(pstate, typeName, typmod_p);
    *typeid_p = (*(((*tup).t_data as *mut libc::c_char)
        .offset((*(*tup).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_type))
        .oid;
    ReleaseSysCache(tup);
}
unsafe extern "C" fn typenameTypeMod(
    mut pstate: *mut ParseState,
    mut typeName: *const TypeName,
    mut typ: Type,
) -> i32 {
    let mut result: i32 = 0;
    let mut typmodin: Oid = 0;
    let mut datums: *mut Datum = 0 as *mut Datum;
    let mut n: libc::c_int = 0;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut arrtypmod: *mut ArrayType = 0 as *mut ArrayType;
    let mut pcbstate: ParseCallbackState = ParseCallbackState {
        pstate: 0 as *mut ParseState,
        location: 0,
        errcallback: ErrorContextCallback {
            previous: 0 as *mut ErrorContextCallback,
            callback: None,
            arg: 0 as *mut libc::c_void,
        },
    };
    if ((*typeName).typmods).is_null() {
        return (*typeName).typemod;
    }
    if (*(((*typ).t_data as *mut libc::c_char)
        .offset((*(*typ).t_data).t_hoff as libc::c_int as isize) as Form_pg_type))
        .typisdefined
        == 0
    {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    typmodin = (*(((*typ).t_data as *mut libc::c_char)
        .offset((*(*typ).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_type))
        .typmodin;
    if typmodin == 0 as libc::c_int as Oid {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    datums = palloc(
        (list_length((*typeName).typmods) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Datum>() as libc::c_ulong),
    ) as *mut Datum;
    n = 0 as libc::c_int;
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: (*typeName).typmods,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(l__state.l).is_null() && l__state.i < (*l__state.l).length {
        l = &mut *((*l__state.l).elements).offset(l__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        l = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut tm: *mut Node = (*l).ptr_value as *mut Node;
        let mut cstr: *mut libc::c_char = 0 as *mut libc::c_char;
        if (*(tm as *const Node)).tag as libc::c_uint == T_A_Const as libc::c_int as libc::c_uint {
            let mut ac: *mut A_Const = tm as *mut A_Const;
            if (*(&mut (*ac).val as *mut Value as *const Node)).tag as libc::c_uint
                == T_Integer as libc::c_int as libc::c_uint
            {
                cstr = psprintf(
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    (*ac).val.val.ival as libc::c_long,
                );
            } else if (*(&mut (*ac).val as *mut Value as *const Node)).tag as libc::c_uint
                == T_Float as libc::c_int as libc::c_uint
                || (*(&mut (*ac).val as *mut Value as *const Node)).tag as libc::c_uint
                    == T_String as libc::c_int as libc::c_uint
            {
                cstr = (*ac).val.val.str_0;
            }
        } else if (*(tm as *const Node)).tag as libc::c_uint
            == T_ColumnRef as libc::c_int as libc::c_uint
        {
            let mut cr: *mut ColumnRef = tm as *mut ColumnRef;
            if list_length((*cr).fields) == 1 as libc::c_int
                && (*((*list_nth_cell((*cr).fields, 0 as libc::c_int)).ptr_value as *const Node))
                    .tag as libc::c_uint
                    == T_String as libc::c_int as libc::c_uint
            {
                cstr = (*((*list_nth_cell((*cr).fields, 0 as libc::c_int)).ptr_value
                    as *mut Value))
                    .val
                    .str_0;
            }
        }
        if cstr.is_null() {
            let elevel__1: libc::c_int = 21 as libc::c_int;
            let mut __error_1: libc::c_int = 0;
            if elevel__1 >= 21 as libc::c_int {
                abort();
            }
        }
        let fresh0 = n;
        n = n + 1;
        *datums.offset(fresh0 as isize) = cstr as Datum;
        l__state.i += 1;
        l__state.i;
    }
    setup_parser_errposition_callback(&mut pcbstate, pstate, (*typeName).location);
    result = OidFunctionCall1Coll(typmodin, 0 as libc::c_int as Oid, arrtypmod as Datum) as i32;
    cancel_parser_errposition_callback(&mut pcbstate);
    pfree(datums as *mut libc::c_void);
    pfree(arrtypmod as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn appendTypeNameToBuffer(mut typeName: *const TypeName, mut string: StringInfo) {
    if !((*typeName).names).is_null() {
        let mut l: *mut ListCell = 0 as *mut ListCell;
        let mut l__state: ForEachState = {
            let mut init = ForEachState {
                l: (*typeName).names,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(l__state.l).is_null() && l__state.i < (*l__state.l).length {
            l = &mut *((*l__state.l).elements).offset(l__state.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            l = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            if l != list_head((*typeName).names) {
                appendStringInfoChar(string, '.' as i32 as libc::c_char);
            }
            appendStringInfoString(string, (*((*l).ptr_value as *mut Value)).val.str_0);
            l__state.i += 1;
            l__state.i;
        }
    } else {
        appendStringInfoString(string, format_type_be((*typeName).typeOid));
    }
    if (*typeName).pct_type != 0 {
        appendStringInfoString(string, b"%TYPE\0" as *const u8 as *const libc::c_char);
    }
    if !((*typeName).arrayBounds).is_null() {
        appendStringInfoString(string, b"[]\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn TypeNameToString(mut typeName: *const TypeName) -> *mut libc::c_char {
    let mut string: StringInfoData = StringInfoData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
        cursor: 0,
    };
    initStringInfo(&mut string);
    appendTypeNameToBuffer(typeName, &mut string);
    return string.data;
}
#[no_mangle]
pub unsafe extern "C" fn TypeNameListToString(mut typenames: *mut List) -> *mut libc::c_char {
    let mut string: StringInfoData = StringInfoData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
        cursor: 0,
    };
    let mut l: *mut ListCell = 0 as *mut ListCell;
    initStringInfo(&mut string);
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: typenames,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(l__state.l).is_null() && l__state.i < (*l__state.l).length {
        l = &mut *((*l__state.l).elements).offset(l__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        l = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut typeName: *mut TypeName = (*l).ptr_value as *mut TypeName;
        if l != list_head(typenames) {
            appendStringInfoChar(&mut string, ',' as i32 as libc::c_char);
        }
        appendTypeNameToBuffer(typeName, &mut string);
        l__state.i += 1;
        l__state.i;
    }
    return string.data;
}
#[no_mangle]
pub unsafe extern "C" fn LookupCollation(
    mut pstate: *mut ParseState,
    mut collnames: *mut List,
    mut location: libc::c_int,
) -> Oid {
    let mut colloid: Oid = 0;
    let mut pcbstate: ParseCallbackState = ParseCallbackState {
        pstate: 0 as *mut ParseState,
        location: 0,
        errcallback: ErrorContextCallback {
            previous: 0 as *mut ErrorContextCallback,
            callback: None,
            arg: 0 as *mut libc::c_void,
        },
    };
    if !pstate.is_null() {
        setup_parser_errposition_callback(&mut pcbstate, pstate, location);
    }
    colloid = get_collation_oid(collnames, false);
    if !pstate.is_null() {
        cancel_parser_errposition_callback(&mut pcbstate);
    }
    return colloid;
}
#[no_mangle]
pub unsafe extern "C" fn GetColumnDefCollation(
    mut pstate: *mut ParseState,
    mut coldef: *mut ColumnDef,
    mut typeOid: Oid,
) -> Oid {
    let mut result: Oid = 0;
    let mut typcollation: Oid = get_typcollation(typeOid);
    let mut location: libc::c_int = (*coldef).location;
    if !((*coldef).collClause).is_null() {
        location = (*(*coldef).collClause).location;
        result = LookupCollation(pstate, (*(*coldef).collClause).collname, location);
    } else if ((*coldef).collOid != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
        result = (*coldef).collOid;
    } else {
        result = typcollation;
    }
    if (result != 0 as libc::c_int as Oid) as libc::c_int as bool as libc::c_int != 0
        && (typcollation != 0 as libc::c_int as Oid) as libc::c_int as bool == 0
    {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn typeidType(mut id: Oid) -> Type {
    let mut tup: HeapTuple = 0 as *mut HeapTupleData;
    tup = SearchSysCache1(TYPEOID as libc::c_int, id as Datum);
    if (tup as *const libc::c_void).is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"cache lookup failed for type %u\0" as *const u8 as *const libc::c_char,
                id,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_type.c\0"
                    as *const u8 as *const libc::c_char,
                582 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return tup;
}
#[no_mangle]
pub unsafe extern "C" fn typeTypeId(mut tp: Type) -> Oid {
    if tp.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"typeTypeId() called with NULL type struct\0" as *const u8 as *const libc::c_char,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_type.c\0"
                    as *const u8 as *const libc::c_char,
                591 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return (*(((*tp).t_data as *mut libc::c_char)
        .offset((*(*tp).t_data).t_hoff as libc::c_int as isize) as Form_pg_type))
        .oid;
}
#[no_mangle]
pub unsafe extern "C" fn typeLen(mut t: Type) -> i16 {
    let mut typ: Form_pg_type = 0 as *mut FormData_pg_type;
    typ = ((*t).t_data as *mut libc::c_char).offset((*(*t).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_type;
    return (*typ).typlen;
}
#[no_mangle]
pub unsafe extern "C" fn typeByVal(mut t: Type) -> bool {
    let mut typ: Form_pg_type = 0 as *mut FormData_pg_type;
    typ = ((*t).t_data as *mut libc::c_char).offset((*(*t).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_type;
    return (*typ).typbyval;
}
#[no_mangle]
pub unsafe extern "C" fn typeTypeName(mut t: Type) -> *mut libc::c_char {
    let mut typ: Form_pg_type = 0 as *mut FormData_pg_type;
    typ = ((*t).t_data as *mut libc::c_char).offset((*(*t).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_type;
    return pstrdup(((*typ).typname.data).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn typeTypeRelid(mut typ: Type) -> Oid {
    let mut typtup: Form_pg_type = 0 as *mut FormData_pg_type;
    typtup = ((*typ).t_data as *mut libc::c_char)
        .offset((*(*typ).t_data).t_hoff as libc::c_int as isize) as Form_pg_type;
    return (*typtup).typrelid;
}
#[no_mangle]
pub unsafe extern "C" fn typeTypeCollation(mut typ: Type) -> Oid {
    let mut typtup: Form_pg_type = 0 as *mut FormData_pg_type;
    typtup = ((*typ).t_data as *mut libc::c_char)
        .offset((*(*typ).t_data).t_hoff as libc::c_int as isize) as Form_pg_type;
    return (*typtup).typcollation;
}
#[no_mangle]
pub unsafe extern "C" fn stringTypeDatum(
    mut tp: Type,
    mut string: *mut libc::c_char,
    mut atttypmod: i32,
) -> Datum {
    let mut typform: Form_pg_type = ((*tp).t_data as *mut libc::c_char)
        .offset((*(*tp).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_type;
    let mut typinput: Oid = (*typform).typinput;
    let mut typioparam: Oid = getTypeIOParam(tp);
    return OidInputFunctionCall(typinput, string, typioparam, atttypmod);
}
#[no_mangle]
pub unsafe extern "C" fn typeidTypeRelid(mut type_id: Oid) -> Oid {
    let mut typeTuple: HeapTuple = 0 as *mut HeapTupleData;
    let mut type_0: Form_pg_type = 0 as *mut FormData_pg_type;
    let mut result: Oid = 0;
    typeTuple = SearchSysCache1(TYPEOID as libc::c_int, type_id as Datum);
    if (typeTuple as *const libc::c_void).is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"cache lookup failed for type %u\0" as *const u8 as *const libc::c_char,
                type_id,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_type.c\0"
                    as *const u8 as *const libc::c_char,
                674 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    type_0 = ((*typeTuple).t_data as *mut libc::c_char)
        .offset((*(*typeTuple).t_data).t_hoff as libc::c_int as isize) as Form_pg_type;
    result = (*type_0).typrelid;
    ReleaseSysCache(typeTuple);
    return result;
}
unsafe extern "C" fn pts_error_callback(mut arg: *mut libc::c_void) {
    let mut str: *const libc::c_char = arg as *const libc::c_char;
    set_errcontext_domain(0 as *const libc::c_char);
    errcontext_msg(
        b"invalid type name \"%s\"\0" as *const u8 as *const libc::c_char,
        str,
    );
}
#[no_mangle]
pub unsafe extern "C" fn typeStringToTypeName(mut str: *const libc::c_char) -> *mut TypeName {
    let mut raw_parsetree_list: *mut List = 0 as *mut List;
    let mut typeName: *mut TypeName = 0 as *mut TypeName;
    let mut ptserrcontext: ErrorContextCallback = ErrorContextCallback {
        previous: 0 as *mut ErrorContextCallback,
        callback: None,
        arg: 0 as *mut libc::c_void,
    };
    if !(strspn(str, b" \t\n\r\x0C\0" as *const u8 as *const libc::c_char) == strlen(str)) {
        ptserrcontext.callback =
            Some(pts_error_callback as unsafe extern "C" fn(*mut libc::c_void) -> ());
        ptserrcontext.arg = str as *mut libc::c_char as *mut libc::c_void;
        ptserrcontext.previous = error_context_stack;
        error_context_stack = &mut ptserrcontext;
        raw_parsetree_list = raw_parser(str, RAW_PARSE_TYPE_NAME);
        error_context_stack = ptserrcontext.previous;
        typeName =
            (*list_nth_cell(raw_parsetree_list, 0 as libc::c_int)).ptr_value as *mut TypeName;
        if !((*typeName).setof != 0) {
            return typeName;
        }
    }
    let elevel_: libc::c_int = 21 as libc::c_int;
    let mut __error: libc::c_int = 0;
    if elevel_ >= 21 as libc::c_int {
        abort();
    }
    return 0 as *mut TypeName;
}
#[no_mangle]
pub unsafe extern "C" fn parseTypeString(
    mut str: *const libc::c_char,
    mut typeid_p: *mut Oid,
    mut typmod_p: *mut i32,
    mut missing_ok: bool,
) {
    let mut typeName: *mut TypeName = 0 as *mut TypeName;
    let mut tup: Type = 0 as *mut HeapTupleData;
    typeName = typeStringToTypeName(str);
    tup = LookupTypeName(0 as *mut ParseState, typeName, typmod_p, missing_ok);
    if tup.is_null() {
        if missing_ok == 0 {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        *typeid_p = 0 as libc::c_int as Oid;
    } else {
        let mut typ: Form_pg_type = ((*tup).t_data as *mut libc::c_char)
            .offset((*(*tup).t_data).t_hoff as libc::c_int as isize)
            as Form_pg_type;
        if (*typ).typisdefined == 0 {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
        *typeid_p = (*typ).oid;
        ReleaseSysCache(tup);
    };
}
