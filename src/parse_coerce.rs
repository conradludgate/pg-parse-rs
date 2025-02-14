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
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
//     fn psprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
//     fn DecrTupleDescRefCount(tupdesc: TupleDesc);
//     fn pg_detoast_datum(datum: *mut varlena) -> *mut varlena;
//     fn typeInheritsFrom(subclassTypeId: Oid, superclassTypeId: Oid) -> bool;
//     fn makeFuncExpr(
//         funcid: Oid,
//         rettype: Oid,
//         args: *mut List,
//         funccollid: Oid,
//         inputcollid: Oid,
//         fformat: CoercionForm,
//     ) -> *mut FuncExpr;
//     fn makeRelabelType(
//         arg: *mut Expr,
//         rtype: Oid,
//         rtypmod: i32,
//         rcollid: Oid,
//         rformat: CoercionForm,
//     ) -> *mut RelabelType;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprTypmod(expr: *const Node) -> i32;
//     fn expression_returns_set(clause: *mut Node) -> bool;
//     fn exprLocation(expr: *const Node) -> libc::c_int;
//     fn parser_errposition(pstate: *mut ParseState, location: libc::c_int) -> libc::c_int;
//     fn setup_parser_errposition_callback(
//         pcbstate: *mut ParseCallbackState,
//         pstate: *mut ParseState,
//         location: libc::c_int,
//     );
//     fn cancel_parser_errposition_callback(pcbstate: *mut ParseCallbackState);
//     fn GetNSItemByRangeTablePosn(
//         pstate: *mut ParseState,
//         varno: libc::c_int,
//         sublevels_up: libc::c_int,
//     ) -> *mut ParseNamespaceItem;
//     fn expandNSItemVars(
//         nsitem: *mut ParseNamespaceItem,
//         sublevels_up: libc::c_int,
//         location: libc::c_int,
//         colnames: *mut *mut List,
//     ) -> *mut List;
//     fn typeidType(id: Oid) -> Type;
//     fn typeLen(t: Type) -> i16;
//     fn typeByVal(t: Type) -> bool;
//     fn typeTypeCollation(typ: Type) -> Oid;
//     fn stringTypeDatum(tp: Type, string: *mut libc::c_char, atttypmod: i32) -> Datum;
//     fn typeOrDomainTypeRelid(type_id: Oid) -> Oid;
//     fn format_type_be(type_oid: Oid) -> *mut libc::c_char;
//     fn type_is_enum(typid: Oid) -> bool;
//     fn type_is_range(typid: Oid) -> bool;
//     fn type_is_multirange(typid: Oid) -> bool;
//     fn get_type_category_preferred(
//         typid: Oid,
//         typcategory: *mut libc::c_char,
//         typispreferred: *mut bool,
//     );
//     fn get_element_type(typid: Oid) -> Oid;
//     fn get_array_type(typid: Oid) -> Oid;
//     fn get_base_element_type(typid: Oid) -> Oid;
//     fn getBaseType(typid: Oid) -> Oid;
//     fn getBaseTypeAndTypmod(typid: Oid, typmod: *mut i32) -> Oid;
//     fn get_range_subtype(rangeOid: Oid) -> Oid;
//     fn get_range_multirange(rangeOid: Oid) -> Oid;
//     fn get_multirange_range(multirangeOid: Oid) -> Oid;
//     fn SearchSysCache1(cacheId: libc::c_int, key1: Datum) -> HeapTuple;
//     fn SearchSysCache2(cacheId: libc::c_int, key1: Datum, key2: Datum) -> HeapTuple;
//     fn ReleaseSysCache(tuple: HeapTuple);
//     fn lookup_rowtype_tupdesc(type_id: Oid, typmod: i32) -> TupleDesc;
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
pub type uint64 = libc::c_ulong;
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
// pub type CoercionContext = libc::c_uint;
pub const COERCION_EXPLICIT: CoercionContext = 3;
pub const COERCION_PLPGSQL: CoercionContext = 2;
pub const COERCION_ASSIGNMENT: CoercionContext = 1;
pub const COERCION_IMPLICIT: CoercionContext = 0;
// pub type CoercionForm = libc::c_uint;
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
pub struct RelabelType {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub resulttype: Oid,
    pub resulttypmod: i32,
    pub resultcollid: Oid,
    pub relabelformat: CoercionForm,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoerceViaIO {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub resulttype: Oid,
    pub resultcollid: Oid,
    pub coerceformat: CoercionForm,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArrayCoerceExpr {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub elemexpr: *mut Expr,
    pub resulttype: Oid,
    pub resulttypmod: i32,
    pub resultcollid: Oid,
    pub coerceformat: CoercionForm,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConvertRowtypeExpr {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub resulttype: Oid,
    pub convertformat: CoercionForm,
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
pub struct CaseTestExpr {
    pub xpr: Expr,
    pub typeId: Oid,
    pub typeMod: i32,
    pub collation: Oid,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_cast {
    pub oid: Oid,
    pub castsource: Oid,
    pub casttarget: Oid,
    pub castfunc: Oid,
    pub castcontext: libc::c_char,
    pub castmethod: libc::c_char,
}
pub type Form_pg_cast = *mut FormData_pg_cast;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_class {
    pub oid: Oid,
    pub relname: NameData,
    pub relnamespace: Oid,
    pub reltype: Oid,
    pub reloftype: Oid,
    pub relowner: Oid,
    pub relam: Oid,
    pub relfilenode: Oid,
    pub reltablespace: Oid,
    pub relpages: i32,
    pub reltuples: float4,
    pub relallvisible: i32,
    pub reltoastrelid: Oid,
    pub relhasindex: bool,
    pub relisshared: bool,
    pub relpersistence: libc::c_char,
    pub relkind: libc::c_char,
    pub relnatts: i16,
    pub relchecks: i16,
    pub relhasrules: bool,
    pub relhastriggers: bool,
    pub relhassubclass: bool,
    pub relrowsecurity: bool,
    pub relforcerowsecurity: bool,
    pub relispopulated: bool,
    pub relreplident: libc::c_char,
    pub relispartition: bool,
    pub relrewrite: Oid,
    pub relfrozenxid: TransactionId,
    pub relminmxid: TransactionId,
}
pub type Form_pg_class = *mut FormData_pg_class;
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
pub const CASTSOURCETARGET: SysCacheIdentifier = 12;
pub const PROCOID: SysCacheIdentifier = 43;
pub type Type = HeapTuple;
pub const RELOID: SysCacheIdentifier = 51;
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
unsafe extern "C" fn list_cell_number(mut l: *const List, mut c: *const ListCell) -> libc::c_int {
    return c.offset_from((*l).elements) as libc::c_long as libc::c_int;
}
#[inline]
unsafe extern "C" fn list_nth_cell(mut list: *const List, mut n: libc::c_int) -> *mut ListCell {
    return &mut *((*list).elements).offset(n as isize) as *mut ListCell;
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
unsafe extern "C" fn list_second_cell(mut l: *const List) -> *mut ListCell {
    if !l.is_null() && (*l).length >= 2 as libc::c_int {
        return &mut *((*l).elements).offset(1 as libc::c_int as isize) as *mut ListCell;
    } else {
        return 0 as *mut ListCell;
    };
}
#[inline]
unsafe extern "C" fn list_head(mut l: *const List) -> *mut ListCell {
    return if !l.is_null() {
        &mut *((*l).elements).offset(0 as libc::c_int as isize) as *mut ListCell
    } else {
        0 as *mut ListCell
    };
}
#[no_mangle]
pub unsafe extern "C" fn coerce_to_target_type(
    mut pstate: *mut ParseState,
    mut expr: *mut Node,
    mut exprtype: Oid,
    mut targettype: Oid,
    mut targettypmod: i32,
    mut ccontext: CoercionContext,
    mut cformat: CoercionForm,
    mut location: libc::c_int,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut origexpr: *mut Node = 0 as *mut Node;
    if can_coerce_type(1 as libc::c_int, &mut exprtype, &mut targettype, ccontext) == 0 {
        return 0 as *mut Node;
    }
    origexpr = expr;
    while !expr.is_null()
        && (*(expr as *const Node)).tag as libc::c_uint
            == T_CollateExpr as libc::c_int as libc::c_uint
    {
        expr = (*(expr as *mut CollateExpr)).arg as *mut Node;
    }
    result = coerce_type(
        pstate,
        expr,
        exprtype,
        targettype,
        targettypmod,
        ccontext,
        cformat,
        location,
    );
    result = coerce_type_typmod(
        result,
        targettype,
        targettypmod,
        ccontext,
        cformat,
        location,
        (result != expr
            && !((*(result as *const Node)).tag as libc::c_uint
                == T_Const as libc::c_int as libc::c_uint)) as libc::c_int as bool,
    );
    if expr != origexpr {
        let mut coll: *mut CollateExpr = origexpr as *mut CollateExpr;
        let mut newcoll: *mut CollateExpr = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).tag = T_CollateExpr;
            _result
        }) as *mut CollateExpr;
        (*newcoll).arg = result as *mut Expr;
        (*newcoll).collOid = (*coll).collOid;
        (*newcoll).location = (*coll).location;
        result = newcoll as *mut Node;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn coerce_to_domain(
    mut arg: *mut Node,
    mut baseTypeId: Oid,
    mut baseTypeMod: i32,
    mut typeId: Oid,
    mut ccontext: CoercionContext,
    mut cformat: CoercionForm,
    mut location: libc::c_int,
    mut hideInputCoercion: bool,
) -> *mut Node {
    let mut result: *mut CoerceToDomain = 0 as *mut CoerceToDomain;
    if baseTypeId == 0 as libc::c_int as Oid {
        baseTypeId = getBaseTypeAndTypmod(typeId, &mut baseTypeMod);
    }
    if baseTypeId == typeId {
        return arg;
    }
    if hideInputCoercion != 0 {
        hide_coercion_node(arg);
    }
    arg = coerce_type_typmod(
        arg,
        baseTypeId,
        baseTypeMod,
        ccontext,
        COERCE_IMPLICIT_CAST,
        location,
        false,
    );
    result = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_CoerceToDomain;
        _result
    }) as *mut CoerceToDomain;
    (*result).arg = arg as *mut Expr;
    (*result).resulttype = typeId;
    (*result).resulttypmod = -(1 as libc::c_int);
    (*result).coercionformat = cformat;
    (*result).location = location;
    return result as *mut Node;
}
unsafe extern "C" fn coerce_type_typmod(
    mut node: *mut Node,
    mut targetTypeId: Oid,
    mut targetTypMod: i32,
    mut ccontext: CoercionContext,
    mut cformat: CoercionForm,
    mut location: libc::c_int,
    mut hideInputCoercion: bool,
) -> *mut Node {
    let mut pathtype: CoercionPathType = COERCION_PATH_NONE;
    let mut funcId: Oid = 0;
    if targetTypMod < 0 as libc::c_int || targetTypMod == exprTypmod(node) {
        return node;
    }
    pathtype = find_typmod_coercion_function(targetTypeId, &mut funcId);
    if pathtype as libc::c_uint != COERCION_PATH_NONE as libc::c_int as libc::c_uint {
        if hideInputCoercion != 0 {
            hide_coercion_node(node);
        }
        node = build_coercion_expression(
            node,
            pathtype,
            funcId,
            targetTypeId,
            targetTypMod,
            ccontext,
            cformat,
            location,
        );
    }
    return node;
}
unsafe extern "C" fn hide_coercion_node(mut node: *mut Node) {
    if (*(node as *const Node)).tag as libc::c_uint == T_FuncExpr as libc::c_int as libc::c_uint {
        (*(node as *mut FuncExpr)).funcformat = COERCE_IMPLICIT_CAST;
    } else if (*(node as *const Node)).tag as libc::c_uint
        == T_RelabelType as libc::c_int as libc::c_uint
    {
        (*(node as *mut RelabelType)).relabelformat = COERCE_IMPLICIT_CAST;
    } else if (*(node as *const Node)).tag as libc::c_uint
        == T_CoerceViaIO as libc::c_int as libc::c_uint
    {
        (*(node as *mut CoerceViaIO)).coerceformat = COERCE_IMPLICIT_CAST;
    } else if (*(node as *const Node)).tag as libc::c_uint
        == T_ArrayCoerceExpr as libc::c_int as libc::c_uint
    {
        (*(node as *mut ArrayCoerceExpr)).coerceformat = COERCE_IMPLICIT_CAST;
    } else if (*(node as *const Node)).tag as libc::c_uint
        == T_ConvertRowtypeExpr as libc::c_int as libc::c_uint
    {
        (*(node as *mut ConvertRowtypeExpr)).convertformat = COERCE_IMPLICIT_CAST;
    } else if (*(node as *const Node)).tag as libc::c_uint
        == T_RowExpr as libc::c_int as libc::c_uint
    {
        (*(node as *mut RowExpr)).row_format = COERCE_IMPLICIT_CAST;
    } else if (*(node as *const Node)).tag as libc::c_uint
        == T_CoerceToDomain as libc::c_int as libc::c_uint
    {
        (*(node as *mut CoerceToDomain)).coercionformat = COERCE_IMPLICIT_CAST;
    } else {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"unsupported node type: %d\0" as *const u8 as *const libc::c_char,
                (*(node as *const Node)).tag as libc::c_int,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_coerce.c\0"
                    as *const u8 as *const libc::c_char,
                805 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    };
}
unsafe extern "C" fn build_coercion_expression(
    mut node: *mut Node,
    mut pathtype: CoercionPathType,
    mut funcId: Oid,
    mut targetTypeId: Oid,
    mut targetTypMod: i32,
    mut ccontext: CoercionContext,
    mut cformat: CoercionForm,
    mut location: libc::c_int,
) -> *mut Node {
    let mut nargs: libc::c_int = 0 as libc::c_int;
    if (funcId != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
        let mut tp: HeapTuple = 0 as *mut HeapTupleData;
        let mut procstruct: Form_pg_proc = 0 as *mut FormData_pg_proc;
        tp = SearchSysCache1(PROCOID as libc::c_int, funcId as Datum);
        if (tp as *const libc::c_void).is_null() {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"cache lookup failed for function %u\0" as *const u8 as *const libc::c_char,
                    funcId,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_coerce.c\0"
                        as *const u8 as *const libc::c_char,
                    832 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        procstruct = ((*tp).t_data as *mut libc::c_char)
            .offset((*(*tp).t_data).t_hoff as libc::c_int as isize)
            as Form_pg_proc;
        nargs = (*procstruct).pronargs as libc::c_int;
        ReleaseSysCache(tp);
    }
    if pathtype as libc::c_uint == COERCION_PATH_FUNC as libc::c_int as libc::c_uint {
        let mut fexpr: *mut FuncExpr = 0 as *mut FuncExpr;
        let mut args: *mut List = 0 as *mut List;
        let mut cons: *mut Const = 0 as *mut Const;
        args = list_make1_impl(
            T_List,
            ListCell {
                ptr_value: node as *mut libc::c_void,
            },
        );
        if nargs >= 2 as libc::c_int {
            args = lappend(args, cons as *mut libc::c_void);
        }
        if nargs == 3 as libc::c_int {
            args = lappend(args, cons as *mut libc::c_void);
        }
        fexpr = makeFuncExpr(
            funcId,
            targetTypeId,
            args,
            0 as libc::c_int as Oid,
            0 as libc::c_int as Oid,
            cformat,
        );
        (*fexpr).location = location;
        return fexpr as *mut Node;
    } else if pathtype as libc::c_uint == COERCION_PATH_ARRAYCOERCE as libc::c_int as libc::c_uint {
        let mut acoerce: *mut ArrayCoerceExpr = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).tag = T_ArrayCoerceExpr;
            _result
        }) as *mut ArrayCoerceExpr;
        let mut ctest: *mut CaseTestExpr = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).tag = T_CaseTestExpr;
            _result
        }) as *mut CaseTestExpr;
        let mut sourceBaseTypeId: Oid = 0;
        let mut sourceBaseTypeMod: i32 = 0;
        let mut targetElementType: Oid = 0;
        let mut elemexpr: *mut Node = 0 as *mut Node;
        sourceBaseTypeMod = exprTypmod(node);
        sourceBaseTypeId = getBaseTypeAndTypmod(exprType(node), &mut sourceBaseTypeMod);
        (*ctest).typeId = get_element_type(sourceBaseTypeId);
        (*ctest).typeMod = sourceBaseTypeMod;
        (*ctest).collation = 0 as libc::c_int as Oid;
        targetElementType = get_element_type(targetTypeId);
        elemexpr = coerce_to_target_type(
            0 as *mut ParseState,
            ctest as *mut Node,
            (*ctest).typeId,
            targetElementType,
            targetTypMod,
            ccontext,
            cformat,
            location,
        );
        if elemexpr.is_null() {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"failed to coerce array element type as expected\0" as *const u8
                        as *const libc::c_char,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_coerce.c\0"
                        as *const u8 as *const libc::c_char,
                    940 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
        (*acoerce).arg = node as *mut Expr;
        (*acoerce).elemexpr = elemexpr as *mut Expr;
        (*acoerce).resulttype = targetTypeId;
        (*acoerce).resulttypmod = exprTypmod(elemexpr);
        (*acoerce).coerceformat = cformat;
        (*acoerce).location = location;
        return acoerce as *mut Node;
    } else if pathtype as libc::c_uint == COERCION_PATH_COERCEVIAIO as libc::c_int as libc::c_uint {
        let mut iocoerce: *mut CoerceViaIO = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).tag = T_CoerceViaIO;
            _result
        }) as *mut CoerceViaIO;
        (*iocoerce).arg = node as *mut Expr;
        (*iocoerce).resulttype = targetTypeId;
        (*iocoerce).coerceformat = cformat;
        (*iocoerce).location = location;
        return iocoerce as *mut Node;
    } else {
        let elevel__1: libc::c_int = 21 as libc::c_int;
        let mut __error_1: libc::c_int = 0;
        if errstart(elevel__1, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"unsupported pathtype %d in build_coercion_expression\0" as *const u8
                    as *const libc::c_char,
                pathtype as libc::c_int,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_coerce.c\0"
                    as *const u8 as *const libc::c_char,
                975 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel__1 >= 21 as libc::c_int {
            abort();
        }
        return 0 as *mut Node;
    };
}
unsafe extern "C" fn coerce_record_to_complex(
    mut pstate: *mut ParseState,
    mut node: *mut Node,
    mut targetTypeId: Oid,
    mut ccontext: CoercionContext,
    mut cformat: CoercionForm,
    mut location: libc::c_int,
) -> *mut Node {
    let mut rowexpr: *mut RowExpr = 0 as *mut RowExpr;
    let mut baseTypeId: Oid = 0;
    let mut baseTypeMod: i32 = -(1 as libc::c_int);
    let mut tupdesc: TupleDesc = 0 as *mut TupleDescData;
    let mut args: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut newargs: *mut List = 0 as *mut List;
    let mut i: libc::c_int = 0;
    let mut ucolno: libc::c_int = 0;
    let mut arg: *mut ListCell = 0 as *mut ListCell;
    if !node.is_null()
        && (*(node as *const Node)).tag as libc::c_uint == T_RowExpr as libc::c_int as libc::c_uint
    {
        args = (*(node as *mut RowExpr)).args;
    } else if !node.is_null()
        && (*(node as *const Node)).tag as libc::c_uint == T_Var as libc::c_int as libc::c_uint
        && (*(node as *mut Var)).varattno as libc::c_int == 0 as libc::c_int
    {
        let mut rtindex: libc::c_int = (*(node as *mut Var)).varno as libc::c_int;
        let mut sublevels_up: libc::c_int = (*(node as *mut Var)).varlevelsup as libc::c_int;
        let mut vlocation: libc::c_int = (*(node as *mut Var)).location;
        let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
        nsitem = GetNSItemByRangeTablePosn(pstate, rtindex, sublevels_up);
        args = expandNSItemVars(nsitem, sublevels_up, vlocation, 0 as *mut *mut List);
    } else {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    baseTypeId = getBaseTypeAndTypmod(targetTypeId, &mut baseTypeMod);
    tupdesc = lookup_rowtype_tupdesc(baseTypeId, baseTypeMod);
    newargs = 0 as *mut libc::c_void as *mut List;
    ucolno = 1 as libc::c_int;
    arg = list_head(args);
    i = 0 as libc::c_int;
    while i < (*tupdesc).natts {
        let mut expr: *mut Node = 0 as *mut Node;
        let mut cexpr: *mut Node = 0 as *mut Node;
        let mut exprtype: Oid = 0;
        let mut attr: Form_pg_attribute =
            &mut *((*tupdesc).attrs).as_mut_ptr().offset(i as isize) as *mut FormData_pg_attribute;
        if !((*attr).attisdropped != 0) {
            if arg.is_null() {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            expr = (*arg).ptr_value as *mut Node;
            exprtype = exprType(expr);
            cexpr = coerce_to_target_type(
                pstate,
                expr,
                exprtype,
                (*attr).atttypid,
                (*attr).atttypmod,
                ccontext,
                COERCE_IMPLICIT_CAST,
                -(1 as libc::c_int),
            );
            if cexpr.is_null() {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
            newargs = lappend(newargs, cexpr as *mut libc::c_void);
            ucolno += 1;
            ucolno;
            arg = lnext(args, arg);
        }
        i += 1;
        i;
    }
    if !arg.is_null() {
        let elevel__2: libc::c_int = 21 as libc::c_int;
        let mut __error_2: libc::c_int = 0;
        if elevel__2 >= 21 as libc::c_int {
            abort();
        }
    }
    if (*tupdesc).tdrefcount >= 0 as libc::c_int {
        DecrTupleDescRefCount(tupdesc);
    }
    rowexpr = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).tag = T_RowExpr;
        _result
    }) as *mut RowExpr;
    (*rowexpr).args = newargs;
    (*rowexpr).row_typeid = baseTypeId;
    (*rowexpr).row_format = cformat;
    (*rowexpr).colnames = 0 as *mut libc::c_void as *mut List;
    (*rowexpr).location = location;
    if baseTypeId != targetTypeId {
        (*rowexpr).row_format = COERCE_IMPLICIT_CAST;
        return coerce_to_domain(
            rowexpr as *mut Node,
            baseTypeId,
            baseTypeMod,
            targetTypeId,
            ccontext,
            cformat,
            location,
            false,
        );
    }
    return rowexpr as *mut Node;
}
#[no_mangle]
pub unsafe extern "C" fn coerce_to_specific_type_typmod(
    mut pstate: *mut ParseState,
    mut node: *mut Node,
    mut targetTypeId: Oid,
    mut targetTypmod: i32,
    mut constructName: *const libc::c_char,
) -> *mut Node {
    let mut inputTypeId: Oid = exprType(node);
    if inputTypeId != targetTypeId {
        let mut newnode: *mut Node = 0 as *mut Node;
        newnode = coerce_to_target_type(
            pstate,
            node,
            inputTypeId,
            targetTypeId,
            targetTypmod,
            COERCION_ASSIGNMENT,
            COERCE_IMPLICIT_CAST,
            -(1 as libc::c_int),
        );
        if newnode.is_null() {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        node = newnode;
    }
    if expression_returns_set(node) != 0 {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn coerce_to_specific_type(
    mut pstate: *mut ParseState,
    mut node: *mut Node,
    mut targetTypeId: Oid,
    mut constructName: *const libc::c_char,
) -> *mut Node {
    return coerce_to_specific_type_typmod(
        pstate,
        node,
        targetTypeId,
        -(1 as libc::c_int),
        constructName,
    );
}
#[no_mangle]
pub unsafe extern "C" fn parser_coercion_errposition(
    mut pstate: *mut ParseState,
    mut coerce_location: libc::c_int,
    mut input_expr: *mut Node,
) -> libc::c_int {
    if coerce_location >= 0 as libc::c_int {
        return parser_errposition(pstate, coerce_location);
    } else {
        return parser_errposition(pstate, exprLocation(input_expr));
    };
}
#[no_mangle]
pub unsafe extern "C" fn coerce_to_common_type(
    mut pstate: *mut ParseState,
    mut node: *mut Node,
    mut targetTypeId: Oid,
    mut context: *const libc::c_char,
) -> *mut Node {
    let mut inputTypeId: Oid = exprType(node);
    if inputTypeId == targetTypeId {
        return node;
    }
    if can_coerce_type(
        1 as libc::c_int,
        &mut inputTypeId,
        &mut targetTypeId,
        COERCION_IMPLICIT,
    ) != 0
    {
        node = coerce_type(
            pstate,
            node,
            inputTypeId,
            targetTypeId,
            -(1 as libc::c_int),
            COERCION_IMPLICIT,
            COERCE_IMPLICIT_CAST,
            -(1 as libc::c_int),
        );
    } else {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn select_common_typmod(
    mut pstate: *mut ParseState,
    mut exprs: *mut List,
    mut common_type: Oid,
) -> i32 {
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut first: bool = true;
    let mut result: i32 = -(1 as libc::c_int);
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
        let mut expr: *mut Node = (*lc).ptr_value as *mut Node;
        if exprType(expr) != common_type {
            return -(1 as libc::c_int);
        } else if first != 0 {
            result = exprTypmod(expr);
            first = false;
        } else if result != exprTypmod(expr) {
            return -(1 as libc::c_int);
        }
        lc__state.i += 1;
        lc__state.i;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn TypeCategory(mut type_0: Oid) -> TYPCATEGORY {
    let mut typcategory: libc::c_char = 0;
    let mut typispreferred: bool = false;
    get_type_category_preferred(type_0, &mut typcategory, &mut typispreferred);
    return typcategory;
}
#[no_mangle]
pub unsafe extern "C" fn find_typmod_coercion_function(
    mut typeId: Oid,
    mut funcid: *mut Oid,
) -> CoercionPathType {
    let mut result: CoercionPathType = COERCION_PATH_NONE;
    let mut targetType: Type = 0 as *mut HeapTupleData;
    let mut typeForm: Form_pg_type = 0 as *mut FormData_pg_type;
    let mut tuple: HeapTuple = 0 as *mut HeapTupleData;
    *funcid = 0 as libc::c_int as Oid;
    result = COERCION_PATH_FUNC;
    targetType = typeidType(typeId);
    typeForm = ((*targetType).t_data as *mut libc::c_char)
        .offset((*(*targetType).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_type;
    if IsTrueArrayType(typeForm) != 0 {
        typeId = (*typeForm).typelem;
        result = COERCION_PATH_ARRAYCOERCE;
    }
    ReleaseSysCache(targetType);
    tuple = SearchSysCache2(
        CASTSOURCETARGET as libc::c_int,
        typeId as Datum,
        typeId as Datum,
    );
    if !(tuple as *const libc::c_void).is_null() {
        let mut castForm: Form_pg_cast = ((*tuple).t_data as *mut libc::c_char)
            .offset((*(*tuple).t_data).t_hoff as libc::c_int as isize)
            as Form_pg_cast;
        *funcid = (*castForm).castfunc;
        ReleaseSysCache(tuple);
    }
    if (*funcid != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
        result = COERCION_PATH_NONE;
    }
    return result;
}
unsafe extern "C" fn is_complex_array(mut typid: Oid) -> bool {
    let mut elemtype: Oid = get_element_type(typid);
    return ((elemtype != 0 as libc::c_int as Oid) as libc::c_int as bool as libc::c_int != 0
        && typeOrDomainTypeRelid(elemtype) != 0 as libc::c_int as Oid) as libc::c_int
        as bool;
}
unsafe extern "C" fn typeIsOfTypedTable(mut reltypeId: Oid, mut reloftypeId: Oid) -> bool {
    let mut relid: Oid = typeOrDomainTypeRelid(reltypeId);
    let mut result: bool = false;
    if relid != 0 {
        let mut tp: HeapTuple = 0 as *mut HeapTupleData;
        let mut reltup: Form_pg_class = 0 as *mut FormData_pg_class;
        tp = SearchSysCache1(RELOID as libc::c_int, relid as Datum);
        if (tp as *const libc::c_void).is_null() {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"cache lookup failed for relation %u\0" as *const u8 as *const libc::c_char,
                    relid,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_coerce.c\0"
                        as *const u8 as *const libc::c_char,
                    3211 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        reltup = ((*tp).t_data as *mut libc::c_char)
            .offset((*(*tp).t_data).t_hoff as libc::c_int as isize)
            as Form_pg_class;
        if (*reltup).reloftype == reloftypeId {
            result = true;
        }
        ReleaseSysCache(tp);
    }
    return result;
}
