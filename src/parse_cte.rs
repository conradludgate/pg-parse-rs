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
//     pub type TypeCacheEnumData;
//     pub type DomainConstraintCache;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn abort() -> !;
//     fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
//     fn palloc0(size: usize) -> *mut libc::c_void;
//     fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
//     fn bms_is_empty(a: *const Bitmapset) -> bool;
//     fn bms_add_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
//     fn bms_del_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
//     fn copyObjectImpl(obj: *const libc::c_void) -> *mut libc::c_void;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn lappend_int(list: *mut List, datum: libc::c_int) -> *mut List;
//     fn lappend_oid(list: *mut List, datum: Oid) -> *mut List;
//     fn lcons(datum: *mut libc::c_void, list: *mut List) -> *mut List;
//     fn list_member(list: *const List, datum: *const libc::c_void) -> bool;
//     fn list_delete_first(list: *mut List) -> *mut List;
//     fn list_copy(list: *const List) -> *mut List;
//     fn makeString(str: *mut libc::c_char) -> *mut Value;
//     fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprTypmod(expr: *const Node) -> i32;
//     fn exprCollation(expr: *const Node) -> Oid;
//     fn raw_expression_tree_walker(
//         node: *mut Node,
//         walker: Option::<unsafe extern "C" fn() -> bool>,
//         context: *mut libc::c_void,
//     ) -> bool;
//     fn parse_sub_analyze(
//         parseTree: *mut Node,
//         parentParseState: *mut ParseState,
//         parentCTE: *mut CommonTableExpr,
//         locked_from_parent: bool,
//         resolve_unknowns: bool,
//     ) -> *mut Query;
//     fn select_common_type(
//         pstate: *mut ParseState,
//         exprs: *mut List,
//         context: *const libc::c_char,
//         which_expr: *mut *mut Node,
//     ) -> Oid;
//     fn coerce_to_common_type(
//         pstate: *mut ParseState,
//         node: *mut Node,
//         targetTypeId: Oid,
//         context: *const libc::c_char,
//     ) -> *mut Node;
//     fn select_common_typmod(
//         pstate: *mut ParseState,
//         exprs: *mut List,
//         common_type: Oid,
//     ) -> i32;
//     fn select_common_collation(
//         pstate: *mut ParseState,
//         exprs: *mut List,
//         none_ok: bool,
//     ) -> Oid;
//     fn transformExpr(
//         pstate: *mut ParseState,
//         expr: *mut Node,
//         exprKind: ParseExprKind,
//     ) -> *mut Node;
//     fn get_negator(opno: Oid) -> Oid;
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
// pub type u16 = libc::c_ushort;
// pub type u32 = libc::c_uint;
// pub type uint64 = libc::c_ulong;
// pub type usize = isize;
// pub type Index = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameData {
    pub data: [libc::c_char; 64],
}
pub type NameData = nameData;
pub type MemoryContext = *mut MemoryContextData;
// pub type Datum = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NullableDatum {
    pub value: Datum,
    pub isnull: bool,
}
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

