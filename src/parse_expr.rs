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
//     fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
//     fn pg_snprintf(
//         str: *mut libc::c_char,
//         count: isize,
//         fmt: *const libc::c_char,
//         _: ...
//     ) -> libc::c_int;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn palloc(size: usize) -> *mut libc::c_void;
//     fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
//     fn bms_add_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
//     fn bms_int_members(a: *mut Bitmapset, b: *const Bitmapset) -> *mut Bitmapset;
//     fn bms_first_member(a: *mut Bitmapset) -> libc::c_int;
//     fn copyObjectImpl(obj: *const libc::c_void) -> *mut libc::c_void;
//     fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
//     fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn lappend_oid(list: *mut List, datum: Oid) -> *mut List;
//     fn lcons(datum: *mut libc::c_void, list: *mut List) -> *mut List;
//     fn list_concat(list1: *mut List, list2: *const List) -> *mut List;
//     fn list_delete_last(list: *mut List) -> *mut List;
//     fn makeString(str: *mut libc::c_char) -> *mut Value;
//     fn transformContainerSubscripts(
//         pstate: *mut ParseState,
//         containerBase: *mut Node,
//         containerType: Oid,
//         containerTypMod: i32,
//         indirection: *mut List,
//         isAssignment: bool,
//     ) -> *mut SubscriptingRef;
//     fn make_const(
//         pstate: *mut ParseState,
//         value: *mut Value,
//         location: libc::c_int,
//     ) -> *mut Const;
//     fn get_database_name(dbid: Oid) -> *mut libc::c_char;
//     static mut MyDatabaseId: Oid;
//     fn check_stack_depth();
//     fn makeRangeVar(
//         schemaname: *mut libc::c_char,
//         relname: *mut libc::c_char,
//         location: libc::c_int,
//     ) -> *mut RangeVar;
//     fn makeBoolExpr(
//         boolop: BoolExprType,
//         args: *mut List,
//         location: libc::c_int,
//     ) -> *mut Expr;
//     fn makeBoolConst(value: bool, isnull: bool) -> *mut Node;
//     fn makeTargetEntry(
//         expr: *mut Expr,
//         resno: AttrNumber,
//         resname: *mut libc::c_char,
//         resjunk: bool,
//     ) -> *mut TargetEntry;
//     fn makeWholeRowVar(
//         rte: *mut RangeTblEntry,
//         varno: Index,
//         varlevelsup: Index,
//         allowScalar: bool,
//     ) -> *mut Var;
//     fn makeSimpleA_Expr(
//         kind: ExprKind,
//         name: *mut libc::c_char,
//         lexpr: *mut Node,
//         rexpr: *mut Node,
//         location: libc::c_int,
//     ) -> *mut A_Expr;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprTypmod(expr: *const Node) -> i32;
//     fn expression_returns_set(clause: *mut Node) -> bool;
//     fn exprCollation(expr: *const Node) -> Oid;
//     fn exprLocation(expr: *const Node) -> libc::c_int;
//     fn count_nonjunk_tlist_entries(tlist: *mut List) -> libc::c_int;
//     fn contain_vars_of_level(node: *mut Node, levelsup: libc::c_int) -> bool;
//     fn parse_sub_analyze(
//         parseTree: *mut Node,
//         parentParseState: *mut ParseState,
//         parentCTE: *mut CommonTableExpr,
//         locked_from_parent: bool,
//         resolve_unknowns: bool,
//     ) -> *mut Query;
//     fn transformGroupingFunc(pstate: *mut ParseState, g: *mut GroupingFunc) -> *mut Node;
//     fn coerce_to_target_type(
//         pstate: *mut ParseState,
//         expr: *mut Node,
//         exprtype: Oid,
//         targettype: Oid,
//         targettypmod: i32,
//         ccontext: CoercionContext,
//         cformat: CoercionForm,
//         location: libc::c_int,
//     ) -> *mut Node;
//     fn coerce_to_boolean(
//         pstate: *mut ParseState,
//         node: *mut Node,
//         constructName: *const libc::c_char,
//     ) -> *mut Node;
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
//     fn assign_expr_collations(pstate: *mut ParseState, expr: *mut Node);
//     fn anytimestamp_typmod_check(istz: bool, typmod: i32) -> i32;
//     fn ParseFuncOrColumn(
//         pstate: *mut ParseState,
//         funcname: *mut List,
//         fargs: *mut List,
//         last_srf: *mut Node,
//         fn_0: *mut FuncCall,
//         proc_call: bool,
//         location: libc::c_int,
//     ) -> *mut Node;
//     fn make_op(
//         pstate: *mut ParseState,
//         opname: *mut List,
//         ltree: *mut Node,
//         rtree: *mut Node,
//         last_srf: *mut Node,
//         location: libc::c_int,
//     ) -> *mut Expr;
//     fn make_scalar_array_op(
//         pstate: *mut ParseState,
//         opname: *mut List,
//         useOr: bool,
//         ltree: *mut Node,
//         rtree: *mut Node,
//         location: libc::c_int,
//     ) -> *mut Expr;
//     fn refnameNamespaceItem(
//         pstate: *mut ParseState,
//         schemaname: *const libc::c_char,
//         refname: *const libc::c_char,
//         location: libc::c_int,
//         sublevels_up: *mut libc::c_int,
//     ) -> *mut ParseNamespaceItem;
//     fn GetRTEByRangeTablePosn(
//         pstate: *mut ParseState,
//         varno: libc::c_int,
//         sublevels_up: libc::c_int,
//     ) -> *mut RangeTblEntry;
//     fn scanNSItemForColumn(
//         pstate: *mut ParseState,
//         nsitem: *mut ParseNamespaceItem,
//         sublevels_up: libc::c_int,
//         colname: *const libc::c_char,
//         location: libc::c_int,
//     ) -> *mut Node;
//     fn colNameToVar(
//         pstate: *mut ParseState,
//         colname: *const libc::c_char,
//         localonly: bool,
//         location: libc::c_int,
//     ) -> *mut Node;
//     fn markVarForSelectPriv(pstate: *mut ParseState, var: *mut Var);
//     fn errorMissingRTE(pstate: *mut ParseState, relation: *mut RangeVar) -> !;
//     fn errorMissingColumn(
//         pstate: *mut ParseState,
//         relname: *const libc::c_char,
//         colname: *const libc::c_char,
//         location: libc::c_int,
//     ) -> !;
//     fn transformExpressionList(
//         pstate: *mut ParseState,
//         exprlist: *mut List,
//         exprKind: ParseExprKind,
//         allowDefault: bool,
//     ) -> *mut List;
//     fn FigureColname(node: *mut Node) -> *mut libc::c_char;
//     fn typenameTypeIdAndMod(
//         pstate: *mut ParseState,
//         typeName: *const TypeName,
//         typeid_p: *mut Oid,
//         typmod_p: *mut i32,
//     );
//     fn LookupCollation(
//         pstate: *mut ParseState,
//         collnames: *mut List,
//         location: libc::c_int,
//     ) -> Oid;
//     fn typeOrDomainTypeRelid(type_id: Oid) -> Oid;
//     fn anytime_typmod_check(istz: bool, typmod: i32) -> i32;
//     fn get_op_btree_interpretation(opno: Oid) -> *mut List;
//     fn type_is_rowtype(typid: Oid) -> bool;
//     fn get_element_type(typid: Oid) -> Oid;
//     fn get_array_type(typid: Oid) -> Oid;
//     fn getBaseTypeAndTypmod(typid: Oid, typmod: *mut i32) -> Oid;
//     fn map_sql_identifier_to_xml_name(
//         ident: *const libc::c_char,
//         fully_escaped: bool,
//         escape_period: bool,
//     ) -> *mut libc::c_char;
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
// pub type uint64 = libc::c_ulong;
// pub type usize = isize;
// pub type Index = libc::c_uint;
// pub type Datum = usize;
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
pub struct ForBothState {
    pub l1: *const List,
    pub l2: *const List,
    pub i: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Alias {
    pub type_0: NodeTag,
    pub aliasname: *mut libc::c_char,
    pub colnames: *mut List,
}
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
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Expr {
//     pub type_0: NodeTag,
// }
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Var {
//     pub xpr: Expr,
//     pub varno: Index,
//     pub varattno: AttrNumber,
//     pub vartype: Oid,
//     pub vartypmod: i32,
//     pub varcollid: Oid,
//     pub varlevelsup: Index,
//     pub varnosyn: Index,
//     pub varattnosyn: AttrNumber,
//     pub location: libc::c_int,
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
pub struct GroupingFunc {
    pub xpr: Expr,
    pub args: *mut List,
    pub refs: *mut List,
    pub cols: *mut List,
    pub agglevelsup: Index,
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
pub type CoercionContext = libc::c_uint;
pub const COERCION_EXPLICIT: CoercionContext = 3;
pub const COERCION_PLPGSQL: CoercionContext = 2;
pub const COERCION_ASSIGNMENT: CoercionContext = 1;
pub const COERCION_IMPLICIT: CoercionContext = 0;
pub type CoercionForm = libc::c_uint;
pub const COERCE_SQL_SYNTAX: CoercionForm = 3;
pub const COERCE_IMPLICIT_CAST: CoercionForm = 2;
pub const COERCE_EXPLICIT_CAST: CoercionForm = 1;
pub const COERCE_EXPLICIT_CALL: CoercionForm = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NamedArgExpr {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub name: *mut libc::c_char,
    pub argnumber: libc::c_int,
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
pub type BoolExprType = libc::c_uint;
pub const NOT_EXPR: BoolExprType = 2;
pub const OR_EXPR: BoolExprType = 1;
pub const AND_EXPR: BoolExprType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BoolExpr {
    pub xpr: Expr,
    pub boolop: BoolExprType,
    pub args: *mut List,
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
pub struct CaseTestExpr {
    pub xpr: Expr,
    pub typeId: Oid,
    pub typeMod: i32,
    pub collation: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArrayExpr {
    pub xpr: Expr,
    pub array_typeid: Oid,
    pub array_collid: Oid,
    pub element_typeid: Oid,
    pub elements: *mut List,
    pub multidims: bool,
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
pub struct CoalesceExpr {
    pub xpr: Expr,
    pub coalescetype: Oid,
    pub coalescecollid: Oid,
    pub args: *mut List,
    pub location: libc::c_int,
}
pub type MinMaxOp = libc::c_uint;
pub const IS_LEAST: MinMaxOp = 1;
pub const IS_GREATEST: MinMaxOp = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MinMaxExpr {
    pub xpr: Expr,
    pub minmaxtype: Oid,
    pub minmaxcollid: Oid,
    pub inputcollid: Oid,
    pub op: MinMaxOp,
    pub args: *mut List,
    pub location: libc::c_int,
}
pub type SQLValueFunctionOp = libc::c_uint;
pub const SVFOP_CURRENT_SCHEMA: SQLValueFunctionOp = 14;
pub const SVFOP_CURRENT_CATALOG: SQLValueFunctionOp = 13;
pub const SVFOP_SESSION_USER: SQLValueFunctionOp = 12;
pub const SVFOP_USER: SQLValueFunctionOp = 11;
pub const SVFOP_CURRENT_USER: SQLValueFunctionOp = 10;
pub const SVFOP_CURRENT_ROLE: SQLValueFunctionOp = 9;
pub const SVFOP_LOCALTIMESTAMP_N: SQLValueFunctionOp = 8;
pub const SVFOP_LOCALTIMESTAMP: SQLValueFunctionOp = 7;
pub const SVFOP_LOCALTIME_N: SQLValueFunctionOp = 6;
pub const SVFOP_LOCALTIME: SQLValueFunctionOp = 5;
pub const SVFOP_CURRENT_TIMESTAMP_N: SQLValueFunctionOp = 4;
pub const SVFOP_CURRENT_TIMESTAMP: SQLValueFunctionOp = 3;
pub const SVFOP_CURRENT_TIME_N: SQLValueFunctionOp = 2;
pub const SVFOP_CURRENT_TIME: SQLValueFunctionOp = 1;
pub const SVFOP_CURRENT_DATE: SQLValueFunctionOp = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SQLValueFunction {
    pub xpr: Expr,
    pub op: SQLValueFunctionOp,
    pub type_0: Oid,
    pub typmod: i32,
    pub location: libc::c_int,
}
pub type XmlExprOp = libc::c_uint;
pub const IS_DOCUMENT: XmlExprOp = 7;
pub const IS_XMLSERIALIZE: XmlExprOp = 6;
pub const IS_XMLROOT: XmlExprOp = 5;
pub const IS_XMLPI: XmlExprOp = 4;
pub const IS_XMLPARSE: XmlExprOp = 3;
pub const IS_XMLFOREST: XmlExprOp = 2;
pub const IS_XMLELEMENT: XmlExprOp = 1;
pub const IS_XMLCONCAT: XmlExprOp = 0;
pub type XmlOptionType = libc::c_uint;
pub const XMLOPTION_CONTENT: XmlOptionType = 1;
pub const XMLOPTION_DOCUMENT: XmlOptionType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XmlExpr {
    pub xpr: Expr,
    pub op: XmlExprOp,
    pub name: *mut libc::c_char,
    pub named_args: *mut List,
    pub arg_names: *mut List,
    pub args: *mut List,
    pub xmloption: XmlOptionType,
    pub type_0: Oid,
    pub typmod: i32,
    pub location: libc::c_int,
}
pub type NullTestType = libc::c_uint;
pub const IS_NOT_NULL: NullTestType = 1;
pub const IS_NULL: NullTestType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NullTest {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub nulltesttype: NullTestType,
    pub argisrow: bool,
    pub location: libc::c_int,
}
pub type BoolTestType = libc::c_uint;
pub const IS_NOT_UNKNOWN: BoolTestType = 5;
pub const IS_UNKNOWN: BoolTestType = 4;
pub const IS_NOT_FALSE: BoolTestType = 3;
pub const IS_FALSE: BoolTestType = 2;
pub const IS_NOT_TRUE: BoolTestType = 1;
pub const IS_TRUE: BoolTestType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BooleanTest {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub booltesttype: BoolTestType,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CurrentOfExpr {
    pub xpr: Expr,
    pub cvarno: Index,
    pub cursor_name: *mut libc::c_char,
    pub cursor_param: libc::c_int,
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
pub type SortByDir = libc::c_uint;
pub const SORTBY_USING: SortByDir = 3;
pub const SORTBY_DESC: SortByDir = 2;
pub const SORTBY_ASC: SortByDir = 1;
pub const SORTBY_DEFAULT: SortByDir = 0;
pub type SortByNulls = libc::c_uint;
pub const SORTBY_NULLS_LAST: SortByNulls = 2;
pub const SORTBY_NULLS_FIRST: SortByNulls = 1;
pub const SORTBY_NULLS_DEFAULT: SortByNulls = 0;
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
// pub type ExprKind = libc::c_uint;
// pub const AEXPR_NOT_BETWEEN_SYM: ExprKind = 13;
// pub const AEXPR_BETWEEN_SYM: ExprKind = 12;
// pub const AEXPR_NOT_BETWEEN: ExprKind = 11;
// pub const AEXPR_BETWEEN: ExprKind = 10;
// pub const AEXPR_SIMILAR: ExprKind = 9;
// pub const AEXPR_ILIKE: ExprKind = 8;
// pub const AEXPR_LIKE: ExprKind = 7;
// pub const AEXPR_IN: ExprKind = 6;
// pub const AEXPR_NULLIF: ExprKind = 5;
// pub const AEXPR_NOT_DISTINCT: ExprKind = 4;
// pub const AEXPR_DISTINCT: ExprKind = 3;
// pub const AEXPR_OP_ALL: ExprKind = 2;
// pub const AEXPR_OP_ANY: ExprKind = 1;
// pub const AEXPR_OP: ExprKind = 0;
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct A_Expr {
//     pub type_0: NodeTag,
//     pub kind: ExprKind,
//     pub name: *mut List,
//     pub lexpr: *mut Node,
//     pub rexpr: *mut Node,
//     pub location: libc::c_int,
// }
#[derive(Copy, Clone)]
#[repr(C)]
pub struct A_Const {
    pub type_0: NodeTag,
    pub val: Value,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TypeCast {
    pub type_0: NodeTag,
    pub arg: *mut Node,
    pub typeName: *mut TypeName,
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
pub struct FuncCall {
    pub type_0: NodeTag,
    pub funcname: *mut List,
    pub args: *mut List,
    pub agg_order: *mut List,
    pub agg_filter: *mut Node,
    pub over: *mut WindowDef,
    pub agg_within_group: bool,
    pub agg_star: bool,
    pub agg_distinct: bool,
    pub func_variadic: bool,
    pub funcformat: CoercionForm,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WindowDef {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub refname: *mut libc::c_char,
    pub partitionClause: *mut List,
    pub orderClause: *mut List,
    pub frameOptions: libc::c_int,
    pub startOffset: *mut Node,
    pub endOffset: *mut Node,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct A_Indirection {
    pub type_0: NodeTag,
    pub arg: *mut Node,
    pub indirection: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct A_ArrayExpr {
    pub type_0: NodeTag,
    pub elements: *mut List,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ResTarget {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub indirection: *mut List,
    pub val: *mut Node,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultiAssignRef {
    pub type_0: NodeTag,
    pub source: *mut Node,
    pub colno: libc::c_int,
    pub ncolumns: libc::c_int,
}
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
pub struct XmlSerialize {
    pub type_0: NodeTag,
    pub xmloption: XmlOptionType,
    pub expr: *mut Node,
    pub typeName: *mut TypeName,
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
pub struct OpBtreeInterpretation {
    pub opfamily_id: Oid,
    pub strategy: libc::c_int,
    pub oplefttype: Oid,
    pub oprighttype: Oid,
}
pub const CRERR_TOO_MANY: C2RustUnnamed = 3;
pub const CRERR_WRONG_DB: C2RustUnnamed = 2;
pub const CRERR_NO_RTE: C2RustUnnamed = 1;
pub const CRERR_NO_COLUMN: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
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
unsafe extern "C" fn list_last_cell(mut list: *const List) -> *mut ListCell {
    return &mut *((*list).elements).offset(((*list).length - 1 as libc::c_int) as isize)
        as *mut ListCell;
}
#[inline]
unsafe extern "C" fn list_nth(mut list: *const List, mut n: libc::c_int) -> *mut libc::c_void {
    return (*list_nth_cell(list, n)).ptr_value;
}
#[no_mangle]
pub static mut Transform_null_equals: bool = false;
#[no_mangle]
pub unsafe extern "C" fn transformExpr(
    mut pstate: *mut ParseState,
    mut expr: *mut Node,
    mut exprKind: ParseExprKind,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut sv_expr_kind: ParseExprKind = EXPR_KIND_NONE;
    sv_expr_kind = (*pstate).p_expr_kind;
    (*pstate).p_expr_kind = exprKind;
    result = transformExprRecurse(pstate, expr);
    (*pstate).p_expr_kind = sv_expr_kind;
    return result;
}
unsafe extern "C" fn transformExprRecurse(
    mut pstate: *mut ParseState,
    mut expr: *mut Node,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    if expr.is_null() {
        return 0 as *mut Node;
    }
    check_stack_depth();
    match (*(expr as *const Node)).tag as libc::c_uint {
        349 => {
            result = transformColumnRef(pstate, expr as *mut ColumnRef);
        }
        350 => {
            result = transformParamRef(pstate, expr as *mut ParamRef);
        }
        351 => {
            let mut con: *mut A_Const = expr as *mut A_Const;
            let mut val: *mut Value = &mut (*con).val;
            result = make_const(pstate, val, (*con).location) as *mut Node;
        }
        355 => {
            result = transformIndirection(pstate, expr as *mut A_Indirection);
        }
        356 => {
            result = transformArrayExpr(
                pstate,
                expr as *mut A_ArrayExpr,
                0 as libc::c_int as Oid,
                0 as libc::c_int as Oid,
                -(1 as libc::c_int),
            );
        }
        359 => {
            result = transformTypeCast(pstate, expr as *mut TypeCast);
        }
        360 => {
            result = transformCollateClause(pstate, expr as *mut CollateClause);
        }
        348 => {
            let mut a: *mut A_Expr = expr as *mut A_Expr;
            match (*a).kind as libc::c_uint {
                0 => {
                    result = transformAExprOp(pstate, a);
                }
                1 => {
                    result = transformAExprOpAny(pstate, a);
                }
                2 => {
                    result = transformAExprOpAll(pstate, a);
                }
                3 | 4 => {
                    result = transformAExprDistinct(pstate, a);
                }
                5 => {
                    result = transformAExprNullIf(pstate, a);
                }
                6 => {
                    result = transformAExprIn(pstate, a);
                }
                7 | 8 | 9 => {
                    result = transformAExprOp(pstate, a);
                }
                10 | 11 | 12 | 13 => {
                    result = transformAExprBetween(pstate, a);
                }
                _ => {
                    let elevel_: libc::c_int = 21 as libc::c_int;
                    let mut __error: libc::c_int = 0;
                    if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                        errmsg_internal(
                            b"unrecognized A_Expr kind: %d\0" as *const u8 as *const libc::c_char,
                            (*a).kind as libc::c_uint,
                        );
                        errfinish(
                            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_expr.c\0"
                                as *const u8 as *const libc::c_char,
                            196 as libc::c_int,
                            0 as *const libc::c_char,
                        );
                    }
                    if elevel_ >= 21 as libc::c_int {
                        abort();
                    }
                    result = 0 as *mut Node;
                }
            }
        }
        121 => {
            result = transformBoolExpr(pstate, expr as *mut BoolExpr);
        }
        352 => {
            result = transformFuncCall(pstate, expr as *mut FuncCall);
        }
        358 => {
            result = transformMultiAssignRef(pstate, expr as *mut MultiAssignRef);
        }
        112 => {
            result = transformGroupingFunc(pstate, expr as *mut GroupingFunc);
        }
        116 => {
            let mut na: *mut NamedArgExpr = expr as *mut NamedArgExpr;
            (*na).arg = transformExprRecurse(pstate, (*na).arg as *mut Node) as *mut Expr;
            result = expr;
        }
        122 => {
            result = transformSubLink(pstate, expr as *mut SubLink);
        }
        132 => {
            result = transformCaseExpr(pstate, expr as *mut CaseExpr);
        }
        136 => {
            result = transformRowExpr(pstate, expr as *mut RowExpr, false);
        }
        138 => {
            result = transformCoalesceExpr(pstate, expr as *mut CoalesceExpr);
        }
        139 => {
            result = transformMinMaxExpr(pstate, expr as *mut MinMaxExpr);
        }
        140 => {
            result = transformSQLValueFunction(pstate, expr as *mut SQLValueFunction);
        }
        141 => {
            result = transformXmlExpr(pstate, expr as *mut XmlExpr);
        }
        387 => {
            result = transformXmlSerialize(pstate, expr as *mut XmlSerialize);
        }
        142 => {
            let mut n: *mut NullTest = expr as *mut NullTest;
            (*n).arg = transformExprRecurse(pstate, (*n).arg as *mut Node) as *mut Expr;
            (*n).argisrow = type_is_rowtype(exprType((*n).arg as *mut Node));
            result = expr;
        }
        143 => {
            result = transformBooleanTest(pstate, expr as *mut BooleanTest);
        }
        147 => {
            result = transformCurrentOfExpr(pstate, expr as *mut CurrentOfExpr);
        }
        146 => {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
        134 | 108 => {
            result = expr;
        }
        _ => {
            let elevel__1: libc::c_int = 21 as libc::c_int;
            let mut __error_1: libc::c_int = 0;
            if errstart(elevel__1, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unrecognized node type: %d\0" as *const u8 as *const libc::c_char,
                    (*(expr as *const Node)).tag as libc::c_int,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_expr.c\0"
                        as *const u8 as *const libc::c_char,
                    311 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel__1 >= 21 as libc::c_int {
                abort();
            }
            result = 0 as *mut Node;
        }
    }
    return result;
}
unsafe extern "C" fn transformIndirection(
    mut pstate: *mut ParseState,
    mut ind: *mut A_Indirection,
) -> *mut Node {
    let mut last_srf: *mut Node = (*pstate).p_last_srf;
    let mut result: *mut Node = transformExprRecurse(pstate, (*ind).arg);
    let mut subscripts: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut location: libc::c_int = exprLocation(result);
    let mut i: *mut ListCell = 0 as *mut ListCell;
    let mut i__state: ForEachState = {
        let mut init = ForEachState {
            l: (*ind).indirection,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(i__state.l).is_null() && i__state.i < (*i__state.l).length {
        i = &mut *((*i__state.l).elements).offset(i__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        i = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut n: *mut Node = (*i).ptr_value as *mut Node;
        if (*(n as *const Node)).tag as libc::c_uint == T_A_Indices as libc::c_int as libc::c_uint {
            subscripts = lappend(subscripts, n as *mut libc::c_void);
        } else if (*(n as *const Node)).tag as libc::c_uint
            == T_A_Star as libc::c_int as libc::c_uint
        {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        } else {
            let mut newresult: *mut Node = 0 as *mut Node;
            if !subscripts.is_null() {
                result = transformContainerSubscripts(
                    pstate,
                    result,
                    exprType(result),
                    exprTypmod(result),
                    subscripts,
                    false,
                ) as *mut Node;
            }
            subscripts = 0 as *mut libc::c_void as *mut List;
            newresult = ParseFuncOrColumn(
                pstate,
                list_make1_impl(
                    T_List,
                    ListCell {
                        ptr_value: n as *mut libc::c_void,
                    },
                ),
                list_make1_impl(
                    T_List,
                    ListCell {
                        ptr_value: result as *mut libc::c_void,
                    },
                ),
                last_srf,
                0 as *mut FuncCall,
                false,
                location,
            );
            if newresult.is_null() {
                unknown_attribute(pstate, result, (*(n as *mut Value)).val.str_0, location);
            }
            result = newresult;
        }
        i__state.i += 1;
        i__state.i;
    }
    if !subscripts.is_null() {
        result = transformContainerSubscripts(
            pstate,
            result,
            exprType(result),
            exprTypmod(result),
            subscripts,
            false,
        ) as *mut Node;
    }
    return result;
}
unsafe extern "C" fn transformColumnRef(
    mut pstate: *mut ParseState,
    mut cref: *mut ColumnRef,
) -> *mut Node {
    let mut node: *mut Node = 0 as *mut Node;
    let mut nspname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut relname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut colname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    let mut levels_up: libc::c_int = 0;
    let mut crerr: C2RustUnnamed = CRERR_NO_COLUMN;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    err = 0 as *const libc::c_char;
    match (*pstate).p_expr_kind as libc::c_uint {
        1 | 2 | 3 | 4 | 5 | 6 | 35 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18
        | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 29 | 30 | 31 | 32 | 33 | 34 | 37 | 38
        | 39 | 40 | 41 => {}
        28 => {
            err = b"cannot use column reference in DEFAULT expression\0" as *const u8
                as *const libc::c_char;
        }
        36 => {
            err = b"cannot use column reference in partition bound expression\0" as *const u8
                as *const libc::c_char;
        }
        0 | _ => {}
    }
    if !err.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if ((*pstate).p_pre_columnref_hook).is_some() {
        node = ((*pstate).p_pre_columnref_hook).expect("non-null function pointer")(pstate, cref);
        if !node.is_null() {
            return node;
        }
    }
    match list_length((*cref).fields) {
        1 => {
            let mut field1: *mut Node =
                (*list_nth_cell((*cref).fields, 0 as libc::c_int)).ptr_value as *mut Node;
            colname = (*(field1 as *mut Value)).val.str_0;
            node = colNameToVar(pstate, colname, false, (*cref).location);
            if node.is_null() {
                nsitem = refnameNamespaceItem(
                    pstate,
                    0 as *const libc::c_char,
                    colname,
                    (*cref).location,
                    &mut levels_up,
                );
                if !nsitem.is_null() {
                    node = transformWholeRowRef(pstate, nsitem, levels_up, (*cref).location);
                }
            }
        }
        2 => {
            let mut field1_0: *mut Node =
                (*list_nth_cell((*cref).fields, 0 as libc::c_int)).ptr_value as *mut Node;
            let mut field2: *mut Node =
                (*list_nth_cell((*cref).fields, 1 as libc::c_int)).ptr_value as *mut Node;
            relname = (*(field1_0 as *mut Value)).val.str_0;
            nsitem =
                refnameNamespaceItem(pstate, nspname, relname, (*cref).location, &mut levels_up);
            if nsitem.is_null() {
                crerr = CRERR_NO_RTE;
            } else if (*(field2 as *const Node)).tag as libc::c_uint
                == T_A_Star as libc::c_int as libc::c_uint
            {
                node = transformWholeRowRef(pstate, nsitem, levels_up, (*cref).location);
            } else {
                colname = (*(field2 as *mut Value)).val.str_0;
                node = scanNSItemForColumn(pstate, nsitem, levels_up, colname, (*cref).location);
                if node.is_null() {
                    node = transformWholeRowRef(pstate, nsitem, levels_up, (*cref).location);
                    node = ParseFuncOrColumn(
                        pstate,
                        list_make1_impl(
                            T_List,
                            ListCell {
                                ptr_value: makeString(colname) as *mut libc::c_void,
                            },
                        ),
                        list_make1_impl(
                            T_List,
                            ListCell {
                                ptr_value: node as *mut libc::c_void,
                            },
                        ),
                        (*pstate).p_last_srf,
                        0 as *mut FuncCall,
                        false,
                        (*cref).location,
                    );
                }
            }
        }
        3 => {
            let mut field1_1: *mut Node =
                (*list_nth_cell((*cref).fields, 0 as libc::c_int)).ptr_value as *mut Node;
            let mut field2_0: *mut Node =
                (*list_nth_cell((*cref).fields, 1 as libc::c_int)).ptr_value as *mut Node;
            let mut field3: *mut Node =
                (*list_nth_cell((*cref).fields, 2 as libc::c_int)).ptr_value as *mut Node;
            nspname = (*(field1_1 as *mut Value)).val.str_0;
            relname = (*(field2_0 as *mut Value)).val.str_0;
            nsitem =
                refnameNamespaceItem(pstate, nspname, relname, (*cref).location, &mut levels_up);
            if nsitem.is_null() {
                crerr = CRERR_NO_RTE;
            } else if (*(field3 as *const Node)).tag as libc::c_uint
                == T_A_Star as libc::c_int as libc::c_uint
            {
                node = transformWholeRowRef(pstate, nsitem, levels_up, (*cref).location);
            } else {
                colname = (*(field3 as *mut Value)).val.str_0;
                node = scanNSItemForColumn(pstate, nsitem, levels_up, colname, (*cref).location);
                if node.is_null() {
                    node = transformWholeRowRef(pstate, nsitem, levels_up, (*cref).location);
                    node = ParseFuncOrColumn(
                        pstate,
                        list_make1_impl(
                            T_List,
                            ListCell {
                                ptr_value: makeString(colname) as *mut libc::c_void,
                            },
                        ),
                        list_make1_impl(
                            T_List,
                            ListCell {
                                ptr_value: node as *mut libc::c_void,
                            },
                        ),
                        (*pstate).p_last_srf,
                        0 as *mut FuncCall,
                        false,
                        (*cref).location,
                    );
                }
            }
        }
        4 => {
            let mut field1_2: *mut Node =
                (*list_nth_cell((*cref).fields, 0 as libc::c_int)).ptr_value as *mut Node;
            let mut field2_1: *mut Node =
                (*list_nth_cell((*cref).fields, 1 as libc::c_int)).ptr_value as *mut Node;
            let mut field3_0: *mut Node =
                (*list_nth_cell((*cref).fields, 2 as libc::c_int)).ptr_value as *mut Node;
            let mut field4: *mut Node =
                (*list_nth_cell((*cref).fields, 3 as libc::c_int)).ptr_value as *mut Node;
            let mut catname: *mut libc::c_char = 0 as *mut libc::c_char;
            catname = (*(field1_2 as *mut Value)).val.str_0;
            nspname = (*(field2_1 as *mut Value)).val.str_0;
            relname = (*(field3_0 as *mut Value)).val.str_0;
            if strcmp(catname, get_database_name(MyDatabaseId)) != 0 as libc::c_int {
                crerr = CRERR_WRONG_DB;
            } else {
                nsitem = refnameNamespaceItem(
                    pstate,
                    nspname,
                    relname,
                    (*cref).location,
                    &mut levels_up,
                );
                if nsitem.is_null() {
                    crerr = CRERR_NO_RTE;
                } else if (*(field4 as *const Node)).tag as libc::c_uint
                    == T_A_Star as libc::c_int as libc::c_uint
                {
                    node = transformWholeRowRef(pstate, nsitem, levels_up, (*cref).location);
                } else {
                    colname = (*(field4 as *mut Value)).val.str_0;
                    node =
                        scanNSItemForColumn(pstate, nsitem, levels_up, colname, (*cref).location);
                    if node.is_null() {
                        node = transformWholeRowRef(pstate, nsitem, levels_up, (*cref).location);
                        node = ParseFuncOrColumn(
                            pstate,
                            list_make1_impl(
                                T_List,
                                ListCell {
                                    ptr_value: makeString(colname) as *mut libc::c_void,
                                },
                            ),
                            list_make1_impl(
                                T_List,
                                ListCell {
                                    ptr_value: node as *mut libc::c_void,
                                },
                            ),
                            (*pstate).p_last_srf,
                            0 as *mut FuncCall,
                            false,
                            (*cref).location,
                        );
                    }
                }
            }
        }
        _ => {
            crerr = CRERR_TOO_MANY;
        }
    }
    if ((*pstate).p_post_columnref_hook).is_some() {
        let mut hookresult: *mut Node = 0 as *mut Node;
        hookresult = ((*pstate).p_post_columnref_hook).expect("non-null function pointer")(
            pstate, cref, node,
        );
        if node.is_null() {
            node = hookresult;
        } else if !hookresult.is_null() {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
    }
    if node.is_null() {
        match crerr as libc::c_uint {
            0 => {
                errorMissingColumn(pstate, relname, colname, (*cref).location);
            }
            1 => {
                errorMissingRTE(pstate, makeRangeVar(nspname, relname, (*cref).location));
            }
            2 => {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
            3 => {
                let elevel__2: libc::c_int = 21 as libc::c_int;
                let mut __error_2: libc::c_int = 0;
                if elevel__2 >= 21 as libc::c_int {
                    abort();
                }
            }
            _ => {}
        }
    }
    return node;
}
unsafe extern "C" fn transformParamRef(
    mut pstate: *mut ParseState,
    mut pref: *mut ParamRef,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    if ((*pstate).p_paramref_hook).is_some() {
        result = ((*pstate).p_paramref_hook).expect("non-null function pointer")(pstate, pref);
    } else {
        result = 0 as *mut Node;
    }
    if result.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return result;
}
unsafe extern "C" fn exprIsNullConstant(mut arg: *mut Node) -> bool {
    if !arg.is_null()
        && (*(arg as *const Node)).tag as libc::c_uint == T_A_Const as libc::c_int as libc::c_uint
    {
        let mut con: *mut A_Const = arg as *mut A_Const;
        if (*con).val.tag as libc::c_uint == T_Null as libc::c_int as libc::c_uint {
            return true;
        }
    }
    return false;
}
unsafe extern "C" fn transformAExprOp(
    mut pstate: *mut ParseState,
    mut a: *mut A_Expr,
) -> *mut Node {
    let mut lexpr: *mut Node = (*a).lexpr;
    let mut rexpr: *mut Node = (*a).rexpr;
    let mut result: *mut Node = 0 as *mut Node;
    if Transform_null_equals as libc::c_int != 0
        && list_length((*a).name) == 1 as libc::c_int
        && strcmp(
            (*((*list_nth_cell((*a).name, 0 as libc::c_int)).ptr_value as *mut Value))
                .val
                .str_0,
            b"=\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        && (exprIsNullConstant(lexpr) as libc::c_int != 0
            || exprIsNullConstant(rexpr) as libc::c_int != 0)
        && (!((*(lexpr as *const Node)).tag as libc::c_uint
            == T_CaseTestExpr as libc::c_int as libc::c_uint)
            && !((*(rexpr as *const Node)).tag as libc::c_uint
                == T_CaseTestExpr as libc::c_int as libc::c_uint))
    {
        let mut n: *mut NullTest = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).tag = T_NullTest;
            _result
        }) as *mut NullTest;
        (*n).nulltesttype = IS_NULL;
        (*n).location = (*a).location;
        if exprIsNullConstant(lexpr) != 0 {
            (*n).arg = rexpr as *mut Expr;
        } else {
            (*n).arg = lexpr as *mut Expr;
        }
        result = transformExprRecurse(pstate, n as *mut Node);
    } else if !lexpr.is_null()
        && (*(lexpr as *const Node)).tag as libc::c_uint == T_RowExpr as libc::c_int as libc::c_uint
        && !rexpr.is_null()
        && (*(rexpr as *const Node)).tag as libc::c_uint == T_SubLink as libc::c_int as libc::c_uint
        && (*(rexpr as *mut SubLink)).subLinkType as libc::c_uint
            == EXPR_SUBLINK as libc::c_int as libc::c_uint
    {
        let mut s: *mut SubLink = rexpr as *mut SubLink;
        (*s).subLinkType = ROWCOMPARE_SUBLINK;
        (*s).testexpr = lexpr;
        (*s).operName = (*a).name;
        (*s).location = (*a).location;
        result = transformExprRecurse(pstate, s as *mut Node);
    } else if !lexpr.is_null()
        && (*(lexpr as *const Node)).tag as libc::c_uint == T_RowExpr as libc::c_int as libc::c_uint
        && !rexpr.is_null()
        && (*(rexpr as *const Node)).tag as libc::c_uint == T_RowExpr as libc::c_int as libc::c_uint
    {
        lexpr = transformExprRecurse(pstate, lexpr);
        rexpr = transformExprRecurse(pstate, rexpr);
        result = make_row_comparison_op(
            pstate,
            (*a).name,
            (*(lexpr as *mut RowExpr)).args,
            (*(rexpr as *mut RowExpr)).args,
            (*a).location,
        );
    } else {
        let mut last_srf: *mut Node = (*pstate).p_last_srf;
        lexpr = transformExprRecurse(pstate, lexpr);
        rexpr = transformExprRecurse(pstate, rexpr);
        result = make_op(pstate, (*a).name, lexpr, rexpr, last_srf, (*a).location) as *mut Node;
    }
    return result;
}
unsafe extern "C" fn transformAExprOpAny(
    mut pstate: *mut ParseState,
    mut a: *mut A_Expr,
) -> *mut Node {
    let mut lexpr: *mut Node = transformExprRecurse(pstate, (*a).lexpr);
    let mut rexpr: *mut Node = transformExprRecurse(pstate, (*a).rexpr);
    return make_scalar_array_op(pstate, (*a).name, true, lexpr, rexpr, (*a).location) as *mut Node;
}
unsafe extern "C" fn transformAExprOpAll(
    mut pstate: *mut ParseState,
    mut a: *mut A_Expr,
) -> *mut Node {
    let mut lexpr: *mut Node = transformExprRecurse(pstate, (*a).lexpr);
    let mut rexpr: *mut Node = transformExprRecurse(pstate, (*a).rexpr);
    return make_scalar_array_op(pstate, (*a).name, false, lexpr, rexpr, (*a).location)
        as *mut Node;
}
unsafe extern "C" fn transformAExprDistinct(
    mut pstate: *mut ParseState,
    mut a: *mut A_Expr,
) -> *mut Node {
    let mut lexpr: *mut Node = (*a).lexpr;
    let mut rexpr: *mut Node = (*a).rexpr;
    let mut result: *mut Node = 0 as *mut Node;
    if exprIsNullConstant(rexpr) != 0 {
        return make_nulltest_from_distinct(pstate, a, lexpr);
    }
    if exprIsNullConstant(lexpr) != 0 {
        return make_nulltest_from_distinct(pstate, a, rexpr);
    }
    lexpr = transformExprRecurse(pstate, lexpr);
    rexpr = transformExprRecurse(pstate, rexpr);
    if !lexpr.is_null()
        && (*(lexpr as *const Node)).tag as libc::c_uint == T_RowExpr as libc::c_int as libc::c_uint
        && !rexpr.is_null()
        && (*(rexpr as *const Node)).tag as libc::c_uint == T_RowExpr as libc::c_int as libc::c_uint
    {
        result = make_row_distinct_op(
            pstate,
            (*a).name,
            lexpr as *mut RowExpr,
            rexpr as *mut RowExpr,
            (*a).location,
        );
    } else {
        result = make_distinct_op(pstate, (*a).name, lexpr, rexpr, (*a).location) as *mut Node;
    }
    if (*a).kind as libc::c_uint == AEXPR_NOT_DISTINCT as libc::c_int as libc::c_uint {
        result = makeBoolExpr(
            NOT_EXPR,
            list_make1_impl(
                T_List,
                ListCell {
                    ptr_value: result as *mut libc::c_void,
                },
            ),
            (*a).location,
        ) as *mut Node;
    }
    return result;
}
unsafe extern "C" fn transformAExprBetween(
    mut pstate: *mut ParseState,
    mut a: *mut A_Expr,
) -> *mut Node {
    let mut aexpr: *mut Node = 0 as *mut Node;
    let mut bexpr: *mut Node = 0 as *mut Node;
    let mut cexpr: *mut Node = 0 as *mut Node;
    let mut result: *mut Node = 0 as *mut Node;
    let mut sub1: *mut Node = 0 as *mut Node;
    let mut sub2: *mut Node = 0 as *mut Node;
    let mut args: *mut List = 0 as *mut List;
    aexpr = (*a).lexpr;
    args = (*a).rexpr as *mut List;
    bexpr = (*list_nth_cell(args, 0 as libc::c_int)).ptr_value as *mut Node;
    cexpr = (*list_nth_cell(args, 1 as libc::c_int)).ptr_value as *mut Node;
    match (*a).kind as libc::c_uint {
        10 => {
            args = list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b">=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        aexpr,
                        bexpr,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b"<=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        copyObjectImpl(aexpr as *const libc::c_void) as *mut Node,
                        cexpr,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
            );
            result = makeBoolExpr(AND_EXPR, args, (*a).location) as *mut Node;
        }
        11 => {
            args = list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b"<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        aexpr,
                        bexpr,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b">\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        copyObjectImpl(aexpr as *const libc::c_void) as *mut Node,
                        cexpr,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
            );
            result = makeBoolExpr(OR_EXPR, args, (*a).location) as *mut Node;
        }
        12 => {
            args = list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b">=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        aexpr,
                        bexpr,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b"<=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        copyObjectImpl(aexpr as *const libc::c_void) as *mut Node,
                        cexpr,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
            );
            sub1 = makeBoolExpr(AND_EXPR, args, (*a).location) as *mut Node;
            args = list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b">=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        copyObjectImpl(aexpr as *const libc::c_void) as *mut Node,
                        copyObjectImpl(cexpr as *const libc::c_void) as *mut Node,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b"<=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        copyObjectImpl(aexpr as *const libc::c_void) as *mut Node,
                        copyObjectImpl(bexpr as *const libc::c_void) as *mut Node,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
            );
            sub2 = makeBoolExpr(AND_EXPR, args, (*a).location) as *mut Node;
            args = list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: sub1 as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: sub2 as *mut libc::c_void,
                },
            );
            result = makeBoolExpr(OR_EXPR, args, (*a).location) as *mut Node;
        }
        13 => {
            args = list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b"<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        aexpr,
                        bexpr,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b">\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        copyObjectImpl(aexpr as *const libc::c_void) as *mut Node,
                        cexpr,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
            );
            sub1 = makeBoolExpr(OR_EXPR, args, (*a).location) as *mut Node;
            args = list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b"<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        copyObjectImpl(aexpr as *const libc::c_void) as *mut Node,
                        copyObjectImpl(cexpr as *const libc::c_void) as *mut Node,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: makeSimpleA_Expr(
                        AEXPR_OP,
                        b">\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        copyObjectImpl(aexpr as *const libc::c_void) as *mut Node,
                        copyObjectImpl(bexpr as *const libc::c_void) as *mut Node,
                        (*a).location,
                    ) as *mut libc::c_void,
                },
            );
            sub2 = makeBoolExpr(OR_EXPR, args, (*a).location) as *mut Node;
            args = list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: sub1 as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: sub2 as *mut libc::c_void,
                },
            );
            result = makeBoolExpr(AND_EXPR, args, (*a).location) as *mut Node;
        }
        _ => {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unrecognized A_Expr kind: %d\0" as *const u8 as *const libc::c_char,
                    (*a).kind as libc::c_uint,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_expr.c\0"
                        as *const u8 as *const libc::c_char,
                    1304 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
            result = 0 as *mut Node;
        }
    }
    return transformExprRecurse(pstate, result);
}
unsafe extern "C" fn transformBoolExpr(
    mut pstate: *mut ParseState,
    mut a: *mut BoolExpr,
) -> *mut Node {
    let mut args: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut opname: *const libc::c_char = 0 as *const libc::c_char;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    match (*a).boolop as libc::c_uint {
        0 => {
            opname = b"AND\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            opname = b"OR\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            opname = b"NOT\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unrecognized boolop: %d\0" as *const u8 as *const libc::c_char,
                    (*a).boolop as libc::c_int,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_expr.c\0"
                        as *const u8 as *const libc::c_char,
                    1331 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
            opname = 0 as *const libc::c_char;
        }
    }
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*a).args,
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
        let mut arg: *mut Node = (*lc).ptr_value as *mut Node;
        arg = transformExprRecurse(pstate, arg);
        arg = coerce_to_boolean(pstate, arg, opname);
        args = lappend(args, arg as *mut libc::c_void);
        lc__state.i += 1;
        lc__state.i;
    }
    return makeBoolExpr((*a).boolop, args, (*a).location) as *mut Node;
}
unsafe extern "C" fn transformFuncCall(
    mut pstate: *mut ParseState,
    mut fn_0: *mut FuncCall,
) -> *mut Node {
    let mut last_srf: *mut Node = (*pstate).p_last_srf;
    let mut targs: *mut List = 0 as *mut List;
    let mut args: *mut ListCell = 0 as *mut ListCell;
    targs = 0 as *mut libc::c_void as *mut List;
    let mut args__state: ForEachState = {
        let mut init = ForEachState {
            l: (*fn_0).args,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(args__state.l).is_null() && args__state.i < (*args__state.l).length {
        args = &mut *((*args__state.l).elements).offset(args__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        args = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        targs = lappend(
            targs,
            transformExprRecurse(pstate, (*args).ptr_value as *mut Node) as *mut libc::c_void,
        );
        args__state.i += 1;
        args__state.i;
    }
    if (*fn_0).agg_within_group != 0 {
        let mut args__state_0: ForEachState = {
            let mut init = ForEachState {
                l: (*fn_0).agg_order,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(args__state_0.l).is_null() && args__state_0.i < (*args__state_0.l).length {
            args = &mut *((*args__state_0.l).elements).offset(args__state_0.i as isize)
                as *mut ListCell;
            true as libc::c_int
        } else {
            args = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut arg: *mut SortBy = (*args).ptr_value as *mut SortBy;
            targs = lappend(
                targs,
                transformExpr(pstate, (*arg).node, EXPR_KIND_ORDER_BY) as *mut libc::c_void,
            );
            args__state_0.i += 1;
            args__state_0.i;
        }
    }
    return ParseFuncOrColumn(
        pstate,
        (*fn_0).funcname,
        targs,
        last_srf,
        fn_0,
        false,
        (*fn_0).location,
    );
}
unsafe extern "C" fn transformMultiAssignRef(
    mut pstate: *mut ParseState,
    mut maref: *mut MultiAssignRef,
) -> *mut Node {
    let mut sublink: *mut SubLink = 0 as *mut SubLink;
    let mut rexpr: *mut RowExpr = 0 as *mut RowExpr;
    let mut qtree: *mut Query = 0 as *mut Query;
    let mut tle: *mut TargetEntry = 0 as *mut TargetEntry;
    if (*maref).colno == 1 as libc::c_int {
        if (*((*maref).source as *const Node)).tag as libc::c_uint
            == T_SubLink as libc::c_int as libc::c_uint
            && (*((*maref).source as *mut SubLink)).subLinkType as libc::c_uint
                == EXPR_SUBLINK as libc::c_int as libc::c_uint
        {
            sublink = (*maref).source as *mut SubLink;
            (*sublink).subLinkType = MULTIEXPR_SUBLINK;
            sublink = transformExprRecurse(pstate, sublink as *mut Node) as *mut SubLink;
            qtree = (*sublink).subselect as *mut Query;
            if count_nonjunk_tlist_entries((*qtree).targetList) != (*maref).ncolumns {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            tle = makeTargetEntry(
                sublink as *mut Expr,
                0 as libc::c_int as AttrNumber,
                0 as *mut libc::c_char,
                true,
            );
            (*pstate).p_multiassign_exprs =
                lappend((*pstate).p_multiassign_exprs, tle as *mut libc::c_void);
            (*sublink).subLinkId = list_length((*pstate).p_multiassign_exprs);
        } else if (*((*maref).source as *const Node)).tag as libc::c_uint
            == T_RowExpr as libc::c_int as libc::c_uint
        {
            rexpr = transformRowExpr(pstate, (*maref).source as *mut RowExpr, true) as *mut RowExpr;
            if list_length((*rexpr).args) != (*maref).ncolumns {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            tle = makeTargetEntry(
                rexpr as *mut Expr,
                0 as libc::c_int as AttrNumber,
                0 as *mut libc::c_char,
                true,
            );
            (*pstate).p_multiassign_exprs =
                lappend((*pstate).p_multiassign_exprs, tle as *mut libc::c_void);
        } else {
            let elevel__1: libc::c_int = 21 as libc::c_int;
            let mut __error_1: libc::c_int = 0;
            if elevel__1 >= 21 as libc::c_int {
                abort();
            }
        }
    } else {
        tle = (*list_last_cell((*pstate).p_multiassign_exprs)).ptr_value as *mut TargetEntry;
    }
    if (*((*tle).expr as *const Node)).tag as libc::c_uint
        == T_SubLink as libc::c_int as libc::c_uint
    {
        let mut param: *mut Param = 0 as *mut Param;
        sublink = (*tle).expr as *mut SubLink;
        qtree = (*sublink).subselect as *mut Query;
        tle = list_nth((*qtree).targetList, (*maref).colno - 1 as libc::c_int) as *mut TargetEntry;
        param = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).tag = T_Param;
            _result
        }) as *mut Param;
        (*param).paramkind = PARAM_MULTIEXPR;
        (*param).paramid = (*sublink).subLinkId << 16 as libc::c_int | (*maref).colno;
        (*param).paramtype = exprType((*tle).expr as *mut Node);
        (*param).paramtypmod = exprTypmod((*tle).expr as *mut Node);
        (*param).paramcollid = exprCollation((*tle).expr as *mut Node);
        (*param).location = exprLocation((*tle).expr as *mut Node);
        return param as *mut Node;
    }
    if (*((*tle).expr as *const Node)).tag as libc::c_uint
        == T_RowExpr as libc::c_int as libc::c_uint
    {
        let mut result: *mut Node = 0 as *mut Node;
        rexpr = (*tle).expr as *mut RowExpr;
        result = list_nth((*rexpr).args, (*maref).colno - 1 as libc::c_int) as *mut Node;
        if (*maref).colno == (*maref).ncolumns {
            (*pstate).p_multiassign_exprs = list_delete_last((*pstate).p_multiassign_exprs);
        }
        return result;
    }
    let elevel__2: libc::c_int = 21 as libc::c_int;
    let mut __error_2: libc::c_int = 0;
    if errstart(elevel__2, 0 as *const libc::c_char) != 0 {
        errmsg_internal(
            b"unexpected expr type in multiassign list\0" as *const u8 as *const libc::c_char,
        );
        errfinish(
            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_expr.c\0" as *const u8
                as *const libc::c_char,
            1537 as libc::c_int,
            0 as *const libc::c_char,
        );
    }
    if elevel__2 >= 21 as libc::c_int {
        abort();
    }
    return 0 as *mut Node;
}
unsafe extern "C" fn transformCaseExpr(
    mut pstate: *mut ParseState,
    mut c: *mut CaseExpr,
) -> *mut Node {
    let mut newc: *mut CaseExpr = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_CaseExpr;
        _result
    }) as *mut CaseExpr;
    let mut last_srf: *mut Node = (*pstate).p_last_srf;
    let mut arg: *mut Node = 0 as *mut Node;
    let mut placeholder: *mut CaseTestExpr = 0 as *mut CaseTestExpr;
    let mut newargs: *mut List = 0 as *mut List;
    let mut resultexprs: *mut List = 0 as *mut List;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut defresult: *mut Node = 0 as *mut Node;
    let mut ptype: Oid = 0;
    arg = transformExprRecurse(pstate, (*c).arg as *mut Node);
    if !arg.is_null() {
        assign_expr_collations(pstate, arg);
        placeholder = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).tag = T_CaseTestExpr;
            _result
        }) as *mut CaseTestExpr;
        (*placeholder).typeId = exprType(arg);
        (*placeholder).typeMod = exprTypmod(arg);
        (*placeholder).collation = exprCollation(arg);
    } else {
        placeholder = 0 as *mut CaseTestExpr;
    }
    (*newc).arg = arg as *mut Expr;
    newargs = 0 as *mut libc::c_void as *mut List;
    resultexprs = 0 as *mut libc::c_void as *mut List;
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: (*c).args,
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
        let mut w: *mut CaseWhen = (*l).ptr_value as *mut CaseWhen;
        let mut neww: *mut CaseWhen = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).tag = T_CaseWhen;
            _result
        }) as *mut CaseWhen;
        let mut warg: *mut Node = 0 as *mut Node;
        warg = (*w).expr as *mut Node;
        if !placeholder.is_null() {
            warg = makeSimpleA_Expr(
                AEXPR_OP,
                b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                placeholder as *mut Node,
                warg,
                (*w).location,
            ) as *mut Node;
        }
        (*neww).expr = transformExprRecurse(pstate, warg) as *mut Expr;
        (*neww).expr = coerce_to_boolean(
            pstate,
            (*neww).expr as *mut Node,
            b"CASE/WHEN\0" as *const u8 as *const libc::c_char,
        ) as *mut Expr;
        warg = (*w).result as *mut Node;
        (*neww).result = transformExprRecurse(pstate, warg) as *mut Expr;
        (*neww).location = (*w).location;
        newargs = lappend(newargs, neww as *mut libc::c_void);
        resultexprs = lappend(resultexprs, (*neww).result as *mut libc::c_void);
        l__state.i += 1;
        l__state.i;
    }
    (*newc).args = newargs;
    defresult = (*c).defresult as *mut Node;
    if defresult.is_null() {
        let mut n: *mut A_Const = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).tag = T_A_Const;
            _result
        }) as *mut A_Const;
        (*n).val.tag = T_Null;
        (*n).location = -(1 as libc::c_int);
        defresult = n as *mut Node;
    }
    (*newc).defresult = transformExprRecurse(pstate, defresult) as *mut Expr;
    resultexprs = lcons((*newc).defresult as *mut libc::c_void, resultexprs);
    ptype = select_common_type(
        pstate,
        resultexprs,
        b"CASE\0" as *const u8 as *const libc::c_char,
        0 as *mut *mut Node,
    );
    (*newc).casetype = ptype;
    (*newc).defresult = coerce_to_common_type(
        pstate,
        (*newc).defresult as *mut Node,
        ptype,
        b"CASE/ELSE\0" as *const u8 as *const libc::c_char,
    ) as *mut Expr;
    let mut l__state_0: ForEachState = {
        let mut init = ForEachState {
            l: (*newc).args,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(l__state_0.l).is_null() && l__state_0.i < (*l__state_0.l).length {
        l = &mut *((*l__state_0.l).elements).offset(l__state_0.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        l = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut w_0: *mut CaseWhen = (*l).ptr_value as *mut CaseWhen;
        (*w_0).result = coerce_to_common_type(
            pstate,
            (*w_0).result as *mut Node,
            ptype,
            b"CASE/WHEN\0" as *const u8 as *const libc::c_char,
        ) as *mut Expr;
        l__state_0.i += 1;
        l__state_0.i;
    }
    if (*pstate).p_last_srf != last_srf {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    (*newc).location = (*c).location;
    return newc as *mut Node;
}
unsafe extern "C" fn transformSubLink(
    mut pstate: *mut ParseState,
    mut sublink: *mut SubLink,
) -> *mut Node {
    let mut result: *mut Node = sublink as *mut Node;
    let mut qtree: *mut Query = 0 as *mut Query;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    err = 0 as *const libc::c_char;
    match (*pstate).p_expr_kind as libc::c_uint {
        1 => {}
        2 | 3 | 4 | 5 | 6 | 35 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19
        | 20 | 21 | 22 | 23 | 24 | 25 | 41 => {}
        26 | 27 => {
            err = b"cannot use subquery in check constraint\0" as *const u8 as *const libc::c_char;
        }
        28 | 29 => {
            err =
                b"cannot use subquery in DEFAULT expression\0" as *const u8 as *const libc::c_char;
        }
        30 => {
            err = b"cannot use subquery in index expression\0" as *const u8 as *const libc::c_char;
        }
        31 => {
            err = b"cannot use subquery in index predicate\0" as *const u8 as *const libc::c_char;
        }
        32 => {
            err = b"cannot use subquery in transform expression\0" as *const u8
                as *const libc::c_char;
        }
        33 => {
            err = b"cannot use subquery in EXECUTE parameter\0" as *const u8 as *const libc::c_char;
        }
        34 => {
            err = b"cannot use subquery in trigger WHEN condition\0" as *const u8
                as *const libc::c_char;
        }
        36 => {
            err = b"cannot use subquery in partition bound\0" as *const u8 as *const libc::c_char;
        }
        37 => {
            err = b"cannot use subquery in partition key expression\0" as *const u8
                as *const libc::c_char;
        }
        38 => {
            err = b"cannot use subquery in CALL argument\0" as *const u8 as *const libc::c_char;
        }
        39 => {
            err = b"cannot use subquery in COPY FROM WHERE condition\0" as *const u8
                as *const libc::c_char;
        }
        40 => {
            err = b"cannot use subquery in column generation expression\0" as *const u8
                as *const libc::c_char;
        }
        0 | _ => {}
    }
    if !err.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    (*pstate).p_hasSubLinks = true;
    qtree = parse_sub_analyze(
        (*sublink).subselect,
        pstate,
        0 as *mut CommonTableExpr,
        false,
        true,
    );
    if !((*(qtree as *const Node)).tag as libc::c_uint == T_Query as libc::c_int as libc::c_uint)
        || (*qtree).commandType as libc::c_uint != CMD_SELECT as libc::c_int as libc::c_uint
    {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"unexpected non-SELECT command in SubLink\0" as *const u8 as *const libc::c_char,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_expr.c\0"
                    as *const u8 as *const libc::c_char,
                1796 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    (*sublink).subselect = qtree as *mut Node;
    if (*sublink).subLinkType as libc::c_uint == EXISTS_SUBLINK as libc::c_int as libc::c_uint {
        (*sublink).testexpr = 0 as *mut Node;
        (*sublink).operName = 0 as *mut libc::c_void as *mut List;
    } else if (*sublink).subLinkType as libc::c_uint == EXPR_SUBLINK as libc::c_int as libc::c_uint
        || (*sublink).subLinkType as libc::c_uint == ARRAY_SUBLINK as libc::c_int as libc::c_uint
    {
        if count_nonjunk_tlist_entries((*qtree).targetList) != 1 as libc::c_int {
            let elevel__1: libc::c_int = 21 as libc::c_int;
            let mut __error_1: libc::c_int = 0;
            if elevel__1 >= 21 as libc::c_int {
                abort();
            }
        }
        (*sublink).testexpr = 0 as *mut Node;
        (*sublink).operName = 0 as *mut libc::c_void as *mut List;
    } else if (*sublink).subLinkType as libc::c_uint
        == MULTIEXPR_SUBLINK as libc::c_int as libc::c_uint
    {
        (*sublink).testexpr = 0 as *mut Node;
        (*sublink).operName = 0 as *mut libc::c_void as *mut List;
    } else {
        let mut lefthand: *mut Node = 0 as *mut Node;
        let mut left_list: *mut List = 0 as *mut List;
        let mut right_list: *mut List = 0 as *mut List;
        let mut l: *mut ListCell = 0 as *mut ListCell;
        if ((*sublink).operName).is_null() {
            (*sublink).operName = list_make1_impl(
                T_List,
                ListCell {
                    ptr_value: makeString(
                        b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) as *mut libc::c_void,
                },
            );
        }
        lefthand = transformExprRecurse(pstate, (*sublink).testexpr);
        if !lefthand.is_null()
            && (*(lefthand as *const Node)).tag as libc::c_uint
                == T_RowExpr as libc::c_int as libc::c_uint
        {
            left_list = (*(lefthand as *mut RowExpr)).args;
        } else {
            left_list = list_make1_impl(
                T_List,
                ListCell {
                    ptr_value: lefthand as *mut libc::c_void,
                },
            );
        }
        right_list = 0 as *mut libc::c_void as *mut List;
        let mut l__state: ForEachState = {
            let mut init = ForEachState {
                l: (*qtree).targetList,
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
            let mut tent: *mut TargetEntry = (*l).ptr_value as *mut TargetEntry;
            let mut param: *mut Param = 0 as *mut Param;
            if !((*tent).resjunk != 0) {
                param = ({
                    let mut _result: *mut Node = 0 as *mut Node;
                    (*_result).tag = T_Param;
                    _result
                }) as *mut Param;
                (*param).paramkind = PARAM_SUBLINK;
                (*param).paramid = (*tent).resno as libc::c_int;
                (*param).paramtype = exprType((*tent).expr as *mut Node);
                (*param).paramtypmod = exprTypmod((*tent).expr as *mut Node);
                (*param).paramcollid = exprCollation((*tent).expr as *mut Node);
                (*param).location = -(1 as libc::c_int);
                right_list = lappend(right_list, param as *mut libc::c_void);
            }
            l__state.i += 1;
            l__state.i;
        }
        if list_length(left_list) < list_length(right_list) {
            let elevel__2: libc::c_int = 21 as libc::c_int;
            let mut __error_2: libc::c_int = 0;
            if elevel__2 >= 21 as libc::c_int {
                abort();
            }
        }
        if list_length(left_list) > list_length(right_list) {
            let elevel__3: libc::c_int = 21 as libc::c_int;
            let mut __error_3: libc::c_int = 0;
            if elevel__3 >= 21 as libc::c_int {
                abort();
            }
        }
        (*sublink).testexpr = make_row_comparison_op(
            pstate,
            (*sublink).operName,
            left_list,
            right_list,
            (*sublink).location,
        );
    }
    return result;
}
unsafe extern "C" fn transformArrayExpr(
    mut pstate: *mut ParseState,
    mut a: *mut A_ArrayExpr,
    mut array_type: Oid,
    mut element_type: Oid,
    mut typmod: i32,
) -> *mut Node {
    let mut newa: *mut ArrayExpr = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_ArrayExpr;
        _result
    }) as *mut ArrayExpr;
    let mut newelems: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut newcoercedelems: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut element: *mut ListCell = 0 as *mut ListCell;
    let mut coerce_type: Oid = 0;
    let mut coerce_hard: bool = false;
    (*newa).multidims = false;
    let mut element__state: ForEachState = {
        let mut init = ForEachState {
            l: (*a).elements,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(element__state.l).is_null() && element__state.i < (*element__state.l).length {
        element =
            &mut *((*element__state.l).elements).offset(element__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        element = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut e: *mut Node = (*element).ptr_value as *mut Node;
        let mut newe: *mut Node = 0 as *mut Node;
        if (*(e as *const Node)).tag as libc::c_uint == T_A_ArrayExpr as libc::c_int as libc::c_uint
        {
            newe = transformArrayExpr(
                pstate,
                e as *mut A_ArrayExpr,
                array_type,
                element_type,
                typmod,
            );
            (*newa).multidims = true;
        } else {
            newe = transformExprRecurse(pstate, e);
            if (*newa).multidims == 0 && get_element_type(exprType(newe)) != 0 as libc::c_int as Oid
            {
                (*newa).multidims = true;
            }
        }
        newelems = lappend(newelems, newe as *mut libc::c_void);
        element__state.i += 1;
        element__state.i;
    }
    if (array_type != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
        coerce_type = if (*newa).multidims as libc::c_int != 0 {
            array_type
        } else {
            element_type
        };
        coerce_hard = true;
    } else {
        if newelems.is_null() {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        coerce_type = select_common_type(
            pstate,
            newelems,
            b"ARRAY\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut Node,
        );
        if (*newa).multidims != 0 {
            array_type = coerce_type;
            element_type = get_element_type(array_type);
            if (element_type != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
        } else {
            element_type = coerce_type;
            array_type = get_array_type(element_type);
            if (array_type != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        coerce_hard = false;
    }
    let mut element__state_0: ForEachState = {
        let mut init = ForEachState {
            l: newelems,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(element__state_0.l).is_null() && element__state_0.i < (*element__state_0.l).length {
        element = &mut *((*element__state_0.l).elements).offset(element__state_0.i as isize)
            as *mut ListCell;
        true as libc::c_int
    } else {
        element = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut e_0: *mut Node = (*element).ptr_value as *mut Node;
        let mut newe_0: *mut Node = 0 as *mut Node;
        if coerce_hard != 0 {
            newe_0 = coerce_to_target_type(
                pstate,
                e_0,
                exprType(e_0),
                coerce_type,
                typmod,
                COERCION_EXPLICIT,
                COERCE_EXPLICIT_CAST,
                -(1 as libc::c_int),
            );
            if newe_0.is_null() {
                let elevel__2: libc::c_int = 21 as libc::c_int;
                let mut __error_2: libc::c_int = 0;
                if elevel__2 >= 21 as libc::c_int {
                    abort();
                }
            }
        } else {
            newe_0 = coerce_to_common_type(
                pstate,
                e_0,
                coerce_type,
                b"ARRAY\0" as *const u8 as *const libc::c_char,
            );
        }
        newcoercedelems = lappend(newcoercedelems, newe_0 as *mut libc::c_void);
        element__state_0.i += 1;
        element__state_0.i;
    }
    (*newa).array_typeid = array_type;
    (*newa).element_typeid = element_type;
    (*newa).elements = newcoercedelems;
    (*newa).location = (*a).location;
    return newa as *mut Node;
}
unsafe extern "C" fn transformRowExpr(
    mut pstate: *mut ParseState,
    mut r: *mut RowExpr,
    mut allowDefault: bool,
) -> *mut Node {
    let mut newr: *mut RowExpr = 0 as *mut RowExpr;
    let mut fname: [libc::c_char; 16] = [0; 16];
    let mut fnum: libc::c_int = 0;
    newr = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_RowExpr;
        _result
    }) as *mut RowExpr;
    (*newr).args = transformExpressionList(pstate, (*r).args, (*pstate).p_expr_kind, allowDefault);
    (*newr).row_format = COERCE_IMPLICIT_CAST;
    (*newr).colnames = 0 as *mut libc::c_void as *mut List;
    fnum = 1 as libc::c_int;
    while fnum <= list_length((*newr).args) {
        pg_snprintf(
            fname.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"f%d\0" as *const u8 as *const libc::c_char,
            fnum,
        );
        (*newr).colnames = lappend(
            (*newr).colnames,
            makeString(pstrdup(fname.as_mut_ptr())) as *mut libc::c_void,
        );
        fnum += 1;
        fnum;
    }
    (*newr).location = (*r).location;
    return newr as *mut Node;
}
unsafe extern "C" fn transformCoalesceExpr(
    mut pstate: *mut ParseState,
    mut c: *mut CoalesceExpr,
) -> *mut Node {
    let mut newc: *mut CoalesceExpr = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_CoalesceExpr;
        _result
    }) as *mut CoalesceExpr;
    let mut last_srf: *mut Node = (*pstate).p_last_srf;
    let mut newargs: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut newcoercedargs: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut args: *mut ListCell = 0 as *mut ListCell;
    let mut args__state: ForEachState = {
        let mut init = ForEachState {
            l: (*c).args,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(args__state.l).is_null() && args__state.i < (*args__state.l).length {
        args = &mut *((*args__state.l).elements).offset(args__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        args = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut e: *mut Node = (*args).ptr_value as *mut Node;
        let mut newe: *mut Node = 0 as *mut Node;
        newe = transformExprRecurse(pstate, e);
        newargs = lappend(newargs, newe as *mut libc::c_void);
        args__state.i += 1;
        args__state.i;
    }
    (*newc).coalescetype = select_common_type(
        pstate,
        newargs,
        b"COALESCE\0" as *const u8 as *const libc::c_char,
        0 as *mut *mut Node,
    );
    let mut args__state_0: ForEachState = {
        let mut init = ForEachState {
            l: newargs,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(args__state_0.l).is_null() && args__state_0.i < (*args__state_0.l).length {
        args =
            &mut *((*args__state_0.l).elements).offset(args__state_0.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        args = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut e_0: *mut Node = (*args).ptr_value as *mut Node;
        let mut newe_0: *mut Node = 0 as *mut Node;
        newe_0 = coerce_to_common_type(
            pstate,
            e_0,
            (*newc).coalescetype,
            b"COALESCE\0" as *const u8 as *const libc::c_char,
        );
        newcoercedargs = lappend(newcoercedargs, newe_0 as *mut libc::c_void);
        args__state_0.i += 1;
        args__state_0.i;
    }
    if (*pstate).p_last_srf != last_srf {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    (*newc).args = newcoercedargs;
    (*newc).location = (*c).location;
    return newc as *mut Node;
}
unsafe extern "C" fn transformMinMaxExpr(
    mut pstate: *mut ParseState,
    mut m: *mut MinMaxExpr,
) -> *mut Node {
    let mut newm: *mut MinMaxExpr = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_MinMaxExpr;
        _result
    }) as *mut MinMaxExpr;
    let mut newargs: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut newcoercedargs: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut funcname: *const libc::c_char =
        if (*m).op as libc::c_uint == IS_GREATEST as libc::c_int as libc::c_uint {
            b"GREATEST\0" as *const u8 as *const libc::c_char
        } else {
            b"LEAST\0" as *const u8 as *const libc::c_char
        };
    let mut args: *mut ListCell = 0 as *mut ListCell;
    (*newm).op = (*m).op;
    let mut args__state: ForEachState = {
        let mut init = ForEachState {
            l: (*m).args,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(args__state.l).is_null() && args__state.i < (*args__state.l).length {
        args = &mut *((*args__state.l).elements).offset(args__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        args = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut e: *mut Node = (*args).ptr_value as *mut Node;
        let mut newe: *mut Node = 0 as *mut Node;
        newe = transformExprRecurse(pstate, e);
        newargs = lappend(newargs, newe as *mut libc::c_void);
        args__state.i += 1;
        args__state.i;
    }
    (*newm).minmaxtype = select_common_type(pstate, newargs, funcname, 0 as *mut *mut Node);
    let mut args__state_0: ForEachState = {
        let mut init = ForEachState {
            l: newargs,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(args__state_0.l).is_null() && args__state_0.i < (*args__state_0.l).length {
        args =
            &mut *((*args__state_0.l).elements).offset(args__state_0.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        args = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut e_0: *mut Node = (*args).ptr_value as *mut Node;
        let mut newe_0: *mut Node = 0 as *mut Node;
        newe_0 = coerce_to_common_type(pstate, e_0, (*newm).minmaxtype, funcname);
        newcoercedargs = lappend(newcoercedargs, newe_0 as *mut libc::c_void);
        args__state_0.i += 1;
        args__state_0.i;
    }
    (*newm).args = newcoercedargs;
    (*newm).location = (*m).location;
    return newm as *mut Node;
}
unsafe extern "C" fn transformSQLValueFunction(
    mut pstate: *mut ParseState,
    mut svf: *mut SQLValueFunction,
) -> *mut Node {
    match (*svf).op as libc::c_uint {
        2 => {
            (*svf).typmod = anytime_typmod_check(true, (*svf).typmod);
        }
        4 => {
            (*svf).typmod = anytimestamp_typmod_check(true, (*svf).typmod);
        }
        6 => {
            (*svf).typmod = anytime_typmod_check(false, (*svf).typmod);
        }
        8 => {
            (*svf).typmod = anytimestamp_typmod_check(false, (*svf).typmod);
        }
        0 | 1 | 3 | 5 | 7 | 9 | 10 | 11 | 12 | 13 | 14 | _ => {}
    }
    return svf as *mut Node;
}
unsafe extern "C" fn transformXmlExpr(
    mut pstate: *mut ParseState,
    mut x: *mut XmlExpr,
) -> *mut Node {
    let mut newx: *mut XmlExpr = 0 as *mut XmlExpr;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut i: libc::c_int = 0;
    newx = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_XmlExpr;
        _result
    }) as *mut XmlExpr;
    (*newx).op = (*x).op;
    if !((*x).name).is_null() {
        (*newx).name = map_sql_identifier_to_xml_name((*x).name, false, false);
    } else {
        (*newx).name = 0 as *mut libc::c_char;
    }
    (*newx).xmloption = (*x).xmloption;
    (*newx).typmod = -(1 as libc::c_int);
    (*newx).location = (*x).location;
    (*newx).named_args = 0 as *mut libc::c_void as *mut List;
    (*newx).arg_names = 0 as *mut libc::c_void as *mut List;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*x).named_args,
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
        let mut r: *mut ResTarget = (*lc).ptr_value as *mut ResTarget;
        let mut expr: *mut Node = 0 as *mut Node;
        let mut argname: *mut libc::c_char = 0 as *mut libc::c_char;
        expr = transformExprRecurse(pstate, (*r).val);
        if !((*r).name).is_null() {
            argname = map_sql_identifier_to_xml_name((*r).name, false, false);
        } else if (*((*r).val as *const Node)).tag as libc::c_uint
            == T_ColumnRef as libc::c_int as libc::c_uint
        {
            argname = map_sql_identifier_to_xml_name(FigureColname((*r).val), true, false);
        } else {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
            argname = 0 as *mut libc::c_char;
        }
        if (*x).op as libc::c_uint == IS_XMLELEMENT as libc::c_int as libc::c_uint {
            let mut lc2: *mut ListCell = 0 as *mut ListCell;
            let mut lc2__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*newx).arg_names,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(lc2__state.l).is_null() && lc2__state.i < (*lc2__state.l).length {
                lc2 =
                    &mut *((*lc2__state.l).elements).offset(lc2__state.i as isize) as *mut ListCell;
                true as libc::c_int
            } else {
                lc2 = 0 as *mut ListCell;
                false as libc::c_int
            } != 0
            {
                if strcmp(argname, (*((*lc2).ptr_value as *mut Value)).val.str_0)
                    == 0 as libc::c_int
                {
                    let elevel__0: libc::c_int = 21 as libc::c_int;
                    let mut __error_0: libc::c_int = 0;
                    if elevel__0 >= 21 as libc::c_int {
                        abort();
                    }
                }
                lc2__state.i += 1;
                lc2__state.i;
            }
        }
        (*newx).named_args = lappend((*newx).named_args, expr as *mut libc::c_void);
        (*newx).arg_names = lappend((*newx).arg_names, makeString(argname) as *mut libc::c_void);
        lc__state.i += 1;
        lc__state.i;
    }
    (*newx).args = 0 as *mut libc::c_void as *mut List;
    i = 0 as libc::c_int;
    let mut lc__state_0: ForEachState = {
        let mut init = ForEachState {
            l: (*x).args,
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
        let mut e: *mut Node = (*lc).ptr_value as *mut Node;
        let mut newe: *mut Node = 0 as *mut Node;
        newe = transformExprRecurse(pstate, e);
        match (*x).op as libc::c_uint {
            1 => {}
            3 => {
                if !(i == 0 as libc::c_int) {
                    newe = coerce_to_boolean(
                        pstate,
                        newe,
                        b"XMLPARSE\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            0 | 2 | 4 | 5 | 6 | 7 | _ => {}
        }
        (*newx).args = lappend((*newx).args, newe as *mut libc::c_void);
        i += 1;
        i;
        lc__state_0.i += 1;
        lc__state_0.i;
    }
    return newx as *mut Node;
}
unsafe extern "C" fn transformXmlSerialize(
    mut pstate: *mut ParseState,
    mut xs: *mut XmlSerialize,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut xexpr: *mut XmlExpr = 0 as *mut XmlExpr;
    let mut targetType: Oid = 0;
    let mut targetTypmod: i32 = 0;
    xexpr = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_XmlExpr;
        _result
    }) as *mut XmlExpr;
    (*xexpr).op = IS_XMLSERIALIZE;
    typenameTypeIdAndMod(pstate, (*xs).typeName, &mut targetType, &mut targetTypmod);
    (*xexpr).xmloption = (*xs).xmloption;
    (*xexpr).location = (*xs).location;
    (*xexpr).tag = targetType;
    (*xexpr).typmod = targetTypmod;
    if result.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return result;
}
unsafe extern "C" fn transformBooleanTest(
    mut pstate: *mut ParseState,
    mut b: *mut BooleanTest,
) -> *mut Node {
    let mut clausename: *const libc::c_char = 0 as *const libc::c_char;
    match (*b).booltesttype as libc::c_uint {
        0 => {
            clausename = b"IS TRUE\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            clausename = b"IS NOT TRUE\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            clausename = b"IS FALSE\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            clausename = b"IS NOT FALSE\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            clausename = b"IS UNKNOWN\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            clausename = b"IS NOT UNKNOWN\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unrecognized booltesttype: %d\0" as *const u8 as *const libc::c_char,
                    (*b).booltesttype as libc::c_int,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_expr.c\0"
                        as *const u8 as *const libc::c_char,
                    2442 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
            clausename = 0 as *const libc::c_char;
        }
    }
    (*b).arg = transformExprRecurse(pstate, (*b).arg as *mut Node) as *mut Expr;
    (*b).arg = coerce_to_boolean(pstate, (*b).arg as *mut Node, clausename) as *mut Expr;
    return b as *mut Node;
}
unsafe extern "C" fn transformWholeRowRef(
    mut pstate: *mut ParseState,
    mut nsitem: *mut ParseNamespaceItem,
    mut sublevels_up: libc::c_int,
    mut location: libc::c_int,
) -> *mut Node {
    let mut result: *mut Var = 0 as *mut Var;
    result = makeWholeRowVar(
        (*nsitem).p_rte,
        (*nsitem).p_rtindex as Index,
        sublevels_up as Index,
        true,
    );
    (*result).location = location;
    markVarForSelectPriv(pstate, result);
    return result as *mut Node;
}
unsafe extern "C" fn transformTypeCast(
    mut pstate: *mut ParseState,
    mut tc: *mut TypeCast,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut arg: *mut Node = (*tc).arg;
    let mut expr: *mut Node = 0 as *mut Node;
    let mut inputType: Oid = 0;
    let mut targetType: Oid = 0;
    let mut targetTypmod: i32 = 0;
    let mut location: libc::c_int = 0;
    typenameTypeIdAndMod(pstate, (*tc).typeName, &mut targetType, &mut targetTypmod);
    if (*(arg as *const Node)).tag as libc::c_uint == T_A_ArrayExpr as libc::c_int as libc::c_uint {
        let mut targetBaseType: Oid = 0;
        let mut targetBaseTypmod: i32 = 0;
        let mut elementType: Oid = 0;
        targetBaseTypmod = targetTypmod;
        targetBaseType = getBaseTypeAndTypmod(targetType, &mut targetBaseTypmod);
        elementType = get_element_type(targetBaseType);
        if (elementType != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
            expr = transformArrayExpr(
                pstate,
                arg as *mut A_ArrayExpr,
                targetBaseType,
                elementType,
                targetBaseTypmod,
            );
        } else {
            expr = transformExprRecurse(pstate, arg);
        }
    } else {
        expr = transformExprRecurse(pstate, arg);
    }
    inputType = exprType(expr);
    if inputType == 0 as libc::c_int as Oid {
        return expr;
    }
    location = (*tc).location;
    if location < 0 as libc::c_int {
        location = (*(*tc).typeName).location;
    }
    result = coerce_to_target_type(
        pstate,
        expr,
        inputType,
        targetType,
        targetTypmod,
        COERCION_EXPLICIT,
        COERCE_EXPLICIT_CAST,
        location,
    );
    if result.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return result;
}
unsafe extern "C" fn make_row_distinct_op(
    mut pstate: *mut ParseState,
    mut opname: *mut List,
    mut lrow: *mut RowExpr,
    mut rrow: *mut RowExpr,
    mut location: libc::c_int,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut largs: *mut List = (*lrow).args;
    let mut rargs: *mut List = (*rrow).args;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut r: *mut ListCell = 0 as *mut ListCell;
    if list_length(largs) != list_length(rargs) {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    let mut l__state: ForBothState = {
        let mut init = ForBothState {
            l1: largs,
            l2: rargs,
            i: 0 as libc::c_int,
        };
        init
    };
    loop {
        l = (if !(l__state.l1).is_null() && l__state.i < (*l__state.l1).length {
            &mut *((*l__state.l1).elements).offset(l__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        r = (if !(l__state.l2).is_null() && l__state.i < (*l__state.l2).length {
            &mut *((*l__state.l2).elements).offset(l__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        if !(!l.is_null() && !r.is_null()) {
            break;
        }
        let mut larg: *mut Node = (*l).ptr_value as *mut Node;
        let mut rarg: *mut Node = (*r).ptr_value as *mut Node;
        let mut cmp: *mut Node = 0 as *mut Node;
        cmp = make_distinct_op(pstate, opname, larg, rarg, location) as *mut Node;
        if result.is_null() {
            result = cmp;
        } else {
            result = makeBoolExpr(
                OR_EXPR,
                list_make2_impl(
                    T_List,
                    ListCell {
                        ptr_value: result as *mut libc::c_void,
                    },
                    ListCell {
                        ptr_value: cmp as *mut libc::c_void,
                    },
                ),
                location,
            ) as *mut Node;
        }
        l__state.i += 1;
        l__state.i;
    }
    if result.is_null() {
        result = makeBoolConst(false, false);
    }
    return result;
}
unsafe extern "C" fn make_nulltest_from_distinct(
    mut pstate: *mut ParseState,
    mut distincta: *mut A_Expr,
    mut arg: *mut Node,
) -> *mut Node {
    let mut nt: *mut NullTest = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_NullTest;
        _result
    }) as *mut NullTest;
    (*nt).arg = transformExprRecurse(pstate, arg) as *mut Expr;
    if (*distincta).kind as libc::c_uint == AEXPR_NOT_DISTINCT as libc::c_int as libc::c_uint {
        (*nt).nulltesttype = IS_NULL;
    } else {
        (*nt).nulltesttype = IS_NOT_NULL;
    }
    (*nt).argisrow = false;
    (*nt).location = (*distincta).location;
    return nt as *mut Node;
}
#[no_mangle]
pub unsafe extern "C" fn ParseExprKindName(mut exprKind: ParseExprKind) -> *const libc::c_char {
    match exprKind as libc::c_uint {
        0 => return b"invalid expression context\0" as *const u8 as *const libc::c_char,
        1 => return b"extension expression\0" as *const u8 as *const libc::c_char,
        2 => return b"JOIN/ON\0" as *const u8 as *const libc::c_char,
        3 => return b"JOIN/USING\0" as *const u8 as *const libc::c_char,
        4 => return b"sub-SELECT in FROM\0" as *const u8 as *const libc::c_char,
        5 => return b"function in FROM\0" as *const u8 as *const libc::c_char,
        6 => return b"WHERE\0" as *const u8 as *const libc::c_char,
        35 => return b"POLICY\0" as *const u8 as *const libc::c_char,
        7 => return b"HAVING\0" as *const u8 as *const libc::c_char,
        8 => return b"FILTER\0" as *const u8 as *const libc::c_char,
        9 => return b"window PARTITION BY\0" as *const u8 as *const libc::c_char,
        10 => return b"window ORDER BY\0" as *const u8 as *const libc::c_char,
        11 => return b"window RANGE\0" as *const u8 as *const libc::c_char,
        12 => return b"window ROWS\0" as *const u8 as *const libc::c_char,
        13 => return b"window GROUPS\0" as *const u8 as *const libc::c_char,
        14 => return b"SELECT\0" as *const u8 as *const libc::c_char,
        15 => return b"INSERT\0" as *const u8 as *const libc::c_char,
        16 | 17 => return b"UPDATE\0" as *const u8 as *const libc::c_char,
        18 => return b"GROUP BY\0" as *const u8 as *const libc::c_char,
        19 => return b"ORDER BY\0" as *const u8 as *const libc::c_char,
        20 => return b"DISTINCT ON\0" as *const u8 as *const libc::c_char,
        21 => return b"LIMIT\0" as *const u8 as *const libc::c_char,
        22 => return b"OFFSET\0" as *const u8 as *const libc::c_char,
        23 => return b"RETURNING\0" as *const u8 as *const libc::c_char,
        24 | 25 => return b"VALUES\0" as *const u8 as *const libc::c_char,
        26 | 27 => return b"CHECK\0" as *const u8 as *const libc::c_char,
        28 | 29 => return b"DEFAULT\0" as *const u8 as *const libc::c_char,
        30 => return b"index expression\0" as *const u8 as *const libc::c_char,
        31 => return b"index predicate\0" as *const u8 as *const libc::c_char,
        32 => return b"USING\0" as *const u8 as *const libc::c_char,
        33 => return b"EXECUTE\0" as *const u8 as *const libc::c_char,
        34 => return b"WHEN\0" as *const u8 as *const libc::c_char,
        36 => return b"partition bound\0" as *const u8 as *const libc::c_char,
        37 => return b"PARTITION BY\0" as *const u8 as *const libc::c_char,
        38 => return b"CALL\0" as *const u8 as *const libc::c_char,
        39 => return b"WHERE\0" as *const u8 as *const libc::c_char,
        40 => return b"GENERATED AS\0" as *const u8 as *const libc::c_char,
        41 => return b"CYCLE\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return b"unrecognized expression kind\0" as *const u8 as *const libc::c_char;
}
