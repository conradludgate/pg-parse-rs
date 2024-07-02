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
//     fn repalloc(pointer: *mut libc::c_void, size: usize) -> *mut libc::c_void;
//     fn palloc(size: usize) -> *mut libc::c_void;
//     fn memset(
//         _: *mut libc::c_void,
//         _: libc::c_int,
//         _: libc::c_ulong,
//     ) -> *mut libc::c_void;
//     fn abort() -> !;
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
//     fn get_typcollation(typid: Oid) -> Oid;
// }
use super::*;
// pub type Oid = libc::c_uint;
pub type __darwin_size_t = libc::c_ulong;
// pub type isize = __darwin_size_t;
// pub type bool = libc::c_uchar;
// pub type i16 = libc::c_short;
// pub type i32 = libc::c_int;
// pub type u32 = libc::c_uint;
// pub type uint64 = libc::c_ulong;
// pub type usize = isize;
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
pub struct Alias {
    pub type_0: NodeTag,
    pub aliasname: *mut libc::c_char,
    pub colnames: *mut List,
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
pub struct FixedParamState {
    pub paramTypes: *mut Oid,
    pub numParams: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VarParamState {
    pub paramTypes: *mut *mut Oid,
    pub numParams: *mut libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn parse_fixed_parameters(
    mut pstate: *mut ParseState,
    mut paramTypes: *mut Oid,
    mut numParams: libc::c_int,
) {
    let mut parstate: *mut FixedParamState =
        palloc(::core::mem::size_of::<FixedParamState>() as libc::c_ulong) as *mut FixedParamState;
    (*parstate).paramTypes = paramTypes;
    (*parstate).numParams = numParams;
    (*pstate).p_ref_hook_state = parstate as *mut libc::c_void;
    (*pstate).p_paramref_hook = Some(
        fixed_paramref_hook as unsafe extern "C" fn(*mut ParseState, *mut ParamRef) -> *mut Node,
    );
}
#[no_mangle]
pub unsafe extern "C" fn parse_variable_parameters(
    mut pstate: *mut ParseState,
    mut paramTypes: *mut *mut Oid,
    mut numParams: *mut libc::c_int,
) {
    let mut parstate: *mut VarParamState =
        palloc(::core::mem::size_of::<VarParamState>() as libc::c_ulong) as *mut VarParamState;
    (*parstate).paramTypes = paramTypes;
    (*parstate).numParams = numParams;
    (*pstate).p_ref_hook_state = parstate as *mut libc::c_void;
    (*pstate).p_paramref_hook = Some(
        variable_paramref_hook as unsafe extern "C" fn(*mut ParseState, *mut ParamRef) -> *mut Node,
    );
    (*pstate).p_coerce_param_hook = Some(
        variable_coerce_param_hook
            as unsafe extern "C" fn(
                *mut ParseState,
                *mut Param,
                Oid,
                i32,
                libc::c_int,
            ) -> *mut Node,
    );
}
unsafe extern "C" fn fixed_paramref_hook(
    mut pstate: *mut ParseState,
    mut pref: *mut ParamRef,
) -> *mut Node {
    let mut parstate: *mut FixedParamState = (*pstate).p_ref_hook_state as *mut FixedParamState;
    let mut paramno: libc::c_int = (*pref).number;
    let mut param: *mut Param = 0 as *mut Param;
    if paramno <= 0 as libc::c_int
        || paramno > (*parstate).numParams
        || (*((*parstate).paramTypes).offset((paramno - 1 as libc::c_int) as isize)
            != 0 as libc::c_int as Oid) as libc::c_int as bool
            == 0
    {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    param = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_Param;
        _result
    }) as *mut Param;
    (*param).paramkind = PARAM_EXTERN;
    (*param).paramid = paramno;
    (*param).paramtype = *((*parstate).paramTypes).offset((paramno - 1 as libc::c_int) as isize);
    (*param).paramtypmod = -(1 as libc::c_int);
    (*param).paramcollid = get_typcollation((*param).paramtype);
    (*param).location = (*pref).location;
    return param as *mut Node;
}
#[no_mangle]
pub unsafe extern "C" fn check_variable_parameters(
    mut pstate: *mut ParseState,
    mut query: *mut Query,
) {
    let mut parstate: *mut VarParamState = (*pstate).p_ref_hook_state as *mut VarParamState;
    if *(*parstate).numParams > 0 as libc::c_int {
        query_tree_walker(
            query,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Node, *mut ParseState) -> bool>,
                Option<unsafe extern "C" fn() -> bool>,
            >(Some(
                check_parameter_resolution_walker
                    as unsafe extern "C" fn(*mut Node, *mut ParseState) -> bool,
            )),
            pstate as *mut libc::c_void,
            0 as libc::c_int,
        );
    }
}
unsafe extern "C" fn check_parameter_resolution_walker(
    mut node: *mut Node,
    mut pstate: *mut ParseState,
) -> bool {
    if node.is_null() {
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_Param as libc::c_int as libc::c_uint {
        let mut param: *mut Param = node as *mut Param;
        if (*param).paramkind as libc::c_uint == PARAM_EXTERN as libc::c_int as libc::c_uint {
            let mut parstate: *mut VarParamState = (*pstate).p_ref_hook_state as *mut VarParamState;
            let mut paramno: libc::c_int = (*param).paramid;
            if paramno <= 0 as libc::c_int || paramno > *(*parstate).numParams {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            if (*param).paramtype
                != *(*(*parstate).paramTypes).offset((paramno - 1 as libc::c_int) as isize)
            {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_Query as libc::c_int as libc::c_uint {
        return query_tree_walker(
            node as *mut Query,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Node, *mut ParseState) -> bool>,
                Option<unsafe extern "C" fn() -> bool>,
            >(Some(
                check_parameter_resolution_walker
                    as unsafe extern "C" fn(*mut Node, *mut ParseState) -> bool,
            )),
            pstate as *mut libc::c_void,
            0 as libc::c_int,
        );
    }
    return expression_tree_walker(
        node,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Node, *mut ParseState) -> bool>,
            Option<unsafe extern "C" fn() -> bool>,
        >(Some(
            check_parameter_resolution_walker
                as unsafe extern "C" fn(*mut Node, *mut ParseState) -> bool,
        )),
        pstate as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn query_contains_extern_params(mut query: *mut Query) -> bool {
    return query_tree_walker(
        query,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Node, *mut libc::c_void) -> bool>,
            Option<unsafe extern "C" fn() -> bool>,
        >(Some(
            query_contains_extern_params_walker
                as unsafe extern "C" fn(*mut Node, *mut libc::c_void) -> bool,
        )),
        0 as *mut libc::c_void,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn query_contains_extern_params_walker(
    mut node: *mut Node,
    mut context: *mut libc::c_void,
) -> bool {
    if node.is_null() {
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_Param as libc::c_int as libc::c_uint {
        let mut param: *mut Param = node as *mut Param;
        if (*param).paramkind as libc::c_uint == PARAM_EXTERN as libc::c_int as libc::c_uint {
            return true;
        }
        return false;
    }
    if (*(node as *const Node)).tag as libc::c_uint == T_Query as libc::c_int as libc::c_uint {
        return query_tree_walker(
            node as *mut Query,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut Node, *mut libc::c_void) -> bool>,
                Option<unsafe extern "C" fn() -> bool>,
            >(Some(
                query_contains_extern_params_walker
                    as unsafe extern "C" fn(*mut Node, *mut libc::c_void) -> bool,
            )),
            context,
            0 as libc::c_int,
        );
    }
    return expression_tree_walker(
        node,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Node, *mut libc::c_void) -> bool>,
            Option<unsafe extern "C" fn() -> bool>,
        >(Some(
            query_contains_extern_params_walker
                as unsafe extern "C" fn(*mut Node, *mut libc::c_void) -> bool,
        )),
        context,
    );
}
