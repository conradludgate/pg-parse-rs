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
//     pub type SubscriptingRefState;
//     pub type SubscriptExecSteps;
//     fn abort() -> !;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn errposition(cursorpos: libc::c_int) -> libc::c_int;
//     static mut error_context_stack: *mut ErrorContextCallback;
//     fn palloc0(size: usize) -> *mut libc::c_void;
//     fn pfree(pointer: *mut libc::c_void);
//     fn i64GetDatum(X: i64) -> Datum;
//     fn table_close(relation: Relation, lockmode: LOCKMODE);
//     fn pg_mbstrlen_with_len(mbstr: *const libc::c_char, len: libc::c_int) -> libc::c_int;
//     fn makeConst(
//         consttype: Oid,
//         consttypmod: i32,
//         constcollid: Oid,
//         constlen: libc::c_int,
//         constvalue: Datum,
//         constisnull: bool,
//         constbyval: bool,
//     ) -> *mut Const;
//     fn scanint8(str: *const libc::c_char, errorOK: bool, result: *mut i64) -> bool;
//     fn getSubscriptingRoutines(
//         typid: Oid,
//         typelemp: *mut Oid,
//     ) -> *const SubscriptRoutines;
//     fn getBaseTypeAndTypmod(typid: Oid, typmod: *mut i32) -> Oid;
// }
use super::*;
// pub type Oid = libc::c_uint;
pub type __darwin_size_t = libc::c_ulong;
// pub type usize = libc::c_ulong;
// pub type isize = __darwin_size_t;
// pub type bool = libc::c_uchar;
// pub type i16 = libc::c_short;
// pub type i32 = libc::c_int;
// pub type u32 = libc::c_uint;
// pub type i64 = libc::c_long;
// pub type uint64 = libc::c_ulong;
// pub type usize = isize;
// pub type Index = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ErrorContextCallback {
    pub previous: *mut ErrorContextCallback,
    pub callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
