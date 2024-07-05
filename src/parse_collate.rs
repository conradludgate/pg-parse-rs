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
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
//     fn lappend_oid(list: *mut List, datum: Oid) -> *mut List;
//     fn makeRelabelType(
//         arg: *mut Expr,
//         rtype: Oid,
//         rtypmod: i32,
//         rcollid: Oid,
//         rformat: CoercionForm,
//     ) -> *mut RelabelType;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprTypmod(expr: *const Node) -> i32;
//     fn exprCollation(expr: *const Node) -> Oid;
//     fn exprSetCollation(expr: *mut Node, collation: Oid);
//     fn exprSetInputCollation(expr: *mut Node, inputcollation: Oid);
//     fn exprLocation(expr: *const Node) -> libc::c_int;
//     fn expression_tree_walker(
//         node: *mut Node,
//         walker: Option::<unsafe extern "C" fn() -> bool>,
//         context: *mut libc::c_void,
//     ) -> bool;
//     fn query_tree_walker(
//         query: *mut Query,
//         walker: Option::<unsafe extern "C" fn() -> bool>,
//         context: *mut libc::c_void,
//         flags: libc::c_int,
//     ) -> bool;
//     fn get_func_variadictype(funcid: Oid) -> Oid;
//     fn get_typcollation(typid: Oid) -> Oid;
// }
use super::*;
// pub type Oid = libc::c_uint;
// pub type bool = libc::c_uchar;
// pub type i16 = libc::c_short;
// pub type i32 = libc::c_int;
// pub type u32 = libc::c_uint;
// pub type uint64 = libc::c_ulong;
// pub type Index = libc::c_uint;
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
pub type AggSplit = libc::c_uint;
pub const AGGSPLIT_FINAL_DESERIAL: AggSplit = 9;
pub const AGGSPLIT_INITIAL_SERIAL: AggSplit = 6;
pub const AGGSPLIT_SIMPLE: AggSplit = 0;
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
pub struct ForBothState {
    pub l1: *const List,
    pub l2: *const List,
    pub i: libc::c_int,
}
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
pub struct Aggref {
    pub xpr: Expr,
    pub aggfnoid: Oid,
    pub aggtype: Oid,
    pub aggcollid: Oid,
    pub inputcollid: Oid,
    pub aggtranstype: Oid,
    pub aggargtypes: *mut List,
    pub aggdirectargs: *mut List,
    pub args: *mut List,
    pub aggorder: *mut List,
    pub aggdistinct: *mut List,
    pub aggfilter: *mut Expr,
    pub aggstar: bool,
    pub aggvariadic: bool,
    pub aggkind: libc::c_char,
    pub agglevelsup: Index,
    pub aggsplit: AggSplit,
    pub aggno: libc::c_int,
    pub aggtransno: libc::c_int,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WindowFunc {
    pub xpr: Expr,
    pub winfnoid: Oid,
    pub wintype: Oid,
    pub wincollid: Oid,
    pub inputcollid: Oid,
    pub args: *mut List,
    pub aggfilter: *mut Expr,
    pub winref: Index,
    pub winstar: bool,
    pub winagg: bool,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubscriptingRef {
    pub xpr: Expr,
    pub refcontainertype: Oid,
    pub refelemtype: Oid,
    pub refrestype: Oid,
    pub reftypmod: i32,
    pub refcollid: Oid,
    pub refupperindexpr: *mut List,
    pub reflowerindexpr: *mut List,
    pub refexpr: *mut Expr,
    pub refassgnexpr: *mut Expr,
}
// pub type CoercionForm = libc::c_uint;
pub const COERCE_SQL_SYNTAX: CoercionForm = 3;
pub const COERCE_IMPLICIT_CAST: CoercionForm = 2;
pub const COERCE_EXPLICIT_CAST: CoercionForm = 1;
pub const COERCE_EXPLICIT_CALL: CoercionForm = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FieldSelect {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub fieldnum: AttrNumber,
    pub resulttype: Oid,
    pub resulttypmod: i32,
    pub resultcollid: Oid,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct RelabelType {
//     pub xpr: Expr,
//     pub arg: *mut Expr,
//     pub resulttype: Oid,
//     pub resulttypmod: i32,
//     pub resultcollid: Oid,
//     pub relabelformat: CoercionForm,
//     pub location: libc::c_int,
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CollateExpr {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub collOid: Oid,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaseExpr {
    pub xpr: Expr,
    pub casetype: Oid,
    pub casecollid: Oid,
    pub arg: *mut Expr,
    pub args: *mut List,
    pub defresult: *mut Expr,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaseWhen {
    pub xpr: Expr,
    pub expr: *mut Expr,
    pub result: *mut Expr,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RowExpr {
    pub xpr: Expr,
    pub args: *mut List,
    pub row_typeid: Oid,
    pub row_format: CoercionForm,
    pub colnames: *mut List,
    pub location: libc::c_int,
}
pub type RowCompareType = libc::c_uint;
pub const ROWCOMPARE_NE: RowCompareType = 6;
pub const ROWCOMPARE_GT: RowCompareType = 5;
pub const ROWCOMPARE_GE: RowCompareType = 4;
pub const ROWCOMPARE_EQ: RowCompareType = 3;
pub const ROWCOMPARE_LE: RowCompareType = 2;
pub const ROWCOMPARE_LT: RowCompareType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RowCompareExpr {
    pub xpr: Expr,
    pub rctype: RowCompareType,
    pub opnos: *mut List,
    pub opfamilies: *mut List,
    pub inputcollids: *mut List,
    pub largs: *mut List,
    pub rargs: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoerceToDomain {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub resulttype: Oid,
    pub resulttypmod: i32,
    pub resultcollid: Oid,
    pub coercionformat: CoercionForm,
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
pub struct assign_collations_context {
    pub pstate: *mut ParseState,
    pub collation: Oid,
    pub strength: CollateStrength,
    pub location: libc::c_int,
    pub collation2: Oid,
    pub location2: libc::c_int,
}
pub type CollateStrength = libc::c_uint;
pub const COLLATE_EXPLICIT: CollateStrength = 3;
pub const COLLATE_CONFLICT: CollateStrength = 2;
pub const COLLATE_IMPLICIT: CollateStrength = 1;
pub const COLLATE_NONE: CollateStrength = 0;
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
#[no_mangle]
pub unsafe extern "C" fn assign_query_collations(
    mut pstate: *mut ParseState,
    mut query: *mut Query,
) {
    query_tree_walker(
        query,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Node, *mut ParseState) -> bool>,
            Option<unsafe extern "C" fn() -> bool>,
        >(Some(
            assign_query_collations_walker
                as unsafe extern "C" fn(*mut Node, *mut ParseState) -> bool,
        )),
        pstate as *mut libc::c_void,
        0x8 as libc::c_int | 0x2 as libc::c_int,
    );
}
unsafe extern "C" fn assign_query_collations_walker(
    mut node: *mut Node,
    mut pstate: *mut ParseState,
) -> bool {
    if node.is_null() {
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint
        == T_SetOperationStmt as libc::c_int as libc::c_uint
    {
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_List as libc::c_int as libc::c_uint {
        assign_list_collations(pstate, node as *mut List);
    } else {
        assign_expr_collations(pstate, node);
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn assign_list_collations(mut pstate: *mut ParseState, mut exprs: *mut List) {
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: exprs,
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
        let mut node: *mut Node = (*lc).ptr_value as *mut Node;
        assign_expr_collations(pstate, node);
        lc__state.i += 1;
        lc__state.i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn assign_expr_collations(mut pstate: *mut ParseState, mut expr: *mut Node) {
    let mut context: assign_collations_context = assign_collations_context {
        pstate: 0 as *mut ParseState,
        collation: 0,
        strength: COLLATE_NONE,
        location: 0,
        collation2: 0,
        location2: 0,
    };
    context.pstate = pstate;
    context.collation = 0 as libc::c_int as Oid;
    context.strength = COLLATE_NONE;
    context.location = -(1 as libc::c_int);
    assign_collations_walker(expr, &mut context);
}
#[no_mangle]
pub unsafe extern "C" fn select_common_collation(
    mut pstate: *mut ParseState,
    mut exprs: *mut List,
    mut none_ok: bool,
) -> Oid {
    let mut context: assign_collations_context = assign_collations_context {
        pstate: 0 as *mut ParseState,
        collation: 0,
        strength: COLLATE_NONE,
        location: 0,
        collation2: 0,
        location2: 0,
    };
    context.pstate = pstate;
    context.collation = 0 as libc::c_int as Oid;
    context.strength = COLLATE_NONE;
    context.location = -(1 as libc::c_int);
    assign_collations_walker(exprs as *mut Node, &mut context);
    if context.strength as libc::c_uint == COLLATE_CONFLICT as libc::c_int as libc::c_uint {
        if none_ok {
            return 0 as libc::c_int as Oid;
        }
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return context.collation;
}
unsafe extern "C" fn assign_aggregate_collations(
    mut aggref: *mut Aggref,
    mut loccontext: *mut assign_collations_context,
) {
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*aggref).args,
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
        let mut tle: *mut TargetEntry = (*lc).ptr_value as *mut TargetEntry;
        if (*tle).resjunk {
            assign_expr_collations((*loccontext).pstate, tle as *mut Node);
        } else {
            assign_collations_walker(tle as *mut Node, loccontext);
        }
        lc__state.i += 1;
        lc__state.i;
    }
}
unsafe extern "C" fn assign_ordered_set_collations(
    mut aggref: *mut Aggref,
    mut loccontext: *mut assign_collations_context,
) {
    let mut merge_sort_collations: bool = false;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    merge_sort_collations = (list_length((*aggref).args) == 1 as libc::c_int
        && get_func_variadictype((*aggref).aggfnoid) == 0 as libc::c_int as Oid)
        as libc::c_int as bool;
    assign_collations_walker((*aggref).aggdirectargs as *mut Node, loccontext);
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*aggref).args,
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
        let mut tle: *mut TargetEntry = (*lc).ptr_value as *mut TargetEntry;
        if merge_sort_collations {
            assign_collations_walker(tle as *mut Node, loccontext);
        } else {
            assign_expr_collations((*loccontext).pstate, tle as *mut Node);
        }
        lc__state.i += 1;
        lc__state.i;
    }
}
unsafe extern "C" fn assign_hypothetical_collations(
    mut aggref: *mut Aggref,
    mut loccontext: *mut assign_collations_context,
) {
    let mut h_cell: *mut ListCell = list_head((*aggref).aggdirectargs);
    let mut s_cell: *mut ListCell = list_head((*aggref).args);
    let mut merge_sort_collations: bool = false;
    let mut extra_args: libc::c_int = 0;
    merge_sort_collations = (list_length((*aggref).args) == 1 as libc::c_int
        && get_func_variadictype((*aggref).aggfnoid) == 0 as libc::c_int as Oid)
        as libc::c_int as bool;
    extra_args = list_length((*aggref).aggdirectargs) - list_length((*aggref).args);
    loop {
        let fresh0 = extra_args;
        extra_args = extra_args - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        assign_collations_walker((*h_cell).ptr_value as *mut Node, loccontext);
        h_cell = lnext((*aggref).aggdirectargs, h_cell);
    }
    while !h_cell.is_null() && !s_cell.is_null() {
        let mut h_arg: *mut Node = (*h_cell).ptr_value as *mut Node;
        let mut s_tle: *mut TargetEntry = (*s_cell).ptr_value as *mut TargetEntry;
        let mut paircontext: assign_collations_context = assign_collations_context {
            pstate: 0 as *mut ParseState,
            collation: 0,
            strength: COLLATE_NONE,
            location: 0,
            collation2: 0,
            location2: 0,
        };
        paircontext.pstate = (*loccontext).pstate;
        paircontext.collation = 0 as libc::c_int as Oid;
        paircontext.strength = COLLATE_NONE;
        paircontext.location = -(1 as libc::c_int);
        paircontext.collation2 = 0 as libc::c_int as Oid;
        paircontext.location2 = -(1 as libc::c_int);
        assign_collations_walker(h_arg, &mut paircontext);
        assign_collations_walker((*s_tle).expr as *mut Node, &mut paircontext);
        if paircontext.strength as libc::c_uint == COLLATE_CONFLICT as libc::c_int as libc::c_uint {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        if (paircontext.collation != 0 as libc::c_int as Oid) as libc::c_int as bool as libc::c_int
            != 0
            && paircontext.collation != exprCollation((*s_tle).expr as *mut Node)
        {
            (*s_tle).expr = makeRelabelType(
                (*s_tle).expr,
                exprType((*s_tle).expr as *mut Node),
                exprTypmod((*s_tle).expr as *mut Node),
                paircontext.collation,
                COERCE_IMPLICIT_CAST,
            ) as *mut Expr;
        }
        if merge_sort_collations {
            merge_collation_state(
                paircontext.collation,
                paircontext.strength,
                paircontext.location,
                paircontext.collation2,
                paircontext.location2,
                loccontext,
            );
        }
        h_cell = lnext((*aggref).aggdirectargs, h_cell);
        s_cell = lnext((*aggref).args, s_cell);
    }
}
