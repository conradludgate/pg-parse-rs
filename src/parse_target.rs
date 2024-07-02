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
//     pub type PartitionKeyData;
//     pub type PartitionDescData;
//     pub type AttrMissing;
//     pub type PgStat_TableStatus;
//     pub type FdwRoutine;
//     pub type IndexAmRoutine;
//     pub type TableAmRoutine;
//     pub type RowSecurityDesc;
//     pub type SMgrRelationData;
//     pub type QueryEnvironment;
//     fn abort() -> !;
//     fn memset(
//         _: *mut libc::c_void,
//         _: libc::c_int,
//         _: libc::c_ulong,
//     ) -> *mut libc::c_void;
//     fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
//     fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn copyObjectImpl(obj: *const libc::c_void) -> *mut libc::c_void;
//     fn bms_add_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
//     fn bms_is_member(x: libc::c_int, a: *const Bitmapset) -> bool;
//     fn lappend_int(list: *mut List, datum: libc::c_int) -> *mut List;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
//     fn CreateTemplateTupleDesc(natts: libc::c_int) -> TupleDesc;
//     fn TupleDescInitEntry(
//         desc: TupleDesc,
//         attributeNumber: AttrNumber,
//         attributeName: *const libc::c_char,
//         oidtypeid: Oid,
//         typmod: i32,
//         attdim: libc::c_int,
//     );
//     fn TupleDescInitEntryCollation(
//         desc: TupleDesc,
//         attributeNumber: AttrNumber,
//         collationid: Oid,
//     );
//     fn list_truncate(list: *mut List, new_size: libc::c_int) -> *mut List;
//     fn list_concat(list1: *mut List, list2: *const List) -> *mut List;
//     fn transformContainerSubscripts(
//         pstate: *mut ParseState,
//         containerBase: *mut Node,
//         containerType: Oid,
//         containerTypMod: i32,
//         indirection: *mut List,
//         isAssignment: bool,
//     ) -> *mut SubscriptingRef;
//     fn transformContainerType(containerType: *mut Oid, containerTypmod: *mut i32);
//     fn get_database_name(dbid: Oid) -> *mut libc::c_char;
//     fn get_expr_result_tupdesc(expr: *mut Node, noError: bool) -> TupleDesc;
//     static mut MyDatabaseId: Oid;
//     fn makeVar(
//         varno: Index,
//         varattno: AttrNumber,
//         vartype: Oid,
//         vartypmod: i32,
//         varcollid: Oid,
//         varlevelsup: Index,
//     ) -> *mut Var;
//     fn makeTargetEntry(
//         expr: *mut Expr,
//         resno: AttrNumber,
//         resname: *mut libc::c_char,
//         resjunk: bool,
//     ) -> *mut TargetEntry;
//     fn makeNullConst(consttype: Oid, consttypmod: i32, constcollid: Oid) -> *mut Const;
//     fn makeRangeVar(
//         schemaname: *mut libc::c_char,
//         relname: *mut libc::c_char,
//         location: libc::c_int,
//     ) -> *mut RangeVar;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprTypmod(expr: *const Node) -> i32;
//     fn exprCollation(expr: *const Node) -> Oid;
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
//     fn coerce_to_domain(
//         arg: *mut Node,
//         baseTypeId: Oid,
//         baseTypeMod: i32,
//         typeId: Oid,
//         ccontext: CoercionContext,
//         cformat: CoercionForm,
//         location: libc::c_int,
//         hideInputCoercion: bool,
//     ) -> *mut Node;
//     fn transformExpr(
//         pstate: *mut ParseState,
//         expr: *mut Node,
//         exprKind: ParseExprKind,
//     ) -> *mut Node;
//     fn refnameNamespaceItem(
//         pstate: *mut ParseState,
//         schemaname: *const libc::c_char,
//         refname: *const libc::c_char,
//         location: libc::c_int,
//         sublevels_up: *mut libc::c_int,
//     ) -> *mut ParseNamespaceItem;
//     fn GetNSItemByRangeTablePosn(
//         pstate: *mut ParseState,
//         varno: libc::c_int,
//         sublevels_up: libc::c_int,
//     ) -> *mut ParseNamespaceItem;
//     fn GetRTEByRangeTablePosn(
//         pstate: *mut ParseState,
//         varno: libc::c_int,
//         sublevels_up: libc::c_int,
//     ) -> *mut RangeTblEntry;
//     fn GetCTEForRTE(
//         pstate: *mut ParseState,
//         rte: *mut RangeTblEntry,
//         rtelevelsup: libc::c_int,
//     ) -> *mut CommonTableExpr;
//     fn markVarForSelectPriv(pstate: *mut ParseState, var: *mut Var);
//     fn errorMissingRTE(pstate: *mut ParseState, relation: *mut RangeVar) -> !;
//     fn expandRTE(
//         rte: *mut RangeTblEntry,
//         rtindex: libc::c_int,
//         sublevels_up: libc::c_int,
//         location: libc::c_int,
//         include_dropped: bool,
//         colnames: *mut *mut List,
//         colvars: *mut *mut List,
//     );
//     fn expandNSItemVars(
//         nsitem: *mut ParseNamespaceItem,
//         sublevels_up: libc::c_int,
//         location: libc::c_int,
//         colnames: *mut *mut List,
//     ) -> *mut List;
//     fn expandNSItemAttrs(
//         pstate: *mut ParseState,
//         nsitem: *mut ParseNamespaceItem,
//         sublevels_up: libc::c_int,
//         location: libc::c_int,
//     ) -> *mut List;
//     fn attnameAttNum(
//         rd: Relation,
//         attname: *const libc::c_char,
//         sysColOK: bool,
//     ) -> libc::c_int;
//     fn attnumTypeId(rd: Relation, attid: libc::c_int) -> Oid;
//     fn typeidTypeRelid(type_id: Oid) -> Oid;
//     fn get_tle_by_resno(tlist: *mut List, resno: AttrNumber) -> *mut TargetEntry;
//     fn get_attnum(relid: Oid, attname: *const libc::c_char) -> AttrNumber;
//     fn get_atttypetypmodcoll(
//         relid: Oid,
//         attnum: AttrNumber,
//         typid: *mut Oid,
//         typmod: *mut i32,
//         collid: *mut Oid,
//     );
//     fn get_typcollation(typid: Oid) -> Oid;
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
// pub type u8 = libc::c_uchar;
// pub type u16 = libc::c_ushort;
// pub type u32 = libc::c_uint;
pub type bits8 = u8;
// pub type uint64 = libc::c_ulong;
// pub type usize = isize;
// pub type Index = libc::c_uint;
pub type float4 = libc::c_float;
pub type regproc = Oid;
pub type RegProcedure = regproc;
pub type TransactionId = u32;
pub type SubTransactionId = u32;
pub type CommandId = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct varlena {
    pub vl_len_: [libc::c_char; 4],
    pub vl_dat: [libc::c_char; 0],
}
pub type bytea = varlena;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int2vector {
    pub vl_len_: i32,
    pub ndim: libc::c_int,
    pub dataoffset: i32,
    pub elemtype: Oid,
    pub dim1: libc::c_int,
    pub lbound1: libc::c_int,
    pub values: [i16; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nameData {
    pub data: [libc::c_char; 64],
}
pub type NameData = nameData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryContextData {
    pub type_0: NodeTag,
    pub isReset: bool,
    pub allowInCritSection: bool,
    pub mem_allocated: usize,
    pub methods: *const MemoryContextMethods,
    pub parent: MemoryContext,
    pub firstchild: MemoryContext,
    pub prevchild: MemoryContext,
    pub nextchild: MemoryContext,
    pub name: *const libc::c_char,
    pub ident: *const libc::c_char,
    pub reset_cbs: *mut MemoryContextCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryContextCallback {
    pub func: MemoryContextCallbackFunction,
    pub arg: *mut libc::c_void,
    pub next: *mut MemoryContextCallback,
}
pub type MemoryContextCallbackFunction = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type MemoryContext = *mut MemoryContextData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryContextMethods {
    pub alloc: Option<unsafe extern "C" fn(MemoryContext, usize) -> *mut libc::c_void>,
    pub free_p: Option<unsafe extern "C" fn(MemoryContext, *mut libc::c_void) -> ()>,
    pub realloc:
        Option<unsafe extern "C" fn(MemoryContext, *mut libc::c_void, usize) -> *mut libc::c_void>,
    pub reset: Option<unsafe extern "C" fn(MemoryContext) -> ()>,
    pub delete_context: Option<unsafe extern "C" fn(MemoryContext) -> ()>,
    pub get_chunk_space: Option<unsafe extern "C" fn(MemoryContext, *mut libc::c_void) -> usize>,
    pub is_empty: Option<unsafe extern "C" fn(MemoryContext) -> bool>,
    pub stats: Option<
        unsafe extern "C" fn(
            MemoryContext,
            MemoryStatsPrintFunc,
            *mut libc::c_void,
            *mut MemoryContextCounters,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryContextCounters {
    pub nblocks: usize,
    pub freechunks: usize,
    pub totalspace: usize,
    pub freespace: usize,
}
pub type MemoryStatsPrintFunc =
    Option<unsafe extern "C" fn(MemoryContext, *mut libc::c_void, *const libc::c_char) -> ()>;

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
pub struct FieldStore {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub newvals: *mut List,
    pub fieldnums: *mut List,
    pub resulttype: Oid,
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
pub struct CaseTestExpr {
    pub xpr: Expr,
    pub typeId: Oid,
    pub typeMod: i32,
    pub collation: Oid,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SetToDefault {
    pub xpr: Expr,
    pub typeId: Oid,
    pub typeMod: i32,
    pub collation: Oid,
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
pub type PartitionKey = *mut PartitionKeyData;
pub type PartitionDesc = *mut PartitionDescData;
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
pub struct A_Indices {
    pub type_0: NodeTag,
    pub is_slice: bool,
    pub lidx: *mut Node,
    pub uidx: *mut Node,
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
pub struct ResTarget {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub indirection: *mut List,
    pub val: *mut Node,
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
pub type Form_pg_index = *mut FormData_pg_index;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_index {
    pub indexrelid: Oid,
    pub indrelid: Oid,
    pub indnatts: i16,
    pub indnkeyatts: i16,
    pub indisunique: bool,
    pub indisprimary: bool,
    pub indisexclusion: bool,
    pub indimmediate: bool,
    pub indisclustered: bool,
    pub indisvalid: bool,
    pub indcheckxmin: bool,
    pub indisready: bool,
    pub indislive: bool,
    pub indisreplident: bool,
    pub indkey: int2vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PublicationActions {
    pub pubinsert: bool,
    pub pubupdate: bool,
    pub pubdelete: bool,
    pub pubtruncate: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerDesc {
    pub triggers: *mut Trigger,
    pub numtriggers: libc::c_int,
    pub trig_insert_before_row: bool,
    pub trig_insert_after_row: bool,
    pub trig_insert_instead_row: bool,
    pub trig_insert_before_statement: bool,
    pub trig_insert_after_statement: bool,
    pub trig_update_before_row: bool,
    pub trig_update_after_row: bool,
    pub trig_update_instead_row: bool,
    pub trig_update_before_statement: bool,
    pub trig_update_after_statement: bool,
    pub trig_delete_before_row: bool,
    pub trig_delete_after_row: bool,
    pub trig_delete_instead_row: bool,
    pub trig_delete_before_statement: bool,
    pub trig_delete_after_statement: bool,
    pub trig_truncate_before_statement: bool,
    pub trig_truncate_after_statement: bool,
    pub trig_insert_new_table: bool,
    pub trig_update_old_table: bool,
    pub trig_update_new_table: bool,
    pub trig_delete_old_table: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Trigger {
    pub tgoid: Oid,
    pub tgname: *mut libc::c_char,
    pub tgfoid: Oid,
    pub tgtype: i16,
    pub tgenabled: libc::c_char,
    pub tgisinternal: bool,
    pub tgisclone: bool,
    pub tgconstrrelid: Oid,
    pub tgconstrindid: Oid,
    pub tgconstraint: Oid,
    pub tgdeferrable: bool,
    pub tginitdeferred: bool,
    pub tgnargs: i16,
    pub tgnattr: i16,
    pub tgattr: *mut i16,
    pub tgargs: *mut *mut libc::c_char,
    pub tgqual: *mut libc::c_char,
    pub tgoldtable: *mut libc::c_char,
    pub tgnewtable: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RuleLock {
    pub numLocks: libc::c_int,
    pub rules: *mut *mut RewriteRule,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RewriteRule {
    pub ruleId: Oid,
    pub event: CmdType,
    pub qual: *mut Node,
    pub actions: *mut List,
    pub enabled: libc::c_char,
    pub isInstead: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LockInfoData {
    pub lockRelId: LockRelId,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LockRelId {
    pub relId: Oid,
    pub dbId: Oid,
}
pub type Form_pg_class = *mut FormData_pg_class;
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
pub type BackendId = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RelFileNode {
    pub spcNode: Oid,
    pub dbNode: Oid,
    pub relNode: Oid,
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
pub const CRSERR_TOO_MANY: C2RustUnnamed_1 = 2;
pub const CRSERR_WRONG_DB: C2RustUnnamed_1 = 1;
pub const CRSERR_NO_RTE: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
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
unsafe extern "C" fn list_nth(mut list: *const List, mut n: libc::c_int) -> *mut libc::c_void {
    return (*list_nth_cell(list, n)).ptr_value;
}
#[inline]
unsafe extern "C" fn list_last_cell(mut list: *const List) -> *mut ListCell {
    return &mut *((*list).elements).offset(((*list).length - 1 as libc::c_int) as isize)
        as *mut ListCell;
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
unsafe extern "C" fn list_head(mut l: *const List) -> *mut ListCell {
    return if !l.is_null() {
        &mut *((*l).elements).offset(0 as libc::c_int as isize) as *mut ListCell
    } else {
        0 as *mut ListCell
    };
}
#[no_mangle]
pub unsafe extern "C" fn transformTargetEntry(
    mut pstate: *mut ParseState,
    mut node: *mut Node,
    mut expr: *mut Node,
    mut exprKind: ParseExprKind,
    mut colname: *mut libc::c_char,
    mut resjunk: bool,
) -> *mut TargetEntry {
    if expr.is_null() {
        if exprKind as libc::c_uint == EXPR_KIND_UPDATE_SOURCE as libc::c_int as libc::c_uint
            && (*(node as *const Node)).tag as libc::c_uint
                == T_SetToDefault as libc::c_int as libc::c_uint
        {
            expr = node;
        } else {
            expr = transformExpr(pstate, node, exprKind);
        }
    }
    if colname.is_null() && resjunk == 0 {
        colname = FigureColname(node);
    }
    let fresh0 = (*pstate).p_next_resno;
    (*pstate).p_next_resno = (*pstate).p_next_resno + 1;
    return makeTargetEntry(expr as *mut Expr, fresh0 as AttrNumber, colname, resjunk);
}
#[no_mangle]
pub unsafe extern "C" fn transformTargetList(
    mut pstate: *mut ParseState,
    mut targetlist: *mut List,
    mut exprKind: ParseExprKind,
) -> *mut List {
    let mut p_target: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut expand_star: bool = false;
    let mut o_target: *mut ListCell = 0 as *mut ListCell;
    expand_star = (exprKind as libc::c_uint
        != EXPR_KIND_UPDATE_SOURCE as libc::c_int as libc::c_uint) as libc::c_int
        as bool;
    let mut current_block_4: u64;
    let mut o_target__state: ForEachState = {
        let mut init = ForEachState {
            l: targetlist,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(o_target__state.l).is_null() && o_target__state.i < (*o_target__state.l).length {
        o_target = &mut *((*o_target__state.l).elements).offset(o_target__state.i as isize)
            as *mut ListCell;
        true as libc::c_int
    } else {
        o_target = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut res: *mut ResTarget = (*o_target).ptr_value as *mut ResTarget;
        if expand_star {
            if (*((*res).val as *const Node)).tag as libc::c_uint
                == T_ColumnRef as libc::c_int as libc::c_uint
            {
                let mut cref: *mut ColumnRef = (*res).val as *mut ColumnRef;
                if (*((*list_last_cell((*cref).fields)).ptr_value as *const Node)).tag
                    as libc::c_uint
                    == T_A_Star as libc::c_int as libc::c_uint
                {
                    p_target = list_concat(p_target, ExpandColumnRefStar(pstate, cref, true));
                    current_block_4 = 12517898123489920830;
                } else {
                    current_block_4 = 12800627514080957624;
                }
            } else if (*((*res).val as *const Node)).tag as libc::c_uint
                == T_A_Indirection as libc::c_int as libc::c_uint
            {
                let mut ind: *mut A_Indirection = (*res).val as *mut A_Indirection;
                if (*((*list_last_cell((*ind).indirection)).ptr_value as *const Node)).tag
                    as libc::c_uint
                    == T_A_Star as libc::c_int as libc::c_uint
                {
                    p_target =
                        list_concat(p_target, ExpandIndirectionStar(pstate, ind, true, exprKind));
                    current_block_4 = 12517898123489920830;
                } else {
                    current_block_4 = 12800627514080957624;
                }
            } else {
                current_block_4 = 12800627514080957624;
            }
        } else {
            current_block_4 = 12800627514080957624;
        }
        match current_block_4 {
            12800627514080957624 => {
                p_target = lappend(
                    p_target,
                    transformTargetEntry(
                        pstate,
                        (*res).val,
                        0 as *mut Node,
                        exprKind,
                        (*res).name,
                        false,
                    ) as *mut libc::c_void,
                );
            }
            _ => {}
        }
        o_target__state.i += 1;
        o_target__state.i;
    }
    if !((*pstate).p_multiassign_exprs).is_null() {
        p_target = list_concat(p_target, (*pstate).p_multiassign_exprs);
        (*pstate).p_multiassign_exprs = 0 as *mut libc::c_void as *mut List;
    }
    return p_target;
}
#[no_mangle]
pub unsafe extern "C" fn transformExpressionList(
    mut pstate: *mut ParseState,
    mut exprlist: *mut List,
    mut exprKind: ParseExprKind,
    mut allowDefault: bool,
) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut current_block_5: u64;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: exprlist,
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
        let mut e: *mut Node = (*lc).ptr_value as *mut Node;
        if (*(e as *const Node)).tag as libc::c_uint == T_ColumnRef as libc::c_int as libc::c_uint {
            let mut cref: *mut ColumnRef = e as *mut ColumnRef;
            if (*((*list_last_cell((*cref).fields)).ptr_value as *const Node)).tag as libc::c_uint
                == T_A_Star as libc::c_int as libc::c_uint
            {
                result = list_concat(result, ExpandColumnRefStar(pstate, cref, false));
                current_block_5 = 16668937799742929182;
            } else {
                current_block_5 = 13109137661213826276;
            }
        } else if (*(e as *const Node)).tag as libc::c_uint
            == T_A_Indirection as libc::c_int as libc::c_uint
        {
            let mut ind: *mut A_Indirection = e as *mut A_Indirection;
            if (*((*list_last_cell((*ind).indirection)).ptr_value as *const Node)).tag
                as libc::c_uint
                == T_A_Star as libc::c_int as libc::c_uint
            {
                result = list_concat(result, ExpandIndirectionStar(pstate, ind, false, exprKind));
                current_block_5 = 16668937799742929182;
            } else {
                current_block_5 = 13109137661213826276;
            }
        } else {
            current_block_5 = 13109137661213826276;
        }
        match current_block_5 {
            13109137661213826276 => {
                if !(allowDefault as libc::c_int != 0
                    && (*(e as *const Node)).tag as libc::c_uint
                        == T_SetToDefault as libc::c_int as libc::c_uint)
                {
                    e = transformExpr(pstate, e, exprKind);
                }
                result = lappend(result, e as *mut libc::c_void);
            }
            _ => {}
        }
        lc__state.i += 1;
        lc__state.i;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn markTargetListOrigins(
    mut pstate: *mut ParseState,
    mut targetlist: *mut List,
) {
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: targetlist,
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
        let mut tle: *mut TargetEntry = (*l).ptr_value as *mut TargetEntry;
        markTargetListOrigin(pstate, tle, (*tle).expr as *mut Var, 0 as libc::c_int);
        l__state.i += 1;
        l__state.i;
    }
}
unsafe extern "C" fn markTargetListOrigin(
    mut pstate: *mut ParseState,
    mut tle: *mut TargetEntry,
    mut var: *mut Var,
    mut levelsup: libc::c_int,
) {
    let mut netlevelsup: libc::c_int = 0;
    let mut rte: *mut RangeTblEntry = 0 as *mut RangeTblEntry;
    let mut attnum: AttrNumber = 0;
    if var.is_null()
        || !((*(var as *const Node)).tag as libc::c_uint == T_Var as libc::c_int as libc::c_uint)
    {
        return;
    }
    netlevelsup = ((*var).varlevelsup).wrapping_add(levelsup as Index) as libc::c_int;
    rte = GetRTEByRangeTablePosn(pstate, (*var).varno as libc::c_int, netlevelsup);
    attnum = (*var).varattno;
    match (*rte).rtekind as libc::c_uint {
        0 => {
            (*tle).resorigtbl = (*rte).relid;
            (*tle).resorigcol = attnum;
        }
        1 => {
            if attnum as libc::c_int != 0 as libc::c_int {
                let mut ste: *mut TargetEntry =
                    get_tle_by_resno((*(*rte).subquery).targetList, attnum);
                if ste.is_null() || (*ste).resjunk as libc::c_int != 0 {
                    let elevel_: libc::c_int = 21 as libc::c_int;
                    let mut __error: libc::c_int = 0;
                    if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                        errmsg_internal(
                            b"subquery %s does not have attribute %d\0" as *const u8
                                as *const libc::c_char,
                            (*(*rte).eref).aliasname,
                            attnum as libc::c_int,
                        );
                        errfinish(
                            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_target.c\0"
                                as *const u8 as *const libc::c_char,
                            375 as libc::c_int,
                            0 as *const libc::c_char,
                        );
                    }
                    if elevel_ >= 21 as libc::c_int {
                        abort();
                    }
                }
                (*tle).resorigtbl = (*ste).resorigtbl;
                (*tle).resorigcol = (*ste).resorigcol;
            }
        }
        6 => {
            if attnum as libc::c_int != 0 as libc::c_int && (*rte).self_reference == 0 {
                let mut cte: *mut CommonTableExpr = GetCTEForRTE(pstate, rte, netlevelsup);
                let mut ste_0: *mut TargetEntry = 0 as *mut TargetEntry;
                let mut tl: *mut List = (if (*((*cte).ctequery as *mut Query)).commandType
                    as libc::c_uint
                    == CMD_SELECT as libc::c_int as libc::c_uint
                {
                    (*((*cte).ctequery as *mut Query)).targetList
                } else {
                    (*((*cte).ctequery as *mut Query)).returningList
                });
                let mut extra_cols: libc::c_int = 0 as libc::c_int;
                if !((*cte).search_clause).is_null() {
                    extra_cols += 1 as libc::c_int;
                }
                if !((*cte).cycle_clause).is_null() {
                    extra_cols += 2 as libc::c_int;
                }
                if !(extra_cols != 0
                    && attnum as libc::c_int > list_length(tl)
                    && attnum as libc::c_int <= list_length(tl) + extra_cols)
                {
                    ste_0 = get_tle_by_resno(tl, attnum);
                    if ste_0.is_null() || (*ste_0).resjunk as libc::c_int != 0 {
                        let elevel__0: libc::c_int = 21 as libc::c_int;
                        let mut __error_0: libc::c_int = 0;
                        if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
                            errmsg_internal(
                                b"CTE %s does not have attribute %d\0" as *const u8
                                    as *const libc::c_char,
                                (*(*rte).eref).aliasname,
                                attnum as libc::c_int,
                            );
                            errfinish(
                                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_target.c\0"
                                    as *const u8 as *const libc::c_char,
                                421 as libc::c_int,
                                0 as *const libc::c_char,
                            );
                        }
                        if elevel__0 >= 21 as libc::c_int {
                            abort();
                        }
                    }
                    (*tle).resorigtbl = (*ste_0).resorigtbl;
                    (*tle).resorigcol = (*ste_0).resorigcol;
                }
            }
        }
        2 | 3 | 5 | 4 | 7 | 8 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn transformAssignedExpr(
    mut pstate: *mut ParseState,
    mut expr: *mut Expr,
    mut exprKind: ParseExprKind,
    mut colname: *const libc::c_char,
    mut attrno: libc::c_int,
    mut indirection: *mut List,
    mut location: libc::c_int,
) -> *mut Expr {
    let mut rd: Relation = (*pstate).p_target_relation;
    let mut type_id: Oid = 0;
    let mut attrtype: Oid = 0;
    let mut attrtypmod: i32 = 0;
    let mut attrcollation: Oid = 0;
    let mut sv_expr_kind: ParseExprKind = EXPR_KIND_NONE;
    sv_expr_kind = (*pstate).p_expr_kind;
    (*pstate).p_expr_kind = exprKind;
    if attrno <= 0 as libc::c_int {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    attrtype = attnumTypeId(rd, attrno);
    attrtypmod = (*((*(*rd).rd_att).attrs)
        .as_mut_ptr()
        .offset((attrno - 1 as libc::c_int) as isize))
    .atttypmod;
    attrcollation = (*((*(*rd).rd_att).attrs)
        .as_mut_ptr()
        .offset((attrno - 1 as libc::c_int) as isize))
    .attcollation;
    if !expr.is_null()
        && (*(expr as *const Node)).tag as libc::c_uint
            == T_SetToDefault as libc::c_int as libc::c_uint
    {
        let mut def: *mut SetToDefault = expr as *mut SetToDefault;
        (*def).typeId = attrtype;
        (*def).typeMod = attrtypmod;
        (*def).collation = attrcollation;
        if !indirection.is_null() {
            if (*((*list_nth_cell(indirection, 0 as libc::c_int)).ptr_value as *const Node)).tag
                as libc::c_uint
                == T_A_Indices as libc::c_int as libc::c_uint
            {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            } else {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
    }
    type_id = exprType(expr as *mut Node);
    if !indirection.is_null() {
        let mut colVar: *mut Node = 0 as *mut Node;
        if (*pstate).p_is_insert != 0 {
            colVar = makeNullConst(attrtype, attrtypmod, attrcollation) as *mut Node;
        } else {
            let mut var: *mut Var = 0 as *mut Var;
            var = makeVar(
                (*(*pstate).p_target_nsitem).p_rtindex as Index,
                attrno as AttrNumber,
                attrtype,
                attrtypmod,
                attrcollation,
                0 as libc::c_int as Index,
            );
            (*var).location = location;
            colVar = var as *mut Node;
        }
        expr = transformAssignmentIndirection(
            pstate,
            colVar,
            colname,
            false,
            attrtype,
            attrtypmod,
            attrcollation,
            indirection,
            list_head(indirection),
            expr as *mut Node,
            COERCION_ASSIGNMENT,
            location,
        ) as *mut Expr;
    } else {
        let mut orig_expr: *mut Node = expr as *mut Node;
        expr = coerce_to_target_type(
            pstate,
            orig_expr,
            type_id,
            attrtype,
            attrtypmod,
            COERCION_ASSIGNMENT,
            COERCE_IMPLICIT_CAST,
            -(1 as libc::c_int),
        ) as *mut Expr;
        if expr.is_null() {
            let elevel__2: libc::c_int = 21 as libc::c_int;
            let mut __error_2: libc::c_int = 0;
            if elevel__2 >= 21 as libc::c_int {
                abort();
            }
        }
    }
    (*pstate).p_expr_kind = sv_expr_kind;
    return expr;
}
#[no_mangle]
pub unsafe extern "C" fn updateTargetListEntry(
    mut pstate: *mut ParseState,
    mut tle: *mut TargetEntry,
    mut colname: *mut libc::c_char,
    mut attrno: libc::c_int,
    mut indirection: *mut List,
    mut location: libc::c_int,
) {
    (*tle).expr = transformAssignedExpr(
        pstate,
        (*tle).expr,
        EXPR_KIND_UPDATE_TARGET,
        colname,
        attrno,
        indirection,
        location,
    );
    (*tle).resno = attrno as AttrNumber;
    (*tle).resname = colname;
}
#[no_mangle]
pub unsafe extern "C" fn transformAssignmentIndirection(
    mut pstate: *mut ParseState,
    mut basenode: *mut Node,
    mut targetName: *const libc::c_char,
    mut targetIsSubscripting: bool,
    mut targetTypeId: Oid,
    mut targetTypMod: i32,
    mut targetCollation: Oid,
    mut indirection: *mut List,
    mut indirection_cell: *mut ListCell,
    mut rhs: *mut Node,
    mut ccontext: CoercionContext,
    mut location: libc::c_int,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut subscripts: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut isSlice: bool = false;
    let mut i: *mut ListCell = 0 as *mut ListCell;
    if !indirection_cell.is_null() && basenode.is_null() {
        let mut ctest: *mut CaseTestExpr = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).tag = T_CaseTestExpr;
            _result
        }) as *mut CaseTestExpr;
        (*ctest).typeId = targetTypeId;
        (*ctest).typeMod = targetTypMod;
        (*ctest).collation = targetCollation;
        basenode = ctest as *mut Node;
    }
    let mut i__state: ForEachState = for_each_cell_setup(indirection, indirection_cell);
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
            if (*(n as *mut A_Indices)).is_slice != 0 {
                isSlice = true;
            }
        } else if (*(n as *const Node)).tag as libc::c_uint
            == T_A_Star as libc::c_int as libc::c_uint
        {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        } else {
            let mut fstore: *mut FieldStore = 0 as *mut FieldStore;
            let mut baseTypeId: Oid = 0;
            let mut baseTypeMod: i32 = 0;
            let mut typrelid: Oid = 0;
            let mut attnum: AttrNumber = 0;
            let mut fieldTypeId: Oid = 0;
            let mut fieldTypMod: i32 = 0;
            let mut fieldCollation: Oid = 0;
            if !subscripts.is_null() {
                return transformAssignmentSubscripts(
                    pstate,
                    basenode,
                    targetName,
                    targetTypeId,
                    targetTypMod,
                    targetCollation,
                    subscripts,
                    isSlice,
                    indirection,
                    i,
                    rhs,
                    ccontext,
                    location,
                );
            }
            baseTypeMod = targetTypMod;
            baseTypeId = getBaseTypeAndTypmod(targetTypeId, &mut baseTypeMod);
            typrelid = typeidTypeRelid(baseTypeId);
            if typrelid == 0 {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            attnum = get_attnum(typrelid, (*(n as *mut Value)).val.str_0);
            if attnum as libc::c_int == 0 as libc::c_int {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
            if (attnum as libc::c_int) < 0 as libc::c_int {
                let elevel__2: libc::c_int = 21 as libc::c_int;
                let mut __error_2: libc::c_int = 0;
                if elevel__2 >= 21 as libc::c_int {
                    abort();
                }
            }
            get_atttypetypmodcoll(
                typrelid,
                attnum,
                &mut fieldTypeId,
                &mut fieldTypMod,
                &mut fieldCollation,
            );
            rhs = transformAssignmentIndirection(
                pstate,
                0 as *mut Node,
                (*(n as *mut Value)).val.str_0,
                false,
                fieldTypeId,
                fieldTypMod,
                fieldCollation,
                indirection,
                lnext(indirection, i),
                rhs,
                ccontext,
                location,
            );
            fstore = ({
                let mut _result: *mut Node = 0 as *mut Node;
                (*_result).tag = T_FieldStore;
                _result
            }) as *mut FieldStore;
            (*fstore).arg = basenode as *mut Expr;
            (*fstore).newvals = list_make1_impl(
                T_List,
                ListCell {
                    ptr_value: rhs as *mut libc::c_void,
                },
            );
            (*fstore).fieldnums = list_make1_impl(
                T_IntList,
                ListCell {
                    int_value: attnum as libc::c_int,
                },
            );
            (*fstore).resulttype = baseTypeId;
            if baseTypeId != targetTypeId {
                return coerce_to_domain(
                    fstore as *mut Node,
                    baseTypeId,
                    baseTypeMod,
                    targetTypeId,
                    COERCION_IMPLICIT,
                    COERCE_IMPLICIT_CAST,
                    location,
                    false,
                );
            }
            return fstore as *mut Node;
        }
        i__state.i += 1;
        i__state.i;
    }
    if !subscripts.is_null() {
        return transformAssignmentSubscripts(
            pstate,
            basenode,
            targetName,
            targetTypeId,
            targetTypMod,
            targetCollation,
            subscripts,
            isSlice,
            indirection,
            0 as *mut ListCell,
            rhs,
            ccontext,
            location,
        );
    }
    result = coerce_to_target_type(
        pstate,
        rhs,
        exprType(rhs),
        targetTypeId,
        targetTypMod,
        ccontext,
        COERCE_IMPLICIT_CAST,
        -(1 as libc::c_int),
    );
    if result.is_null() {
        if targetIsSubscripting != 0 {
            let elevel__3: libc::c_int = 21 as libc::c_int;
            let mut __error_3: libc::c_int = 0;
            if elevel__3 >= 21 as libc::c_int {
                abort();
            }
        } else {
            let elevel__4: libc::c_int = 21 as libc::c_int;
            let mut __error_4: libc::c_int = 0;
            if elevel__4 >= 21 as libc::c_int {
                abort();
            }
        }
    }
    return result;
}
unsafe extern "C" fn transformAssignmentSubscripts(
    mut pstate: *mut ParseState,
    mut basenode: *mut Node,
    mut targetName: *const libc::c_char,
    mut targetTypeId: Oid,
    mut targetTypMod: i32,
    mut targetCollation: Oid,
    mut subscripts: *mut List,
    mut isSlice: bool,
    mut indirection: *mut List,
    mut next_indirection: *mut ListCell,
    mut rhs: *mut Node,
    mut ccontext: CoercionContext,
    mut location: libc::c_int,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut sbsref: *mut SubscriptingRef = 0 as *mut SubscriptingRef;
    let mut containerType: Oid = 0;
    let mut containerTypMod: i32 = 0;
    let mut typeNeeded: Oid = 0;
    let mut typmodNeeded: i32 = 0;
    let mut collationNeeded: Oid = 0;
    containerType = targetTypeId;
    containerTypMod = targetTypMod;
    transformContainerType(&mut containerType, &mut containerTypMod);
    sbsref = transformContainerSubscripts(
        pstate,
        basenode,
        containerType,
        containerTypMod,
        subscripts,
        true,
    );
    typeNeeded = (*sbsref).refrestype;
    typmodNeeded = (*sbsref).reftypmod;
    if containerType == targetTypeId {
        collationNeeded = targetCollation;
    } else {
        collationNeeded = get_typcollation(containerType);
    }
    rhs = transformAssignmentIndirection(
        pstate,
        0 as *mut Node,
        targetName,
        true,
        typeNeeded,
        typmodNeeded,
        collationNeeded,
        indirection,
        next_indirection,
        rhs,
        ccontext,
        location,
    );
    (*sbsref).refassgnexpr = rhs as *mut Expr;
    (*sbsref).refrestype = containerType;
    (*sbsref).reftypmod = containerTypMod;
    result = sbsref as *mut Node;
    if containerType != targetTypeId {
        let mut resulttype: Oid = exprType(result);
        result = coerce_to_target_type(
            pstate,
            result,
            resulttype,
            targetTypeId,
            targetTypMod,
            ccontext,
            COERCE_IMPLICIT_CAST,
            -(1 as libc::c_int),
        );
        if result.is_null() {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn checkInsertTargets(
    mut pstate: *mut ParseState,
    mut cols: *mut List,
    mut attrnos: *mut *mut List,
) -> *mut List {
    *attrnos = 0 as *mut libc::c_void as *mut List;
    if cols.is_null() {
        let mut numcol: libc::c_int =
            (*(*(*pstate).p_target_relation).rd_rel).relnatts as libc::c_int;
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < numcol {
            let mut col: *mut ResTarget = 0 as *mut ResTarget;
            let mut attr: Form_pg_attribute = 0 as *mut FormData_pg_attribute;
            attr = &mut *((*(*(*pstate).p_target_relation).rd_att).attrs)
                .as_mut_ptr()
                .offset(i as isize) as *mut FormData_pg_attribute;
            if !((*attr).attisdropped != 0) {
                col = ({
                    let mut _result: *mut Node = 0 as *mut Node;
                    (*_result).tag = T_ResTarget;
                    _result
                }) as *mut ResTarget;
                (*col).name = pstrdup(((*attr).attname.data).as_mut_ptr());
                (*col).indirection = 0 as *mut libc::c_void as *mut List;
                (*col).val = 0 as *mut Node;
                (*col).location = -(1 as libc::c_int);
                cols = lappend(cols, col as *mut libc::c_void);
                *attrnos = lappend_int(*attrnos, i + 1 as libc::c_int);
            }
            i += 1;
            i;
        }
    } else {
        let mut wholecols: *mut Bitmapset = 0 as *mut Bitmapset;
        let mut partialcols: *mut Bitmapset = 0 as *mut Bitmapset;
        let mut tl: *mut ListCell = 0 as *mut ListCell;
        let mut tl__state: ForEachState = {
            let mut init = ForEachState {
                l: cols,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(tl__state.l).is_null() && tl__state.i < (*tl__state.l).length {
            tl = &mut *((*tl__state.l).elements).offset(tl__state.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            tl = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut col_0: *mut ResTarget = (*tl).ptr_value as *mut ResTarget;
            let mut name: *mut libc::c_char = (*col_0).name;
            let mut attrno: libc::c_int = 0;
            attrno = attnameAttNum((*pstate).p_target_relation, name, false);
            if attrno == 0 as libc::c_int {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            if ((*col_0).indirection).is_null() {
                if bms_is_member(attrno, wholecols) as libc::c_int != 0
                    || bms_is_member(attrno, partialcols) as libc::c_int != 0
                {
                    let elevel__0: libc::c_int = 21 as libc::c_int;
                    let mut __error_0: libc::c_int = 0;
                    if elevel__0 >= 21 as libc::c_int {
                        abort();
                    }
                }
                wholecols = bms_add_member(wholecols, attrno);
            } else {
                if bms_is_member(attrno, wholecols) {
                    let elevel__1: libc::c_int = 21 as libc::c_int;
                    let mut __error_1: libc::c_int = 0;
                    if elevel__1 >= 21 as libc::c_int {
                        abort();
                    }
                }
                partialcols = bms_add_member(partialcols, attrno);
            }
            *attrnos = lappend_int(*attrnos, attrno);
            tl__state.i += 1;
            tl__state.i;
        }
    }
    return cols;
}
unsafe extern "C" fn ExpandColumnRefStar(
    mut pstate: *mut ParseState,
    mut cref: *mut ColumnRef,
    mut make_target_entry: bool,
) -> *mut List {
    let mut fields: *mut List = (*cref).fields;
    let mut numnames: libc::c_int = list_length(fields);
    if numnames == 1 as libc::c_int {
        return ExpandAllTables(pstate, (*cref).location);
    } else {
        let mut nspname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut relname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
        let mut levels_up: libc::c_int = 0;
        let mut crserr: C2RustUnnamed_1 = CRSERR_NO_RTE;
        if ((*pstate).p_pre_columnref_hook).is_some() {
            let mut node: *mut Node = 0 as *mut Node;
            node =
                ((*pstate).p_pre_columnref_hook).expect("non-null function pointer")(pstate, cref);
            if !node.is_null() {
                return ExpandRowReference(pstate, node, make_target_entry);
            }
        }
        match numnames {
            2 => {
                relname = (*((*list_nth_cell(fields, 0 as libc::c_int)).ptr_value as *mut Value))
                    .val
                    .str_0;
                nsitem = refnameNamespaceItem(
                    pstate,
                    nspname,
                    relname,
                    (*cref).location,
                    &mut levels_up,
                );
            }
            3 => {
                nspname = (*((*list_nth_cell(fields, 0 as libc::c_int)).ptr_value as *mut Value))
                    .val
                    .str_0;
                relname = (*((*list_nth_cell(fields, 1 as libc::c_int)).ptr_value as *mut Value))
                    .val
                    .str_0;
                nsitem = refnameNamespaceItem(
                    pstate,
                    nspname,
                    relname,
                    (*cref).location,
                    &mut levels_up,
                );
            }
            4 => {
                let mut catname: *mut libc::c_char =
                    (*((*list_nth_cell(fields, 0 as libc::c_int)).ptr_value as *mut Value))
                        .val
                        .str_0;
                if strcmp(catname, get_database_name(MyDatabaseId)) != 0 as libc::c_int {
                    crserr = CRSERR_WRONG_DB;
                } else {
                    nspname = (*((*list_nth_cell(fields, 1 as libc::c_int)).ptr_value
                        as *mut Value))
                        .val
                        .str_0;
                    relname = (*((*list_nth_cell(fields, 2 as libc::c_int)).ptr_value
                        as *mut Value))
                        .val
                        .str_0;
                    nsitem = refnameNamespaceItem(
                        pstate,
                        nspname,
                        relname,
                        (*cref).location,
                        &mut levels_up,
                    );
                }
            }
            _ => {
                crserr = CRSERR_TOO_MANY;
            }
        }
        if ((*pstate).p_post_columnref_hook).is_some() {
            let mut node_0: *mut Node = 0 as *mut Node;
            node_0 = ((*pstate).p_post_columnref_hook).expect("non-null function pointer")(
                pstate,
                cref,
                (if !nsitem.is_null() {
                    (*nsitem).p_rte
                } else {
                    0 as *mut RangeTblEntry
                }) as *mut Node,
            );
            if !node_0.is_null() {
                if !nsitem.is_null() {
                    let elevel_: libc::c_int = 21 as libc::c_int;
                    let mut __error: libc::c_int = 0;
                    if elevel_ >= 21 as libc::c_int {
                        abort();
                    }
                }
                return ExpandRowReference(pstate, node_0, make_target_entry);
            }
        }
        if nsitem.is_null() {
            match crserr as libc::c_uint {
                0 => {
                    errorMissingRTE(pstate, makeRangeVar(nspname, relname, (*cref).location));
                }
                1 => {
                    let elevel__0: libc::c_int = 21 as libc::c_int;
                    let mut __error_0: libc::c_int = 0;
                    if elevel__0 >= 21 as libc::c_int {
                        abort();
                    }
                }
                2 => {
                    let elevel__1: libc::c_int = 21 as libc::c_int;
                    let mut __error_1: libc::c_int = 0;
                    if elevel__1 >= 21 as libc::c_int {
                        abort();
                    }
                }
                _ => {}
            }
        }
        return ExpandSingleTable(
            pstate,
            nsitem,
            levels_up,
            (*cref).location,
            make_target_entry,
        );
    };
}
unsafe extern "C" fn ExpandAllTables(
    mut pstate: *mut ParseState,
    mut location: libc::c_int,
) -> *mut List {
    let mut target: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut found_table: bool = false;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: (*pstate).p_namespace,
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
        let mut nsitem: *mut ParseNamespaceItem = (*l).ptr_value as *mut ParseNamespaceItem;
        if !((*nsitem).p_cols_visible == 0) {
            found_table = true;
            target = list_concat(
                target,
                expandNSItemAttrs(pstate, nsitem, 0 as libc::c_int, location),
            );
        }
        l__state.i += 1;
        l__state.i;
    }
    if found_table == 0 {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return target;
}
unsafe extern "C" fn ExpandIndirectionStar(
    mut pstate: *mut ParseState,
    mut ind: *mut A_Indirection,
    mut make_target_entry: bool,
    mut exprKind: ParseExprKind,
) -> *mut List {
    let mut expr: *mut Node = 0 as *mut Node;
    ind = copyObjectImpl(ind as *const libc::c_void) as *mut A_Indirection;
    (*ind).indirection = list_truncate(
        (*ind).indirection,
        list_length((*ind).indirection) - 1 as libc::c_int,
    );
    expr = transformExpr(pstate, ind as *mut Node, exprKind);
    return ExpandRowReference(pstate, expr, make_target_entry);
}
unsafe extern "C" fn ExpandSingleTable(
    mut pstate: *mut ParseState,
    mut nsitem: *mut ParseNamespaceItem,
    mut sublevels_up: libc::c_int,
    mut location: libc::c_int,
    mut make_target_entry: bool,
) -> *mut List {
    if make_target_entry != 0 {
        return expandNSItemAttrs(pstate, nsitem, sublevels_up, location);
    } else {
        let mut rte: *mut RangeTblEntry = (*nsitem).p_rte;
        let mut vars: *mut List = 0 as *mut List;
        let mut l: *mut ListCell = 0 as *mut ListCell;
        vars = expandNSItemVars(nsitem, sublevels_up, location, 0 as *mut *mut List);
        if (*rte).rtekind as libc::c_uint == RTE_RELATION as libc::c_int as libc::c_uint {
            (*rte).requiredPerms |= ((1 as libc::c_int) << 1 as libc::c_int) as AclMode;
        }
        let mut l__state: ForEachState = {
            let mut init = ForEachState {
                l: vars,
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
            let mut var: *mut Var = (*l).ptr_value as *mut Var;
            markVarForSelectPriv(pstate, var);
            l__state.i += 1;
            l__state.i;
        }
        return vars;
    };
}
#[no_mangle]
pub unsafe extern "C" fn FigureColname(mut node: *mut Node) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    FigureColnameInternal(node, &mut name);
    if !name.is_null() {
        return name;
    }
    return b"?column?\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn FigureIndexColname(mut node: *mut Node) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    FigureColnameInternal(node, &mut name);
    return name;
}
unsafe extern "C" fn FigureColnameInternal(
    mut node: *mut Node,
    mut name: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut strength: libc::c_int = 0 as libc::c_int;
    if node.is_null() {
        return strength;
    }
    match (*(node as *const Node)).tag as libc::c_uint {
        349 => {
            let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut l: *mut ListCell = 0 as *mut ListCell;
            let mut l__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*(node as *mut ColumnRef)).fields,
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
                let mut i: *mut Node = (*l).ptr_value as *mut Node;
                if (*(i as *const Node)).tag as libc::c_uint
                    == T_String as libc::c_int as libc::c_uint
                {
                    fname = (*(i as *mut Value)).val.str_0;
                }
                l__state.i += 1;
                l__state.i;
            }
            if !fname.is_null() {
                *name = fname;
                return 2 as libc::c_int;
            }
        }
        355 => {
            let mut ind: *mut A_Indirection = node as *mut A_Indirection;
            let mut fname_0: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut l_0: *mut ListCell = 0 as *mut ListCell;
            let mut l__state_0: ForEachState = {
                let mut init = ForEachState {
                    l: (*ind).indirection,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(l__state_0.l).is_null() && l__state_0.i < (*l__state_0.l).length {
                l_0 =
                    &mut *((*l__state_0.l).elements).offset(l__state_0.i as isize) as *mut ListCell;
                true as libc::c_int
            } else {
                l_0 = 0 as *mut ListCell;
                false as libc::c_int
            } != 0
            {
                let mut i_0: *mut Node = (*l_0).ptr_value as *mut Node;
                if (*(i_0 as *const Node)).tag as libc::c_uint
                    == T_String as libc::c_int as libc::c_uint
                {
                    fname_0 = (*(i_0 as *mut Value)).val.str_0;
                }
                l__state_0.i += 1;
                l__state_0.i;
            }
            if !fname_0.is_null() {
                *name = fname_0;
                return 2 as libc::c_int;
            }
            return FigureColnameInternal((*ind).arg, name);
        }
        352 => {
            *name = (*((*list_last_cell((*(node as *mut FuncCall)).funcname)).ptr_value
                as *mut Value))
                .val
                .str_0;
            return 2 as libc::c_int;
        }
        348 => {
            if (*(node as *mut A_Expr)).kind as libc::c_uint
                == AEXPR_NULLIF as libc::c_int as libc::c_uint
            {
                *name = b"nullif\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
        }
        359 => {
            strength = FigureColnameInternal((*(node as *mut TypeCast)).arg, name);
            if strength <= 1 as libc::c_int {
                if !((*(node as *mut TypeCast)).typeName).is_null() {
                    *name = (*((*list_last_cell((*(*(node as *mut TypeCast)).typeName).names))
                        .ptr_value as *mut Value))
                        .val
                        .str_0;
                    return 1 as libc::c_int;
                }
            }
        }
        360 => return FigureColnameInternal((*(node as *mut CollateClause)).arg, name),
        112 => {
            *name = b"grouping\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            return 2 as libc::c_int;
        }
        122 => match (*(node as *mut SubLink)).subLinkType as libc::c_uint {
            0 => {
                *name = b"exists\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            6 => {
                *name = b"array\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            4 => {
                let mut sublink: *mut SubLink = node as *mut SubLink;
                let mut query: *mut Query = (*sublink).subselect as *mut Query;
                if (*(query as *const Node)).tag as libc::c_uint
                    == T_Query as libc::c_int as libc::c_uint
                {
                    let mut te: *mut TargetEntry =
                        (*list_nth_cell((*query).targetList, 0 as libc::c_int)).ptr_value
                            as *mut TargetEntry;
                    if !((*te).resname).is_null() {
                        *name = (*te).resname;
                        return 2 as libc::c_int;
                    }
                }
            }
            5 | 1 | 2 | 3 | 7 | _ => {}
        },
        132 => {
            strength =
                FigureColnameInternal((*(node as *mut CaseExpr)).defresult as *mut Node, name);
            if strength <= 1 as libc::c_int {
                *name = b"case\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 1 as libc::c_int;
            }
        }
        356 => {
            *name = b"array\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            return 2 as libc::c_int;
        }
        136 => {
            *name = b"row\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            return 2 as libc::c_int;
        }
        138 => {
            *name = b"coalesce\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            return 2 as libc::c_int;
        }
        139 => match (*(node as *mut MinMaxExpr)).op as libc::c_uint {
            0 => {
                *name = b"greatest\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            1 => {
                *name = b"least\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            _ => {}
        },
        140 => match (*(node as *mut SQLValueFunction)).op as libc::c_uint {
            0 => {
                *name = b"current_date\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            1 | 2 => {
                *name = b"current_time\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            3 | 4 => {
                *name =
                    b"current_timestamp\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            5 | 6 => {
                *name = b"localtime\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            7 | 8 => {
                *name =
                    b"localtimestamp\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            9 => {
                *name = b"current_role\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            10 => {
                *name = b"current_user\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            11 => {
                *name = b"user\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            12 => {
                *name = b"session_user\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            13 => {
                *name =
                    b"current_catalog\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            14 => {
                *name =
                    b"current_schema\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            _ => {}
        },
        141 => match (*(node as *mut XmlExpr)).op as libc::c_uint {
            0 => {
                *name = b"xmlconcat\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            1 => {
                *name = b"xmlelement\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            2 => {
                *name = b"xmlforest\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            3 => {
                *name = b"xmlparse\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            4 => {
                *name = b"xmlpi\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            5 => {
                *name = b"xmlroot\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            6 => {
                *name = b"xmlserialize\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                return 2 as libc::c_int;
            }
            7 | _ => {}
        },
        387 => {
            *name = b"xmlserialize\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            return 2 as libc::c_int;
        }
        _ => {}
    }
    return strength;
}