pub type bitmapword = u32;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Bitmapset {
//     pub nwords: libc::c_int,
//     pub words: [bitmapword; 0],
// }
// pub type AttrNumber = i16;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Node {
//     pub type_0: NodeTag,
// }
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ForEachState {
    pub l: *const List,
    pub i: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Alias {
    pub type_0: NodeTag,
    pub aliasname: *mut libc::c_char,
    pub colnames: *mut List,
}
pub type OnCommitAction = libc::c_uint;
pub const ONCOMMIT_DROP: OnCommitAction = 3;
pub const ONCOMMIT_DELETE_ROWS: OnCommitAction = 2;
pub const ONCOMMIT_PRESERVE_ROWS: OnCommitAction = 1;
pub const ONCOMMIT_NOOP: OnCommitAction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RangeVar {
    pub type_0: NodeTag,
    pub catalogname: *mut libc::c_char,
    pub schemaname: *mut libc::c_char,
    pub relname: *mut libc::c_char,
    pub inh: bool,
    pub relpersistence: libc::c_char,
    pub alias: *mut Alias,
    pub location: libc::c_int,
}
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
pub type SubLinkType = libc::c_uint;
pub const CTE_SUBLINK: SubLinkType = 7;
pub const ARRAY_SUBLINK: SubLinkType = 6;
pub const MULTIEXPR_SUBLINK: SubLinkType = 5;
pub const EXPR_SUBLINK: SubLinkType = 4;
pub const ROWCOMPARE_SUBLINK: SubLinkType = 3;
pub const ANY_SUBLINK: SubLinkType = 2;
pub const ALL_SUBLINK: SubLinkType = 1;
pub const EXISTS_SUBLINK: SubLinkType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubLink {
    pub xpr: Expr,
    pub subLinkType: SubLinkType,
    pub subLinkId: libc::c_int,
    pub testexpr: *mut Node,
    pub operName: *mut List,
    pub subselect: *mut Node,
    pub location: libc::c_int,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct TargetEntry {
//     pub xpr: Expr,
//     pub expr: *mut Expr,
//     pub resno: AttrNumber,
//     pub resname: *mut libc::c_char,
//     pub ressortgroupref: Index,
//     pub resorigtbl: Oid,
//     pub resorigcol: AttrNumber,
//     pub resjunk: bool,
// }
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Query {
    pub type_0: NodeTag,
    pub commandType: CmdType,
    pub querySource: QuerySource,
    pub queryId: uint64,
    pub canSetTag: bool,
    pub utilityStmt: *mut Node,
    pub resultRelation: libc::c_int,
    pub hasAggs: bool,
    pub hasWindowFuncs: bool,
    pub hasTargetSRFs: bool,
    pub hasSubLinks: bool,
    pub hasDistinctOn: bool,
    pub hasRecursive: bool,
    pub hasModifyingCTE: bool,
    pub hasForUpdate: bool,
    pub hasRowSecurity: bool,
    pub cteList: *mut List,
    pub rtable: *mut List,
    pub jointree: *mut FromExpr,
    pub targetList: *mut List,
    pub override_0: OverridingKind,
    pub onConflict: *mut OnConflictExpr,
    pub returningList: *mut List,
    pub groupClause: *mut List,
    pub groupingSets: *mut List,
    pub havingQual: *mut Node,
    pub windowClause: *mut List,
    pub distinctClause: *mut List,
    pub sortClause: *mut List,
    pub limitOffset: *mut Node,
    pub limitCount: *mut Node,
    pub limitOption: LimitOption,
    pub rowMarks: *mut List,
    pub setOperations: *mut Node,
    pub constraintDeps: *mut List,
    pub withCheckOptions: *mut List,
    pub stmt_location: libc::c_int,
    pub stmt_len: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WithClause {
    pub type_0: NodeTag,
    pub ctes: *mut List,
    pub recursive: bool,
    pub location: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommonTableExpr {
    pub type_0: NodeTag,
    pub ctename: *mut libc::c_char,
    pub aliascolnames: *mut List,
    pub ctematerialized: CTEMaterialize,
    pub ctequery: *mut Node,
    pub search_clause: *mut CTESearchClause,
    pub cycle_clause: *mut CTECycleClause,
    pub location: libc::c_int,
    pub cterecursive: bool,
    pub cterefcount: libc::c_int,
    pub ctecolnames: *mut List,
    pub ctecoltypes: *mut List,
    pub ctecoltypmods: *mut List,
    pub ctecolcollations: *mut List,
}
pub type SetOperation = libc::c_uint;
pub const SETOP_EXCEPT: SetOperation = 3;
pub const SETOP_INTERSECT: SetOperation = 2;
pub const SETOP_UNION: SetOperation = 1;
pub const SETOP_NONE: SetOperation = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SelectStmt {
    pub type_0: NodeTag,
    pub distinctClause: *mut List,
    pub intoClause: *mut IntoClause,
    pub targetList: *mut List,
    pub fromClause: *mut List,
    pub whereClause: *mut Node,
    pub groupClause: *mut List,
    pub havingClause: *mut Node,
    pub windowClause: *mut List,
    pub valuesLists: *mut List,
    pub sortClause: *mut List,
    pub limitOffset: *mut Node,
    pub limitCount: *mut Node,
    pub limitOption: LimitOption,
    pub lockingClause: *mut List,
    pub withClause: *mut WithClause,
    pub op: SetOperation,
    pub all: bool,
    pub larg: *mut SelectStmt,
    pub rarg: *mut SelectStmt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SetOperationStmt {
    pub type_0: NodeTag,
    pub op: SetOperation,
    pub all: bool,
    pub larg: *mut Node,
    pub rarg: *mut Node,
    pub colTypes: *mut List,
    pub colTypmods: *mut List,
    pub colCollations: *mut List,
    pub groupClauses: *mut List,
}
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
// pub type Relation = *mut RelationData;

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
pub type fmNodePtr = *mut Node;
pub type PGFunction = Option<unsafe extern "C" fn(FunctionCallInfo) -> Datum>;
pub type FunctionCallInfo = *mut FunctionCallInfoBaseData;
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
pub struct CteItem {
    pub cte: *mut CommonTableExpr,
    pub id: libc::c_int,
    pub depends_on: *mut Bitmapset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CteState {
    pub pstate: *mut ParseState,
    pub items: *mut CteItem,
    pub numitems: libc::c_int,
    pub curitem: libc::c_int,
    pub innerwiths: *mut List,
    pub selfrefcount: libc::c_int,
    pub context: RecursionContext,
}
pub type RecursionContext = libc::c_uint;
pub const RECURSION_EXCEPT: RecursionContext = 5;
pub const RECURSION_INTERSECT: RecursionContext = 4;
pub const RECURSION_OUTERJOIN: RecursionContext = 3;
pub const RECURSION_SUBLINK: RecursionContext = 2;
pub const RECURSION_NONRECURSIVETERM: RecursionContext = 1;
pub const RECURSION_OK: RecursionContext = 0;
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
unsafe extern "C" fn list_cell_number(mut l: *const List, mut c: *const ListCell) -> libc::c_int {
    return c.offset_from((*l).elements) as libc::c_long as libc::c_int;
}
#[inline]
unsafe extern "C" fn lnext(mut l: *const List, mut c: *const ListCell) -> *mut ListCell {
    c = c.offset(1);
    c;
    if c < &mut *((*l).elements).offset((*l).length as isize) as *mut ListCell as *const ListCell {
        return c as *mut ListCell;
    } else {
        return 0 as *mut ListCell;
    };
}
#[inline]
unsafe extern "C" fn for_each_cell_setup(
    mut lst: *const List,
    mut initcell: *const ListCell,
) -> ForEachState {
    let mut r: ForEachState = {
        let mut init = ForEachState {
            l: lst,
            i: if !initcell.is_null() {
                list_cell_number(lst, initcell)
            } else {
                list_length(lst)
            },
        };
        init
    };
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn transformWithClause(
    mut pstate: *mut ParseState,
    mut withClause: *mut WithClause,
) -> *mut List {
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*withClause).ctes,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length {
        lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        lc = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut cte: *mut CommonTableExpr = (*lc).ptr_value as *mut CommonTableExpr;
        let mut rest: *mut ListCell = 0 as *mut ListCell;
        let mut rest__state: ForEachState =
            for_each_cell_setup((*withClause).ctes, lnext((*withClause).ctes, lc));
        while if !(rest__state.l).is_null() && rest__state.i < (*rest__state.l).length {
            rest =
                &mut *((*rest__state.l).elements).offset(rest__state.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            rest = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut cte2: *mut CommonTableExpr = (*rest).ptr_value as *mut CommonTableExpr;
            if strcmp((*cte).ctename, (*cte2).ctename) == 0 as libc::c_int {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            rest__state.i += 1;
            rest__state.i;
        }
        (*cte).cterecursive = false;
        (*cte).cterefcount = 0 as libc::c_int;
        if !((*((*cte).ctequery as *const Node)).tag as libc::c_uint
            == T_SelectStmt as libc::c_int as libc::c_uint)
        {
            (*pstate).p_hasModifyingCTE = true;
        }
        lc__state.i += 1;
        lc__state.i;
    }
    if (*withClause).recursive {
        let mut cstate: CteState = CteState {
            pstate: 0 as *mut ParseState,
            items: 0 as *mut CteItem,
            numitems: 0,
            curitem: 0,
            innerwiths: 0 as *mut List,
            selfrefcount: 0,
            context: RECURSION_OK,
        };
        let mut i: libc::c_int = 0;
        cstate.pstate = pstate;
        cstate.numitems = list_length((*withClause).ctes);
        cstate.items = palloc0(
            (cstate.numitems as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<CteItem>() as libc::c_ulong),
        ) as *mut CteItem;
        i = 0 as libc::c_int;
        let mut lc__state_0: ForEachState = {
            let mut init = ForEachState {
                l: (*withClause).ctes,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(lc__state_0.l).is_null() && lc__state_0.i < (*lc__state_0.l).length {
            lc = &mut *((*lc__state_0.l).elements).offset(lc__state_0.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            lc = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let ref mut fresh0 = (*(cstate.items).offset(i as isize)).cte;
            *fresh0 = (*lc).ptr_value as *mut CommonTableExpr;
            (*(cstate.items).offset(i as isize)).id = i;
            i += 1;
            i;
            lc__state_0.i += 1;
            lc__state_0.i;
        }
        makeDependencyGraph(&mut cstate);
        checkWellFormedRecursion(&mut cstate);
        i = 0 as libc::c_int;
        while i < cstate.numitems {
            let mut cte_0: *mut CommonTableExpr = (*(cstate.items).offset(i as isize)).cte;
            (*pstate).p_ctenamespace =
                lappend((*pstate).p_ctenamespace, cte_0 as *mut libc::c_void);
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < cstate.numitems {
            let mut cte_1: *mut CommonTableExpr = (*(cstate.items).offset(i as isize)).cte;
            analyzeCTE(pstate, cte_1);
            i += 1;
            i;
        }
    } else {
        (*pstate).p_future_ctes = list_copy((*withClause).ctes);
        let mut lc__state_1: ForEachState = {
            let mut init = ForEachState {
                l: (*withClause).ctes,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(lc__state_1.l).is_null() && lc__state_1.i < (*lc__state_1.l).length {
            lc = &mut *((*lc__state_1.l).elements).offset(lc__state_1.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            lc = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut cte_2: *mut CommonTableExpr = (*lc).ptr_value as *mut CommonTableExpr;
            analyzeCTE(pstate, cte_2);
            (*pstate).p_ctenamespace =
                lappend((*pstate).p_ctenamespace, cte_2 as *mut libc::c_void);
            (*pstate).p_future_ctes = list_delete_first((*pstate).p_future_ctes);
            lc__state_1.i += 1;
            lc__state_1.i;
        }
    }
    return (*pstate).p_ctenamespace;
}
unsafe extern "C" fn analyzeCTE(mut pstate: *mut ParseState, mut cte: *mut CommonTableExpr) {
    let mut query: *mut Query = 0 as *mut Query;
    query = parse_sub_analyze((*cte).ctequery, pstate, cte, false, true);
    (*cte).ctequery = query as *mut Node;
    if !((*(query as *const Node)).tag as libc::c_uint == T_Query as libc::c_int as libc::c_uint) {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"unexpected non-Query statement in WITH\0" as *const u8 as *const libc::c_char,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_cte.c\0"
                    as *const u8 as *const libc::c_char,
                256 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*query).utilityStmt).is_null() {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"unexpected utility statement in WITH\0" as *const u8 as *const libc::c_char,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_cte.c\0"
                    as *const u8 as *const libc::c_char,
                258 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    if (*query).commandType as libc::c_uint != CMD_SELECT as libc::c_int as libc::c_uint
        && !((*pstate).parentParseState).is_null()
    {
        let elevel__1: libc::c_int = 21 as libc::c_int;
        let mut __error_1: libc::c_int = 0;
        if elevel__1 >= 21 as libc::c_int {
            abort();
        }
    }
    (*query).canSetTag = false;
    if (*cte).cterecursive == 0 {
        analyzeCTETargetList(
            pstate,
            cte,
            (if (*((*cte).ctequery as *mut Query)).commandType as libc::c_uint
                == CMD_SELECT as libc::c_int as libc::c_uint
            {
                (*((*cte).ctequery as *mut Query)).targetList
            } else {
                (*((*cte).ctequery as *mut Query)).returningList
            }),
        );
    } else {
        let mut lctlist: *mut ListCell = 0 as *mut ListCell;
        let mut lctyp: *mut ListCell = 0 as *mut ListCell;
        let mut lctypmod: *mut ListCell = 0 as *mut ListCell;
        let mut lccoll: *mut ListCell = 0 as *mut ListCell;
        let mut varattno: libc::c_int = 0;
        lctyp = list_head((*cte).ctecoltypes);
        lctypmod = list_head((*cte).ctecoltypmods);
        lccoll = list_head((*cte).ctecolcollations);
        varattno = 0 as libc::c_int;
        let mut lctlist__state: ForEachState = {
            let mut init = ForEachState {
                l: (if (*((*cte).ctequery as *mut Query)).commandType as libc::c_uint
                    == CMD_SELECT as libc::c_int as libc::c_uint
                {
                    (*((*cte).ctequery as *mut Query)).targetList
                } else {
                    (*((*cte).ctequery as *mut Query)).returningList
                }),
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(lctlist__state.l).is_null() && lctlist__state.i < (*lctlist__state.l).length {
            lctlist = &mut *((*lctlist__state.l).elements).offset(lctlist__state.i as isize)
                as *mut ListCell;
            true as libc::c_int
        } else {
            lctlist = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut te: *mut TargetEntry = (*lctlist).ptr_value as *mut TargetEntry;
            let mut texpr: *mut Node = 0 as *mut Node;
            if !((*te).resjunk != 0) {
                varattno += 1;
                varattno;
                if lctyp.is_null() || lctypmod.is_null() || lccoll.is_null() {
                    let elevel__2: libc::c_int = 21 as libc::c_int;
                    let mut __error_2: libc::c_int = 0;
                    if errstart(elevel__2, 0 as *const libc::c_char) != 0 {
                        errmsg_internal(
                            b"wrong number of output columns in WITH\0" as *const u8
                                as *const libc::c_char,
                        );
                        errfinish(
                            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_cte.c\0"
                                as *const u8 as *const libc::c_char,
                            311 as libc::c_int,
                            0 as *const libc::c_char,
                        );
                    }
                    if elevel__2 >= 21 as libc::c_int {
                        abort();
                    }
                }
                texpr = (*te).expr as *mut Node;
                if exprType(texpr) != (*lctyp).oid_value
                    || exprTypmod(texpr) != (*lctypmod).int_value
                {
                    let elevel__3: libc::c_int = 21 as libc::c_int;
                    let mut __error_3: libc::c_int = 0;
                    if elevel__3 >= 21 as libc::c_int {
                        abort();
                    }
                }
                if exprCollation(texpr) != (*lccoll).oid_value {
                    let elevel__4: libc::c_int = 21 as libc::c_int;
                    let mut __error_4: libc::c_int = 0;
                    if elevel__4 >= 21 as libc::c_int {
                        abort();
                    }
                }
                lctyp = lnext((*cte).ctecoltypes, lctyp);
                lctypmod = lnext((*cte).ctecoltypmods, lctypmod);
                lccoll = lnext((*cte).ctecolcollations, lccoll);
            }
            lctlist__state.i += 1;
            lctlist__state.i;
        }
        if !lctyp.is_null() || !lctypmod.is_null() || !lccoll.is_null() {
            let elevel__5: libc::c_int = 21 as libc::c_int;
            let mut __error_5: libc::c_int = 0;
            if errstart(elevel__5, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"wrong number of output columns in WITH\0" as *const u8 as *const libc::c_char,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_cte.c\0"
                        as *const u8 as *const libc::c_char,
                    339 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel__5 >= 21 as libc::c_int {
                abort();
            }
        }
    }
    if !((*cte).search_clause).is_null() || !((*cte).cycle_clause).is_null() {
        let mut ctequery: *mut Query = 0 as *mut Query;
        let mut sos: *mut SetOperationStmt = 0 as *mut SetOperationStmt;
        if (*cte).cterecursive == 0 {
            let elevel__6: libc::c_int = 21 as libc::c_int;
            let mut __error_6: libc::c_int = 0;
            if elevel__6 >= 21 as libc::c_int {
                abort();
            }
        }
        ctequery = (*cte).ctequery as *mut Query;
        sos = (*ctequery).setOperations as *mut SetOperationStmt;
        if !((*((*sos).larg as *const Node)).tag as libc::c_uint
            == T_RangeTblRef as libc::c_int as libc::c_uint)
        {
            let elevel__7: libc::c_int = 21 as libc::c_int;
            let mut __error_7: libc::c_int = 0;
            if elevel__7 >= 21 as libc::c_int {
                abort();
            }
        }
        if !((*((*sos).rarg as *const Node)).tag as libc::c_uint
            == T_RangeTblRef as libc::c_int as libc::c_uint)
        {
            let elevel__8: libc::c_int = 21 as libc::c_int;
            let mut __error_8: libc::c_int = 0;
            if elevel__8 >= 21 as libc::c_int {
                abort();
            }
        }
    }
    if !((*cte).search_clause).is_null() {
        let mut lc: *mut ListCell = 0 as *mut ListCell;
        let mut seen: *mut List = 0 as *mut libc::c_void as *mut List;
        let mut lc__state: ForEachState = {
            let mut init = ForEachState {
                l: (*(*cte).search_clause).search_col_list,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length {
            lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            lc = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut colname: *mut Value = (*lc).ptr_value as *mut Value;
            if list_member((*cte).ctecolnames, colname as *const libc::c_void) == 0 {
                let elevel__9: libc::c_int = 21 as libc::c_int;
                let mut __error_9: libc::c_int = 0;
                if elevel__9 >= 21 as libc::c_int {
                    abort();
                }
            }
            if list_member(seen, colname as *const libc::c_void) != 0 {
                let elevel__10: libc::c_int = 21 as libc::c_int;
                let mut __error_10: libc::c_int = 0;
                if elevel__10 >= 21 as libc::c_int {
                    abort();
                }
            }
            seen = lappend(seen, colname as *mut libc::c_void);
            lc__state.i += 1;
            lc__state.i;
        }
        if list_member(
            (*cte).ctecolnames,
            makeString((*(*cte).search_clause).search_seq_column) as *const libc::c_void,
        ) != 0
        {
            let elevel__11: libc::c_int = 21 as libc::c_int;
            let mut __error_11: libc::c_int = 0;
            if elevel__11 >= 21 as libc::c_int {
                abort();
            }
        }
    }
    if !((*cte).cycle_clause).is_null() {
        let mut lc_0: *mut ListCell = 0 as *mut ListCell;
        let mut seen_0: *mut List = 0 as *mut libc::c_void as *mut List;
        let mut typentry: *mut TypeCacheEntry = 0 as *mut TypeCacheEntry;
        let mut op: Oid = 0;
        let mut lc__state_0: ForEachState = {
            let mut init = ForEachState {
                l: (*(*cte).cycle_clause).cycle_col_list,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(lc__state_0.l).is_null() && lc__state_0.i < (*lc__state_0.l).length {
            lc_0 =
                &mut *((*lc__state_0.l).elements).offset(lc__state_0.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            lc_0 = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut colname_0: *mut Value = (*lc_0).ptr_value as *mut Value;
            if list_member((*cte).ctecolnames, colname_0 as *const libc::c_void) == 0 {
                let elevel__12: libc::c_int = 21 as libc::c_int;
                let mut __error_12: libc::c_int = 0;
                if elevel__12 >= 21 as libc::c_int {
                    abort();
                }
            }
            if list_member(seen_0, colname_0 as *const libc::c_void) != 0 {
                let elevel__13: libc::c_int = 21 as libc::c_int;
                let mut __error_13: libc::c_int = 0;
                if elevel__13 >= 21 as libc::c_int {
                    abort();
                }
            }
            seen_0 = lappend(seen_0, colname_0 as *mut libc::c_void);
            lc__state_0.i += 1;
            lc__state_0.i;
        }
        if list_member(
            (*cte).ctecolnames,
            makeString((*(*cte).cycle_clause).cycle_mark_column) as *const libc::c_void,
        ) != 0
        {
            let elevel__14: libc::c_int = 21 as libc::c_int;
            let mut __error_14: libc::c_int = 0;
            if elevel__14 >= 21 as libc::c_int {
                abort();
            }
        }
        (*(*cte).cycle_clause).cycle_mark_value = transformExpr(
            pstate,
            (*(*cte).cycle_clause).cycle_mark_value,
            EXPR_KIND_CYCLE_MARK,
        );
        (*(*cte).cycle_clause).cycle_mark_default = transformExpr(
            pstate,
            (*(*cte).cycle_clause).cycle_mark_default,
            EXPR_KIND_CYCLE_MARK,
        );
        if list_member(
            (*cte).ctecolnames,
            makeString((*(*cte).cycle_clause).cycle_path_column) as *const libc::c_void,
        ) != 0
        {
            let elevel__15: libc::c_int = 21 as libc::c_int;
            let mut __error_15: libc::c_int = 0;
            if elevel__15 >= 21 as libc::c_int {
                abort();
            }
        }
        if strcmp(
            (*(*cte).cycle_clause).cycle_mark_column,
            (*(*cte).cycle_clause).cycle_path_column,
        ) == 0 as libc::c_int
        {
            let elevel__16: libc::c_int = 21 as libc::c_int;
            let mut __error_16: libc::c_int = 0;
            if elevel__16 >= 21 as libc::c_int {
                abort();
            }
        }
        (*(*cte).cycle_clause).cycle_mark_type = select_common_type(
            pstate,
            list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: (*(*cte).cycle_clause).cycle_mark_value as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: (*(*cte).cycle_clause).cycle_mark_default as *mut libc::c_void,
                },
            ),
            b"CYCLE\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut Node,
        );
        (*(*cte).cycle_clause).cycle_mark_value = coerce_to_common_type(
            pstate,
            (*(*cte).cycle_clause).cycle_mark_value,
            (*(*cte).cycle_clause).cycle_mark_type,
            b"CYCLE/SET/TO\0" as *const u8 as *const libc::c_char,
        );
        (*(*cte).cycle_clause).cycle_mark_default = coerce_to_common_type(
            pstate,
            (*(*cte).cycle_clause).cycle_mark_default,
            (*(*cte).cycle_clause).cycle_mark_type,
            b"CYCLE/SET/DEFAULT\0" as *const u8 as *const libc::c_char,
        );
        (*(*cte).cycle_clause).cycle_mark_typmod = select_common_typmod(
            pstate,
            list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: (*(*cte).cycle_clause).cycle_mark_value as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: (*(*cte).cycle_clause).cycle_mark_default as *mut libc::c_void,
                },
            ),
            (*(*cte).cycle_clause).cycle_mark_type,
        );
        (*(*cte).cycle_clause).cycle_mark_collation = select_common_collation(
            pstate,
            list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: (*(*cte).cycle_clause).cycle_mark_value as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: (*(*cte).cycle_clause).cycle_mark_default as *mut libc::c_void,
                },
            ),
            true,
        );
        typentry = lookup_type_cache((*(*cte).cycle_clause).cycle_mark_type, 0x1 as libc::c_int);
        if (*typentry).eq_opr == 0 {
            let elevel__17: libc::c_int = 21 as libc::c_int;
            let mut __error_17: libc::c_int = 0;
            if elevel__17 >= 21 as libc::c_int {
                abort();
            }
        }
        op = get_negator((*typentry).eq_opr);
        if op == 0 {
            let elevel__18: libc::c_int = 21 as libc::c_int;
            let mut __error_18: libc::c_int = 0;
            if elevel__18 >= 21 as libc::c_int {
                abort();
            }
        }
        (*(*cte).cycle_clause).cycle_mark_neop = op;
    }
    if !((*cte).search_clause).is_null() && !((*cte).cycle_clause).is_null() {
        if strcmp(
            (*(*cte).search_clause).search_seq_column,
            (*(*cte).cycle_clause).cycle_mark_column,
        ) == 0 as libc::c_int
        {
            let elevel__19: libc::c_int = 21 as libc::c_int;
            let mut __error_19: libc::c_int = 0;
            if elevel__19 >= 21 as libc::c_int {
                abort();
            }
        }
        if strcmp(
            (*(*cte).search_clause).search_seq_column,
            (*(*cte).cycle_clause).cycle_path_column,
        ) == 0 as libc::c_int
        {
            let elevel__20: libc::c_int = 21 as libc::c_int;
            let mut __error_20: libc::c_int = 0;
            if elevel__20 >= 21 as libc::c_int {
                abort();
            }
        }
    }
}
unsafe extern "C" fn makeDependencyGraph(mut cstate: *mut CteState) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cstate).numitems {
        let mut cte: *mut CommonTableExpr = (*((*cstate).items).offset(i as isize)).cte;
        (*cstate).curitem = i;
        (*cstate).innerwiths = 0 as *mut libc::c_void as *mut List;
        makeDependencyGraphWalker((*cte).ctequery, cstate);
        i += 1;
        i;
    }
    TopologicalSort((*cstate).pstate, (*cstate).items, (*cstate).numitems);
}
unsafe extern "C" fn makeDependencyGraphWalker(
    mut node: *mut Node,
    mut cstate: *mut CteState,
) -> bool {
    if node.is_null() {
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_RangeVar as libc::c_int as libc::c_uint {
        let mut rv: *mut RangeVar = node as *mut RangeVar;
        if ((*rv).schemaname).is_null() {
            let mut lc: *mut ListCell = 0 as *mut ListCell;
            let mut i: libc::c_int = 0;
            let mut lc__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*cstate).innerwiths,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length {
                lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize) as *mut ListCell;
                true as libc::c_int
            } else {
                lc = 0 as *mut ListCell;
                false as libc::c_int
            } != 0
            {
                let mut withlist: *mut List = (*lc).ptr_value as *mut List;
                let mut lc2: *mut ListCell = 0 as *mut ListCell;
                let mut lc2__state: ForEachState = {
                    let mut init = ForEachState {
                        l: withlist,
                        i: 0 as libc::c_int,
                    };
                    init
                };
                while if !(lc2__state.l).is_null() && lc2__state.i < (*lc2__state.l).length {
                    lc2 = &mut *((*lc2__state.l).elements).offset(lc2__state.i as isize)
                        as *mut ListCell;
                    true as libc::c_int
                } else {
                    lc2 = 0 as *mut ListCell;
                    false as libc::c_int
                } != 0
                {
                    let mut cte: *mut CommonTableExpr = (*lc2).ptr_value as *mut CommonTableExpr;
                    if strcmp((*rv).relname, (*cte).ctename) == 0 as libc::c_int {
                        return false;
                    }
                    lc2__state.i += 1;
                    lc2__state.i;
                }
                lc__state.i += 1;
                lc__state.i;
            }
            i = 0 as libc::c_int;
            while i < (*cstate).numitems {
                let mut cte_0: *mut CommonTableExpr = (*((*cstate).items).offset(i as isize)).cte;
                if strcmp((*rv).relname, (*cte_0).ctename) == 0 as libc::c_int {
                    let mut myindex: libc::c_int = (*cstate).curitem;
                    if i != myindex {
                        let ref mut fresh1 =
                            (*((*cstate).items).offset(myindex as isize)).depends_on;
                        *fresh1 = bms_add_member(
                            (*((*cstate).items).offset(myindex as isize)).depends_on,
                            (*((*cstate).items).offset(i as isize)).id,
                        );
                    } else {
                        (*cte_0).cterecursive = true;
                    }
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_SelectStmt as libc::c_int as libc::c_uint {
        let mut stmt: *mut SelectStmt = node as *mut SelectStmt;
        let mut lc_0: *mut ListCell = 0 as *mut ListCell;
        if !((*stmt).withClause).is_null() {
            if (*(*stmt).withClause).recursive {
                (*cstate).innerwiths = lcons(
                    (*(*stmt).withClause).ctes as *mut libc::c_void,
                    (*cstate).innerwiths,
                );
                let mut lc__state_0: ForEachState = {
                    let mut init = ForEachState {
                        l: (*(*stmt).withClause).ctes,
                        i: 0 as libc::c_int,
                    };
                    init
                };
                while if !(lc__state_0.l).is_null() && lc__state_0.i < (*lc__state_0.l).length {
                    lc_0 = &mut *((*lc__state_0.l).elements).offset(lc__state_0.i as isize)
                        as *mut ListCell;
                    true as libc::c_int
                } else {
                    lc_0 = 0 as *mut ListCell;
                    false as libc::c_int
                } != 0
                {
                    let mut cte_1: *mut CommonTableExpr = (*lc_0).ptr_value as *mut CommonTableExpr;
                    makeDependencyGraphWalker((*cte_1).ctequery, cstate);
                    lc__state_0.i += 1;
                    lc__state_0.i;
                }
                raw_expression_tree_walker(
                    node,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut Node, *mut CteState) -> bool>,
                        Option<unsafe extern "C" fn() -> bool>,
                    >(Some(
                        makeDependencyGraphWalker
                            as unsafe extern "C" fn(*mut Node, *mut CteState) -> bool,
                    )),
                    cstate as *mut libc::c_void,
                );
                (*cstate).innerwiths = list_delete_first((*cstate).innerwiths);
            } else {
                (*cstate).innerwiths = lcons(
                    0 as *mut libc::c_void as *mut List as *mut libc::c_void,
                    (*cstate).innerwiths,
                );
                let mut lc__state_1: ForEachState = {
                    let mut init = ForEachState {
                        l: (*(*stmt).withClause).ctes,
                        i: 0 as libc::c_int,
                    };
                    init
                };
                while if !(lc__state_1.l).is_null() && lc__state_1.i < (*lc__state_1.l).length {
                    lc_0 = &mut *((*lc__state_1.l).elements).offset(lc__state_1.i as isize)
                        as *mut ListCell;
                    true as libc::c_int
                } else {
                    lc_0 = 0 as *mut ListCell;
                    false as libc::c_int
                } != 0
                {
                    let mut cte_2: *mut CommonTableExpr = (*lc_0).ptr_value as *mut CommonTableExpr;
                    let mut cell1: *mut ListCell = 0 as *mut ListCell;
                    makeDependencyGraphWalker((*cte_2).ctequery, cstate);
                    cell1 = list_head((*cstate).innerwiths);
                    (*cell1).ptr_value =
                        lappend((*cell1).ptr_value as *mut List, cte_2 as *mut libc::c_void)
                            as *mut libc::c_void;
                    lc__state_1.i += 1;
                    lc__state_1.i;
                }
                raw_expression_tree_walker(
                    node,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut Node, *mut CteState) -> bool>,
                        Option<unsafe extern "C" fn() -> bool>,
                    >(Some(
                        makeDependencyGraphWalker
                            as unsafe extern "C" fn(*mut Node, *mut CteState) -> bool,
                    )),
                    cstate as *mut libc::c_void,
                );
                (*cstate).innerwiths = list_delete_first((*cstate).innerwiths);
            }
            return false;
        }
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_WithClause as libc::c_int as libc::c_uint {
        return false;
    }
    return raw_expression_tree_walker(
        node,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Node, *mut CteState) -> bool>,
            Option<unsafe extern "C" fn() -> bool>,
        >(Some(
            makeDependencyGraphWalker as unsafe extern "C" fn(*mut Node, *mut CteState) -> bool,
        )),
        cstate as *mut libc::c_void,
    );
}
unsafe extern "C" fn TopologicalSort(
    mut pstate: *mut ParseState,
    mut items: *mut CteItem,
    mut numitems: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < numitems {
        j = i;
        while j < numitems {
            if bms_is_empty((*items.offset(j as isize)).depends_on) != 0 {
                break;
            }
            j += 1;
            j;
        }
        if j >= numitems {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        if i != j {
            let mut tmp: CteItem = CteItem {
                cte: 0 as *mut CommonTableExpr,
                id: 0,
                depends_on: 0 as *mut Bitmapset,
            };
            tmp = *items.offset(i as isize);
            *items.offset(i as isize) = *items.offset(j as isize);
            *items.offset(j as isize) = tmp;
        }
        j = i + 1 as libc::c_int;
        while j < numitems {
            let ref mut fresh2 = (*items.offset(j as isize)).depends_on;
            *fresh2 = bms_del_member(
                (*items.offset(j as isize)).depends_on,
                (*items.offset(i as isize)).id,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn checkWellFormedRecursion(mut cstate: *mut CteState) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*cstate).numitems {
        let mut cte: *mut CommonTableExpr = (*((*cstate).items).offset(i as isize)).cte;
        let mut stmt: *mut SelectStmt = (*cte).ctequery as *mut SelectStmt;
        if !((*cte).cterecursive == 0) {
            if !((*(stmt as *const Node)).tag as libc::c_uint
                == T_SelectStmt as libc::c_int as libc::c_uint)
            {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            if (*stmt).op as libc::c_uint != SETOP_UNION as libc::c_int as libc::c_uint {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            (*cstate).curitem = i;
            (*cstate).innerwiths = 0 as *mut libc::c_void as *mut List;
            (*cstate).selfrefcount = 0 as libc::c_int;
            (*cstate).context = RECURSION_NONRECURSIVETERM;
            checkWellFormedRecursionWalker((*stmt).larg as *mut Node, cstate);
            (*cstate).curitem = i;
            (*cstate).innerwiths = 0 as *mut libc::c_void as *mut List;
            (*cstate).selfrefcount = 0 as libc::c_int;
            (*cstate).context = RECURSION_OK;
            checkWellFormedRecursionWalker((*stmt).rarg as *mut Node, cstate);
            if (*cstate).selfrefcount != 1 as libc::c_int {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if errstart(elevel__1, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"missing recursive reference\0" as *const u8 as *const libc::c_char,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_cte.c\0"
                            as *const u8 as *const libc::c_char,
                        871 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
            if !((*stmt).withClause).is_null() {
                (*cstate).curitem = i;
                (*cstate).innerwiths = 0 as *mut libc::c_void as *mut List;
                (*cstate).selfrefcount = 0 as libc::c_int;
                (*cstate).context = RECURSION_SUBLINK;
                checkWellFormedRecursionWalker((*(*stmt).withClause).ctes as *mut Node, cstate);
            }
            if !((*stmt).sortClause).is_null() {
                let elevel__2: libc::c_int = 21 as libc::c_int;
                let mut __error_2: libc::c_int = 0;
                if elevel__2 >= 21 as libc::c_int {
                    abort();
                }
            }
            if !((*stmt).limitOffset).is_null() {
                let elevel__3: libc::c_int = 21 as libc::c_int;
                let mut __error_3: libc::c_int = 0;
                if elevel__3 >= 21 as libc::c_int {
                    abort();
                }
            }
            if !((*stmt).limitCount).is_null() {
                let elevel__4: libc::c_int = 21 as libc::c_int;
                let mut __error_4: libc::c_int = 0;
                if elevel__4 >= 21 as libc::c_int {
                    abort();
                }
            }
            if !((*stmt).lockingClause).is_null() {
                let elevel__5: libc::c_int = 21 as libc::c_int;
                let mut __error_5: libc::c_int = 0;
                if elevel__5 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn checkWellFormedRecursionWalker(
    mut node: *mut Node,
    mut cstate: *mut CteState,
) -> bool {
    let mut save_context: RecursionContext = (*cstate).context;
    if node.is_null() {
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_RangeVar as libc::c_int as libc::c_uint {
        let mut rv: *mut RangeVar = node as *mut RangeVar;
        if ((*rv).schemaname).is_null() {
            let mut lc: *mut ListCell = 0 as *mut ListCell;
            let mut mycte: *mut CommonTableExpr = 0 as *mut CommonTableExpr;
            let mut lc__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*cstate).innerwiths,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length {
                lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize) as *mut ListCell;
                true as libc::c_int
            } else {
                lc = 0 as *mut ListCell;
                false as libc::c_int
            } != 0
            {
                let mut withlist: *mut List = (*lc).ptr_value as *mut List;
                let mut lc2: *mut ListCell = 0 as *mut ListCell;
                let mut lc2__state: ForEachState = {
                    let mut init = ForEachState {
                        l: withlist,
                        i: 0 as libc::c_int,
                    };
                    init
                };
                while if !(lc2__state.l).is_null() && lc2__state.i < (*lc2__state.l).length {
                    lc2 = &mut *((*lc2__state.l).elements).offset(lc2__state.i as isize)
                        as *mut ListCell;
                    true as libc::c_int
                } else {
                    lc2 = 0 as *mut ListCell;
                    false as libc::c_int
                } != 0
                {
                    let mut cte: *mut CommonTableExpr = (*lc2).ptr_value as *mut CommonTableExpr;
                    if strcmp((*rv).relname, (*cte).ctename) == 0 as libc::c_int {
                        return false;
                    }
                    lc2__state.i += 1;
                    lc2__state.i;
                }
                lc__state.i += 1;
                lc__state.i;
            }
            mycte = (*((*cstate).items).offset((*cstate).curitem as isize)).cte;
            if strcmp((*rv).relname, (*mycte).ctename) == 0 as libc::c_int {
                if (*cstate).context as libc::c_uint != RECURSION_OK as libc::c_int as libc::c_uint
                {
                    let elevel_: libc::c_int = 21 as libc::c_int;
                    let mut __error: libc::c_int = 0;
                    if elevel_ >= 21 as libc::c_int {
                        abort();
                    }
                }
                (*cstate).selfrefcount += 1;
                if (*cstate).selfrefcount > 1 as libc::c_int {
                    let elevel__0: libc::c_int = 21 as libc::c_int;
                    let mut __error_0: libc::c_int = 0;
                    if elevel__0 >= 21 as libc::c_int {
                        abort();
                    }
                }
            }
        }
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_SelectStmt as libc::c_int as libc::c_uint {
        let mut stmt: *mut SelectStmt = node as *mut SelectStmt;
        let mut lc_0: *mut ListCell = 0 as *mut ListCell;
        if !((*stmt).withClause).is_null() {
            if (*(*stmt).withClause).recursive {
                (*cstate).innerwiths = lcons(
                    (*(*stmt).withClause).ctes as *mut libc::c_void,
                    (*cstate).innerwiths,
                );
                let mut lc__state_0: ForEachState = {
                    let mut init = ForEachState {
                        l: (*(*stmt).withClause).ctes,
                        i: 0 as libc::c_int,
                    };
                    init
                };
                while if !(lc__state_0.l).is_null() && lc__state_0.i < (*lc__state_0.l).length {
                    lc_0 = &mut *((*lc__state_0.l).elements).offset(lc__state_0.i as isize)
                        as *mut ListCell;
                    true as libc::c_int
                } else {
                    lc_0 = 0 as *mut ListCell;
                    false as libc::c_int
                } != 0
                {
                    let mut cte_0: *mut CommonTableExpr = (*lc_0).ptr_value as *mut CommonTableExpr;
                    checkWellFormedRecursionWalker((*cte_0).ctequery, cstate);
                    lc__state_0.i += 1;
                    lc__state_0.i;
                }
                checkWellFormedSelectStmt(stmt, cstate);
                (*cstate).innerwiths = list_delete_first((*cstate).innerwiths);
            } else {
                (*cstate).innerwiths = lcons(
                    0 as *mut libc::c_void as *mut List as *mut libc::c_void,
                    (*cstate).innerwiths,
                );
                let mut lc__state_1: ForEachState = {
                    let mut init = ForEachState {
                        l: (*(*stmt).withClause).ctes,
                        i: 0 as libc::c_int,
                    };
                    init
                };
                while if !(lc__state_1.l).is_null() && lc__state_1.i < (*lc__state_1.l).length {
                    lc_0 = &mut *((*lc__state_1.l).elements).offset(lc__state_1.i as isize)
                        as *mut ListCell;
                    true as libc::c_int
                } else {
                    lc_0 = 0 as *mut ListCell;
                    false as libc::c_int
                } != 0
                {
                    let mut cte_1: *mut CommonTableExpr = (*lc_0).ptr_value as *mut CommonTableExpr;
                    let mut cell1: *mut ListCell = 0 as *mut ListCell;
                    checkWellFormedRecursionWalker((*cte_1).ctequery, cstate);
                    cell1 = list_head((*cstate).innerwiths);
                    (*cell1).ptr_value =
                        lappend((*cell1).ptr_value as *mut List, cte_1 as *mut libc::c_void)
                            as *mut libc::c_void;
                    lc__state_1.i += 1;
                    lc__state_1.i;
                }
                checkWellFormedSelectStmt(stmt, cstate);
                (*cstate).innerwiths = list_delete_first((*cstate).innerwiths);
            }
        } else {
            checkWellFormedSelectStmt(stmt, cstate);
        }
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_WithClause as libc::c_int as libc::c_uint {
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_JoinExpr as libc::c_int as libc::c_uint {
        let mut j: *mut JoinExpr = node as *mut JoinExpr;
        match (*j).jointype as libc::c_uint {
            0 => {
                checkWellFormedRecursionWalker((*j).larg, cstate);
                checkWellFormedRecursionWalker((*j).rarg, cstate);
                checkWellFormedRecursionWalker((*j).quals, cstate);
            }
            1 => {
                checkWellFormedRecursionWalker((*j).larg, cstate);
                if save_context as libc::c_uint == RECURSION_OK as libc::c_int as libc::c_uint {
                    (*cstate).context = RECURSION_OUTERJOIN;
                }
                checkWellFormedRecursionWalker((*j).rarg, cstate);
                (*cstate).context = save_context;
                checkWellFormedRecursionWalker((*j).quals, cstate);
            }
            2 => {
                if save_context as libc::c_uint == RECURSION_OK as libc::c_int as libc::c_uint {
                    (*cstate).context = RECURSION_OUTERJOIN;
                }
                checkWellFormedRecursionWalker((*j).larg, cstate);
                checkWellFormedRecursionWalker((*j).rarg, cstate);
                (*cstate).context = save_context;
                checkWellFormedRecursionWalker((*j).quals, cstate);
            }
            3 => {
                if save_context as libc::c_uint == RECURSION_OK as libc::c_int as libc::c_uint {
                    (*cstate).context = RECURSION_OUTERJOIN;
                }
                checkWellFormedRecursionWalker((*j).larg, cstate);
                (*cstate).context = save_context;
                checkWellFormedRecursionWalker((*j).rarg, cstate);
                checkWellFormedRecursionWalker((*j).quals, cstate);
            }
            _ => {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if errstart(elevel__1, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"unrecognized join type: %d\0" as *const u8 as *const libc::c_char,
                        (*j).jointype as libc::c_int,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_cte.c\0"
                            as *const u8 as *const libc::c_char,
                        1075 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_SubLink as libc::c_int as libc::c_uint {
        let mut sl: *mut SubLink = node as *mut SubLink;
        (*cstate).context = RECURSION_SUBLINK;
        checkWellFormedRecursionWalker((*sl).subselect, cstate);
        (*cstate).context = save_context;
        checkWellFormedRecursionWalker((*sl).testexpr, cstate);
        return false;
    }
    return raw_expression_tree_walker(
        node,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Node, *mut CteState) -> bool>,
            Option<unsafe extern "C" fn() -> bool>,
        >(Some(
            checkWellFormedRecursionWalker
                as unsafe extern "C" fn(*mut Node, *mut CteState) -> bool,
        )),
        cstate as *mut libc::c_void,
    );
}
unsafe extern "C" fn checkWellFormedSelectStmt(
    mut stmt: *mut SelectStmt,
    mut cstate: *mut CteState,
) {
    let mut save_context: RecursionContext = (*cstate).context;
    if save_context as libc::c_uint != RECURSION_OK as libc::c_int as libc::c_uint {
        raw_expression_tree_walker(
            stmt as *mut Node,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Node, *mut CteState) -> bool>,
                Option<unsafe extern "C" fn() -> bool>,
            >(Some(
                checkWellFormedRecursionWalker
                    as unsafe extern "C" fn(*mut Node, *mut CteState) -> bool,
            )),
            cstate as *mut libc::c_void,
        );
    } else {
        match (*stmt).op as libc::c_uint {
            0 | 1 => {
                raw_expression_tree_walker(
                    stmt as *mut Node,
                    ::core::mem::transmute::<
                        Option<unsafe extern "C" fn(*mut Node, *mut CteState) -> bool>,
                        Option<unsafe extern "C" fn() -> bool>,
                    >(Some(
                        checkWellFormedRecursionWalker
                            as unsafe extern "C" fn(*mut Node, *mut CteState) -> bool,
                    )),
                    cstate as *mut libc::c_void,
                );
            }
            2 => {
                if (*stmt).all {
                    (*cstate).context = RECURSION_INTERSECT;
                }
                checkWellFormedRecursionWalker((*stmt).larg as *mut Node, cstate);
                checkWellFormedRecursionWalker((*stmt).rarg as *mut Node, cstate);
                (*cstate).context = save_context;
                checkWellFormedRecursionWalker((*stmt).sortClause as *mut Node, cstate);
                checkWellFormedRecursionWalker((*stmt).limitOffset, cstate);
                checkWellFormedRecursionWalker((*stmt).limitCount, cstate);
                checkWellFormedRecursionWalker((*stmt).lockingClause as *mut Node, cstate);
            }
            3 => {
                if (*stmt).all {
                    (*cstate).context = RECURSION_EXCEPT;
                }
                checkWellFormedRecursionWalker((*stmt).larg as *mut Node, cstate);
                (*cstate).context = RECURSION_EXCEPT;
                checkWellFormedRecursionWalker((*stmt).rarg as *mut Node, cstate);
                (*cstate).context = save_context;
                checkWellFormedRecursionWalker((*stmt).sortClause as *mut Node, cstate);
                checkWellFormedRecursionWalker((*stmt).limitOffset, cstate);
                checkWellFormedRecursionWalker((*stmt).limitCount, cstate);
                checkWellFormedRecursionWalker((*stmt).lockingClause as *mut Node, cstate);
            }
            _ => {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"unrecognized set op: %d\0" as *const u8 as *const libc::c_char,
                        (*stmt).op as libc::c_int,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_cte.c\0"
                            as *const u8 as *const libc::c_char,
                        1163 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
        }
    };
}
