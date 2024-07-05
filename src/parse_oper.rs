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
//     pub type MemoryContextData;
//     pub type AttrMissing;
//     pub type RelationData;
//     pub type QueryEnvironment;
//     pub type HASHHDR;
//     pub type HTAB;
//     pub type TypeCacheEnumData;
//     pub type DomainConstraintCache;
//     fn abort() -> !;
//     fn memset(
//         _: *mut libc::c_void,
//         _: libc::c_int,
//         _: libc::c_ulong,
//     ) -> *mut libc::c_void;
//     fn strlcpy(
//         _: *mut libc::c_char,
//         _: *const libc::c_char,
//         _: libc::c_ulong,
//     ) -> libc::c_ulong;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
//     fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
//     fn exprType(expr: *const Node) -> Oid;
//     fn IsBinaryCoercible(srctype: Oid, targettype: Oid) -> bool;
//     fn enforce_generic_type_consistency(
//         actual_arg_types: *const Oid,
//         declared_arg_types: *mut Oid,
//         nargs: libc::c_int,
//         rettype: Oid,
//         allow_poly: bool,
//     ) -> Oid;
//     fn setup_parser_errposition_callback(
//         pcbstate: *mut ParseCallbackState,
//         pstate: *mut ParseState,
//         location: libc::c_int,
//     );
//     fn cancel_parser_errposition_callback(pcbstate: *mut ParseCallbackState);
//     fn hash_create(
//         tabname: *const libc::c_char,
//         nelem: libc::c_long,
//         info: *const HASHCTL,
//         flags: libc::c_int,
//     ) -> *mut HTAB;
//     fn hash_search(
//         hashp: *mut HTAB,
//         keyPtr: *const libc::c_void,
//         action: HASHACTION,
//         foundPtr: *mut bool,
//     ) -> *mut libc::c_void;
//     fn hash_seq_init(status: *mut HASH_SEQ_STATUS, hashp: *mut HTAB);
//     fn hash_seq_search(status: *mut HASH_SEQ_STATUS) -> *mut libc::c_void;
//     fn OpernameGetOprid(names: *mut List, oprleft: Oid, oprright: Oid) -> Oid;
//     fn OpernameGetCandidates(
//         names: *mut List,
//         oprkind: libc::c_char,
//         missing_schema_ok: bool,
//     ) -> FuncCandidateList;
//     fn DeconstructQualifiedName(
//         names: *mut List,
//         nspname_p: *mut *mut libc::c_char,
//         objname_p: *mut *mut libc::c_char,
//     );
//     fn LookupExplicitNamespace(nspname: *const libc::c_char, missing_ok: bool) -> Oid;
//     fn fetch_search_path_array(sarray: *mut Oid, sarray_len: libc::c_int) -> libc::c_int;
//     fn func_match_argtypes(
//         nargs: libc::c_int,
//         input_typeids: *mut Oid,
//         raw_candidates: FuncCandidateList,
//         candidates: *mut FuncCandidateList,
//     ) -> libc::c_int;
//     fn func_select_candidate(
//         nargs: libc::c_int,
//         input_typeids: *mut Oid,
//         candidates: FuncCandidateList,
//     ) -> FuncCandidateList;
//     fn make_fn_arguments(
//         pstate: *mut ParseState,
//         fargs: *mut List,
//         actual_arg_types: *mut Oid,
//         declared_arg_types: *mut Oid,
//     );
//     fn check_srf_call_placement(
//         pstate: *mut ParseState,
//         last_srf: *mut Node,
//         location: libc::c_int,
//     );
//     fn LookupTypeNameOid(
//         pstate: *mut ParseState,
//         typeName: *const TypeName,
//         missing_ok: bool,
//     ) -> Oid;
//     fn CacheRegisterSyscacheCallback(
//         cacheid: libc::c_int,
//         func: SyscacheCallbackFunction,
//         arg: Datum,
//     );
//     fn get_func_retset(funcid: Oid) -> bool;
//     fn get_array_type(typid: Oid) -> Oid;
//     fn get_base_element_type(typid: Oid) -> Oid;
//     fn getBaseType(typid: Oid) -> Oid;
//     fn SearchSysCache1(cacheId: libc::c_int, key1: Datum) -> HeapTuple;
//     fn ReleaseSysCache(tuple: HeapTuple);
//     fn lookup_type_cache(type_id: Oid, flags: libc::c_int) -> *mut TypeCacheEntry;
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
pub type MemoryContext = *mut MemoryContextData;
// pub type Datum = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NullableDatum {
    pub value: Datum,
    pub isnull: bool,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_attribute {
    pub attrelid: Oid,
    pub attname: NameData,
    pub atttypid: Oid,
    pub attstattarget: i32,
    pub attlen: i16,
    pub attnum: i16,
    pub attndims: i32,
    pub attcacheoff: i32,
    pub atttypmod: i32,
    pub attbyval: bool,
    pub attstorage: libc::c_char,
    pub attalign: libc::c_char,
    pub attnotnull: bool,
    pub atthasdef: bool,
    pub atthasmissing: bool,
    pub attidentity: libc::c_char,
    pub attgenerated: libc::c_char,
    pub attisdropped: bool,
    pub attislocal: bool,
    pub attinhcount: i32,
    pub attcollation: Oid,
}

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
pub struct AttrDefault {
    pub adnum: AttrNumber,
    pub adbin: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConstrCheck {
    pub ccname: *mut libc::c_char,
    pub ccbin: *mut libc::c_char,
    pub ccvalid: bool,
    pub ccnoinherit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleConstr {
    pub defval: *mut AttrDefault,
    pub check: *mut ConstrCheck,
    pub missing: *mut AttrMissing,
    pub num_defval: u16,
    pub num_check: u16,
    pub has_not_null: bool,
    pub has_generated_stored: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleDescData {
    pub natts: libc::c_int,
    pub tdtypeid: Oid,
    pub tdtypmod: i32,
    pub tdrefcount: libc::c_int,
    pub constr: *mut TupleConstr,
    pub attrs: [FormData_pg_attribute; 0],
}
pub type TupleDesc = *mut TupleDescData;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Alias {
//     pub type_0: NodeTag,
//     pub aliasname: *mut libc::c_char,
//     pub colnames: *mut List,
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
pub struct OpExpr {
    pub xpr: Expr,
    pub opno: Oid,
    pub opfuncid: Oid,
    pub opresulttype: Oid,
    pub opretset: bool,
    pub opcollid: Oid,
    pub inputcollid: Oid,
    pub args: *mut List,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScalarArrayOpExpr {
    pub xpr: Expr,
    pub opno: Oid,
    pub opfuncid: Oid,
    pub useOr: bool,
    pub inputcollid: Oid,
    pub args: *mut List,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectWithArgs {
    pub type_0: NodeTag,
    pub objname: *mut List,
    pub objargs: *mut List,
    pub args_unspecified: bool,
}
// pub type Relation = *mut RelationData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_operator {
    pub oid: Oid,
    pub oprname: NameData,
    pub oprnamespace: Oid,
    pub oprowner: Oid,
    pub oprkind: libc::c_char,
    pub oprcanmerge: bool,
    pub oprcanhash: bool,
    pub oprleft: Oid,
    pub oprright: Oid,
    pub oprresult: Oid,
    pub oprcom: Oid,
    pub oprnegate: Oid,
    pub oprcode: regproc,
    pub oprrest: regproc,
    pub oprjoin: regproc,
}
pub type Form_pg_operator = *mut FormData_pg_operator;

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
pub type HashValueFunc = Option<unsafe extern "C" fn(*const libc::c_void, usize) -> u32>;
pub type HashCompareFunc =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void, usize) -> libc::c_int>;
pub type HashCopyFunc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void, usize) -> *mut libc::c_void,
>;
pub type HashAllocFunc = Option<unsafe extern "C" fn(usize) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASHELEMENT {
    pub link: *mut HASHELEMENT,
    pub hashvalue: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASHCTL {
    pub num_partitions: libc::c_long,
    pub ssize: libc::c_long,
    pub dsize: libc::c_long,
    pub max_dsize: libc::c_long,
    pub keysize: usize,
    pub entrysize: usize,
    pub hash: HashValueFunc,
    pub match_0: HashCompareFunc,
    pub keycopy: HashCopyFunc,
    pub alloc: HashAllocFunc,
    pub hcxt: MemoryContext,
    pub hctl: *mut HASHHDR,
}
pub type HASHACTION = libc::c_uint;
pub const HASH_ENTER_NULL: HASHACTION = 3;
pub const HASH_REMOVE: HASHACTION = 2;
pub const HASH_ENTER: HASHACTION = 1;
pub const HASH_FIND: HASHACTION = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASH_SEQ_STATUS {
    pub hashp: *mut HTAB,
    pub curBucket: u32,
    pub curEntry: *mut HASHELEMENT,
}
pub type fmNodePtr = *mut Node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionCallInfoBaseData {
    pub flinfo: *mut FmgrInfo,
    pub context: fmNodePtr,
    pub resultinfo: fmNodePtr,
    pub fncollation: Oid,
    pub isnull: bool,
    pub nargs: libc::c_short,
    pub args: [NullableDatum; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FmgrInfo {
    pub fn_addr: PGFunction,
    pub fn_oid: Oid,
    pub fn_nargs: libc::c_short,
    pub fn_strict: bool,
    pub fn_retset: bool,
    pub fn_stats: libc::c_uchar,
    pub fn_extra: *mut libc::c_void,
    pub fn_mcxt: MemoryContext,
    pub fn_expr: fmNodePtr,
}
pub type PGFunction = Option<unsafe extern "C" fn(FunctionCallInfo) -> Datum>;
pub type FunctionCallInfo = *mut FunctionCallInfoBaseData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FuncCandidateList {
    pub next: *mut _FuncCandidateList,
    pub pathpos: libc::c_int,
    pub oid: Oid,
    pub nargs: libc::c_int,
    pub nvargs: libc::c_int,
    pub ndargs: libc::c_int,
    pub argnumbers: *mut libc::c_int,
    pub args: [Oid; 0],
}
pub type FuncCandidateList = *mut _FuncCandidateList;
pub type FuncDetailCode = libc::c_uint;
pub const FUNCDETAIL_COERCION: FuncDetailCode = 6;
pub const FUNCDETAIL_WINDOWFUNC: FuncDetailCode = 5;
pub const FUNCDETAIL_AGGREGATE: FuncDetailCode = 4;
pub const FUNCDETAIL_PROCEDURE: FuncDetailCode = 3;
pub const FUNCDETAIL_NORMAL: FuncDetailCode = 2;
pub const FUNCDETAIL_MULTIPLE: FuncDetailCode = 1;
pub const FUNCDETAIL_NOTFOUND: FuncDetailCode = 0;
pub type Operator = HeapTuple;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OprCacheKey {
    pub oprname: [libc::c_char; 64],
    pub left_arg: Oid,
    pub right_arg: Oid,
    pub search_path: [Oid; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OprCacheEntry {
    pub key: OprCacheKey,
    pub opr_oid: Oid,
}
pub const OPEROID: SysCacheIdentifier = 38;
pub const CASTSOURCETARGET: SysCacheIdentifier = 12;
pub type SyscacheCallbackFunction = Option<unsafe extern "C" fn(Datum, libc::c_int, u32) -> ()>;
pub const OPERNAMENSP: SysCacheIdentifier = 37;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TypeCacheEntry {
    pub type_id: Oid,
    pub type_id_hash: u32,
    pub typlen: i16,
    pub typbyval: bool,
    pub typalign: libc::c_char,
    pub typstorage: libc::c_char,
    pub typtype: libc::c_char,
    pub typrelid: Oid,
    pub typsubscript: Oid,
    pub typelem: Oid,
    pub typcollation: Oid,
    pub btree_opf: Oid,
    pub btree_opintype: Oid,
    pub hash_opf: Oid,
    pub hash_opintype: Oid,
    pub eq_opr: Oid,
    pub lt_opr: Oid,
    pub gt_opr: Oid,
    pub cmp_proc: Oid,
    pub hash_proc: Oid,
    pub hash_extended_proc: Oid,
    pub eq_opr_finfo: FmgrInfo,
    pub cmp_proc_finfo: FmgrInfo,
    pub hash_proc_finfo: FmgrInfo,
    pub hash_extended_proc_finfo: FmgrInfo,
    pub tupDesc: TupleDesc,
    pub tupDesc_identifier: uint64,
    pub rngelemtype: *mut TypeCacheEntry,
    pub rng_collation: Oid,
    pub rng_cmp_proc_finfo: FmgrInfo,
    pub rng_canonical_finfo: FmgrInfo,
    pub rng_subdiff_finfo: FmgrInfo,
    pub rngtype: *mut TypeCacheEntry,
    pub domainBaseType: Oid,
    pub domainBaseTypmod: i32,
    pub domainData: *mut DomainConstraintCache,
    pub flags: libc::c_int,
    pub enumData: *mut TypeCacheEnumData,
    pub nextDomain: *mut TypeCacheEntry,
}
pub type SysCacheIdentifier = libc::c_uint;
pub const USERMAPPINGUSERSERVER: SysCacheIdentifier = 78;
pub const USERMAPPINGOID: SysCacheIdentifier = 77;
pub const TYPEOID: SysCacheIdentifier = 76;
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
unsafe extern "C" fn list_nth_cell(mut list: *const List, mut n: libc::c_int) -> *mut ListCell {
    return &mut *((*list).elements).offset(n as isize) as *mut ListCell;
}
#[no_mangle]
pub unsafe extern "C" fn LookupOperName(
    mut pstate: *mut ParseState,
    mut opername: *mut List,
    mut oprleft: Oid,
    mut oprright: Oid,
    mut noError: bool,
    mut location: libc::c_int,
) -> Oid {
    let mut result: Oid = 0;
    result = OpernameGetOprid(opername, oprleft, oprright);
    if (result != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
        return result;
    }
    if noError == 0 {
        let mut oprkind: libc::c_char = 0;
        if (oprleft != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
            oprkind = 'l' as i32 as libc::c_char;
        } else if (oprright != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
            oprkind = 'b' as i32 as libc::c_char;
        } else {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
            oprkind = 0 as libc::c_int as libc::c_char;
        }
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    return 0 as libc::c_int as Oid;
}
#[no_mangle]
pub unsafe extern "C" fn LookupOperWithArgs(
    mut oper_0: *mut ObjectWithArgs,
    mut noError: bool,
) -> Oid {
    let mut oprleft: *mut TypeName = 0 as *mut TypeName;
    let mut oprright: *mut TypeName = 0 as *mut TypeName;
    let mut leftoid: Oid = 0;
    let mut rightoid: Oid = 0;
    oprleft = (*list_nth_cell((*oper_0).objargs, 0 as libc::c_int)).ptr_value as *mut TypeName;
    oprright = (*list_nth_cell((*oper_0).objargs, 1 as libc::c_int)).ptr_value as *mut TypeName;
    if oprleft.is_null() {
        leftoid = 0 as libc::c_int as Oid;
    } else {
        leftoid = LookupTypeNameOid(0 as *mut ParseState, oprleft, noError);
    }
    if oprright.is_null() {
        rightoid = 0 as libc::c_int as Oid;
    } else {
        rightoid = LookupTypeNameOid(0 as *mut ParseState, oprright, noError);
    }
    return LookupOperName(
        0 as *mut ParseState,
        (*oper_0).objname,
        leftoid,
        rightoid,
        noError,
        -(1 as libc::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn get_sort_group_operators(
    mut argtype: Oid,
    mut needLT: bool,
    mut needEQ: bool,
    mut needGT: bool,
    mut ltOpr: *mut Oid,
    mut eqOpr: *mut Oid,
    mut gtOpr: *mut Oid,
    mut isHashable: *mut bool,
) {
    let mut typentry: *mut TypeCacheEntry = 0 as *mut TypeCacheEntry;
    let mut cache_flags: libc::c_int = 0;
    let mut lt_opr: Oid = 0;
    let mut eq_opr: Oid = 0;
    let mut gt_opr: Oid = 0;
    let mut hashable: bool = false;
    if !isHashable.is_null() {
        cache_flags =
            0x2 as libc::c_int | 0x1 as libc::c_int | 0x4 as libc::c_int | 0x10 as libc::c_int;
    } else {
        cache_flags = 0x2 as libc::c_int | 0x1 as libc::c_int | 0x4 as libc::c_int;
    }
    typentry = lookup_type_cache(argtype, cache_flags);
    lt_opr = (*typentry).lt_opr;
    eq_opr = (*typentry).eq_opr;
    gt_opr = (*typentry).gt_opr;
    hashable = ((*typentry).hash_proc != 0 as libc::c_int as Oid) as libc::c_int as bool;
    if needLT as libc::c_int != 0 && (lt_opr != 0 as libc::c_int as Oid) as libc::c_int as bool == 0
        || needGT as libc::c_int != 0
            && (gt_opr != 0 as libc::c_int as Oid) as libc::c_int as bool == 0
    {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if needEQ as libc::c_int != 0 && (eq_opr != 0 as libc::c_int as Oid) as libc::c_int as bool == 0
    {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    if !ltOpr.is_null() {
        *ltOpr = lt_opr;
    }
    if !eqOpr.is_null() {
        *eqOpr = eq_opr;
    }
    if !gtOpr.is_null() {
        *gtOpr = gt_opr;
    }
    if !isHashable.is_null() {
        *isHashable = hashable;
    }
}
#[no_mangle]
pub unsafe extern "C" fn oprid(mut op: Operator) -> Oid {
    return (*(((*op).t_data as *mut libc::c_char)
        .offset((*(*op).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_operator))
        .oid;
}
#[no_mangle]
pub unsafe extern "C" fn oprfuncid(mut op: Operator) -> Oid {
    let mut pgopform: Form_pg_operator = ((*op).t_data as *mut libc::c_char)
        .offset((*(*op).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_operator;
    return (*pgopform).oprcode;
}
unsafe extern "C" fn oper_select_candidate(
    mut nargs: libc::c_int,
    mut input_typeids: *mut Oid,
    mut candidates: FuncCandidateList,
    mut operOid: *mut Oid,
) -> FuncDetailCode {
    let mut ncandidates: libc::c_int = 0;
    ncandidates = func_match_argtypes(nargs, input_typeids, candidates, &mut candidates);
    if ncandidates == 0 as libc::c_int {
        *operOid = 0 as libc::c_int as Oid;
        return FUNCDETAIL_NOTFOUND;
    }
    if ncandidates == 1 as libc::c_int {
        *operOid = (*candidates).oid;
        return FUNCDETAIL_NORMAL;
    }
    candidates = func_select_candidate(nargs, input_typeids, candidates);
    if !candidates.is_null() {
        *operOid = (*candidates).oid;
        return FUNCDETAIL_NORMAL;
    }
    *operOid = 0 as libc::c_int as Oid;
    return FUNCDETAIL_MULTIPLE;
}
#[no_mangle]
pub unsafe extern "C" fn oper(
    mut pstate: *mut ParseState,
    mut opname: *mut List,
    mut ltypeId: Oid,
    mut rtypeId: Oid,
    mut noError: bool,
    mut location: libc::c_int,
) -> Operator {
    let mut operOid: Oid = 0;
    let mut key: OprCacheKey = OprCacheKey {
        oprname: [0; 64],
        left_arg: 0,
        right_arg: 0,
        search_path: [0; 16],
    };
    let mut key_ok: bool = false;
    let mut fdresult: FuncDetailCode = FUNCDETAIL_NOTFOUND;
    let mut tup: HeapTuple = 0 as HeapTuple;
    key_ok = make_oper_cache_key(pstate, &mut key, opname, ltypeId, rtypeId, location);
    if key_ok {
        operOid = find_oper_cache_entry(&mut key);
        if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
            tup = SearchSysCache1(OPEROID as libc::c_int, operOid as Datum);
            if !(tup as *const libc::c_void).is_null() {
                return tup;
            }
        }
    }
    operOid = binary_oper_exact(opname, ltypeId, rtypeId);
    if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
        let mut clist: FuncCandidateList = 0 as *mut _FuncCandidateList;
        clist = OpernameGetCandidates(opname, 'b' as i32 as libc::c_char, false);
        if !clist.is_null() {
            let mut inputOids: [Oid; 2] = [0; 2];
            if rtypeId == 0 as libc::c_int as Oid {
                rtypeId = ltypeId;
            } else if ltypeId == 0 as libc::c_int as Oid {
                ltypeId = rtypeId;
            }
            inputOids[0 as libc::c_int as usize] = ltypeId;
            inputOids[1 as libc::c_int as usize] = rtypeId;
            fdresult = oper_select_candidate(
                2 as libc::c_int,
                inputOids.as_mut_ptr(),
                clist,
                &mut operOid,
            );
        }
    }
    if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
        tup = SearchSysCache1(OPEROID as libc::c_int, operOid as Datum);
    }
    if !(tup as *const libc::c_void).is_null() {
        if key_ok {
            make_oper_cache_entry(&mut key, operOid);
        }
    } else if noError == 0 {
        op_error(
            pstate,
            opname,
            'b' as i32 as libc::c_char,
            ltypeId,
            rtypeId,
            fdresult,
            location,
        );
    }
    return tup;
}
#[no_mangle]
pub unsafe extern "C" fn compatible_oper(
    mut pstate: *mut ParseState,
    mut op: *mut List,
    mut arg1: Oid,
    mut arg2: Oid,
    mut noError: bool,
    mut location: libc::c_int,
) -> Operator {
    let mut optup: Operator = 0 as *mut HeapTupleData;
    let mut opform: Form_pg_operator = 0 as *mut FormData_pg_operator;
    optup = oper(pstate, op, arg1, arg2, noError, location);
    if optup.is_null() {
        return 0 as *mut libc::c_void as Operator;
    }
    opform = ((*optup).t_data as *mut libc::c_char)
        .offset((*(*optup).t_data).t_hoff as libc::c_int as isize) as Form_pg_operator;
    if IsBinaryCoercible(arg1, (*opform).oprleft) as libc::c_int != 0
        && IsBinaryCoercible(arg2, (*opform).oprright) as libc::c_int != 0
    {
        return optup;
    }
    ReleaseSysCache(optup);
    if noError == 0 {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return 0 as *mut libc::c_void as Operator;
}
#[no_mangle]
pub unsafe extern "C" fn compatible_oper_opid(
    mut op: *mut List,
    mut arg1: Oid,
    mut arg2: Oid,
    mut noError: bool,
) -> Oid {
    let mut optup: Operator = 0 as *mut HeapTupleData;
    let mut result: Oid = 0;
    optup = compatible_oper(
        0 as *mut ParseState,
        op,
        arg1,
        arg2,
        noError,
        -(1 as libc::c_int),
    );
    if !optup.is_null() {
        result = oprid(optup);
        ReleaseSysCache(optup);
        return result;
    }
    return 0 as libc::c_int as Oid;
}
#[no_mangle]
pub unsafe extern "C" fn left_oper(
    mut pstate: *mut ParseState,
    mut op: *mut List,
    mut arg: Oid,
    mut noError: bool,
    mut location: libc::c_int,
) -> Operator {
    let mut operOid: Oid = 0;
    let mut key: OprCacheKey = OprCacheKey {
        oprname: [0; 64],
        left_arg: 0,
        right_arg: 0,
        search_path: [0; 16],
    };
    let mut key_ok: bool = false;
    let mut fdresult: FuncDetailCode = FUNCDETAIL_NOTFOUND;
    let mut tup: HeapTuple = 0 as HeapTuple;
    key_ok = make_oper_cache_key(pstate, &mut key, op, 0 as libc::c_int as Oid, arg, location);
    if key_ok {
        operOid = find_oper_cache_entry(&mut key);
        if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
            tup = SearchSysCache1(OPEROID as libc::c_int, operOid as Datum);
            if !(tup as *const libc::c_void).is_null() {
                return tup;
            }
        }
    }
    operOid = OpernameGetOprid(op, 0 as libc::c_int as Oid, arg);
    if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
        let mut clist: FuncCandidateList = 0 as *mut _FuncCandidateList;
        clist = OpernameGetCandidates(op, 'l' as i32 as libc::c_char, false);
        if !clist.is_null() {
            let mut clisti: FuncCandidateList = 0 as *mut _FuncCandidateList;
            clisti = clist;
            while !clisti.is_null() {
                *((*clisti).args)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) = *((*clisti).args)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize);
                clisti = (*clisti).next;
            }
            fdresult = oper_select_candidate(1 as libc::c_int, &mut arg, clist, &mut operOid);
        }
    }
    if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
        tup = SearchSysCache1(OPEROID as libc::c_int, operOid as Datum);
    }
    if !(tup as *const libc::c_void).is_null() {
        if key_ok {
            make_oper_cache_entry(&mut key, operOid);
        }
    } else if noError == 0 {
        op_error(
            pstate,
            op,
            'l' as i32 as libc::c_char,
            0 as libc::c_int as Oid,
            arg,
            fdresult,
            location,
        );
    }
    return tup;
}
unsafe extern "C" fn op_error(
    mut pstate: *mut ParseState,
    mut op: *mut List,
    mut oprkind: libc::c_char,
    mut arg1: Oid,
    mut arg2: Oid,
    mut fdresult: FuncDetailCode,
    mut location: libc::c_int,
) {
    if fdresult as libc::c_uint == FUNCDETAIL_MULTIPLE as libc::c_int as libc::c_uint {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    } else {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn make_op(
    mut pstate: *mut ParseState,
    mut opname: *mut List,
    mut ltree: *mut Node,
    mut rtree: *mut Node,
    mut last_srf: *mut Node,
    mut location: libc::c_int,
) -> *mut Expr {
    let mut ltypeId: Oid = 0;
    let mut rtypeId: Oid = 0;
    let mut tup: Operator = 0 as *mut HeapTupleData;
    let mut opform: Form_pg_operator = 0 as *mut FormData_pg_operator;
    let mut actual_arg_types: [Oid; 2] = [0; 2];
    let mut declared_arg_types: [Oid; 2] = [0; 2];
    let mut nargs: libc::c_int = 0;
    let mut args: *mut List = 0 as *mut List;
    let mut rettype: Oid = 0;
    let mut result: *mut OpExpr = 0 as *mut OpExpr;
    if rtree.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if ltree.is_null() {
        rtypeId = exprType(rtree);
        ltypeId = 0 as libc::c_int as Oid;
        tup = left_oper(pstate, opname, rtypeId, false, location);
    } else {
        ltypeId = exprType(ltree);
        rtypeId = exprType(rtree);
        tup = oper(pstate, opname, ltypeId, rtypeId, false, location);
    }
    opform = ((*tup).t_data as *mut libc::c_char)
        .offset((*(*tup).t_data).t_hoff as libc::c_int as isize) as Form_pg_operator;
    if ((*opform).oprcode != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    if ltree.is_null() {
        args = list_make1_impl(
            T_List,
            ListCell {
                ptr_value: rtree as *mut libc::c_void,
            },
        );
        actual_arg_types[0 as libc::c_int as usize] = rtypeId;
        declared_arg_types[0 as libc::c_int as usize] = (*opform).oprright;
        nargs = 1 as libc::c_int;
    } else {
        args = list_make2_impl(
            T_List,
            ListCell {
                ptr_value: ltree as *mut libc::c_void,
            },
            ListCell {
                ptr_value: rtree as *mut libc::c_void,
            },
        );
        actual_arg_types[0 as libc::c_int as usize] = ltypeId;
        actual_arg_types[1 as libc::c_int as usize] = rtypeId;
        declared_arg_types[0 as libc::c_int as usize] = (*opform).oprleft;
        declared_arg_types[1 as libc::c_int as usize] = (*opform).oprright;
        nargs = 2 as libc::c_int;
    }
    rettype = enforce_generic_type_consistency(
        actual_arg_types.as_mut_ptr(),
        declared_arg_types.as_mut_ptr(),
        nargs,
        (*opform).oprresult,
        false,
    );
    make_fn_arguments(
        pstate,
        args,
        actual_arg_types.as_mut_ptr(),
        declared_arg_types.as_mut_ptr(),
    );
    result = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_OpExpr;
        _result
    }) as *mut OpExpr;
    (*result).opno = oprid(tup);
    (*result).opfuncid = (*opform).oprcode;
    (*result).opresulttype = rettype;
    (*result).opretset = get_func_retset((*opform).oprcode);
    (*result).args = args;
    (*result).location = location;
    if (*result).opretset {
        check_srf_call_placement(pstate, last_srf, location);
        (*pstate).p_last_srf = result as *mut Node;
    }
    ReleaseSysCache(tup);
    return result as *mut Expr;
}
static mut OprCacheHash: *mut HTAB = 0 as *const HTAB as *mut HTAB;
unsafe extern "C" fn find_oper_cache_entry(mut key: *mut OprCacheKey) -> Oid {
    let mut oprentry: *mut OprCacheEntry = 0 as *mut OprCacheEntry;
    if OprCacheHash.is_null() {
        let mut ctl: HASHCTL = HASHCTL {
            num_partitions: 0,
            ssize: 0,
            dsize: 0,
            max_dsize: 0,
            keysize: 0,
            entrysize: 0,
            hash: None,
            match_0: None,
            keycopy: None,
            alloc: None,
            hcxt: 0 as *mut MemoryContextData,
            hctl: 0 as *mut HASHHDR,
        };
        ctl.keysize = ::core::mem::size_of::<OprCacheKey>() as libc::c_ulong;
        ctl.entrysize = ::core::mem::size_of::<OprCacheEntry>() as libc::c_ulong;
        OprCacheHash = hash_create(
            b"Operator lookup cache\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int as libc::c_long,
            &mut ctl,
            0x8 as libc::c_int | 0x20 as libc::c_int,
        );
        CacheRegisterSyscacheCallback(
            OPERNAMENSP as libc::c_int,
            Some(InvalidateOprCacheCallBack as unsafe extern "C" fn(Datum, libc::c_int, u32) -> ()),
            0 as libc::c_int as Datum,
        );
        CacheRegisterSyscacheCallback(
            CASTSOURCETARGET as libc::c_int,
            Some(InvalidateOprCacheCallBack as unsafe extern "C" fn(Datum, libc::c_int, u32) -> ()),
            0 as libc::c_int as Datum,
        );
    }
    oprentry = hash_search(
        OprCacheHash,
        key as *mut libc::c_void,
        HASH_FIND,
        0 as *mut bool,
    ) as *mut OprCacheEntry;
    if oprentry.is_null() {
        return 0 as libc::c_int as Oid;
    }
    return (*oprentry).opr_oid;
}
unsafe extern "C" fn make_oper_cache_entry(mut key: *mut OprCacheKey, mut opr_oid: Oid) {
    let mut oprentry: *mut OprCacheEntry = 0 as *mut OprCacheEntry;
    oprentry = hash_search(
        OprCacheHash,
        key as *mut libc::c_void,
        HASH_ENTER,
        0 as *mut bool,
    ) as *mut OprCacheEntry;
    (*oprentry).opr_oid = opr_oid;
}
unsafe extern "C" fn InvalidateOprCacheCallBack(
    mut arg: Datum,
    mut cacheid: libc::c_int,
    mut hashvalue: u32,
) {
    let mut status: HASH_SEQ_STATUS = HASH_SEQ_STATUS {
        hashp: 0 as *mut HTAB,
        curBucket: 0,
        curEntry: 0 as *mut HASHELEMENT,
    };
    let mut hentry: *mut OprCacheEntry = 0 as *mut OprCacheEntry;
    hash_seq_init(&mut status, OprCacheHash);
    loop {
        hentry = hash_seq_search(&mut status) as *mut OprCacheEntry;
        if hentry.is_null() {
            break;
        }
        if (hash_search(
            OprCacheHash,
            &mut (*hentry).key as *mut OprCacheKey as *mut libc::c_void,
            HASH_REMOVE,
            0 as *mut bool,
        ))
        .is_null()
        {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(b"hash table corrupted\0" as *const u8 as *const libc::c_char);
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_oper.c\0"
                        as *const u8 as *const libc::c_char,
                    1063 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
    }
}
