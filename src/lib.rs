#![feature(extern_types, linkage)]
extern "C" {
    pub type MemoryContextData;
    pub type AttrMissing;
    pub type dsa_area;
    pub type TIDBitmap;
    pub type PgStat_TableStatus;
    pub type FdwRoutine;
    pub type GlobalVisState;
    pub type PartitionBoundInfoData;
    pub type HTAB;
    pub type PartitionDirectoryData;
    pub type QueryEnvironment;
    pub type JitInstrumentation;
    pub type JitContext;
    pub type CopyMultiInsertBuffer;
    pub type SharedJitInstrumentation;
    pub type ExprEvalStep;
    pub type BufferAccessStrategyData;
    pub type ValidateIndexState;
    pub type VacuumParams;
    pub type BulkInsertStateData;
    pub type PartitionDescData;
    pub type PartitionKeyData;
    pub type RowSecurityDesc;
    pub type SMgrRelationData;
    fn abort() -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
    fn errfinish(
        filename: *const libc::c_char,
        lineno: libc::c_int,
        funcname: *const libc::c_char,
    );
    fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn palloc(size: Size) -> *mut libc::c_void;
    fn palloc0(size: Size) -> *mut libc::c_void;
    fn pfree(pointer: *mut libc::c_void);
    fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
    fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
    fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
    fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
    fn lappend_int(list: *mut List, datum: libc::c_int) -> *mut List;
    fn lappend_oid(list: *mut List, datum: Oid) -> *mut List;
    fn list_concat(list1: *mut List, list2: *const List) -> *mut List;
    fn list_truncate(list: *mut List, new_size: libc::c_int) -> *mut List;
    fn list_member_int(list: *const List, datum: libc::c_int) -> bool;
    fn copyObjectImpl(obj: *const libc::c_void) -> *mut libc::c_void;
    fn equal(a: *const libc::c_void, b: *const libc::c_void) -> bool;
    fn bms_is_member(x: libc::c_int, a: *const Bitmapset) -> bool;
    fn bms_add_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
    fn bms_add_members(a: *mut Bitmapset, b: *const Bitmapset) -> *mut Bitmapset;
    fn makeString(str: *mut libc::c_char) -> *mut Value;
    fn table_close(relation: Relation, lockmode: LOCKMODE);
    fn GetTsmRoutine(tsmhandler: Oid) -> *mut TsmRoutine;
    fn IsCatalogRelation(relation: Relation) -> bool;
    fn setup_parser_errposition_callback(
        pcbstate: *mut ParseCallbackState,
        pstate: *mut ParseState,
        location: libc::c_int,
    );
    fn cancel_parser_errposition_callback(pcbstate: *mut ParseCallbackState);
    fn get_relation_constraint_attnos(
        relid: Oid,
        conname: *const libc::c_char,
        missing_ok: bool,
        constraintOid: *mut Oid,
    ) -> *mut Bitmapset;
    fn check_stack_depth();
    fn makeSimpleA_Expr(
        kind: A_Expr_Kind,
        name: *mut libc::c_char,
        lexpr: *mut Node,
        rexpr: *mut Node,
        location: libc::c_int,
    ) -> *mut A_Expr;
    fn makeVar(
        varno: Index,
        varattno: AttrNumber,
        vartype: Oid,
        vartypmod: i32,
        varcollid: Oid,
        varlevelsup: Index,
    ) -> *mut Var;
    fn makeBoolExpr(
        boolop: BoolExprType,
        args: *mut List,
        location: libc::c_int,
    ) -> *mut Expr;
    fn makeRelabelType(
        arg: *mut Expr,
        rtype: Oid,
        rtypmod: i32,
        rcollid: Oid,
        rformat: CoercionForm,
    ) -> *mut RelabelType;
    fn makeFuncCall(
        name: *mut List,
        args: *mut List,
        funcformat: CoercionForm,
        location: libc::c_int,
    ) -> *mut FuncCall;
    fn makeGroupingSet(
        kind: GroupingSetKind,
        content: *mut List,
        location: libc::c_int,
    ) -> *mut GroupingSet;
    fn exprType(expr: *const Node) -> Oid;
    fn exprTypmod(expr: *const Node) -> i32;
    fn strip_implicit_coercions(node: *mut Node) -> *mut Node;
    fn exprCollation(expr: *const Node) -> Oid;
    fn exprLocation(expr: *const Node) -> libc::c_int;
    fn get_sortgroupref_tle(sortref: Index, targetList: *mut List) -> *mut TargetEntry;
    fn get_sortgroupclause_tle(
        sgClause: *mut SortGroupClause,
        targetList: *mut List,
    ) -> *mut TargetEntry;
    fn get_sortgroupclause_expr(
        sgClause: *mut SortGroupClause,
        targetList: *mut List,
    ) -> *mut Node;
    fn contain_vars_of_level(node: *mut Node, levelsup: libc::c_int) -> bool;
    fn parse_sub_analyze(
        parseTree: *mut Node,
        parentParseState: *mut ParseState,
        parentCTE: *mut CommonTableExpr,
        locked_from_parent: bool,
        resolve_unknowns: bool,
    ) -> *mut Query;
    fn can_coerce_type(
        nargs: libc::c_int,
        input_typeids: *const Oid,
        target_typeids: *const Oid,
        ccontext: CoercionContext,
    ) -> bool;
    fn coerce_type(
        pstate: *mut ParseState,
        node: *mut Node,
        inputTypeId: Oid,
        targetTypeId: Oid,
        targetTypeMod: i32,
        ccontext: CoercionContext,
        cformat: CoercionForm,
        location: libc::c_int,
    ) -> *mut Node;
    fn coerce_to_boolean(
        pstate: *mut ParseState,
        node: *mut Node,
        constructName: *const libc::c_char,
    ) -> *mut Node;
    fn coerce_to_specific_type(
        pstate: *mut ParseState,
        node: *mut Node,
        targetTypeId: Oid,
        constructName: *const libc::c_char,
    ) -> *mut Node;
    fn coerce_to_specific_type_typmod(
        pstate: *mut ParseState,
        node: *mut Node,
        targetTypeId: Oid,
        targetTypmod: i32,
        constructName: *const libc::c_char,
    ) -> *mut Node;
    fn select_common_type(
        pstate: *mut ParseState,
        exprs: *mut List,
        context: *const libc::c_char,
        which_expr: *mut *mut Node,
    ) -> Oid;
    fn select_common_typmod(
        pstate: *mut ParseState,
        exprs: *mut List,
        common_type: Oid,
    ) -> i32;
    fn assign_list_collations(pstate: *mut ParseState, exprs: *mut List);
    fn assign_expr_collations(pstate: *mut ParseState, expr: *mut Node);
    fn transformExpr(
        pstate: *mut ParseState,
        expr: *mut Node,
        exprKind: ParseExprKind,
    ) -> *mut Node;
    fn LookupFuncName(
        funcname: *mut List,
        nargs: libc::c_int,
        argtypes: *const Oid,
        missing_ok: bool,
    ) -> Oid;
    fn get_sort_group_operators(
        argtype: Oid,
        needLT: bool,
        needEQ: bool,
        needGT: bool,
        ltOpr: *mut Oid,
        eqOpr: *mut Oid,
        gtOpr: *mut Oid,
        isHashable: *mut bool,
    );
    fn compatible_oper_opid(op: *mut List, arg1: Oid, arg2: Oid, noError: bool) -> Oid;
    fn scanNameSpaceForCTE(
        pstate: *mut ParseState,
        refname: *const libc::c_char,
        ctelevelsup: *mut Index,
    ) -> *mut CommonTableExpr;
    fn scanNameSpaceForENR(
        pstate: *mut ParseState,
        refname: *const libc::c_char,
    ) -> bool;
    fn checkNameSpaceConflicts(
        pstate: *mut ParseState,
        namespace1: *mut List,
        namespace2: *mut List,
    );
    fn colNameToVar(
        pstate: *mut ParseState,
        colname: *const libc::c_char,
        localonly: bool,
        location: libc::c_int,
    ) -> *mut Node;
    fn markVarForSelectPriv(pstate: *mut ParseState, var: *mut Var);
    fn parserOpenTable(
        pstate: *mut ParseState,
        relation: *const RangeVar,
        lockmode: libc::c_int,
    ) -> Relation;
    fn addRangeTableEntry(
        pstate: *mut ParseState,
        relation: *mut RangeVar,
        alias: *mut Alias,
        inh: bool,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForRelation(
        pstate: *mut ParseState,
        rel: Relation,
        lockmode: libc::c_int,
        alias: *mut Alias,
        inh: bool,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForSubquery(
        pstate: *mut ParseState,
        subquery: *mut Query,
        alias: *mut Alias,
        lateral: bool,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForFunction(
        pstate: *mut ParseState,
        funcnames: *mut List,
        funcexprs: *mut List,
        coldeflists: *mut List,
        rangefunc: *mut RangeFunction,
        lateral: bool,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForTableFunc(
        pstate: *mut ParseState,
        tf: *mut TableFunc,
        alias: *mut Alias,
        lateral: bool,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForJoin(
        pstate: *mut ParseState,
        colnames: *mut List,
        nscolumns: *mut ParseNamespaceColumn,
        jointype: JoinType,
        nummergedcols: libc::c_int,
        aliasvars: *mut List,
        leftcols: *mut List,
        rightcols: *mut List,
        alias: *mut Alias,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForCTE(
        pstate: *mut ParseState,
        cte: *mut CommonTableExpr,
        levelsup: Index,
        rv: *mut RangeVar,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForENR(
        pstate: *mut ParseState,
        rv: *mut RangeVar,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn isLockedRefname(pstate: *mut ParseState, refname: *const libc::c_char) -> bool;
    fn addNSItemToQuery(
        pstate: *mut ParseState,
        nsitem: *mut ParseNamespaceItem,
        addToJoinList: bool,
        addToRelNameSpace: bool,
        addToVarNameSpace: bool,
    );
    fn transformTargetEntry(
        pstate: *mut ParseState,
        node: *mut Node,
        expr: *mut Node,
        exprKind: ParseExprKind,
        colname: *mut libc::c_char,
        resjunk: bool,
    ) -> *mut TargetEntry;
    fn FigureColname(node: *mut Node) -> *mut libc::c_char;
    fn typenameTypeIdAndMod(
        pstate: *mut ParseState,
        typeName: *const TypeName,
        typeid_p: *mut Oid,
        typmod_p: *mut i32,
    );
    fn LookupCollation(
        pstate: *mut ParseState,
        collnames: *mut List,
        location: libc::c_int,
    ) -> Oid;
    fn SystemFuncName(name: *mut libc::c_char) -> *mut List;
    fn contain_aggs_of_level(node: *mut Node, levelsup: libc::c_int) -> bool;
    fn contain_windowfuncs(node: *mut Node) -> bool;
    fn ReleaseCatCacheList(list: *mut CatCList);
    fn get_ordering_op_properties(
        opno: Oid,
        opfamily: *mut Oid,
        opcintype: *mut Oid,
        strategy: *mut i16,
    ) -> bool;
    fn get_equality_op_for_ordering_op(opno: Oid, reverse: *mut bool) -> Oid;
    fn op_hashjoinable(opno: Oid, inputtype: Oid) -> bool;
    fn get_commutator(opno: Oid) -> Oid;
    fn get_typcollation(typid: Oid) -> Oid;
    fn SearchSysCacheList(
        cacheId: libc::c_int,
        nkeys: libc::c_int,
        key1: Datum,
        key2: Datum,
        key3: Datum,
    ) -> *mut catclist;
}

mod analyze;
mod gram;
mod parse_agg;
mod parse_clause;
mod parse_coerce;
mod parse_collate;
mod parse_cte;
mod parse_enr;
mod parse_expr;
mod parse_func;
mod parse_node;
mod parse_oper;
mod parse_param;
mod parse_relation;
mod parse_target;
mod parse_type;
mod parse_utilcmd;
mod parser;
mod scansup;

use analyze::*;
use gram::*;
use parse_agg::*;
use parse_clause::*;
use parse_coerce::*;
use parse_collate::*;
use parse_cte::*;
use parse_enr::*;
use parse_expr::*;
use parse_func::*;
use parse_node::*;
use parse_oper::*;
use parse_param::*;
use parse_relation::*;
use parse_target::*;
use parse_type::*;
use parse_utilcmd::*;
use parser::*;
use scansup::*;
