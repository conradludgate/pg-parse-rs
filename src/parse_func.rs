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
//     pub type AttrMissing;
//     pub type RelationData;
//     pub type QueryEnvironment;
//     fn abort() -> !;
//     fn memcmp(
//         _: *const libc::c_void,
//         _: *const libc::c_void,
//         _: libc::c_ulong,
//     ) -> libc::c_int;
//     fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn pfree(pointer: *mut libc::c_void);
//     fn stringToNode(str: *const libc::c_char) -> *mut libc::c_void;
//     fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn list_truncate(list: *mut List, new_size: libc::c_int) -> *mut List;
//     fn list_delete_cell(list: *mut List, cell: *mut ListCell) -> *mut List;
//     fn list_copy_tail(list: *const List, nskip: libc::c_int) -> *mut List;
//     fn bms_add_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
//     fn bms_is_member(x: libc::c_int, a: *const Bitmapset) -> bool;
//     fn bms_free(a: *mut Bitmapset);
//     fn initStringInfo(str: StringInfo);
//     fn appendStringInfo(str: StringInfo, fmt: *const libc::c_char, _: ...);
//     fn appendStringInfoString(str: StringInfo, s: *const libc::c_char);
//     fn appendStringInfoChar(str: StringInfo, ch: libc::c_char);
//     fn get_expr_result_tupdesc(expr: *mut Node, noError: bool) -> TupleDesc;
//     fn makeTypeNameFromNameList(names: *mut List) -> *mut TypeName;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprLocation(expr: *const Node) -> libc::c_int;
//     fn setup_parser_errposition_callback(
//         pcbstate: *mut ParseCallbackState,
//         pstate: *mut ParseState,
//         location: libc::c_int,
//     );
//     fn cancel_parser_errposition_callback(pcbstate: *mut ParseCallbackState);
//     fn transformAggregateCall(
//         pstate: *mut ParseState,
//         agg: *mut Aggref,
//         args: *mut List,
//         aggorder: *mut List,
//         agg_distinct: bool,
//     );
//     fn transformWindowFuncCall(
//         pstate: *mut ParseState,
//         wfunc: *mut WindowFunc,
//         windef: *mut WindowDef,
//     );
//     fn transformWhereClause(
//         pstate: *mut ParseState,
//         clause: *mut Node,
//         exprKind: ParseExprKind,
//         constructName: *const libc::c_char,
//     ) -> *mut Node;
//     fn IsPreferredType(category: TYPCATEGORY, type_0: Oid) -> bool;
//     fn TypeCategory(type_0: Oid) -> TYPCATEGORY;
//     fn can_coerce_type(
//         nargs: libc::c_int,
//         input_typeids: *const Oid,
//         target_typeids: *const Oid,
//         ccontext: CoercionContext,
//     ) -> bool;
//     fn coerce_type(
//         pstate: *mut ParseState,
//         node: *mut Node,
//         inputTypeId: Oid,
//         targetTypeId: Oid,
//         targetTypeMod: i32,
//         ccontext: CoercionContext,
//         cformat: CoercionForm,
//         location: libc::c_int,
//     ) -> *mut Node;
//     fn select_common_type(
//         pstate: *mut ParseState,
//         exprs: *mut List,
//         context: *const libc::c_char,
//         which_expr: *mut *mut Node,
//     ) -> Oid;
//     fn select_common_typmod(
//         pstate: *mut ParseState,
//         exprs: *mut List,
//         common_type: Oid,
//     ) -> i32;
//     fn enforce_generic_type_consistency(
//         actual_arg_types: *const Oid,
//         declared_arg_types: *mut Oid,
//         nargs: libc::c_int,
//         rettype: Oid,
//         allow_poly: bool,
//     ) -> Oid;
//     fn find_coercion_pathway(
//         targetTypeId: Oid,
//         sourceTypeId: Oid,
//         ccontext: CoercionContext,
//         funcid: *mut Oid,
//     ) -> CoercionPathType;
//     fn FuncnameGetCandidates(
//         names: *mut List,
//         nargs: libc::c_int,
//         argnames: *mut List,
//         expand_variadic: bool,
//         expand_defaults: bool,
//         missing_ok: bool,
//     ) -> FuncCandidateList;
//     fn NameListToString(names: *mut List) -> *mut libc::c_char;
//     fn GetNSItemByRangeTablePosn(
//         pstate: *mut ParseState,
//         varno: libc::c_int,
//         sublevels_up: libc::c_int,
//     ) -> *mut ParseNamespaceItem;
//     fn scanNSItemForColumn(
//         pstate: *mut ParseState,
//         nsitem: *mut ParseNamespaceItem,
//         sublevels_up: libc::c_int,
//         colname: *const libc::c_char,
//         location: libc::c_int,
//     ) -> *mut Node;
//     fn expandRecordVariable(
//         pstate: *mut ParseState,
//         var: *mut Var,
//         levelsup: libc::c_int,
//     ) -> TupleDesc;
//     fn LookupTypeNameExtended(
//         pstate: *mut ParseState,
//         typeName: *const TypeName,
//         typmod_p: *mut i32,
//         temp_ok: bool,
//         missing_ok: bool,
//     ) -> Type;
//     fn LookupTypeNameOid(
//         pstate: *mut ParseState,
//         typeName: *const TypeName,
//         missing_ok: bool,
//     ) -> Oid;
//     fn typeTypeId(tp: Type) -> Oid;
//     fn typeTypeRelid(typ: Type) -> Oid;
//     fn text_to_cstring(t: *const text) -> *mut libc::c_char;
//     fn format_type_be(type_oid: Oid) -> *mut libc::c_char;
//     fn get_type_category_preferred(
//         typid: Oid,
//         typcategory: *mut libc::c_char,
//         typispreferred: *mut bool,
//     );
//     fn get_array_type(typid: Oid) -> Oid;
//     fn get_base_element_type(typid: Oid) -> Oid;
//     fn getBaseType(typid: Oid) -> Oid;
//     fn SearchSysCache1(cacheId: libc::c_int, key1: Datum) -> HeapTuple;
//     fn ReleaseSysCache(tuple: HeapTuple);
// }
use super::*;
// pub type Oid = libc::c_uint;
// pub type usize = libc::c_ulong;
// pub type bool = libc::c_uchar;
pub type Pointer = *mut libc::c_char;
// pub type i16 = libc::c_short;
// pub type i32 = libc::c_int;
// pub type u8 = libc::c_uchar;
// pub type u16 = libc::c_ushort;
// pub type u32 = libc::c_uint;
pub type bits8 = u8;
// pub type uint64 = libc::c_ulong;
// pub type Index = libc::c_uint;
pub type float4 = libc::c_float;
pub type regproc = Oid;
pub type TransactionId = u32;
pub type CommandId = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct varlena {
    pub vl_len_: [libc::c_char; 4],
    pub vl_dat: [libc::c_char; 0],
}
pub type text = varlena;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oidvector {
    pub vl_len_: i32,
    pub ndim: libc::c_int,
    pub dataoffset: i32,
    pub elemtype: Oid,
    pub dim1: libc::c_int,
    pub lbound1: libc::c_int,
    pub values: [Oid; 0],
}
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
pub type Form_pg_attribute = *mut FormData_pg_attribute;
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
pub struct FuncExpr {
    pub xpr: Expr,
    pub funcid: Oid,
    pub funcresulttype: Oid,
    pub funcretset: bool,
    pub funcvariadic: bool,
    pub funcformat: CoercionForm,
    pub funccollid: Oid,
    pub inputcollid: Oid,
    pub args: *mut List,
    pub location: libc::c_int,
}
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
pub struct FieldSelect {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub fieldnum: AttrNumber,
    pub resulttype: Oid,
    pub resulttypmod: i32,
    pub resultcollid: Oid,
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
pub struct FormData_pg_aggregate {
    pub aggfnoid: regproc,
    pub aggkind: libc::c_char,
    pub aggnumdirectargs: i16,
    pub aggtransfn: regproc,
    pub aggfinalfn: regproc,
    pub aggcombinefn: regproc,
    pub aggserialfn: regproc,
    pub aggdeserialfn: regproc,
    pub aggmtransfn: regproc,
    pub aggminvtransfn: regproc,
    pub aggmfinalfn: regproc,
    pub aggfinalextra: bool,
    pub aggmfinalextra: bool,
    pub aggfinalmodify: libc::c_char,
    pub aggmfinalmodify: libc::c_char,
    pub aggsortop: Oid,
    pub aggtranstype: Oid,
    pub aggtransspace: i32,
    pub aggmtranstype: Oid,
    pub aggmtransspace: i32,
}
pub type Form_pg_aggregate = *mut FormData_pg_aggregate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_proc {
    pub oid: Oid,
    pub proname: NameData,
    pub pronamespace: Oid,
    pub proowner: Oid,
    pub prolang: Oid,
    pub procost: float4,
    pub prorows: float4,
    pub provariadic: Oid,
    pub prosupport: regproc,
    pub prokind: libc::c_char,
    pub prosecdef: bool,
    pub proleakproof: bool,
    pub proisstrict: bool,
    pub proretset: bool,
    pub provolatile: libc::c_char,
    pub proparallel: libc::c_char,
    pub pronargs: i16,
    pub pronargdefaults: i16,
    pub prorettype: Oid,
    pub proargtypes: oidvector,
}
pub type Form_pg_proc = *mut FormData_pg_proc;
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
pub type TYPCATEGORY = libc::c_char;
pub type CoercionPathType = libc::c_uint;
pub const COERCION_PATH_COERCEVIAIO: CoercionPathType = 4;
pub const COERCION_PATH_ARRAYCOERCE: CoercionPathType = 3;
pub const COERCION_PATH_RELABELTYPE: CoercionPathType = 2;
pub const COERCION_PATH_FUNC: CoercionPathType = 1;
pub const COERCION_PATH_NONE: CoercionPathType = 0;
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
pub const AGGFNOID: SysCacheIdentifier = 0;
pub const PROCOID: SysCacheIdentifier = 43;
pub type Type = HeapTuple;
pub const FUNCLOOKUP_AMBIGUOUS: FuncLookupError = 1;
pub const FUNCLOOKUP_NOSUCHFUNC: FuncLookupError = 0;
pub type FuncLookupError = libc::c_uint;
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
pub unsafe extern "C" fn func_match_argtypes(
    mut nargs: libc::c_int,
    mut input_typeids: *mut Oid,
    mut raw_candidates: FuncCandidateList,
    mut candidates: *mut FuncCandidateList,
) -> libc::c_int {
    let mut current_candidate: FuncCandidateList = 0 as *mut _FuncCandidateList;
    let mut next_candidate: FuncCandidateList = 0 as *mut _FuncCandidateList;
    let mut ncandidates: libc::c_int = 0 as libc::c_int;
    *candidates = 0 as FuncCandidateList;
    current_candidate = raw_candidates;
    while !current_candidate.is_null() {
        next_candidate = (*current_candidate).next;
        if can_coerce_type(
            nargs,
            input_typeids,
            ((*current_candidate).args).as_mut_ptr(),
            COERCION_IMPLICIT,
        ) != 0
        {
            (*current_candidate).next = *candidates;
            *candidates = current_candidate;
            ncandidates += 1;
            ncandidates;
        }
        current_candidate = next_candidate;
    }
    return ncandidates;
}
#[no_mangle]
pub unsafe extern "C" fn make_fn_arguments(
    mut pstate: *mut ParseState,
    mut fargs: *mut List,
    mut actual_arg_types: *mut Oid,
    mut declared_arg_types: *mut Oid,
) {
    let mut current_fargs: *mut ListCell = 0 as *mut ListCell;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut current_fargs__state: ForEachState = {
        let mut init = ForEachState {
            l: fargs,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(current_fargs__state.l).is_null()
        && current_fargs__state.i < (*current_fargs__state.l).length
    {
        current_fargs = &mut *((*current_fargs__state.l).elements)
            .offset(current_fargs__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        current_fargs = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        if *actual_arg_types.offset(i as isize) != *declared_arg_types.offset(i as isize) {
            let mut node: *mut Node = (*current_fargs).ptr_value as *mut Node;
            if (*(node as *const Node)).tag as libc::c_uint
                == T_NamedArgExpr as libc::c_int as libc::c_uint
            {
                let mut na: *mut NamedArgExpr = node as *mut NamedArgExpr;
                node = coerce_type(
                    pstate,
                    (*na).arg as *mut Node,
                    *actual_arg_types.offset(i as isize),
                    *declared_arg_types.offset(i as isize),
                    -(1 as libc::c_int),
                    COERCION_IMPLICIT,
                    COERCE_IMPLICIT_CAST,
                    -(1 as libc::c_int),
                );
                (*na).arg = node as *mut Expr;
            } else {
                node = coerce_type(
                    pstate,
                    node,
                    *actual_arg_types.offset(i as isize),
                    *declared_arg_types.offset(i as isize),
                    -(1 as libc::c_int),
                    COERCION_IMPLICIT,
                    COERCE_IMPLICIT_CAST,
                    -(1 as libc::c_int),
                );
                (*current_fargs).ptr_value = node as *mut libc::c_void;
            }
        }
        i += 1;
        i;
        current_fargs__state.i += 1;
        current_fargs__state.i;
    }
}
unsafe extern "C" fn FuncNameAsType(mut funcname: *mut List) -> Oid {
    let mut result: Oid = 0;
    let mut typtup: Type = 0 as *mut HeapTupleData;
    typtup = LookupTypeNameExtended(
        0 as *mut ParseState,
        makeTypeNameFromNameList(funcname),
        0 as *mut i32,
        false,
        false,
    );
    if typtup.is_null() {
        return 0 as libc::c_int as Oid;
    }
    if (*(((*typtup).t_data as *mut libc::c_char)
        .offset((*(*typtup).t_data).t_hoff as libc::c_int as isize) as Form_pg_type))
        .typisdefined as libc::c_int
        != 0
        && (typeTypeRelid(typtup) != 0 as libc::c_int as Oid) as libc::c_int as bool == 0
    {
        result = typeTypeId(typtup);
    } else {
        result = 0 as libc::c_int as Oid;
    }
    ReleaseSysCache(typtup);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn funcname_signature_string(
    mut funcname: *const libc::c_char,
    mut nargs: libc::c_int,
    mut argnames: *mut List,
    mut argtypes: *const Oid,
) -> *const libc::c_char {
    let mut argbuf: StringInfoData = StringInfoData {
        data: 0 as *mut libc::c_char,
        len: 0,
        maxlen: 0,
        cursor: 0,
    };
    let mut numposargs: libc::c_int = 0;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut i: libc::c_int = 0;
    initStringInfo(&mut argbuf);
    appendStringInfo(
        &mut argbuf as *mut StringInfoData,
        b"%s(\0" as *const u8 as *const libc::c_char,
        funcname,
    );
    numposargs = nargs - list_length(argnames);
    lc = list_head(argnames);
    i = 0 as libc::c_int;
    while i < nargs {
        if i != 0 {
            appendStringInfoString(&mut argbuf, b", \0" as *const u8 as *const libc::c_char);
        }
        if i >= numposargs {
            appendStringInfo(
                &mut argbuf as *mut StringInfoData,
                b"%s => \0" as *const u8 as *const libc::c_char,
                (*lc).ptr_value as *mut libc::c_char,
            );
            lc = lnext(argnames, lc);
        }
        appendStringInfoString(&mut argbuf, format_type_be(*argtypes.offset(i as isize)));
        i += 1;
        i;
    }
    appendStringInfoChar(&mut argbuf, ')' as i32 as libc::c_char);
    return argbuf.data;
}
#[no_mangle]
pub unsafe extern "C" fn func_signature_string(
    mut funcname: *mut List,
    mut nargs: libc::c_int,
    mut argnames: *mut List,
    mut argtypes: *const Oid,
) -> *const libc::c_char {
    return funcname_signature_string(NameListToString(funcname), nargs, argnames, argtypes);
}
unsafe extern "C" fn LookupFuncNameInternal(
    mut funcname: *mut List,
    mut nargs: libc::c_int,
    mut argtypes: *const Oid,
    mut missing_ok: bool,
    mut lookupError: *mut FuncLookupError,
) -> Oid {
    let mut clist: FuncCandidateList = 0 as *mut _FuncCandidateList;
    *lookupError = FUNCLOOKUP_NOSUCHFUNC;
    clist = FuncnameGetCandidates(
        funcname,
        nargs,
        0 as *mut libc::c_void as *mut List,
        false,
        false,
        missing_ok,
    );
    if nargs < 0 as libc::c_int {
        if !clist.is_null() {
            if !((*clist).next).is_null() {
                *lookupError = FUNCLOOKUP_AMBIGUOUS;
                return 0 as libc::c_int as Oid;
            }
            return (*clist).oid;
        } else {
            return 0 as libc::c_int as Oid;
        }
    }
    while !clist.is_null() {
        if nargs == 0 as libc::c_int
            || memcmp(
                argtypes as *const libc::c_void,
                ((*clist).args).as_mut_ptr() as *const libc::c_void,
                (nargs as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<Oid>() as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            return (*clist).oid;
        }
        clist = (*clist).next;
    }
    return 0 as libc::c_int as Oid;
}
#[no_mangle]
pub unsafe extern "C" fn LookupFuncName(
    mut funcname: *mut List,
    mut nargs: libc::c_int,
    mut argtypes: *const Oid,
    mut missing_ok: bool,
) -> Oid {
    let mut funcoid: Oid = 0;
    let mut lookupError: FuncLookupError = FUNCLOOKUP_NOSUCHFUNC;
    funcoid = LookupFuncNameInternal(funcname, nargs, argtypes, missing_ok, &mut lookupError);
    if (funcoid != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
        return funcoid;
    }
    match lookupError as libc::c_uint {
        0 => {
            if missing_ok {
                return 0 as libc::c_int as Oid;
            }
            if nargs < 0 as libc::c_int {
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
            }
        }
        1 => {
            let elevel__1: libc::c_int = 21 as libc::c_int;
            let mut __error_1: libc::c_int = 0;
            if elevel__1 >= 21 as libc::c_int {
                abort();
            }
        }
        _ => {}
    }
    return 0 as libc::c_int as Oid;
}
#[no_mangle]
pub unsafe extern "C" fn check_srf_call_placement(
    mut pstate: *mut ParseState,
    mut last_srf: *mut Node,
    mut location: libc::c_int,
) {
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    let mut errkind: bool = false;
    err = 0 as *const libc::c_char;
    errkind = false;
    match (*pstate).p_expr_kind as libc::c_uint {
        1 => {}
        2 | 3 => {
            err = b"set-returning functions are not allowed in JOIN conditions\0" as *const u8
                as *const libc::c_char;
        }
        4 => {
            errkind = true;
        }
        5 => {
            if (*pstate).p_last_srf != last_srf {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        6 => {
            errkind = true;
        }
        35 => {
            err = b"set-returning functions are not allowed in policy expressions\0" as *const u8
                as *const libc::c_char;
        }
        7 => {
            errkind = true;
        }
        8 => {
            errkind = true;
        }
        9 | 10 => {
            (*pstate).p_hasTargetSRFs = true;
        }
        11 | 12 | 13 => {
            err = b"set-returning functions are not allowed in window definitions\0" as *const u8
                as *const libc::c_char;
        }
        14 | 15 => {
            (*pstate).p_hasTargetSRFs = true;
        }
        16 | 17 => {
            errkind = true;
        }
        18 | 19 => {
            (*pstate).p_hasTargetSRFs = true;
        }
        20 => {
            (*pstate).p_hasTargetSRFs = true;
        }
        21 | 22 => {
            errkind = true;
        }
        23 => {
            errkind = true;
        }
        24 => {
            errkind = true;
        }
        25 => {
            (*pstate).p_hasTargetSRFs = true;
        }
        26 | 27 => {
            err = b"set-returning functions are not allowed in check constraints\0" as *const u8
                as *const libc::c_char;
        }
        28 | 29 => {
            err = b"set-returning functions are not allowed in DEFAULT expressions\0" as *const u8
                as *const libc::c_char;
        }
        30 => {
            err = b"set-returning functions are not allowed in index expressions\0" as *const u8
                as *const libc::c_char;
        }
        31 => {
            err = b"set-returning functions are not allowed in index predicates\0" as *const u8
                as *const libc::c_char;
        }
        32 => {
            err = b"set-returning functions are not allowed in transform expressions\0" as *const u8
                as *const libc::c_char;
        }
        33 => {
            err = b"set-returning functions are not allowed in EXECUTE parameters\0" as *const u8
                as *const libc::c_char;
        }
        34 => {
            err = b"set-returning functions are not allowed in trigger WHEN conditions\0"
                as *const u8 as *const libc::c_char;
        }
        36 => {
            err = b"set-returning functions are not allowed in partition bound\0" as *const u8
                as *const libc::c_char;
        }
        37 => {
            err = b"set-returning functions are not allowed in partition key expressions\0"
                as *const u8 as *const libc::c_char;
        }
        38 => {
            err = b"set-returning functions are not allowed in CALL arguments\0" as *const u8
                as *const libc::c_char;
        }
        39 => {
            err = b"set-returning functions are not allowed in COPY FROM WHERE conditions\0"
                as *const u8 as *const libc::c_char;
        }
        40 => {
            err = b"set-returning functions are not allowed in column generation expressions\0"
                as *const u8 as *const libc::c_char;
        }
        41 => {
            errkind = true;
        }
        0 | _ => {}
    }
    if !err.is_null() {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    if errkind {
        let elevel__1: libc::c_int = 21 as libc::c_int;
        let mut __error_1: libc::c_int = 0;
        if elevel__1 >= 21 as libc::c_int {
            abort();
        }
    }
}