// pub type Datum = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Const {
    pub xpr: Expr,
    pub consttype: Oid,
    pub consttypmod: i32,
    pub constcollid: Oid,
    pub constlen: libc::c_int,
    pub constvalue: Datum,
    pub constisnull: bool,
    pub constbyval: bool,
    pub location: libc::c_int,
}
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
// pub type Relation = *mut RelationData;
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
pub struct A_Indices {
    pub type_0: NodeTag,
    pub is_slice: bool,
    pub lidx: *mut Node,
    pub uidx: *mut Node,
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
pub type SubscriptTransform = Option<
    unsafe extern "C" fn(*mut SubscriptingRef, *mut List, *mut ParseState, bool, bool) -> (),
>;
pub type SubscriptExecSetup = Option<
    unsafe extern "C" fn(
        *const SubscriptingRef,
        *mut SubscriptingRefState,
        *mut SubscriptExecSteps,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubscriptRoutines {
    pub transform: SubscriptTransform,
    pub exec_setup: SubscriptExecSetup,
    pub fetch_strict: bool,
    pub fetch_leakproof: bool,
    pub store_leakproof: bool,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct ParseCallbackState {
//     pub pstate: *mut ParseState,
//     pub location: libc::c_int,
//     pub errcallback: ErrorContextCallback,
// }
#[no_mangle]
pub unsafe extern "C" fn make_parsestate(mut parentParseState: *mut ParseState) -> *mut ParseState {
    let mut pstate: *mut ParseState = 0 as *mut ParseState;
    pstate = palloc0(::core::mem::size_of::<ParseState>() as libc::c_ulong) as *mut ParseState;
    (*pstate).parentParseState = parentParseState;
    (*pstate).p_next_resno = 1 as libc::c_int;
    (*pstate).p_resolve_unknowns = true;
    if !parentParseState.is_null() {
        (*pstate).p_sourcetext = (*parentParseState).p_sourcetext;
        (*pstate).p_pre_columnref_hook = (*parentParseState).p_pre_columnref_hook;
        (*pstate).p_post_columnref_hook = (*parentParseState).p_post_columnref_hook;
        (*pstate).p_paramref_hook = (*parentParseState).p_paramref_hook;
        (*pstate).p_coerce_param_hook = (*parentParseState).p_coerce_param_hook;
        (*pstate).p_ref_hook_state = (*parentParseState).p_ref_hook_state;
        (*pstate).p_queryEnv = (*parentParseState).p_queryEnv;
    }
    return pstate;
}
#[no_mangle]
pub unsafe extern "C" fn free_parsestate(mut pstate: *mut ParseState) {
    if (*pstate).p_next_resno - 1 as libc::c_int > 1664 as libc::c_int {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*pstate).p_target_relation).is_null() {
        table_close((*pstate).p_target_relation, 0 as libc::c_int);
    }
    pfree(pstate as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn parser_errposition(
    mut pstate: *mut ParseState,
    mut location: libc::c_int,
) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    if location < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if pstate.is_null() || ((*pstate).p_sourcetext).is_null() {
        return 0 as libc::c_int;
    }
    pos = pg_mbstrlen_with_len((*pstate).p_sourcetext, location) + 1 as libc::c_int;
    return errposition(pos);
}
#[no_mangle]
pub unsafe extern "C" fn setup_parser_errposition_callback(
    mut pcbstate: *mut ParseCallbackState,
    mut pstate: *mut ParseState,
    mut location: libc::c_int,
) {
    (*pcbstate).pstate = pstate;
    (*pcbstate).location = location;
    (*pcbstate).errcallback.callback =
        Some(pcb_error_callback as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*pcbstate).errcallback.arg = pcbstate as *mut libc::c_void;
    (*pcbstate).errcallback.previous = error_context_stack;
    error_context_stack = &mut (*pcbstate).errcallback;
}
#[no_mangle]
pub unsafe extern "C" fn cancel_parser_errposition_callback(mut pcbstate: *mut ParseCallbackState) {
    error_context_stack = (*pcbstate).errcallback.previous;
}
#[no_mangle]
pub unsafe extern "C" fn transformContainerType(
    mut containerType: *mut Oid,
    mut containerTypmod: *mut i32,
) {
    *containerType = getBaseTypeAndTypmod(*containerType, containerTypmod);
}
#[no_mangle]
pub unsafe extern "C" fn transformContainerSubscripts(
    mut pstate: *mut ParseState,
    mut containerBase: *mut Node,
    mut containerType: Oid,
    mut containerTypMod: i32,
    mut indirection: *mut List,
    mut isAssignment: bool,
) -> *mut SubscriptingRef {
    let mut sbsref: *mut SubscriptingRef = 0 as *mut SubscriptingRef;
    let mut sbsroutines: *const SubscriptRoutines = 0 as *const SubscriptRoutines;
    let mut elementType: Oid = 0;
    let mut isSlice: bool = false;
    let mut idx: *mut ListCell = 0 as *mut ListCell;
    if isAssignment == 0 {
        transformContainerType(&mut containerType, &mut containerTypMod);
    }
    sbsroutines = getSubscriptingRoutines(containerType, &mut elementType);
    if sbsroutines.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    let mut idx__state: ForEachState = {
        let mut init = ForEachState {
            l: indirection,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(idx__state.l).is_null() && idx__state.i < (*idx__state.l).length {
        idx = &mut *((*idx__state.l).elements).offset(idx__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        idx = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut ai: *mut A_Indices = (*idx).ptr_value as *mut A_Indices;
        if (*ai).is_slice {
            isSlice = true;
            break;
        } else {
            idx__state.i += 1;
            idx__state.i;
        }
    }
    sbsref = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_SubscriptingRef;
        _result
    }) as *mut SubscriptingRef;
    (*sbsref).refcontainertype = containerType;
    (*sbsref).refelemtype = elementType;
    (*sbsref).reftypmod = containerTypMod;
    (*sbsref).refexpr = containerBase as *mut Expr;
    (*sbsref).refassgnexpr = 0 as *mut Expr;
    ((*sbsroutines).transform).expect("non-null function pointer")(
        sbsref,
        indirection,
        pstate,
        isSlice,
        isAssignment,
    );
    if ((*sbsref).refrestype != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    return sbsref;
}
#[no_mangle]
pub unsafe extern "C" fn make_const(
    mut pstate: *mut ParseState,
    mut value: *mut Value,
    mut location: libc::c_int,
) -> *mut Const {
    let mut con: *mut Const = 0 as *mut Const;
    let mut val: Datum = 0;
    let mut val64: i64 = 0;
    let mut typeid: Oid = 0;
    let mut typelen: libc::c_int = 0;
    let mut typebyval: bool = false;
    let mut pcbstate: ParseCallbackState = ParseCallbackState {
        pstate: 0 as *mut ParseState,
        location: 0,
        errcallback: ErrorContextCallback {
            previous: 0 as *mut ErrorContextCallback,
            callback: None,
            arg: 0 as *mut libc::c_void,
        },
    };
    match (*(value as *const Node)).tag as libc::c_uint {
        222 => {
            val = (*value).val.ival as Datum;
            typelen = ::core::mem::size_of::<i32>() as libc::c_ulong as libc::c_int;
            typebyval = true;
        }
        223 => {
            if scanint8((*value).val.str_0, true, &mut val64) != 0 {
                let mut val32: i32 = val64 as i32;
                if val64 == val32 as i64 {
                    val = val32 as Datum;
                    typelen = ::core::mem::size_of::<i32>() as libc::c_ulong as libc::c_int;
                    typebyval = true;
                } else {
                    val = i64GetDatum(val64);
                    typelen = ::core::mem::size_of::<int64>() as libc::c_ulong as libc::c_int;
                    typebyval = false;
                }
            } else {
                setup_parser_errposition_callback(&mut pcbstate, pstate, location);
                cancel_parser_errposition_callback(&mut pcbstate);
                typelen = -(1 as libc::c_int);
                typebyval = false;
            }
        }
        224 => {
            val = (*value).val.str_0 as Datum;
            typelen = -(2 as libc::c_int);
            typebyval = false;
        }
        225 => {
            setup_parser_errposition_callback(&mut pcbstate, pstate, location);
            cancel_parser_errposition_callback(&mut pcbstate);
            typelen = -(1 as libc::c_int);
            typebyval = false;
        }
        226 => {
            (*con).location = location;
            return con;
        }
        _ => {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unrecognized node type: %d\0" as *const u8 as *const libc::c_char,
                    (*(value as *const Node)).tag as libc::c_int,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_node.c\0"
                        as *const u8 as *const libc::c_char,
                    454 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
            return 0 as *mut Const;
        }
    }
    con = makeConst(
        typeid,
        -(1 as libc::c_int),
        0 as libc::c_int as Oid,
        typelen,
        val,
        false,
        typebyval,
    );
    (*con).location = location;
    return con;
}
