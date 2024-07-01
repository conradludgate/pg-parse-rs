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
//     pub type dsa_area;
//     pub type TIDBitmap;
//     pub type PgStat_TableStatus;
//     pub type FdwRoutine;
//     pub type GlobalVisState;
//     pub type PartitionBoundInfoData;
//     pub type HTAB;
//     pub type PartitionDirectoryData;
//     pub type QueryEnvironment;
//     pub type JitInstrumentation;
//     pub type JitContext;
//     pub type CopyMultiInsertBuffer;
//     pub type SharedJitInstrumentation;
//     pub type ExprEvalStep;
//     pub type BufferAccessStrategyData;
//     pub type ValidateIndexState;
//     pub type VacuumParams;
//     pub type BulkInsertStateData;
//     pub type PartitionDescData;
//     pub type PartitionKeyData;
//     pub type RowSecurityDesc;
//     pub type SMgrRelationData;
//     fn abort() -> !;
//     fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn palloc(size: usize) -> *mut libc::c_void;
//     fn palloc0(size: usize) -> *mut libc::c_void;
//     fn pfree(pointer: *mut libc::c_void);
//     fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
//     fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
//     fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn lappend_int(list: *mut List, datum: libc::c_int) -> *mut List;
//     fn lappend_oid(list: *mut List, datum: Oid) -> *mut List;
//     fn list_concat(list1: *mut List, list2: *const List) -> *mut List;
//     fn list_truncate(list: *mut List, new_size: libc::c_int) -> *mut List;
//     fn list_member_int(list: *const List, datum: libc::c_int) -> bool;
//     fn copyObjectImpl(obj: *const libc::c_void) -> *mut libc::c_void;
//     fn equal(a: *const libc::c_void, b: *const libc::c_void) -> bool;
//     fn bms_is_member(x: libc::c_int, a: *const Bitmapset) -> bool;
//     fn bms_add_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
//     fn bms_add_members(a: *mut Bitmapset, b: *const Bitmapset) -> *mut Bitmapset;
//     fn makeString(str: *mut libc::c_char) -> *mut Value;
//     fn table_close(relation: Relation, lockmode: LOCKMODE);
//     fn GetTsmRoutine(tsmhandler: Oid) -> *mut TsmRoutine;
//     fn IsCatalogRelation(relation: Relation) -> bool;
//     fn setup_parser_errposition_callback(
//         pcbstate: *mut ParseCallbackState,
//         pstate: *mut ParseState,
//         location: libc::c_int,
//     );
//     fn cancel_parser_errposition_callback(pcbstate: *mut ParseCallbackState);
//     fn get_relation_constraint_attnos(
//         relid: Oid,
//         conname: *const libc::c_char,
//         missing_ok: bool,
//         constraintOid: *mut Oid,
//     ) -> *mut Bitmapset;
//     fn check_stack_depth();
//     fn makeSimpleA_Expr(
//         kind: A_Expr_Kind,
//         name: *mut libc::c_char,
//         lexpr: *mut Node,
//         rexpr: *mut Node,
//         location: libc::c_int,
//     ) -> *mut A_Expr;
//     fn makeVar(
//         varno: Index,
//         varattno: AttrNumber,
//         vartype: Oid,
//         vartypmod: i32,
//         varcollid: Oid,
//         varlevelsup: Index,
//     ) -> *mut Var;
//     fn makeBoolExpr(
//         boolop: BoolExprType,
//         args: *mut List,
//         location: libc::c_int,
//     ) -> *mut Expr;
//     fn makeRelabelType(
//         arg: *mut Expr,
//         rtype: Oid,
//         rtypmod: i32,
//         rcollid: Oid,
//         rformat: CoercionForm,
//     ) -> *mut RelabelType;
//     fn makeFuncCall(
//         name: *mut List,
//         args: *mut List,
//         funcformat: CoercionForm,
//         location: libc::c_int,
//     ) -> *mut FuncCall;
//     fn makeGroupingSet(
//         kind: GroupingSetKind,
//         content: *mut List,
//         location: libc::c_int,
//     ) -> *mut GroupingSet;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprTypmod(expr: *const Node) -> i32;
//     fn strip_implicit_coercions(node: *mut Node) -> *mut Node;
//     fn exprCollation(expr: *const Node) -> Oid;
//     fn exprLocation(expr: *const Node) -> libc::c_int;
//     fn get_sortgroupref_tle(sortref: Index, targetList: *mut List) -> *mut TargetEntry;
//     fn get_sortgroupclause_tle(
//         sgClause: *mut SortGroupClause,
//         targetList: *mut List,
//     ) -> *mut TargetEntry;
//     fn get_sortgroupclause_expr(
//         sgClause: *mut SortGroupClause,
//         targetList: *mut List,
//     ) -> *mut Node;
//     fn contain_vars_of_level(node: *mut Node, levelsup: libc::c_int) -> bool;
//     fn parse_sub_analyze(
//         parseTree: *mut Node,
//         parentParseState: *mut ParseState,
//         parentCTE: *mut CommonTableExpr,
//         locked_from_parent: bool,
//         resolve_unknowns: bool,
//     ) -> *mut Query;
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
//     fn coerce_to_boolean(
//         pstate: *mut ParseState,
//         node: *mut Node,
//         constructName: *const libc::c_char,
//     ) -> *mut Node;
//     fn coerce_to_specific_type(
//         pstate: *mut ParseState,
//         node: *mut Node,
//         targetTypeId: Oid,
//         constructName: *const libc::c_char,
//     ) -> *mut Node;
//     fn coerce_to_specific_type_typmod(
//         pstate: *mut ParseState,
//         node: *mut Node,
//         targetTypeId: Oid,
//         targetTypmod: i32,
//         constructName: *const libc::c_char,
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
//     fn assign_list_collations(pstate: *mut ParseState, exprs: *mut List);
//     fn assign_expr_collations(pstate: *mut ParseState, expr: *mut Node);
//     fn transformExpr(
//         pstate: *mut ParseState,
//         expr: *mut Node,
//         exprKind: ParseExprKind,
//     ) -> *mut Node;
//     fn LookupFuncName(
//         funcname: *mut List,
//         nargs: libc::c_int,
//         argtypes: *const Oid,
//         missing_ok: bool,
//     ) -> Oid;
//     fn get_sort_group_operators(
//         argtype: Oid,
//         needLT: bool,
//         needEQ: bool,
//         needGT: bool,
//         ltOpr: *mut Oid,
//         eqOpr: *mut Oid,
//         gtOpr: *mut Oid,
//         isHashable: *mut bool,
//     );
//     fn compatible_oper_opid(op: *mut List, arg1: Oid, arg2: Oid, noError: bool) -> Oid;
//     fn scanNameSpaceForCTE(
//         pstate: *mut ParseState,
//         refname: *const libc::c_char,
//         ctelevelsup: *mut Index,
//     ) -> *mut CommonTableExpr;
//     fn scanNameSpaceForENR(
//         pstate: *mut ParseState,
//         refname: *const libc::c_char,
//     ) -> bool;
//     fn checkNameSpaceConflicts(
//         pstate: *mut ParseState,
//         namespace1: *mut List,
//         namespace2: *mut List,
//     );
//     fn colNameToVar(
//         pstate: *mut ParseState,
//         colname: *const libc::c_char,
//         localonly: bool,
//         location: libc::c_int,
//     ) -> *mut Node;
//     fn markVarForSelectPriv(pstate: *mut ParseState, var: *mut Var);
//     fn parserOpenTable(
//         pstate: *mut ParseState,
//         relation: *const RangeVar,
//         lockmode: libc::c_int,
//     ) -> Relation;
//     fn addRangeTableEntry(
//         pstate: *mut ParseState,
//         relation: *mut RangeVar,
//         alias: *mut Alias,
//         inh: bool,
//         inFromCl: bool,
//     ) -> *mut ParseNamespaceItem;
//     fn addRangeTableEntryForRelation(
//         pstate: *mut ParseState,
//         rel: Relation,
//         lockmode: libc::c_int,
//         alias: *mut Alias,
//         inh: bool,
//         inFromCl: bool,
//     ) -> *mut ParseNamespaceItem;
//     fn addRangeTableEntryForSubquery(
//         pstate: *mut ParseState,
//         subquery: *mut Query,
//         alias: *mut Alias,
//         lateral: bool,
//         inFromCl: bool,
//     ) -> *mut ParseNamespaceItem;
//     fn addRangeTableEntryForFunction(
//         pstate: *mut ParseState,
//         funcnames: *mut List,
//         funcexprs: *mut List,
//         coldeflists: *mut List,
//         rangefunc: *mut RangeFunction,
//         lateral: bool,
//         inFromCl: bool,
//     ) -> *mut ParseNamespaceItem;
//     fn addRangeTableEntryForTableFunc(
//         pstate: *mut ParseState,
//         tf: *mut TableFunc,
//         alias: *mut Alias,
//         lateral: bool,
//         inFromCl: bool,
//     ) -> *mut ParseNamespaceItem;
//     fn addRangeTableEntryForJoin(
//         pstate: *mut ParseState,
//         colnames: *mut List,
//         nscolumns: *mut ParseNamespaceColumn,
//         jointype: JoinType,
//         nummergedcols: libc::c_int,
//         aliasvars: *mut List,
//         leftcols: *mut List,
//         rightcols: *mut List,
//         alias: *mut Alias,
//         inFromCl: bool,
//     ) -> *mut ParseNamespaceItem;
//     fn addRangeTableEntryForCTE(
//         pstate: *mut ParseState,
//         cte: *mut CommonTableExpr,
//         levelsup: Index,
//         rv: *mut RangeVar,
//         inFromCl: bool,
//     ) -> *mut ParseNamespaceItem;
//     fn addRangeTableEntryForENR(
//         pstate: *mut ParseState,
//         rv: *mut RangeVar,
//         inFromCl: bool,
//     ) -> *mut ParseNamespaceItem;
//     fn isLockedRefname(pstate: *mut ParseState, refname: *const libc::c_char) -> bool;
//     fn addNSItemToQuery(
//         pstate: *mut ParseState,
//         nsitem: *mut ParseNamespaceItem,
//         addToJoinList: bool,
//         addToRelNameSpace: bool,
//         addToVarNameSpace: bool,
//     );
//     fn transformTargetEntry(
//         pstate: *mut ParseState,
//         node: *mut Node,
//         expr: *mut Node,
//         exprKind: ParseExprKind,
//         colname: *mut libc::c_char,
//         resjunk: bool,
//     ) -> *mut TargetEntry;
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
//     fn SystemFuncName(name: *mut libc::c_char) -> *mut List;
//     fn contain_aggs_of_level(node: *mut Node, levelsup: libc::c_int) -> bool;
//     fn contain_windowfuncs(node: *mut Node) -> bool;
//     fn ReleaseCatCacheList(list: *mut CatCList);
//     fn get_ordering_op_properties(
//         opno: Oid,
//         opfamily: *mut Oid,
//         opcintype: *mut Oid,
//         strategy: *mut i16,
//     ) -> bool;
//     fn get_equality_op_for_ordering_op(opno: Oid, reverse: *mut bool) -> Oid;
//     fn op_hashjoinable(opno: Oid, inputtype: Oid) -> bool;
//     fn get_commutator(opno: Oid) -> Oid;
//     fn get_typcollation(typid: Oid) -> Oid;
//     fn SearchSysCacheList(
//         cacheId: libc::c_int,
//         nkeys: libc::c_int,
//         key1: Datum,
//         key2: Datum,
//         key3: Datum,
//     ) -> *mut catclist;
// }
use super::*;
// pub type Oid = libc::c_uint;
pub type __i32_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_suseconds_t = __i32_t;
// pub type usize = libc::c_ulong;
// pub type isize = __darwin_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}
// pub type bool = libc::c_uchar;
// pub type i16 = libc::c_short;
// pub type i32 = libc::c_int;
// pub type u8 = libc::c_uchar;
// pub type u16 = libc::c_ushort;
// pub type u32 = libc::c_uint;
pub type bits8 = u8;
// pub type i64 = libc::c_long;
// pub type uint64 = libc::c_ulong;
// pub type usize = isize;
pub type Index = libc::c_uint;
pub type float4 = libc::c_float;
pub type regproc = Oid;
pub type RegProcedure = regproc;
pub type TransactionId = u32;
pub type SubTransactionId = u32;
pub type MultiXactId = TransactionId;
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
pub type BlockNumber = u32;
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
pub type ItemPointer = *mut ItemPointerData;
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
pub struct MinimalTupleData {
    pub t_len: u32,
    pub mt_padding: libc::c_char,
    pub t_infomask2: u16,
    pub t_infomask: u16,
    pub t_hoff: u8,
    pub t_bits: [bits8; 0],
}
pub type MinimalTuple = *mut MinimalTupleData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleData {
    pub t_len: u32,
    pub t_self: ItemPointerData,
    pub t_tableOid: Oid,
    pub t_data: HeapTupleHeader,
}
pub type HeapTuple = *mut HeapTupleData;
pub type XLogRecPtr = uint64;
pub type AttrNumber = i16;
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
pub type NodeTag = libc::c_uint;
pub const T_SupportRequestIndexCondition: NodeTag = 425;
pub const T_SupportRequestRows: NodeTag = 424;
pub const T_SupportRequestCost: NodeTag = 423;
pub const T_SupportRequestSelectivity: NodeTag = 422;
pub const T_SupportRequestSimplify: NodeTag = 421;
pub const T_CallContext: NodeTag = 420;
pub const T_ForeignKeyCacheInfo: NodeTag = 419;
pub const T_TsmRoutine: NodeTag = 418;
pub const T_TableAmRoutine: NodeTag = 417;
pub const T_IndexAmRoutine: NodeTag = 416;
pub const T_FdwRoutine: NodeTag = 415;
pub const T_InlineCodeBlock: NodeTag = 414;
pub const T_TIDBitmap: NodeTag = 413;
pub const T_WindowObjectData: NodeTag = 412;
pub const T_ReturnSetInfo: NodeTag = 411;
pub const T_EventTriggerData: NodeTag = 410;
pub const T_TriggerData: NodeTag = 409;
pub const T_SQLCmd: NodeTag = 408;
pub const T_TimeLineHistoryCmd: NodeTag = 407;
pub const T_StartReplicationCmd: NodeTag = 406;
pub const T_DropReplicationSlotCmd: NodeTag = 405;
pub const T_CreateReplicationSlotCmd: NodeTag = 404;
pub const T_BaseBackupCmd: NodeTag = 403;
pub const T_IdentifySystemCmd: NodeTag = 402;
pub const T_VacuumRelation: NodeTag = 401;
pub const T_PartitionCmd: NodeTag = 400;
pub const T_PartitionRangeDatum: NodeTag = 399;
pub const T_PartitionBoundSpec: NodeTag = 398;
pub const T_PartitionSpec: NodeTag = 397;
pub const T_PartitionElem: NodeTag = 396;
pub const T_TriggerTransition: NodeTag = 395;
pub const T_RoleSpec: NodeTag = 394;
pub const T_CommonTableExpr: NodeTag = 393;
pub const T_CTECycleClause: NodeTag = 392;
pub const T_CTESearchClause: NodeTag = 391;
pub const T_OnConflictClause: NodeTag = 390;
pub const T_InferClause: NodeTag = 389;
pub const T_WithClause: NodeTag = 388;
pub const T_XmlSerialize: NodeTag = 387;
pub const T_RowMarkClause: NodeTag = 386;
pub const T_LockingClause: NodeTag = 385;
pub const T_FunctionParameter: NodeTag = 384;
pub const T_TableLikeClause: NodeTag = 383;
pub const T_CreateOpClassItem: NodeTag = 382;
pub const T_AccessPriv: NodeTag = 381;
pub const T_ObjectWithArgs: NodeTag = 380;
pub const T_WindowClause: NodeTag = 379;
pub const T_GroupingSet: NodeTag = 378;
pub const T_SortGroupClause: NodeTag = 377;
pub const T_WithCheckOption: NodeTag = 376;
pub const T_TableSampleClause: NodeTag = 375;
pub const T_RangeTblFunction: NodeTag = 374;
pub const T_RangeTblEntry: NodeTag = 373;
pub const T_DefElem: NodeTag = 372;
pub const T_Constraint: NodeTag = 371;
pub const T_IndexElem: NodeTag = 370;
pub const T_ColumnDef: NodeTag = 369;
pub const T_TypeName: NodeTag = 368;
pub const T_RangeTableFuncCol: NodeTag = 367;
pub const T_RangeTableFunc: NodeTag = 366;
pub const T_RangeTableSample: NodeTag = 365;
pub const T_RangeFunction: NodeTag = 364;
pub const T_RangeSubselect: NodeTag = 363;
pub const T_WindowDef: NodeTag = 362;
pub const T_SortBy: NodeTag = 361;
pub const T_CollateClause: NodeTag = 360;
pub const T_TypeCast: NodeTag = 359;
pub const T_MultiAssignRef: NodeTag = 358;
pub const T_ResTarget: NodeTag = 357;
pub const T_A_ArrayExpr: NodeTag = 356;
pub const T_A_Indirection: NodeTag = 355;
pub const T_A_Indices: NodeTag = 354;
pub const T_A_Star: NodeTag = 353;
pub const T_FuncCall: NodeTag = 352;
pub const T_A_Const: NodeTag = 351;
pub const T_ParamRef: NodeTag = 350;
pub const T_ColumnRef: NodeTag = 349;
pub const T_A_Expr: NodeTag = 348;
pub const T_AlterStatsStmt: NodeTag = 347;
pub const T_CallStmt: NodeTag = 346;
pub const T_AlterCollationStmt: NodeTag = 345;
pub const T_CreateStatsStmt: NodeTag = 344;
pub const T_DropSubscriptionStmt: NodeTag = 343;
pub const T_AlterSubscriptionStmt: NodeTag = 342;
pub const T_CreateSubscriptionStmt: NodeTag = 341;
pub const T_AlterPublicationStmt: NodeTag = 340;
pub const T_CreatePublicationStmt: NodeTag = 339;
pub const T_CreateAmStmt: NodeTag = 338;
pub const T_CreateTransformStmt: NodeTag = 337;
pub const T_AlterPolicyStmt: NodeTag = 336;
pub const T_CreatePolicyStmt: NodeTag = 335;
pub const T_AlterSystemStmt: NodeTag = 334;
pub const T_ReplicaIdentityStmt: NodeTag = 333;
pub const T_RefreshMatViewStmt: NodeTag = 332;
pub const T_AlterEventTrigStmt: NodeTag = 331;
pub const T_CreateEventTrigStmt: NodeTag = 330;
pub const T_AlterExtensionContentsStmt: NodeTag = 329;
pub const T_AlterExtensionStmt: NodeTag = 328;
pub const T_CreateExtensionStmt: NodeTag = 327;
pub const T_ImportForeignSchemaStmt: NodeTag = 326;
pub const T_CreateForeignTableStmt: NodeTag = 325;
pub const T_SecLabelStmt: NodeTag = 324;
pub const T_AlterTableMoveAllStmt: NodeTag = 323;
pub const T_AlterTableSpaceOptionsStmt: NodeTag = 322;
pub const T_DropUserMappingStmt: NodeTag = 321;
pub const T_AlterUserMappingStmt: NodeTag = 320;
pub const T_CreateUserMappingStmt: NodeTag = 319;
pub const T_AlterForeignServerStmt: NodeTag = 318;
pub const T_CreateForeignServerStmt: NodeTag = 317;
pub const T_AlterFdwStmt: NodeTag = 316;
pub const T_CreateFdwStmt: NodeTag = 315;
pub const T_AlterTSConfigurationStmt: NodeTag = 314;
pub const T_AlterTSDictionaryStmt: NodeTag = 313;
pub const T_AlterEnumStmt: NodeTag = 312;
pub const T_CreateRangeStmt: NodeTag = 311;
pub const T_CreateEnumStmt: NodeTag = 310;
pub const T_CompositeTypeStmt: NodeTag = 309;
pub const T_ReassignOwnedStmt: NodeTag = 308;
pub const T_DropOwnedStmt: NodeTag = 307;
pub const T_AlterTypeStmt: NodeTag = 306;
pub const T_AlterOperatorStmt: NodeTag = 305;
pub const T_AlterOwnerStmt: NodeTag = 304;
pub const T_AlterObjectSchemaStmt: NodeTag = 303;
pub const T_AlterObjectDependsStmt: NodeTag = 302;
pub const T_DropTableSpaceStmt: NodeTag = 301;
pub const T_CreateTableSpaceStmt: NodeTag = 300;
pub const T_DeclareCursorStmt: NodeTag = 299;
pub const T_DeallocateStmt: NodeTag = 298;
pub const T_ExecuteStmt: NodeTag = 297;
pub const T_PrepareStmt: NodeTag = 296;
pub const T_AlterOpFamilyStmt: NodeTag = 295;
pub const T_CreateOpFamilyStmt: NodeTag = 294;
pub const T_CreateOpClassStmt: NodeTag = 293;
pub const T_CreateCastStmt: NodeTag = 292;
pub const T_CreateConversionStmt: NodeTag = 291;
pub const T_AlterRoleSetStmt: NodeTag = 290;
pub const T_AlterDatabaseSetStmt: NodeTag = 289;
pub const T_AlterDatabaseStmt: NodeTag = 288;
pub const T_CreateSchemaStmt: NodeTag = 287;
pub const T_CheckPointStmt: NodeTag = 286;
pub const T_ReindexStmt: NodeTag = 285;
pub const T_ConstraintsSetStmt: NodeTag = 284;
pub const T_LockStmt: NodeTag = 283;
pub const T_DropRoleStmt: NodeTag = 282;
pub const T_AlterRoleStmt: NodeTag = 281;
pub const T_CreateRoleStmt: NodeTag = 280;
pub const T_CreatePLangStmt: NodeTag = 279;
pub const T_CreateTrigStmt: NodeTag = 278;
pub const T_DiscardStmt: NodeTag = 277;
pub const T_VariableShowStmt: NodeTag = 276;
pub const T_VariableSetStmt: NodeTag = 275;
pub const T_AlterSeqStmt: NodeTag = 274;
pub const T_CreateSeqStmt: NodeTag = 273;
pub const T_CreateTableAsStmt: NodeTag = 272;
pub const T_ExplainStmt: NodeTag = 271;
pub const T_VacuumStmt: NodeTag = 270;
pub const T_DropdbStmt: NodeTag = 269;
pub const T_CreatedbStmt: NodeTag = 268;
pub const T_CreateDomainStmt: NodeTag = 267;
pub const T_LoadStmt: NodeTag = 266;
pub const T_ViewStmt: NodeTag = 265;
pub const T_TransactionStmt: NodeTag = 264;
pub const T_UnlistenStmt: NodeTag = 263;
pub const T_ListenStmt: NodeTag = 262;
pub const T_NotifyStmt: NodeTag = 261;
pub const T_RuleStmt: NodeTag = 260;
pub const T_RenameStmt: NodeTag = 259;
pub const T_DoStmt: NodeTag = 258;
pub const T_AlterFunctionStmt: NodeTag = 257;
pub const T_CreateFunctionStmt: NodeTag = 256;
pub const T_IndexStmt: NodeTag = 255;
pub const T_FetchStmt: NodeTag = 254;
pub const T_CommentStmt: NodeTag = 253;
pub const T_TruncateStmt: NodeTag = 252;
pub const T_DropStmt: NodeTag = 251;
pub const T_DefineStmt: NodeTag = 250;
pub const T_CreateStmt: NodeTag = 249;
pub const T_CopyStmt: NodeTag = 248;
pub const T_ClusterStmt: NodeTag = 247;
pub const T_ClosePortalStmt: NodeTag = 246;
pub const T_AlterDefaultPrivilegesStmt: NodeTag = 245;
pub const T_GrantRoleStmt: NodeTag = 244;
pub const T_GrantStmt: NodeTag = 243;
pub const T_SetOperationStmt: NodeTag = 242;
pub const T_AlterDomainStmt: NodeTag = 241;
pub const T_AlterTableCmd: NodeTag = 240;
pub const T_AlterTableStmt: NodeTag = 239;
pub const T_PLAssignStmt: NodeTag = 238;
pub const T_SelectStmt: NodeTag = 237;
pub const T_UpdateStmt: NodeTag = 236;
pub const T_DeleteStmt: NodeTag = 235;
pub const T_InsertStmt: NodeTag = 234;
pub const T_PlannedStmt: NodeTag = 233;
pub const T_Query: NodeTag = 232;
pub const T_RawStmt: NodeTag = 231;
pub const T_ExtensibleNode: NodeTag = 230;
pub const T_OidList: NodeTag = 229;
pub const T_IntList: NodeTag = 228;
pub const T_List: NodeTag = 227;
pub const T_Null: NodeTag = 226;
pub const T_BitString: NodeTag = 225;
pub const T_String: NodeTag = 224;
pub const T_Float: NodeTag = 223;
pub const T_Integer: NodeTag = 222;
pub const T_Value: NodeTag = 221;
pub const T_GenerationContext: NodeTag = 220;
pub const T_SlabContext: NodeTag = 219;
pub const T_AllocSetContext: NodeTag = 218;
pub const T_MemoryContext: NodeTag = 217;
pub const T_StatisticExtInfo: NodeTag = 216;
pub const T_GroupingSetData: NodeTag = 215;
pub const T_RollupData: NodeTag = 214;
pub const T_PlannerParamItem: NodeTag = 213;
pub const T_MinMaxAggInfo: NodeTag = 212;
pub const T_PlaceHolderInfo: NodeTag = 211;
pub const T_AppendRelInfo: NodeTag = 210;
pub const T_SpecialJoinInfo: NodeTag = 209;
pub const T_PlaceHolderVar: NodeTag = 208;
pub const T_IndexClause: NodeTag = 207;
pub const T_RestrictInfo: NodeTag = 206;
pub const T_PathTarget: NodeTag = 205;
pub const T_PathKey: NodeTag = 204;
pub const T_EquivalenceMember: NodeTag = 203;
pub const T_EquivalenceClass: NodeTag = 202;
pub const T_LimitPath: NodeTag = 201;
pub const T_ModifyTablePath: NodeTag = 200;
pub const T_LockRowsPath: NodeTag = 199;
pub const T_RecursiveUnionPath: NodeTag = 198;
pub const T_SetOpPath: NodeTag = 197;
pub const T_WindowAggPath: NodeTag = 196;
pub const T_MinMaxAggPath: NodeTag = 195;
pub const T_GroupingSetsPath: NodeTag = 194;
pub const T_AggPath: NodeTag = 193;
pub const T_UpperUniquePath: NodeTag = 192;
pub const T_GroupPath: NodeTag = 191;
pub const T_IncrementalSortPath: NodeTag = 190;
pub const T_SortPath: NodeTag = 189;
pub const T_ProjectSetPath: NodeTag = 188;
pub const T_ProjectionPath: NodeTag = 187;
pub const T_GatherMergePath: NodeTag = 186;
pub const T_GatherPath: NodeTag = 185;
pub const T_UniquePath: NodeTag = 184;
pub const T_MaterialPath: NodeTag = 183;
pub const T_GroupResultPath: NodeTag = 182;
pub const T_MergeAppendPath: NodeTag = 181;
pub const T_AppendPath: NodeTag = 180;
pub const T_HashPath: NodeTag = 179;
pub const T_MergePath: NodeTag = 178;
pub const T_NestPath: NodeTag = 177;
pub const T_CustomPath: NodeTag = 176;
pub const T_ForeignPath: NodeTag = 175;
pub const T_SubqueryScanPath: NodeTag = 174;
pub const T_TidRangePath: NodeTag = 173;
pub const T_TidPath: NodeTag = 172;
pub const T_BitmapOrPath: NodeTag = 171;
pub const T_BitmapAndPath: NodeTag = 170;
pub const T_BitmapHeapPath: NodeTag = 169;
pub const T_IndexPath: NodeTag = 168;
pub const T_Path: NodeTag = 167;
pub const T_ParamPathInfo: NodeTag = 166;
pub const T_ForeignKeyOptInfo: NodeTag = 165;
pub const T_IndexOptInfo: NodeTag = 164;
pub const T_RelOptInfo: NodeTag = 163;
pub const T_PlannerGlobal: NodeTag = 162;
pub const T_PlannerInfo: NodeTag = 161;
pub const T_DomainConstraintState: NodeTag = 160;
pub const T_SubPlanState: NodeTag = 159;
pub const T_SetExprState: NodeTag = 158;
pub const T_WindowFuncExprState: NodeTag = 157;
pub const T_ExprState: NodeTag = 156;
pub const T_IntoClause: NodeTag = 155;
pub const T_OnConflictExpr: NodeTag = 154;
pub const T_FromExpr: NodeTag = 153;
pub const T_JoinExpr: NodeTag = 152;
pub const T_RangeTblRef: NodeTag = 151;
pub const T_TargetEntry: NodeTag = 150;
pub const T_InferenceElem: NodeTag = 149;
pub const T_NextValueExpr: NodeTag = 148;
pub const T_CurrentOfExpr: NodeTag = 147;
pub const T_SetToDefault: NodeTag = 146;
pub const T_CoerceToDomainValue: NodeTag = 145;
pub const T_CoerceToDomain: NodeTag = 144;
pub const T_BooleanTest: NodeTag = 143;
pub const T_NullTest: NodeTag = 142;
pub const T_XmlExpr: NodeTag = 141;
pub const T_SQLValueFunction: NodeTag = 140;
pub const T_MinMaxExpr: NodeTag = 139;
pub const T_CoalesceExpr: NodeTag = 138;
pub const T_RowCompareExpr: NodeTag = 137;
pub const T_RowExpr: NodeTag = 136;
pub const T_ArrayExpr: NodeTag = 135;
pub const T_CaseTestExpr: NodeTag = 134;
pub const T_CaseWhen: NodeTag = 133;
pub const T_CaseExpr: NodeTag = 132;
pub const T_CollateExpr: NodeTag = 131;
pub const T_ConvertRowtypeExpr: NodeTag = 130;
pub const T_ArrayCoerceExpr: NodeTag = 129;
pub const T_CoerceViaIO: NodeTag = 128;
pub const T_RelabelType: NodeTag = 127;
pub const T_FieldStore: NodeTag = 126;
pub const T_FieldSelect: NodeTag = 125;
pub const T_AlternativeSubPlan: NodeTag = 124;
pub const T_SubPlan: NodeTag = 123;
pub const T_SubLink: NodeTag = 122;
pub const T_BoolExpr: NodeTag = 121;
pub const T_ScalarArrayOpExpr: NodeTag = 120;
pub const T_NullIfExpr: NodeTag = 119;
pub const T_DistinctExpr: NodeTag = 118;
pub const T_OpExpr: NodeTag = 117;
pub const T_NamedArgExpr: NodeTag = 116;
pub const T_FuncExpr: NodeTag = 115;
pub const T_SubscriptingRef: NodeTag = 114;
pub const T_WindowFunc: NodeTag = 113;
pub const T_GroupingFunc: NodeTag = 112;
pub const T_Aggref: NodeTag = 111;
pub const T_Param: NodeTag = 110;
pub const T_Const: NodeTag = 109;
pub const T_Var: NodeTag = 108;
pub const T_Expr: NodeTag = 107;
pub const T_TableFunc: NodeTag = 106;
pub const T_RangeVar: NodeTag = 105;
pub const T_Alias: NodeTag = 104;
pub const T_LimitState: NodeTag = 103;
pub const T_LockRowsState: NodeTag = 102;
pub const T_SetOpState: NodeTag = 101;
pub const T_HashState: NodeTag = 100;
pub const T_GatherMergeState: NodeTag = 99;
pub const T_GatherState: NodeTag = 98;
pub const T_UniqueState: NodeTag = 97;
pub const T_WindowAggState: NodeTag = 96;
pub const T_AggState: NodeTag = 95;
pub const T_GroupState: NodeTag = 94;
pub const T_IncrementalSortState: NodeTag = 93;
pub const T_SortState: NodeTag = 92;
pub const T_MaterialState: NodeTag = 91;
pub const T_HashJoinState: NodeTag = 90;
pub const T_MergeJoinState: NodeTag = 89;
pub const T_NestLoopState: NodeTag = 88;
pub const T_JoinState: NodeTag = 87;
pub const T_CustomScanState: NodeTag = 86;
pub const T_ForeignScanState: NodeTag = 85;
pub const T_WorkTableScanState: NodeTag = 84;
pub const T_NamedTuplestoreScanState: NodeTag = 83;
pub const T_CteScanState: NodeTag = 82;
pub const T_ValuesScanState: NodeTag = 81;
pub const T_TableFuncScanState: NodeTag = 80;
pub const T_FunctionScanState: NodeTag = 79;
pub const T_SubqueryScanState: NodeTag = 78;
pub const T_TidRangeScanState: NodeTag = 77;
pub const T_TidScanState: NodeTag = 76;
pub const T_BitmapHeapScanState: NodeTag = 75;
pub const T_BitmapIndexScanState: NodeTag = 74;
pub const T_IndexOnlyScanState: NodeTag = 73;
pub const T_IndexScanState: NodeTag = 72;
pub const T_SampleScanState: NodeTag = 71;
pub const T_SeqScanState: NodeTag = 70;
pub const T_ScanState: NodeTag = 69;
pub const T_BitmapOrState: NodeTag = 68;
pub const T_BitmapAndState: NodeTag = 67;
pub const T_RecursiveUnionState: NodeTag = 66;
pub const T_MergeAppendState: NodeTag = 65;
pub const T_AppendState: NodeTag = 64;
pub const T_ModifyTableState: NodeTag = 63;
pub const T_ProjectSetState: NodeTag = 62;
pub const T_ResultState: NodeTag = 61;
pub const T_PlanState: NodeTag = 60;
pub const T_PlanInvalItem: NodeTag = 59;
pub const T_PartitionPruneStepCombine: NodeTag = 58;
pub const T_PartitionPruneStepOp: NodeTag = 57;
pub const T_PartitionedRelPruneInfo: NodeTag = 56;
pub const T_PartitionPruneInfo: NodeTag = 55;
pub const T_PlanRowMark: NodeTag = 54;
pub const T_NestLoopParam: NodeTag = 53;
pub const T_Limit: NodeTag = 52;
pub const T_LockRows: NodeTag = 51;
pub const T_SetOp: NodeTag = 50;
pub const T_Hash: NodeTag = 49;
pub const T_GatherMerge: NodeTag = 48;
pub const T_Gather: NodeTag = 47;
pub const T_Unique: NodeTag = 46;
pub const T_WindowAgg: NodeTag = 45;
pub const T_Agg: NodeTag = 44;
pub const T_Group: NodeTag = 43;
pub const T_IncrementalSort: NodeTag = 42;
pub const T_Sort: NodeTag = 41;
pub const T_Material: NodeTag = 40;
pub const T_HashJoin: NodeTag = 39;
pub const T_MergeJoin: NodeTag = 38;
pub const T_NestLoop: NodeTag = 37;
pub const T_Join: NodeTag = 36;
pub const T_CustomScan: NodeTag = 35;
pub const T_ForeignScan: NodeTag = 34;
pub const T_WorkTableScan: NodeTag = 33;
pub const T_NamedTuplestoreScan: NodeTag = 32;
pub const T_CteScan: NodeTag = 31;
pub const T_TableFuncScan: NodeTag = 30;
pub const T_ValuesScan: NodeTag = 29;
pub const T_FunctionScan: NodeTag = 28;
pub const T_SubqueryScan: NodeTag = 27;
pub const T_TidRangeScan: NodeTag = 26;
pub const T_TidScan: NodeTag = 25;
pub const T_BitmapHeapScan: NodeTag = 24;
pub const T_BitmapIndexScan: NodeTag = 23;
pub const T_IndexOnlyScan: NodeTag = 22;
pub const T_IndexScan: NodeTag = 21;
pub const T_SampleScan: NodeTag = 20;
pub const T_SeqScan: NodeTag = 19;
pub const T_Scan: NodeTag = 18;
pub const T_BitmapOr: NodeTag = 17;
pub const T_BitmapAnd: NodeTag = 16;
pub const T_RecursiveUnion: NodeTag = 15;
pub const T_MergeAppend: NodeTag = 14;
pub const T_Append: NodeTag = 13;
pub const T_ModifyTable: NodeTag = 12;
pub const T_ProjectSet: NodeTag = 11;
pub const T_Result: NodeTag = 10;
pub const T_Plan: NodeTag = 9;
pub const T_TupleTableSlot: NodeTag = 8;
pub const T_EState: NodeTag = 7;
pub const T_ResultRelInfo: NodeTag = 6;
pub const T_OnConflictSetState: NodeTag = 5;
pub const T_JunkFilter: NodeTag = 4;
pub const T_ProjectionInfo: NodeTag = 3;
pub const T_ExprContext: NodeTag = 2;
pub const T_IndexInfo: NodeTag = 1;
pub const T_Invalid: NodeTag = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub type_0: NodeTag,
}
// #[derive(Copy, Clone)]
// #[repr(C)]
// pub struct Bitmapset {
//     pub nwords: libc::c_int,
//     pub words: [bitmapword; 0],
// }
pub type bitmapword = u32;
pub type Selectivity = libc::c_double;
pub type Cost = libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ForBothState {
    pub l1: *const List,
    pub l2: *const List,
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
pub type ScanDirection = libc::c_int;
pub const ForwardScanDirection: ScanDirection = 1;
pub const NoMovementScanDirection: ScanDirection = 0;
pub const BackwardScanDirection: ScanDirection = -1;
pub type StrategyNumber = u16;
pub type fmNodePtr = *mut Node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr {
    pub type_0: NodeTag,
}
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
pub struct ScanKeyData {
    pub sk_flags: libc::c_int,
    pub sk_attno: AttrNumber,
    pub sk_strategy: StrategyNumber,
    pub sk_subtype: Oid,
    pub sk_collation: Oid,
    pub sk_func: FmgrInfo,
    pub sk_argument: Datum,
}
pub type ScanKey = *mut ScanKeyData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TBMIterateResult {
    pub blockno: BlockNumber,
    pub ntuples: libc::c_int,
    pub recheck: bool,
    pub offsets: [OffsetNumber; 0],
}
pub type LOCKMODE = libc::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexAmRoutine {
    pub type_0: NodeTag,
    pub amstrategies: u16,
    pub amsupport: u16,
    pub amoptsprocnum: u16,
    pub amcanorder: bool,
    pub amcanorderbyop: bool,
    pub amcanbackward: bool,
    pub amcanunique: bool,
    pub amcanmulticol: bool,
    pub amoptionalkey: bool,
    pub amsearcharray: bool,
    pub amsearchnulls: bool,
    pub amstorage: bool,
    pub amclusterable: bool,
    pub ampredlocks: bool,
    pub amcanparallel: bool,
    pub amcaninclude: bool,
    pub amusemaintenanceworkmem: bool,
    pub amparallelvacuumoptions: u8,
    pub amkeytype: Oid,
    pub ambuild: ambuild_function,
    pub ambuildempty: ambuildempty_function,
    pub aminsert: aminsert_function,
    pub ambulkdelete: ambulkdelete_function,
    pub amvacuumcleanup: amvacuumcleanup_function,
    pub amcanreturn: amcanreturn_function,
    pub amcostestimate: amcostestimate_function,
    pub amoptions: amoptions_function,
    pub amproperty: amproperty_function,
    pub ambuildphasename: ambuildphasename_function,
    pub amvalidate: amvalidate_function,
    pub amadjustmembers: amadjustmembers_function,
    pub ambeginscan: ambeginscan_function,
    pub amrescan: amrescan_function,
    pub amgettuple: amgettuple_function,
    pub amgetbitmap: amgetbitmap_function,
    pub amendscan: amendscan_function,
    pub ammarkpos: ammarkpos_function,
    pub amrestrpos: amrestrpos_function,
    pub amestimateparallelscan: amestimateparallelscan_function,
    pub aminitparallelscan: aminitparallelscan_function,
    pub amparallelrescan: amparallelrescan_function,
}
pub type amparallelrescan_function = Option<unsafe extern "C" fn(IndexScanDesc) -> ()>;
pub type IndexScanDesc = *mut IndexScanDescData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexScanDescData {
    pub heapRelation: Relation,
    pub indexRelation: Relation,
    pub xs_snapshot: *mut SnapshotData,
    pub numberOfKeys: libc::c_int,
    pub numberOfOrderBys: libc::c_int,
    pub keyData: *mut ScanKeyData,
    pub orderByData: *mut ScanKeyData,
    pub xs_want_itup: bool,
    pub xs_temp_snap: bool,
    pub kill_prior_tuple: bool,
    pub ignore_killed_tuples: bool,
    pub xactStartedInRecovery: bool,
    pub opaque: *mut libc::c_void,
    pub xs_itup: IndexTuple,
    pub xs_itupdesc: *mut TupleDescData,
    pub xs_hitup: HeapTuple,
    pub xs_hitupdesc: *mut TupleDescData,
    pub xs_heaptid: ItemPointerData,
    pub xs_heap_continue: bool,
    pub xs_heapfetch: *mut IndexFetchTableData,
    pub xs_recheck: bool,
    pub xs_orderbyvals: *mut Datum,
    pub xs_orderbynulls: *mut bool,
    pub xs_recheckorderby: bool,
    pub parallel_scan: *mut ParallelIndexScanDescData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParallelIndexScanDescData {
    pub ps_relid: Oid,
    pub ps_indexid: Oid,
    pub ps_offset: usize,
    pub ps_snapshot_data: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexFetchTableData {
    pub rel: Relation,
}
// pub type Relation = *mut RelationData;
pub type IndexTuple = *mut IndexTupleData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexTupleData {
    pub t_tid: ItemPointerData,
    pub t_info: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SnapshotData {
    pub snapshot_type: SnapshotType,
    pub xmin: TransactionId,
    pub xmax: TransactionId,
    pub xip: *mut TransactionId,
    pub xcnt: u32,
    pub subxip: *mut TransactionId,
    pub subxcnt: i32,
    pub suboverflowed: bool,
    pub takenDuringRecovery: bool,
    pub copied: bool,
    pub curcid: CommandId,
    pub speculativeToken: u32,
    pub vistest: *mut GlobalVisState,
    pub active_count: u32,
    pub regd_count: u32,
    pub ph_node: pairingheap_node,
    pub whenTaken: TimestampTz,
    pub lsn: XLogRecPtr,
    pub snapXactCompletionCount: uint64,
}
pub type TimestampTz = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pairingheap_node {
    pub first_child: *mut pairingheap_node,
    pub next_sibling: *mut pairingheap_node,
    pub prev_or_parent: *mut pairingheap_node,
}
pub type SnapshotType = libc::c_uint;
pub const SNAPSHOT_NON_VACUUMABLE: SnapshotType = 6;
pub const SNAPSHOT_HISTORIC_MVCC: SnapshotType = 5;
pub const SNAPSHOT_DIRTY: SnapshotType = 4;
pub const SNAPSHOT_TOAST: SnapshotType = 3;
pub const SNAPSHOT_ANY: SnapshotType = 2;
pub const SNAPSHOT_SELF: SnapshotType = 1;
pub const SNAPSHOT_MVCC: SnapshotType = 0;
pub type aminitparallelscan_function = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type amestimateparallelscan_function = Option<unsafe extern "C" fn() -> usize>;
pub type amrestrpos_function = Option<unsafe extern "C" fn(IndexScanDesc) -> ()>;
pub type ammarkpos_function = Option<unsafe extern "C" fn(IndexScanDesc) -> ()>;
pub type amendscan_function = Option<unsafe extern "C" fn(IndexScanDesc) -> ()>;
pub type amgetbitmap_function = Option<unsafe extern "C" fn(IndexScanDesc, *mut TIDBitmap) -> i64>;
pub type amgettuple_function = Option<unsafe extern "C" fn(IndexScanDesc, ScanDirection) -> bool>;
pub type amrescan_function =
    Option<unsafe extern "C" fn(IndexScanDesc, ScanKey, libc::c_int, ScanKey, libc::c_int) -> ()>;
pub type ambeginscan_function =
    Option<unsafe extern "C" fn(Relation, libc::c_int, libc::c_int) -> IndexScanDesc>;
pub type amadjustmembers_function =
    Option<unsafe extern "C" fn(Oid, Oid, *mut List, *mut List) -> ()>;
pub type amvalidate_function = Option<unsafe extern "C" fn(Oid) -> bool>;
pub type ambuildphasename_function = Option<unsafe extern "C" fn(i64) -> *mut libc::c_char>;
pub type amproperty_function = Option<
    unsafe extern "C" fn(
        Oid,
        libc::c_int,
        IndexAMProperty,
        *const libc::c_char,
        *mut bool,
        *mut bool,
    ) -> bool,
>;
pub type IndexAMProperty = libc::c_uint;
pub const AMPROP_CAN_INCLUDE: IndexAMProperty = 18;
pub const AMPROP_CAN_EXCLUDE: IndexAMProperty = 17;
pub const AMPROP_CAN_MULTI_COL: IndexAMProperty = 16;
pub const AMPROP_CAN_UNIQUE: IndexAMProperty = 15;
pub const AMPROP_CAN_ORDER: IndexAMProperty = 14;
pub const AMPROP_BACKWARD_SCAN: IndexAMProperty = 13;
pub const AMPROP_BITMAP_SCAN: IndexAMProperty = 12;
pub const AMPROP_INDEX_SCAN: IndexAMProperty = 11;
pub const AMPROP_CLUSTERABLE: IndexAMProperty = 10;
pub const AMPROP_SEARCH_NULLS: IndexAMProperty = 9;
pub const AMPROP_SEARCH_ARRAY: IndexAMProperty = 8;
pub const AMPROP_RETURNABLE: IndexAMProperty = 7;
pub const AMPROP_DISTANCE_ORDERABLE: IndexAMProperty = 6;
pub const AMPROP_ORDERABLE: IndexAMProperty = 5;
pub const AMPROP_NULLS_LAST: IndexAMProperty = 4;
pub const AMPROP_NULLS_FIRST: IndexAMProperty = 3;
pub const AMPROP_DESC: IndexAMProperty = 2;
pub const AMPROP_ASC: IndexAMProperty = 1;
pub const AMPROP_UNKNOWN: IndexAMProperty = 0;
pub type amoptions_function = Option<unsafe extern "C" fn(Datum, bool) -> *mut bytea>;
pub type amcostestimate_function = Option<
    unsafe extern "C" fn(
        *mut PlannerInfo,
        *mut IndexPath,
        libc::c_double,
        *mut Cost,
        *mut Cost,
        *mut Selectivity,
        *mut libc::c_double,
        *mut libc::c_double,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexPath {
    pub path: Path,
    pub indexinfo: *mut IndexOptInfo,
    pub indexclauses: *mut List,
    pub indexorderbys: *mut List,
    pub indexorderbycols: *mut List,
    pub indexscandir: ScanDirection,
    pub indextotalcost: Cost,
    pub indexselectivity: Selectivity,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexOptInfo {
    pub type_0: NodeTag,
    pub indexoid: Oid,
    pub reltablespace: Oid,
    pub rel: *mut RelOptInfo,
    pub pages: BlockNumber,
    pub tuples: libc::c_double,
    pub tree_height: libc::c_int,
    pub ncolumns: libc::c_int,
    pub nkeycolumns: libc::c_int,
    pub indexkeys: *mut libc::c_int,
    pub indexcollations: *mut Oid,
    pub opfamily: *mut Oid,
    pub opcintype: *mut Oid,
    pub sortopfamily: *mut Oid,
    pub reverse_sort: *mut bool,
    pub nulls_first: *mut bool,
    pub opclassoptions: *mut *mut bytea,
    pub canreturn: *mut bool,
    pub relam: Oid,
    pub indexprs: *mut List,
    pub indpred: *mut List,
    pub indextlist: *mut List,
    pub indrestrictinfo: *mut List,
    pub predOK: bool,
    pub unique: bool,
    pub immediate: bool,
    pub hypothetical: bool,
    pub amcanorderbyop: bool,
    pub amoptionalkey: bool,
    pub amsearcharray: bool,
    pub amsearchnulls: bool,
    pub amhasgettuple: bool,
    pub amhasgetbitmap: bool,
    pub amcanparallel: bool,
    pub amcanmarkpos: bool,
    pub amcostestimate: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RelOptInfo {
    pub type_0: NodeTag,
    pub reloptkind: RelOptKind,
    pub relids: Relids,
    pub rows: libc::c_double,
    pub consider_startup: bool,
    pub consider_param_startup: bool,
    pub consider_parallel: bool,
    pub reltarget: *mut PathTarget,
    pub pathlist: *mut List,
    pub ppilist: *mut List,
    pub partial_pathlist: *mut List,
    pub cheapest_startup_path: *mut Path,
    pub cheapest_total_path: *mut Path,
    pub cheapest_unique_path: *mut Path,
    pub cheapest_parameterized_paths: *mut List,
    pub direct_lateral_relids: Relids,
    pub lateral_relids: Relids,
    pub relid: Index,
    pub reltablespace: Oid,
    pub rtekind: RTEKind,
    pub min_attr: AttrNumber,
    pub max_attr: AttrNumber,
    pub attr_needed: *mut Relids,
    pub attr_widths: *mut i32,
    pub lateral_vars: *mut List,
    pub lateral_referencers: Relids,
    pub indexlist: *mut List,
    pub statlist: *mut List,
    pub pages: BlockNumber,
    pub tuples: libc::c_double,
    pub allvisfrac: libc::c_double,
    pub eclass_indexes: *mut Bitmapset,
    pub subroot: *mut PlannerInfo,
    pub subplan_params: *mut List,
    pub rel_parallel_workers: libc::c_int,
    pub amflags: u32,
    pub serverid: Oid,
    pub userid: Oid,
    pub useridiscurrent: bool,
    pub fdwroutine: *mut FdwRoutine,
    pub fdw_private: *mut libc::c_void,
    pub unique_for_rels: *mut List,
    pub non_unique_for_rels: *mut List,
    pub baserestrictinfo: *mut List,
    pub baserestrictcost: QualCost,
    pub baserestrict_min_security: Index,
    pub joininfo: *mut List,
    pub has_eclass_joins: bool,
    pub consider_partitionwise_join: bool,
    pub top_parent_relids: Relids,
    pub part_scheme: PartitionScheme,
    pub nparts: libc::c_int,
    pub boundinfo: *mut PartitionBoundInfoData,
    pub partbounds_merged: bool,
    pub partition_qual: *mut List,
    pub part_rels: *mut *mut RelOptInfo,
    pub all_partrels: Relids,
    pub partexprs: *mut *mut List,
    pub nullable_partexprs: *mut *mut List,
}
pub type Relids = *mut Bitmapset;
pub type PartitionScheme = *mut PartitionSchemeData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionSchemeData {
    pub strategy: libc::c_char,
    pub partnatts: i16,
    pub partopfamily: *mut Oid,
    pub partopcintype: *mut Oid,
    pub partcollation: *mut Oid,
    pub parttyplen: *mut i16,
    pub parttypbyval: *mut bool,
    pub partsupfunc: *mut FmgrInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QualCost {
    pub startup: Cost,
    pub per_tuple: Cost,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlannerInfo {
    pub type_0: NodeTag,
    pub parse: *mut Query,
    pub glob: *mut PlannerGlobal,
    pub query_level: Index,
    pub parent_root: *mut PlannerInfo,
    pub plan_params: *mut List,
    pub outer_params: *mut Bitmapset,
    pub simple_rel_array: *mut *mut RelOptInfo,
    pub simple_rel_array_size: libc::c_int,
    pub simple_rte_array: *mut *mut RangeTblEntry,
    pub append_rel_array: *mut *mut AppendRelInfo,
    pub all_baserels: Relids,
    pub nullable_baserels: Relids,
    pub join_rel_list: *mut List,
    pub join_rel_hash: *mut HTAB,
    pub join_rel_level: *mut *mut List,
    pub join_cur_level: libc::c_int,
    pub init_plans: *mut List,
    pub cte_plan_ids: *mut List,
    pub multiexpr_params: *mut List,
    pub eq_classes: *mut List,
    pub ec_merging_done: bool,
    pub canon_pathkeys: *mut List,
    pub left_join_clauses: *mut List,
    pub right_join_clauses: *mut List,
    pub full_join_clauses: *mut List,
    pub join_info_list: *mut List,
    pub append_rel_list: *mut List,
    pub rowMarks: *mut List,
    pub placeholder_list: *mut List,
    pub fkey_list: *mut List,
    pub query_pathkeys: *mut List,
    pub group_pathkeys: *mut List,
    pub window_pathkeys: *mut List,
    pub distinct_pathkeys: *mut List,
    pub sort_pathkeys: *mut List,
    pub part_schemes: *mut List,
    pub initial_rels: *mut List,
    pub upper_rels: [*mut List; 7],
    pub upper_targets: [*mut PathTarget; 7],
    pub processed_tlist: *mut List,
    pub grouping_map: *mut AttrNumber,
    pub minmax_aggs: *mut List,
    pub planner_cxt: MemoryContext,
    pub total_table_pages: libc::c_double,
    pub tuple_fraction: libc::c_double,
    pub limit_tuples: libc::c_double,
    pub qual_security_level: Index,
    pub inhTargetKind: InheritanceKind,
    pub hasJoinRTEs: bool,
    pub hasLateralRTEs: bool,
    pub hasHavingQual: bool,
    pub hasPseudoConstantQuals: bool,
    pub hasAlternativeSubPlans: bool,
    pub hasRecursion: bool,
    pub agginfos: *mut List,
    pub aggtransinfos: *mut List,
    pub numOrderedAggs: libc::c_int,
    pub hasNonPartialAggs: bool,
    pub hasNonSerialAggs: bool,
    pub wt_param_id: libc::c_int,
    pub non_recursive_path: *mut Path,
    pub curOuterRels: Relids,
    pub curOuterParams: *mut List,
    pub join_search_private: *mut libc::c_void,
    pub partColsUpdated: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Path {
    pub type_0: NodeTag,
    pub pathtype: NodeTag,
    pub parent: *mut RelOptInfo,
    pub pathtarget: *mut PathTarget,
    pub param_info: *mut ParamPathInfo,
    pub parallel_aware: bool,
    pub parallel_safe: bool,
    pub parallel_workers: libc::c_int,
    pub rows: libc::c_double,
    pub startup_cost: Cost,
    pub total_cost: Cost,
    pub pathkeys: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamPathInfo {
    pub type_0: NodeTag,
    pub ppi_req_outer: Relids,
    pub ppi_rows: libc::c_double,
    pub ppi_clauses: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PathTarget {
    pub type_0: NodeTag,
    pub exprs: *mut List,
    pub sortgrouprefs: *mut Index,
    pub cost: QualCost,
    pub width: libc::c_int,
}
pub type InheritanceKind = libc::c_uint;
pub const INHKIND_PARTITIONED: InheritanceKind = 2;
pub const INHKIND_INHERITED: InheritanceKind = 1;
pub const INHKIND_NONE: InheritanceKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AppendRelInfo {
    pub type_0: NodeTag,
    pub parent_relid: Index,
    pub child_relid: Index,
    pub parent_reltype: Oid,
    pub child_reltype: Oid,
    pub translated_vars: *mut List,
    pub num_child_cols: libc::c_int,
    pub parent_colnos: *mut AttrNumber,
    pub parent_reloid: Oid,
}
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
pub type AclMode = u32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FromExpr {
    pub type_0: NodeTag,
    pub fromlist: *mut List,
    pub quals: *mut Node,
}
pub type QuerySource = libc::c_uint;
pub const QSRC_NON_INSTEAD_RULE: QuerySource = 4;
pub const QSRC_QUAL_INSTEAD_RULE: QuerySource = 3;
pub const QSRC_INSTEAD_RULE: QuerySource = 2;
pub const QSRC_PARSER: QuerySource = 1;
pub const QSRC_ORIGINAL: QuerySource = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableSampleClause {
    pub type_0: NodeTag,
    pub tsmhandler: Oid,
    pub args: *mut List,
    pub repeatable: *mut Expr,
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
pub struct PlannerGlobal {
    pub type_0: NodeTag,
    pub boundParams: ParamListInfo,
    pub subplans: *mut List,
    pub subroots: *mut List,
    pub rewindPlanIDs: *mut Bitmapset,
    pub finalrtable: *mut List,
    pub finalrowmarks: *mut List,
    pub resultRelations: *mut List,
    pub appendRelations: *mut List,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub paramExecTypes: *mut List,
    pub lastPHId: Index,
    pub lastRowMarkId: Index,
    pub lastPlanNodeId: libc::c_int,
    pub transientPlan: bool,
    pub dependsOnRole: bool,
    pub parallelModeOK: bool,
    pub parallelModeNeeded: bool,
    pub maxParallelHazard: libc::c_char,
    pub partition_directory: PartitionDirectory,
}
pub type PartitionDirectory = *mut PartitionDirectoryData;
pub type ParamListInfo = *mut ParamListInfoData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamListInfoData {
    pub paramFetch: ParamFetchHook,
    pub paramFetchArg: *mut libc::c_void,
    pub paramCompile: ParamCompileHook,
    pub paramCompileArg: *mut libc::c_void,
    pub parserSetup: ParserSetupHook,
    pub parserSetupArg: *mut libc::c_void,
    pub paramValuesStr: *mut libc::c_char,
    pub numParams: libc::c_int,
    pub params: [ParamExternData; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamExternData {
    pub value: Datum,
    pub isnull: bool,
    pub pflags: u16,
    pub ptype: Oid,
}
pub type ParserSetupHook = Option<unsafe extern "C" fn(*mut ParseState, *mut libc::c_void) -> ()>;

pub type CoerceParamHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut Param, Oid, i32, libc::c_int) -> *mut Node>;
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
pub type ParamKind = libc::c_uint;
pub const PARAM_MULTIEXPR: ParamKind = 3;
pub const PARAM_SUBLINK: ParamKind = 2;
pub const PARAM_EXEC: ParamKind = 1;
pub const PARAM_EXTERN: ParamKind = 0;
pub type ParseParamRefHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut ParamRef) -> *mut Node>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamRef {
    pub type_0: NodeTag,
    pub number: libc::c_int,
    pub location: libc::c_int,
}
pub type PostParseColumnRefHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut ColumnRef, *mut Node) -> *mut Node>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColumnRef {
    pub type_0: NodeTag,
    pub fields: *mut List,
    pub location: libc::c_int,
}
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
pub struct CTESearchClause {
    pub type_0: NodeTag,
    pub search_col_list: *mut List,
    pub search_breadth_first: bool,
    pub search_seq_column: *mut libc::c_char,
    pub location: libc::c_int,
}
pub type CTEMaterialize = libc::c_uint;
pub const CTEMaterializeNever: CTEMaterialize = 2;
pub const CTEMaterializeAlways: CTEMaterialize = 1;
pub const CTEMaterializeDefault: CTEMaterialize = 0;
pub type ParamCompileHook = Option<
    unsafe extern "C" fn(ParamListInfo, *mut Param, *mut ExprState, *mut Datum, *mut bool) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprState {
    pub tag: NodeTag,
    pub flags: u8,
    pub resnull: bool,
    pub resvalue: Datum,
    pub resultslot: *mut TupleTableSlot,
    pub steps: *mut ExprEvalStep,
    pub evalfunc: ExprStateEvalFunc,
    pub expr: *mut Expr,
    pub evalfunc_private: *mut libc::c_void,
    pub steps_len: libc::c_int,
    pub steps_alloc: libc::c_int,
    pub parent: *mut PlanState,
    pub ext_params: ParamListInfo,
    pub innermost_caseval: *mut Datum,
    pub innermost_casenull: *mut bool,
    pub innermost_domainval: *mut Datum,
    pub innermost_domainnull: *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlanState {
    pub type_0: NodeTag,
    pub plan: *mut Plan,
    pub state: *mut EState,
    pub ExecProcNode: ExecProcNodeMtd,
    pub ExecProcNodeReal: ExecProcNodeMtd,
    pub instrument: *mut Instrumentation,
    pub worker_instrument: *mut WorkerInstrumentation,
    pub worker_jit_instrument: *mut SharedJitInstrumentation,
    pub qual: *mut ExprState,
    pub lefttree: *mut PlanState,
    pub righttree: *mut PlanState,
    pub initPlan: *mut List,
    pub subPlan: *mut List,
    pub chgParam: *mut Bitmapset,
    pub ps_ResultTupleDesc: TupleDesc,
    pub ps_ResultTupleSlot: *mut TupleTableSlot,
    pub ps_ExprContext: *mut ExprContext,
    pub ps_ProjInfo: *mut ProjectionInfo,
    pub scandesc: TupleDesc,
    pub scanops: *const TupleTableSlotOps,
    pub outerops: *const TupleTableSlotOps,
    pub innerops: *const TupleTableSlotOps,
    pub resultops: *const TupleTableSlotOps,
    pub scanopsfixed: bool,
    pub outeropsfixed: bool,
    pub inneropsfixed: bool,
    pub resultopsfixed: bool,
    pub scanopsset: bool,
    pub outeropsset: bool,
    pub inneropsset: bool,
    pub resultopsset: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleTableSlotOps {
    pub base_slot_size: isize,
    pub init: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub release: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub clear: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub getsomeattrs: Option<unsafe extern "C" fn(*mut TupleTableSlot, libc::c_int) -> ()>,
    pub getsysattr:
        Option<unsafe extern "C" fn(*mut TupleTableSlot, libc::c_int, *mut bool) -> Datum>,
    pub materialize: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> ()>,
    pub copyslot: Option<unsafe extern "C" fn(*mut TupleTableSlot, *mut TupleTableSlot) -> ()>,
    pub get_heap_tuple: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> HeapTuple>,
    pub get_minimal_tuple: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> MinimalTuple>,
    pub copy_heap_tuple: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> HeapTuple>,
    pub copy_minimal_tuple: Option<unsafe extern "C" fn(*mut TupleTableSlot) -> MinimalTuple>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleTableSlot {
    pub type_0: NodeTag,
    pub tts_flags: u16,
    pub tts_nvalid: AttrNumber,
    pub tts_ops: *const TupleTableSlotOps,
    pub tts_tupleDescriptor: TupleDesc,
    pub tts_values: *mut Datum,
    pub tts_isnull: *mut bool,
    pub tts_mcxt: MemoryContext,
    pub tts_tid: ItemPointerData,
    pub tts_tableOid: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProjectionInfo {
    pub type_0: NodeTag,
    pub pi_state: ExprState,
    pub pi_exprContext: *mut ExprContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprContext {
    pub type_0: NodeTag,
    pub ecxt_scantuple: *mut TupleTableSlot,
    pub ecxt_innertuple: *mut TupleTableSlot,
    pub ecxt_outertuple: *mut TupleTableSlot,
    pub ecxt_per_query_memory: MemoryContext,
    pub ecxt_per_tuple_memory: MemoryContext,
    pub ecxt_param_exec_vals: *mut ParamExecData,
    pub ecxt_param_list_info: ParamListInfo,
    pub ecxt_aggvalues: *mut Datum,
    pub ecxt_aggnulls: *mut bool,
    pub caseValue_datum: Datum,
    pub caseValue_isNull: bool,
    pub domainValue_datum: Datum,
    pub domainValue_isNull: bool,
    pub ecxt_estate: *mut EState,
    pub ecxt_callbacks: *mut ExprContext_CB,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprContext_CB {
    pub next: *mut ExprContext_CB,
    pub function: ExprContextCallbackFunction,
    pub arg: Datum,
}
pub type ExprContextCallbackFunction = Option<unsafe extern "C" fn(Datum) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EState {
    pub type_0: NodeTag,
    pub es_direction: ScanDirection,
    pub es_snapshot: Snapshot,
    pub es_crosscheck_snapshot: Snapshot,
    pub es_range_table: *mut List,
    pub es_range_table_size: Index,
    pub es_relations: *mut Relation,
    pub es_rowmarks: *mut *mut ExecRowMark,
    pub es_plannedstmt: *mut PlannedStmt,
    pub es_sourceText: *const libc::c_char,
    pub es_junkFilter: *mut JunkFilter,
    pub es_output_cid: CommandId,
    pub es_result_relations: *mut *mut ResultRelInfo,
    pub es_opened_result_relations: *mut List,
    pub es_partition_directory: PartitionDirectory,
    pub es_tuple_routing_result_relations: *mut List,
    pub es_trig_target_relations: *mut List,
    pub es_param_list_info: ParamListInfo,
    pub es_param_exec_vals: *mut ParamExecData,
    pub es_queryEnv: *mut QueryEnvironment,
    pub es_query_cxt: MemoryContext,
    pub es_tupleTable: *mut List,
    pub es_processed: uint64,
    pub es_top_eflags: libc::c_int,
    pub es_instrument: libc::c_int,
    pub es_finished: bool,
    pub es_exprcontexts: *mut List,
    pub es_subplanstates: *mut List,
    pub es_auxmodifytables: *mut List,
    pub es_per_tuple_exprcontext: *mut ExprContext,
    pub es_epq_active: *mut EPQState,
    pub es_use_parallel_mode: bool,
    pub es_query_dsa: *mut dsa_area,
    pub es_jit_flags: libc::c_int,
    pub es_jit: *mut JitContext,
    pub es_jit_worker_instr: *mut JitInstrumentation,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EPQState {
    pub parentestate: *mut EState,
    pub epqParam: libc::c_int,
    pub tuple_table: *mut List,
    pub relsubs_slot: *mut *mut TupleTableSlot,
    pub plan: *mut Plan,
    pub arowMarks: *mut List,
    pub origslot: *mut TupleTableSlot,
    pub recheckestate: *mut EState,
    pub relsubs_rowmark: *mut *mut ExecAuxRowMark,
    pub relsubs_done: *mut bool,
    pub recheckplanstate: *mut PlanState,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExecAuxRowMark {
    pub rowmark: *mut ExecRowMark,
    pub ctidAttNo: AttrNumber,
    pub toidAttNo: AttrNumber,
    pub wholeAttNo: AttrNumber,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExecRowMark {
    pub relation: Relation,
    pub relid: Oid,
    pub rti: Index,
    pub prti: Index,
    pub rowmarkId: Index,
    pub markType: RowMarkType,
    pub strength: LockClauseStrength,
    pub waitPolicy: LockWaitPolicy,
    pub ermActive: bool,
    pub curCtid: ItemPointerData,
    pub ermExtra: *mut libc::c_void,
}
pub type LockWaitPolicy = libc::c_uint;
pub const LockWaitError: LockWaitPolicy = 2;
pub const LockWaitSkip: LockWaitPolicy = 1;
pub const LockWaitBlock: LockWaitPolicy = 0;
pub type LockClauseStrength = libc::c_uint;
pub const LCS_FORUPDATE: LockClauseStrength = 4;
pub const LCS_FORNOKEYUPDATE: LockClauseStrength = 3;
pub const LCS_FORSHARE: LockClauseStrength = 2;
pub const LCS_FORKEYSHARE: LockClauseStrength = 1;
pub const LCS_NONE: LockClauseStrength = 0;
pub type RowMarkType = libc::c_uint;
pub const ROW_MARK_COPY: RowMarkType = 5;
pub const ROW_MARK_REFERENCE: RowMarkType = 4;
pub const ROW_MARK_KEYSHARE: RowMarkType = 3;
pub const ROW_MARK_SHARE: RowMarkType = 2;
pub const ROW_MARK_NOKEYEXCLUSIVE: RowMarkType = 1;
pub const ROW_MARK_EXCLUSIVE: RowMarkType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Plan {
    pub type_0: NodeTag,
    pub startup_cost: Cost,
    pub total_cost: Cost,
    pub plan_rows: libc::c_double,
    pub plan_width: libc::c_int,
    pub parallel_aware: bool,
    pub parallel_safe: bool,
    pub plan_node_id: libc::c_int,
    pub targetlist: *mut List,
    pub qual: *mut List,
    pub lefttree: *mut Plan,
    pub righttree: *mut Plan,
    pub initPlan: *mut List,
    pub extParam: *mut Bitmapset,
    pub allParam: *mut Bitmapset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParamExecData {
    pub execPlan: *mut libc::c_void,
    pub value: Datum,
    pub isnull: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ResultRelInfo {
    pub type_0: NodeTag,
    pub ri_RangeTableIndex: Index,
    pub ri_RelationDesc: Relation,
    pub ri_NumIndices: libc::c_int,
    pub ri_IndexRelationDescs: RelationPtr,
    pub ri_IndexRelationInfo: *mut *mut IndexInfo,
    pub ri_TrigDesc: *mut TriggerDesc,
    pub ri_TrigFunctions: *mut FmgrInfo,
    pub ri_TrigWhenExprs: *mut *mut ExprState,
    pub ri_TrigInstrument: *mut Instrumentation,
    pub ri_ReturningSlot: *mut TupleTableSlot,
    pub ri_TrigOldSlot: *mut TupleTableSlot,
    pub ri_TrigNewSlot: *mut TupleTableSlot,
    pub ri_FdwRoutine: *mut FdwRoutine,
    pub ri_FdwState: *mut libc::c_void,
    pub ri_usesFdwDirectModify: bool,
    pub ri_NumSlots: libc::c_int,
    pub ri_Batchusize: libc::c_int,
    pub ri_Slots: *mut *mut TupleTableSlot,
    pub ri_PlanSlots: *mut *mut TupleTableSlot,
    pub ri_WithCheckOptions: *mut List,
    pub ri_WithCheckOptionExprs: *mut List,
    pub ri_ConstraintExprs: *mut *mut ExprState,
    pub ri_GeneratedExprs: *mut *mut ExprState,
    pub ri_NumGeneratedNeeded: libc::c_int,
    pub ri_junkFilter: *mut JunkFilter,
    pub ri_returningList: *mut List,
    pub ri_projectReturning: *mut ProjectionInfo,
    pub ri_onConflictArbiterIndexes: *mut List,
    pub ri_onConflict: *mut OnConflictSetState,
    pub ri_PartitionCheckExpr: *mut ExprState,
    pub ri_RootResultRelInfo: *mut ResultRelInfo,
    pub ri_RootToPartitionMap: *mut TupleConversionMap,
    pub ri_PartitionTupleSlot: *mut TupleTableSlot,
    pub ri_ChildToRootMap: *mut TupleConversionMap,
    pub ri_CopyMultiInsertBuffer: *mut CopyMultiInsertBuffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleConversionMap {
    pub indesc: TupleDesc,
    pub outdesc: TupleDesc,
    pub attrMap: *mut AttrMap,
    pub invalues: *mut Datum,
    pub inisnull: *mut bool,
    pub outvalues: *mut Datum,
    pub outisnull: *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttrMap {
    pub attnums: *mut AttrNumber,
    pub maplen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnConflictSetState {
    pub type_0: NodeTag,
    pub oc_Existing: *mut TupleTableSlot,
    pub oc_ProjSlot: *mut TupleTableSlot,
    pub oc_ProjInfo: *mut ProjectionInfo,
    pub oc_WhereClause: *mut ExprState,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JunkFilter {
    pub type_0: NodeTag,
    pub jf_targetList: *mut List,
    pub jf_cleanTupType: TupleDesc,
    pub jf_cleanMap: *mut AttrNumber,
    pub jf_resultSlot: *mut TupleTableSlot,
    pub jf_junkAttNo: AttrNumber,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Instrumentation {
    pub need_timer: bool,
    pub need_bufusage: bool,
    pub need_walusage: bool,
    pub running: bool,
    pub starttime: instr_time,
    pub counter: instr_time,
    pub firsttuple: libc::c_double,
    pub tuplecount: libc::c_double,
    pub bufusage_start: BufferUsage,
    pub walusage_start: WalUsage,
    pub startup: libc::c_double,
    pub total: libc::c_double,
    pub ntuples: libc::c_double,
    pub ntuples2: libc::c_double,
    pub nloops: libc::c_double,
    pub nfiltered1: libc::c_double,
    pub nfiltered2: libc::c_double,
    pub bufusage: BufferUsage,
    pub walusage: WalUsage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WalUsage {
    pub wal_records: libc::c_long,
    pub wal_fpi: libc::c_long,
    pub wal_bytes: uint64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferUsage {
    pub shared_blks_hit: libc::c_long,
    pub shared_blks_read: libc::c_long,
    pub shared_blks_dirtied: libc::c_long,
    pub shared_blks_written: libc::c_long,
    pub local_blks_hit: libc::c_long,
    pub local_blks_read: libc::c_long,
    pub local_blks_dirtied: libc::c_long,
    pub local_blks_written: libc::c_long,
    pub temp_blks_read: libc::c_long,
    pub temp_blks_written: libc::c_long,
    pub blk_read_time: instr_time,
    pub blk_write_time: instr_time,
}
pub type instr_time = timeval;
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
pub struct IndexInfo {
    pub type_0: NodeTag,
    pub ii_NumIndexAttrs: libc::c_int,
    pub ii_NumIndexKeyAttrs: libc::c_int,
    pub ii_IndexAttrNumbers: [AttrNumber; 32],
    pub ii_Expressions: *mut List,
    pub ii_ExpressionsState: *mut List,
    pub ii_Predicate: *mut List,
    pub ii_PredicateState: *mut ExprState,
    pub ii_ExclusionOps: *mut Oid,
    pub ii_ExclusionProcs: *mut Oid,
    pub ii_ExclusionStrats: *mut u16,
    pub ii_UniqueOps: *mut Oid,
    pub ii_UniqueProcs: *mut Oid,
    pub ii_UniqueStrats: *mut u16,
    pub ii_OpclassOptions: *mut Datum,
    pub ii_Unique: bool,
    pub ii_ReadyForInserts: bool,
    pub ii_Concurrent: bool,
    pub ii_BrokenHotChain: bool,
    pub ii_ParallelWorkers: libc::c_int,
    pub ii_Am: Oid,
    pub ii_AmCache: *mut libc::c_void,
    pub ii_Context: MemoryContext,
}
pub type RelationPtr = *mut Relation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlannedStmt {
    pub type_0: NodeTag,
    pub commandType: CmdType,
    pub queryId: uint64,
    pub hasReturning: bool,
    pub hasModifyingCTE: bool,
    pub canSetTag: bool,
    pub transientPlan: bool,
    pub dependsOnRole: bool,
    pub parallelModeNeeded: bool,
    pub jitFlags: libc::c_int,
    pub planTree: *mut Plan,
    pub rtable: *mut List,
    pub resultRelations: *mut List,
    pub appendRelations: *mut List,
    pub subplans: *mut List,
    pub rewindPlanIDs: *mut Bitmapset,
    pub rowMarks: *mut List,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub paramExecTypes: *mut List,
    pub utilityStmt: *mut Node,
    pub stmt_location: libc::c_int,
    pub stmt_len: libc::c_int,
}
pub type Snapshot = *mut SnapshotData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WorkerInstrumentation {
    pub num_workers: libc::c_int,
    pub instrument: [Instrumentation; 0],
}
pub type ExecProcNodeMtd = Option<unsafe extern "C" fn(*mut PlanState) -> *mut TupleTableSlot>;
pub type ExprStateEvalFunc =
    Option<unsafe extern "C" fn(*mut ExprState, *mut ExprContext, *mut bool) -> Datum>;
pub type ParamFetchHook = Option<
    unsafe extern "C" fn(
        ParamListInfo,
        libc::c_int,
        bool,
        *mut ParamExternData,
    ) -> *mut ParamExternData,
>;
pub type RelOptKind = libc::c_uint;
pub const RELOPT_DEADREL: RelOptKind = 6;
pub const RELOPT_OTHER_UPPER_REL: RelOptKind = 5;
pub const RELOPT_UPPER_REL: RelOptKind = 4;
pub const RELOPT_OTHER_JOINREL: RelOptKind = 3;
pub const RELOPT_OTHER_MEMBER_REL: RelOptKind = 2;
pub const RELOPT_JOINREL: RelOptKind = 1;
pub const RELOPT_BASEREL: RelOptKind = 0;
pub type amcanreturn_function = Option<unsafe extern "C" fn(Relation, libc::c_int) -> bool>;
pub type amvacuumcleanup_function = Option<
    unsafe extern "C" fn(
        *mut IndexVacuumInfo,
        *mut IndexBulkDeleteResult,
    ) -> *mut IndexBulkDeleteResult,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexBulkDeleteResult {
    pub num_pages: BlockNumber,
    pub estimated_count: bool,
    pub num_index_tuples: libc::c_double,
    pub tuples_removed: libc::c_double,
    pub pages_newly_deleted: BlockNumber,
    pub pages_deleted: BlockNumber,
    pub pages_free: BlockNumber,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexVacuumInfo {
    pub index: Relation,
    pub analyze_only: bool,
    pub report_progress: bool,
    pub estimated_count: bool,
    pub message_level: libc::c_int,
    pub num_heap_tuples: libc::c_double,
    pub strategy: BufferAccessStrategy,
}
pub type BufferAccessStrategy = *mut BufferAccessStrategyData;
pub type ambulkdelete_function = Option<
    unsafe extern "C" fn(
        *mut IndexVacuumInfo,
        *mut IndexBulkDeleteResult,
        IndexBulkDeleteCallback,
        *mut libc::c_void,
    ) -> *mut IndexBulkDeleteResult,
>;
pub type IndexBulkDeleteCallback =
    Option<unsafe extern "C" fn(ItemPointer, *mut libc::c_void) -> bool>;
pub type aminsert_function = Option<
    unsafe extern "C" fn(
        Relation,
        *mut Datum,
        *mut bool,
        ItemPointer,
        Relation,
        IndexUniqueCheck,
        bool,
        *mut IndexInfo,
    ) -> bool,
>;
pub type IndexUniqueCheck = libc::c_uint;
pub const UNIQUE_CHECK_EXISTING: IndexUniqueCheck = 3;
pub const UNIQUE_CHECK_PARTIAL: IndexUniqueCheck = 2;
pub const UNIQUE_CHECK_YES: IndexUniqueCheck = 1;
pub const UNIQUE_CHECK_NO: IndexUniqueCheck = 0;
pub type ambuildempty_function = Option<unsafe extern "C" fn(Relation) -> ()>;
pub type ambuild_function =
    Option<unsafe extern "C" fn(Relation, Relation, *mut IndexInfo) -> *mut IndexBuildResult>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexBuildResult {
    pub heap_tuples: libc::c_double,
    pub index_tuples: libc::c_double,
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
pub struct TableAmRoutine {
    pub type_0: NodeTag,
    pub slot_callbacks: Option<unsafe extern "C" fn(Relation) -> *const TupleTableSlotOps>,
    pub scan_begin: Option<
        unsafe extern "C" fn(
            Relation,
            Snapshot,
            libc::c_int,
            *mut ScanKeyData,
            ParallelTableScanDesc,
            u32,
        ) -> TableScanDesc,
    >,
    pub scan_end: Option<unsafe extern "C" fn(TableScanDesc) -> ()>,
    pub scan_rescan:
        Option<unsafe extern "C" fn(TableScanDesc, *mut ScanKeyData, bool, bool, bool, bool) -> ()>,
    pub scan_getnextslot:
        Option<unsafe extern "C" fn(TableScanDesc, ScanDirection, *mut TupleTableSlot) -> bool>,
    pub scan_set_tidrange:
        Option<unsafe extern "C" fn(TableScanDesc, ItemPointer, ItemPointer) -> ()>,
    pub scan_getnextslot_tidrange:
        Option<unsafe extern "C" fn(TableScanDesc, ScanDirection, *mut TupleTableSlot) -> bool>,
    pub parallelscan_estimate: Option<unsafe extern "C" fn(Relation) -> usize>,
    pub parallelscan_initialize:
        Option<unsafe extern "C" fn(Relation, ParallelTableScanDesc) -> usize>,
    pub parallelscan_reinitialize:
        Option<unsafe extern "C" fn(Relation, ParallelTableScanDesc) -> ()>,
    pub index_fetch_begin: Option<unsafe extern "C" fn(Relation) -> *mut IndexFetchTableData>,
    pub index_fetch_reset: Option<unsafe extern "C" fn(*mut IndexFetchTableData) -> ()>,
    pub index_fetch_end: Option<unsafe extern "C" fn(*mut IndexFetchTableData) -> ()>,
    pub index_fetch_tuple: Option<
        unsafe extern "C" fn(
            *mut IndexFetchTableData,
            ItemPointer,
            Snapshot,
            *mut TupleTableSlot,
            *mut bool,
            *mut bool,
        ) -> bool,
    >,
    pub tuple_fetch_row_version:
        Option<unsafe extern "C" fn(Relation, ItemPointer, Snapshot, *mut TupleTableSlot) -> bool>,
    pub tuple_tid_valid: Option<unsafe extern "C" fn(TableScanDesc, ItemPointer) -> bool>,
    pub tuple_get_latest_tid: Option<unsafe extern "C" fn(TableScanDesc, ItemPointer) -> ()>,
    pub tuple_satisfies_snapshot:
        Option<unsafe extern "C" fn(Relation, *mut TupleTableSlot, Snapshot) -> bool>,
    pub index_delete_tuples:
        Option<unsafe extern "C" fn(Relation, *mut TM_IndexDeleteOp) -> TransactionId>,
    pub tuple_insert: Option<
        unsafe extern "C" fn(
            Relation,
            *mut TupleTableSlot,
            CommandId,
            libc::c_int,
            *mut BulkInsertStateData,
        ) -> (),
    >,
    pub tuple_insert_speculative: Option<
        unsafe extern "C" fn(
            Relation,
            *mut TupleTableSlot,
            CommandId,
            libc::c_int,
            *mut BulkInsertStateData,
            u32,
        ) -> (),
    >,
    pub tuple_complete_speculative:
        Option<unsafe extern "C" fn(Relation, *mut TupleTableSlot, u32, bool) -> ()>,
    pub multi_insert: Option<
        unsafe extern "C" fn(
            Relation,
            *mut *mut TupleTableSlot,
            libc::c_int,
            CommandId,
            libc::c_int,
            *mut BulkInsertStateData,
        ) -> (),
    >,
    pub tuple_delete: Option<
        unsafe extern "C" fn(
            Relation,
            ItemPointer,
            CommandId,
            Snapshot,
            Snapshot,
            bool,
            *mut TM_FailureData,
            bool,
        ) -> TM_Result,
    >,
    pub tuple_update: Option<
        unsafe extern "C" fn(
            Relation,
            ItemPointer,
            *mut TupleTableSlot,
            CommandId,
            Snapshot,
            Snapshot,
            bool,
            *mut TM_FailureData,
            *mut LockTupleMode,
            *mut bool,
        ) -> TM_Result,
    >,
    pub tuple_lock: Option<
        unsafe extern "C" fn(
            Relation,
            ItemPointer,
            Snapshot,
            *mut TupleTableSlot,
            CommandId,
            LockTupleMode,
            LockWaitPolicy,
            u8,
            *mut TM_FailureData,
        ) -> TM_Result,
    >,
    pub finish_bulk_insert: Option<unsafe extern "C" fn(Relation, libc::c_int) -> ()>,
    pub relation_set_new_filenode: Option<
        unsafe extern "C" fn(
            Relation,
            *const RelFileNode,
            libc::c_char,
            *mut TransactionId,
            *mut MultiXactId,
        ) -> (),
    >,
    pub relation_nontransactional_truncate: Option<unsafe extern "C" fn(Relation) -> ()>,
    pub relation_copy_data: Option<unsafe extern "C" fn(Relation, *const RelFileNode) -> ()>,
    pub relation_copy_for_cluster: Option<
        unsafe extern "C" fn(
            Relation,
            Relation,
            Relation,
            bool,
            TransactionId,
            *mut TransactionId,
            *mut MultiXactId,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub relation_vacuum:
        Option<unsafe extern "C" fn(Relation, *mut VacuumParams, BufferAccessStrategy) -> ()>,
    pub scan_analyze_next_block:
        Option<unsafe extern "C" fn(TableScanDesc, BlockNumber, BufferAccessStrategy) -> bool>,
    pub scan_analyze_next_tuple: Option<
        unsafe extern "C" fn(
            TableScanDesc,
            TransactionId,
            *mut libc::c_double,
            *mut libc::c_double,
            *mut TupleTableSlot,
        ) -> bool,
    >,
    pub index_build_range_scan: Option<
        unsafe extern "C" fn(
            Relation,
            Relation,
            *mut IndexInfo,
            bool,
            bool,
            bool,
            BlockNumber,
            BlockNumber,
            IndexBuildCallback,
            *mut libc::c_void,
            TableScanDesc,
        ) -> libc::c_double,
    >,
    pub index_validate_scan: Option<
        unsafe extern "C" fn(
            Relation,
            Relation,
            *mut IndexInfo,
            Snapshot,
            *mut ValidateIndexState,
        ) -> (),
    >,
    pub relation_size: Option<unsafe extern "C" fn(Relation, ForkNumber) -> uint64>,
    pub relation_needs_toast_table: Option<unsafe extern "C" fn(Relation) -> bool>,
    pub relation_toast_am: Option<unsafe extern "C" fn(Relation) -> Oid>,
    pub relation_fetch_toast_slice:
        Option<unsafe extern "C" fn(Relation, Oid, i32, i32, i32, *mut varlena) -> ()>,
    pub relation_estimate_size: Option<
        unsafe extern "C" fn(
            Relation,
            *mut i32,
            *mut BlockNumber,
            *mut libc::c_double,
            *mut libc::c_double,
        ) -> (),
    >,
    pub scan_bitmap_next_block:
        Option<unsafe extern "C" fn(TableScanDesc, *mut TBMIterateResult) -> bool>,
    pub scan_bitmap_next_tuple: Option<
        unsafe extern "C" fn(TableScanDesc, *mut TBMIterateResult, *mut TupleTableSlot) -> bool,
    >,
    pub scan_sample_next_block:
        Option<unsafe extern "C" fn(TableScanDesc, *mut SampleScanState) -> bool>,
    pub scan_sample_next_tuple: Option<
        unsafe extern "C" fn(TableScanDesc, *mut SampleScanState, *mut TupleTableSlot) -> bool,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SampleScanState {
    pub ss: ScanState,
    pub args: *mut List,
    pub repeatable: *mut ExprState,
    pub tsmroutine: *mut TsmRoutine,
    pub tsm_state: *mut libc::c_void,
    pub use_bulkread: bool,
    pub use_pagemode: bool,
    pub begun: bool,
    pub seed: u32,
    pub donetuples: i64,
    pub haveblock: bool,
    pub done: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TsmRoutine {
    pub type_0: NodeTag,
    pub parameterTypes: *mut List,
    pub repeatable_across_queries: bool,
    pub repeatable_across_scans: bool,
    pub SampleScanGetSampleusize: SampleScanGetSampleusize_function,
    pub InitSampleScan: InitSampleScan_function,
    pub BeginSampleScan: BeginSampleScan_function,
    pub NextSampleBlock: NextSampleBlock_function,
    pub NextSampleTuple: NextSampleTuple_function,
    pub EndSampleScan: EndSampleScan_function,
}
pub type EndSampleScan_function = Option<unsafe extern "C" fn(*mut SampleScanState) -> ()>;
pub type NextSampleTuple_function =
    Option<unsafe extern "C" fn(*mut SampleScanState, BlockNumber, OffsetNumber) -> OffsetNumber>;
pub type NextSampleBlock_function =
    Option<unsafe extern "C" fn(*mut SampleScanState, BlockNumber) -> BlockNumber>;
pub type BeginSampleScan_function =
    Option<unsafe extern "C" fn(*mut SampleScanState, *mut Datum, libc::c_int, u32) -> ()>;
pub type InitSampleScan_function =
    Option<unsafe extern "C" fn(*mut SampleScanState, libc::c_int) -> ()>;
pub type SampleScanGetSampleusize_function = Option<
    unsafe extern "C" fn(
        *mut PlannerInfo,
        *mut RelOptInfo,
        *mut List,
        *mut BlockNumber,
        *mut libc::c_double,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScanState {
    pub ps: PlanState,
    pub ss_currentRelation: Relation,
    pub ss_currentScanDesc: *mut TableScanDescData,
    pub ss_ScanTupleSlot: *mut TupleTableSlot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableScanDescData {
    pub rs_rd: Relation,
    pub rs_snapshot: *mut SnapshotData,
    pub rs_nkeys: libc::c_int,
    pub rs_key: *mut ScanKeyData,
    pub rs_mintid: ItemPointerData,
    pub rs_maxtid: ItemPointerData,
    pub rs_flags: u32,
    pub rs_private: *mut libc::c_void,
    pub rs_parallel: *mut ParallelTableScanDescData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParallelTableScanDescData {
    pub phs_relid: Oid,
    pub phs_syncscan: bool,
    pub phs_snapshot_any: bool,
    pub phs_snapshot_off: usize,
}
pub type TableScanDesc = *mut TableScanDescData;
pub type ForkNumber = libc::c_int;
pub const INIT_FORKNUM: ForkNumber = 3;
pub const VISIBILITYMAP_FORKNUM: ForkNumber = 2;
pub const FSM_FORKNUM: ForkNumber = 1;
pub const MAIN_FORKNUM: ForkNumber = 0;
pub const InvalidForkNumber: ForkNumber = -1;
pub type IndexBuildCallback = Option<
    unsafe extern "C" fn(
        Relation,
        ItemPointer,
        *mut Datum,
        *mut bool,
        bool,
        *mut libc::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RelFileNode {
    pub spcNode: Oid,
    pub dbNode: Oid,
    pub relNode: Oid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TM_FailureData {
    pub ctid: ItemPointerData,
    pub xmax: TransactionId,
    pub cmax: CommandId,
    pub traversed: bool,
}
pub type LockTupleMode = libc::c_uint;
pub const LockTupleExclusive: LockTupleMode = 3;
pub const LockTupleNoKeyExclusive: LockTupleMode = 2;
pub const LockTupleShare: LockTupleMode = 1;
pub const LockTupleKeyShare: LockTupleMode = 0;
pub type TM_Result = libc::c_uint;
pub const TM_WouldBlock: TM_Result = 6;
pub const TM_BeingModified: TM_Result = 5;
pub const TM_Deleted: TM_Result = 4;
pub const TM_Updated: TM_Result = 3;
pub const TM_SelfModified: TM_Result = 2;
pub const TM_Invisible: TM_Result = 1;
pub const TM_Ok: TM_Result = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TM_IndexDeleteOp {
    pub bottomup: bool,
    pub bottomupfreespace: libc::c_int,
    pub ndeltids: libc::c_int,
    pub deltids: *mut TM_IndexDelete,
    pub status: *mut TM_IndexStatus,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TM_IndexStatus {
    pub idxoffnum: OffsetNumber,
    pub knowndeletable: bool,
    pub promising: bool,
    pub freespace: i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TM_IndexDelete {
    pub tid: ItemPointerData,
    pub id: i16,
}
pub type ParallelTableScanDesc = *mut ParallelTableScanDescData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PublicationActions {
    pub pubinsert: bool,
    pub pubupdate: bool,
    pub pubdelete: bool,
    pub pubtruncate: bool,
}
pub type PartitionDesc = *mut PartitionDescData;
pub type PartitionKey = *mut PartitionKeyData;
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
pub struct Var {
    pub xpr: Expr,
    pub varno: Index,
    pub varattno: AttrNumber,
    pub vartype: Oid,
    pub vartypmod: i32,
    pub varcollid: Oid,
    pub varlevelsup: Index,
    pub varnosyn: Index,
    pub varattnosyn: AttrNumber,
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
pub type BoolExprType = libc::c_uint;
pub const NOT_EXPR: BoolExprType = 2;
pub const OR_EXPR: BoolExprType = 1;
pub const AND_EXPR: BoolExprType = 0;
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
pub struct CoalesceExpr {
    pub xpr: Expr,
    pub coalescetype: Oid,
    pub coalescecollid: Oid,
    pub args: *mut List,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InferenceElem {
    pub xpr: Expr,
    pub expr: *mut Node,
    pub infercollid: Oid,
    pub inferopclass: Oid,
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
pub struct RangeTblRef {
    pub type_0: NodeTag,
    pub rtindex: libc::c_int,
}
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
pub struct Value {
    pub type_0: NodeTag,
    pub val: ValUnion,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ValUnion {
    pub ival: libc::c_int,
    pub str_0: *mut libc::c_char,
}
pub type SortByDir = libc::c_uint;
pub const SORTBY_USING: SortByDir = 3;
pub const SORTBY_DESC: SortByDir = 2;
pub const SORTBY_ASC: SortByDir = 1;
pub const SORTBY_DEFAULT: SortByDir = 0;
pub type SortByNulls = libc::c_uint;
pub const SORTBY_NULLS_LAST: SortByNulls = 2;
pub const SORTBY_NULLS_FIRST: SortByNulls = 1;
pub const SORTBY_NULLS_DEFAULT: SortByNulls = 0;
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
pub type A_Expr_Kind = libc::c_uint;
pub const AEXPR_NOT_BETWEEN_SYM: A_Expr_Kind = 13;
pub const AEXPR_BETWEEN_SYM: A_Expr_Kind = 12;
pub const AEXPR_NOT_BETWEEN: A_Expr_Kind = 11;
pub const AEXPR_BETWEEN: A_Expr_Kind = 10;
pub const AEXPR_SIMILAR: A_Expr_Kind = 9;
pub const AEXPR_ILIKE: A_Expr_Kind = 8;
pub const AEXPR_LIKE: A_Expr_Kind = 7;
pub const AEXPR_IN: A_Expr_Kind = 6;
pub const AEXPR_NULLIF: A_Expr_Kind = 5;
pub const AEXPR_NOT_DISTINCT: A_Expr_Kind = 4;
pub const AEXPR_DISTINCT: A_Expr_Kind = 3;
pub const AEXPR_OP_ALL: A_Expr_Kind = 2;
pub const AEXPR_OP_ANY: A_Expr_Kind = 1;
pub const AEXPR_OP: A_Expr_Kind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct A_Expr {
    pub type_0: NodeTag,
    pub kind: A_Expr_Kind,
    pub name: *mut List,
    pub lexpr: *mut Node,
    pub rexpr: *mut Node,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct A_Const {
    pub type_0: NodeTag,
    pub val: Value,
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
pub struct ResTarget {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub indirection: *mut List,
    pub val: *mut Node,
    pub location: libc::c_int,
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
pub struct RangeSubselect {
    pub type_0: NodeTag,
    pub lateral: bool,
    pub subquery: *mut Node,
    pub alias: *mut Alias,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RangeFunction {
    pub type_0: NodeTag,
    pub lateral: bool,
    pub ordinality: bool,
    pub is_rowsfrom: bool,
    pub functions: *mut List,
    pub alias: *mut Alias,
    pub coldeflist: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RangeTableFunc {
    pub type_0: NodeTag,
    pub lateral: bool,
    pub docexpr: *mut Node,
    pub rowexpr: *mut Node,
    pub namespaces: *mut List,
    pub columns: *mut List,
    pub alias: *mut Alias,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RangeTableFuncCol {
    pub type_0: NodeTag,
    pub colname: *mut libc::c_char,
    pub typeName: *mut TypeName,
    pub for_ordinality: bool,
    pub is_not_null: bool,
    pub colexpr: *mut Node,
    pub coldefexpr: *mut Node,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RangeTableSample {
    pub type_0: NodeTag,
    pub relation: *mut Node,
    pub method: *mut List,
    pub args: *mut List,
    pub repeatable: *mut Node,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexElem {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub expr: *mut Node,
    pub indexcolname: *mut libc::c_char,
    pub collation: *mut List,
    pub opclass: *mut List,
    pub opclassopts: *mut List,
    pub ordering: SortByDir,
    pub nulls_ordering: SortByNulls,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SortGroupClause {
    pub type_0: NodeTag,
    pub tleSortGroupRef: Index,
    pub eqop: Oid,
    pub sortop: Oid,
    pub nulls_first: bool,
    pub hashable: bool,
}
pub type GroupingSetKind = libc::c_uint;
pub const GROUPING_SET_SETS: GroupingSetKind = 4;
pub const GROUPING_SET_CUBE: GroupingSetKind = 3;
pub const GROUPING_SET_ROLLUP: GroupingSetKind = 2;
pub const GROUPING_SET_SIMPLE: GroupingSetKind = 1;
pub const GROUPING_SET_EMPTY: GroupingSetKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GroupingSet {
    pub type_0: NodeTag,
    pub kind: GroupingSetKind,
    pub content: *mut List,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WindowClause {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub refname: *mut libc::c_char,
    pub partitionClause: *mut List,
    pub orderClause: *mut List,
    pub frameOptions: libc::c_int,
    pub startOffset: *mut Node,
    pub endOffset: *mut Node,
    pub startInRangeFunc: Oid,
    pub endInRangeFunc: Oid,
    pub inRangeColl: Oid,
    pub inRangeAsc: bool,
    pub inRangeNullsFirst: bool,
    pub winref: Index,
    pub copiedOrder: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InferClause {
    pub type_0: NodeTag,
    pub indexElems: *mut List,
    pub whereClause: *mut Node,
    pub conname: *mut libc::c_char,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OnConflictClause {
    pub type_0: NodeTag,
    pub action: OnConflictAction,
    pub infer: *mut InferClause,
    pub targetList: *mut List,
    pub whereClause: *mut Node,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseCallbackState {
    pub pstate: *mut ParseState,
    pub location: libc::c_int,
    pub errcallback: ErrorContextCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_amproc {
    pub oid: Oid,
    pub amprocfamily: Oid,
    pub amproclefttype: Oid,
    pub amprocrighttype: Oid,
    pub amprocnum: i16,
    pub amproc: regproc,
}
pub type Form_pg_amproc = *mut FormData_pg_amproc;
pub type CatCList = catclist;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct catclist {
    pub cl_magic: libc::c_int,
    pub hash_value: u32,
    pub cache_elem: dlist_node,
    pub keys: [Datum; 4],
    pub refcount: libc::c_int,
    pub dead: bool,
    pub ordered: bool,
    pub nkeys: libc::c_short,
    pub n_members: libc::c_int,
    pub my_cache: *mut CatCache,
    pub members: [*mut CatCTup; 0],
}
pub type CatCTup = catctup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct catctup {
    pub ct_magic: libc::c_int,
    pub hash_value: u32,
    pub keys: [Datum; 4],
    pub cache_elem: dlist_node,
    pub refcount: libc::c_int,
    pub dead: bool,
    pub negative: bool,
    pub tuple: HeapTupleData,
    pub c_list: *mut catclist,
    pub my_cache: *mut CatCache,
}
pub type CatCache = catcache;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct catcache {
    pub id: libc::c_int,
    pub cc_nbuckets: libc::c_int,
    pub cc_tupdesc: TupleDesc,
    pub cc_bucket: *mut dlist_head,
    pub cc_hashfunc: [CCHashFN; 4],
    pub cc_fastequal: [CCFastEqualFN; 4],
    pub cc_keyno: [libc::c_int; 4],
    pub cc_lists: dlist_head,
    pub cc_ntup: libc::c_int,
    pub cc_nkeys: libc::c_int,
    pub cc_relname: *const libc::c_char,
    pub cc_reloid: Oid,
    pub cc_indexoid: Oid,
    pub cc_relisshared: bool,
    pub cc_next: slist_node,
    pub cc_skey: [ScanKeyData; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slist_node {
    pub next: *mut slist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlist_head {
    pub head: dlist_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlist_node {
    pub prev: *mut dlist_node,
    pub next: *mut dlist_node,
}
pub type CCFastEqualFN = Option<unsafe extern "C" fn(Datum, Datum) -> bool>;
pub type CCHashFN = Option<unsafe extern "C" fn(Datum) -> u32>;
pub const AMPROCNUM: SysCacheIdentifier = 5;
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
pub const AMOPSTRATEGY: SysCacheIdentifier = 4;
pub const AMOPOPID: SysCacheIdentifier = 3;
pub const AMOID: SysCacheIdentifier = 2;
pub const AMNAME: SysCacheIdentifier = 1;
pub const AGGFNOID: SysCacheIdentifier = 0;
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
#[no_mangle]
pub unsafe extern "C" fn transformFromClause(mut pstate: *mut ParseState, mut frmList: *mut List) {
    let mut fl: *mut ListCell = 0 as *mut ListCell;
    let mut fl__state: ForEachState = {
        let mut init = ForEachState {
            l: frmList,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(fl__state.l).is_null() && fl__state.i < (*fl__state.l).length {
        fl = &mut *((*fl__state.l).elements).offset(fl__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        fl = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut n: *mut Node = (*fl).ptr_value as *mut Node;
        let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
        let mut namespace: *mut List = 0 as *mut List;
        n = transformFromClauseItem(pstate, n, &mut nsitem, &mut namespace);
        checkNameSpaceConflicts(pstate, (*pstate).p_namespace, namespace);
        setNamespaceLateralState(namespace, true, true);
        (*pstate).p_joinlist = lappend((*pstate).p_joinlist, n as *mut libc::c_void);
        (*pstate).p_namespace = list_concat((*pstate).p_namespace, namespace);
        fl__state.i += 1;
        fl__state.i;
    }
    setNamespaceLateralState((*pstate).p_namespace, false, true);
}
#[no_mangle]
pub unsafe extern "C" fn setTargetTable(
    mut pstate: *mut ParseState,
    mut relation: *mut RangeVar,
    mut inh: bool,
    mut alsoSource: bool,
    mut requiredPerms: AclMode,
) -> libc::c_int {
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    if ((*relation).schemaname).is_null()
        && scanNameSpaceForENR(pstate, (*relation).relname) as libc::c_int != 0
    {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*pstate).p_target_relation).is_null() {
        table_close((*pstate).p_target_relation, 0 as libc::c_int);
    }
    (*pstate).p_target_relation = parserOpenTable(pstate, relation, 3 as libc::c_int);
    nsitem = addRangeTableEntryForRelation(
        pstate,
        (*pstate).p_target_relation,
        3 as libc::c_int,
        (*relation).alias,
        inh,
        false,
    );
    (*pstate).p_target_nsitem = nsitem;
    (*(*nsitem).p_rte).requiredPerms = requiredPerms;
    if alsoSource != 0 {
        addNSItemToQuery(pstate, nsitem, true, true, true);
    }
    return (*nsitem).p_rtindex;
}
unsafe extern "C" fn extractRemainingColumns(
    mut src_nscolumns: *mut ParseNamespaceColumn,
    mut src_colnames: *mut List,
    mut src_colnos: *mut *mut List,
    mut res_colnames: *mut *mut List,
    mut res_colvars: *mut *mut List,
    mut res_nscolumns: *mut ParseNamespaceColumn,
) -> libc::c_int {
    let mut colcount: libc::c_int = 0 as libc::c_int;
    let mut prevcols: *mut Bitmapset = 0 as *mut Bitmapset;
    let mut attnum: libc::c_int = 0;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    prevcols = 0 as *mut Bitmapset;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: *src_colnos,
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
        prevcols = bms_add_member(prevcols, (*lc).int_value);
        lc__state.i += 1;
        lc__state.i;
    }
    attnum = 0 as libc::c_int;
    let mut lc__state_0: ForEachState = {
        let mut init = ForEachState {
            l: src_colnames,
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
        let mut colname: *mut libc::c_char = (*((*lc).ptr_value as *mut Value)).val.str_0;
        attnum += 1;
        attnum;
        if *colname.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
            && bms_is_member(attnum, prevcols) == 0
        {
            *src_colnos = lappend_int(*src_colnos, attnum);
            *res_colnames = lappend(*res_colnames, (*lc).ptr_value);
            *res_colvars = lappend(
                *res_colvars,
                buildVarFromNSColumn(
                    src_nscolumns
                        .offset(attnum as isize)
                        .offset(-(1 as libc::c_int as isize)),
                ) as *mut libc::c_void,
            );
            *res_nscolumns.offset(colcount as isize) =
                *src_nscolumns.offset((attnum - 1 as libc::c_int) as isize);
            colcount += 1;
            colcount;
        }
        lc__state_0.i += 1;
        lc__state_0.i;
    }
    return colcount;
}
unsafe extern "C" fn transformJoinUsingClause(
    mut pstate: *mut ParseState,
    mut leftVars: *mut List,
    mut rightVars: *mut List,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut andargs: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut lvars: *mut ListCell = 0 as *mut ListCell;
    let mut rvars: *mut ListCell = 0 as *mut ListCell;
    let mut lvars__state: ForBothState = {
        let mut init = ForBothState {
            l1: leftVars,
            l2: rightVars,
            i: 0 as libc::c_int,
        };
        init
    };
    loop {
        lvars = (if !(lvars__state.l1).is_null() && lvars__state.i < (*lvars__state.l1).length {
            &mut *((*lvars__state.l1).elements).offset(lvars__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        rvars = (if !(lvars__state.l2).is_null() && lvars__state.i < (*lvars__state.l2).length {
            &mut *((*lvars__state.l2).elements).offset(lvars__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        if !(!lvars.is_null() && !rvars.is_null()) {
            break;
        }
        let mut lvar: *mut Var = (*lvars).ptr_value as *mut Var;
        let mut rvar: *mut Var = (*rvars).ptr_value as *mut Var;
        let mut e: *mut A_Expr = 0 as *mut A_Expr;
        markVarForSelectPriv(pstate, lvar);
        markVarForSelectPriv(pstate, rvar);
        e = makeSimpleA_Expr(
            AEXPR_OP,
            b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            copyObjectImpl(lvar as *const libc::c_void) as *mut Node,
            copyObjectImpl(rvar as *const libc::c_void) as *mut Node,
            -(1 as libc::c_int),
        );
        andargs = lappend(andargs, e as *mut libc::c_void);
        lvars__state.i += 1;
        lvars__state.i;
    }
    if list_length(andargs) == 1 as libc::c_int {
        result = (*list_nth_cell(andargs, 0 as libc::c_int)).ptr_value as *mut Node;
    } else {
        result = makeBoolExpr(AND_EXPR, andargs, -(1 as libc::c_int)) as *mut Node;
    }
    result = transformExpr(pstate, result, EXPR_KIND_JOIN_USING);
    result = coerce_to_boolean(
        pstate,
        result,
        b"JOIN/USING\0" as *const u8 as *const libc::c_char,
    );
    return result;
}
unsafe extern "C" fn transformJoinOnClause(
    mut pstate: *mut ParseState,
    mut j: *mut JoinExpr,
    mut namespace: *mut List,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut save_namespace: *mut List = 0 as *mut List;
    setNamespaceLateralState(namespace, false, true);
    save_namespace = (*pstate).p_namespace;
    (*pstate).p_namespace = namespace;
    result = transformWhereClause(
        pstate,
        (*j).quals,
        EXPR_KIND_JOIN_ON,
        b"JOIN/ON\0" as *const u8 as *const libc::c_char,
    );
    (*pstate).p_namespace = save_namespace;
    return result;
}
unsafe extern "C" fn transformTableEntry(
    mut pstate: *mut ParseState,
    mut r: *mut RangeVar,
) -> *mut ParseNamespaceItem {
    return addRangeTableEntry(pstate, r, (*r).alias, (*r).inh, true);
}
unsafe extern "C" fn transformRangeSubselect(
    mut pstate: *mut ParseState,
    mut r: *mut RangeSubselect,
) -> *mut ParseNamespaceItem {
    let mut query: *mut Query = 0 as *mut Query;
    if ((*r).alias).is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"subquery in FROM must have an alias\0" as *const u8 as *const libc::c_char,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_clause.c\0"
                    as *const u8 as *const libc::c_char,
                414 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    (*pstate).p_expr_kind = EXPR_KIND_FROM_SUBSELECT;
    (*pstate).p_lateral_active = (*r).lateral;
    query = parse_sub_analyze(
        (*r).subquery,
        pstate,
        0 as *mut CommonTableExpr,
        isLockedRefname(pstate, (*(*r).alias).aliasname),
        true,
    );
    (*pstate).p_lateral_active = false;
    (*pstate).p_expr_kind = EXPR_KIND_NONE;
    if !((*(query as *const Node)).type_0 as libc::c_uint == T_Query as libc::c_int as libc::c_uint)
        || (*query).commandType as libc::c_uint != CMD_SELECT as libc::c_int as libc::c_uint
    {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"unexpected non-SELECT command in subquery in FROM\0" as *const u8
                    as *const libc::c_char,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_clause.c\0"
                    as *const u8 as *const libc::c_char,
                449 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    return addRangeTableEntryForSubquery(pstate, query, (*r).alias, (*r).lateral, true);
}
unsafe extern "C" fn transformRangeFunction(
    mut pstate: *mut ParseState,
    mut r: *mut RangeFunction,
) -> *mut ParseNamespaceItem {
    let mut funcexprs: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut funcnames: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut coldeflists: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut is_lateral: bool = false;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    (*pstate).p_lateral_active = true;
    let mut current_block_32: u64;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*r).functions,
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
        let mut pair: *mut List = (*lc).ptr_value as *mut List;
        let mut fexpr: *mut Node = 0 as *mut Node;
        let mut coldeflist: *mut List = 0 as *mut List;
        let mut newfexpr: *mut Node = 0 as *mut Node;
        let mut last_srf: *mut Node = 0 as *mut Node;
        fexpr = (*list_nth_cell(pair, 0 as libc::c_int)).ptr_value as *mut Node;
        coldeflist = (*list_nth_cell(pair, 1 as libc::c_int)).ptr_value as *mut List;
        if (*(fexpr as *const Node)).type_0 as libc::c_uint
            == T_FuncCall as libc::c_int as libc::c_uint
        {
            let mut fc: *mut FuncCall = fexpr as *mut FuncCall;
            if list_length((*fc).funcname) == 1 as libc::c_int
                && strcmp(
                    (*((*list_nth_cell((*fc).funcname, 0 as libc::c_int)).ptr_value as *mut Value))
                        .val
                        .str_0,
                    b"unnest\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                && list_length((*fc).args) > 1 as libc::c_int
                && ((*fc).agg_order).is_null()
                && ((*fc).agg_filter).is_null()
                && ((*fc).over).is_null()
                && (*fc).agg_star == 0
                && (*fc).agg_distinct == 0
                && (*fc).func_variadic == 0
                && (*fc).funcformat as libc::c_uint
                    == COERCE_EXPLICIT_CALL as libc::c_int as libc::c_uint
                && coldeflist.is_null()
            {
                let mut lc_0: *mut ListCell = 0 as *mut ListCell;
                let mut lc__state_0: ForEachState = {
                    let mut init = ForEachState {
                        l: (*fc).args,
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
                    let mut arg: *mut Node = (*lc_0).ptr_value as *mut Node;
                    let mut newfc: *mut FuncCall = 0 as *mut FuncCall;
                    last_srf = (*pstate).p_last_srf;
                    newfc = makeFuncCall(
                        SystemFuncName(
                            b"unnest\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        ),
                        list_make1_impl(
                            T_List,
                            ListCell {
                                ptr_value: arg as *mut libc::c_void,
                            },
                        ),
                        COERCE_EXPLICIT_CALL,
                        (*fc).location,
                    );
                    newfexpr = transformExpr(pstate, newfc as *mut Node, EXPR_KIND_FROM_FUNCTION);
                    if (*pstate).p_last_srf != last_srf && (*pstate).p_last_srf != newfexpr {
                        let elevel_: libc::c_int = 21 as libc::c_int;
                        let mut __error: libc::c_int = 0;
                        if elevel_ >= 21 as libc::c_int {
                            abort();
                        }
                    }
                    funcexprs = lappend(funcexprs, newfexpr as *mut libc::c_void);
                    funcnames = lappend(
                        funcnames,
                        FigureColname(newfc as *mut Node) as *mut libc::c_void,
                    );
                    coldeflists = lappend(coldeflists, coldeflist as *mut libc::c_void);
                    lc__state_0.i += 1;
                    lc__state_0.i;
                }
                current_block_32 = 4644295000439058019;
            } else {
                current_block_32 = 13472856163611868459;
            }
        } else {
            current_block_32 = 13472856163611868459;
        }
        match current_block_32 {
            13472856163611868459 => {
                last_srf = (*pstate).p_last_srf;
                newfexpr = transformExpr(pstate, fexpr, EXPR_KIND_FROM_FUNCTION);
                if (*pstate).p_last_srf != last_srf && (*pstate).p_last_srf != newfexpr {
                    let elevel__0: libc::c_int = 21 as libc::c_int;
                    let mut __error_0: libc::c_int = 0;
                    if elevel__0 >= 21 as libc::c_int {
                        abort();
                    }
                }
                funcexprs = lappend(funcexprs, newfexpr as *mut libc::c_void);
                funcnames = lappend(funcnames, FigureColname(fexpr) as *mut libc::c_void);
                if !coldeflist.is_null() && !((*r).coldeflist).is_null() {
                    let elevel__1: libc::c_int = 21 as libc::c_int;
                    let mut __error_1: libc::c_int = 0;
                    if elevel__1 >= 21 as libc::c_int {
                        abort();
                    }
                }
                coldeflists = lappend(coldeflists, coldeflist as *mut libc::c_void);
            }
            _ => {}
        }
        lc__state.i += 1;
        lc__state.i;
    }
    (*pstate).p_lateral_active = false;
    assign_list_collations(pstate, funcexprs);
    if !((*r).coldeflist).is_null() {
        if list_length(funcexprs) != 1 as libc::c_int {
            if (*r).is_rowsfrom != 0 {
                let elevel__2: libc::c_int = 21 as libc::c_int;
                let mut __error_2: libc::c_int = 0;
                if elevel__2 >= 21 as libc::c_int {
                    abort();
                }
            } else {
                let elevel__3: libc::c_int = 21 as libc::c_int;
                let mut __error_3: libc::c_int = 0;
                if elevel__3 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        if (*r).ordinality != 0 {
            let elevel__4: libc::c_int = 21 as libc::c_int;
            let mut __error_4: libc::c_int = 0;
            if elevel__4 >= 21 as libc::c_int {
                abort();
            }
        }
        coldeflists = list_make1_impl(
            T_List,
            ListCell {
                ptr_value: (*r).coldeflist as *mut libc::c_void,
            },
        );
    }
    is_lateral = ((*r).lateral as libc::c_int != 0
        || contain_vars_of_level(funcexprs as *mut Node, 0 as libc::c_int) as libc::c_int != 0)
        as libc::c_int as bool;
    return addRangeTableEntryForFunction(
        pstate,
        funcnames,
        funcexprs,
        coldeflists,
        r,
        is_lateral,
        true,
    );
}
unsafe extern "C" fn transformRangeTableFunc(
    mut pstate: *mut ParseState,
    mut rtf: *mut RangeTableFunc,
) -> *mut ParseNamespaceItem {
    let mut tf: *mut TableFunc = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_TableFunc;
        _result
    }) as *mut TableFunc;
    let mut constructName: *const libc::c_char = 0 as *const libc::c_char;
    let mut docType: Oid = 0;
    let mut is_lateral: bool = false;
    let mut col: *mut ListCell = 0 as *mut ListCell;
    let mut names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut colno: libc::c_int = 0;
    constructName = b"XMLTABLE\0" as *const u8 as *const libc::c_char;
    (*pstate).p_lateral_active = true;
    assign_expr_collations(pstate, (*tf).rowexpr);
    (*tf).docexpr = coerce_to_specific_type(
        pstate,
        transformExpr(pstate, (*rtf).docexpr, EXPR_KIND_FROM_FUNCTION),
        docType,
        constructName,
    );
    assign_expr_collations(pstate, (*tf).docexpr);
    (*tf).ordinalitycol = -(1 as libc::c_int);
    names = palloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(list_length((*rtf).columns) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    colno = 0 as libc::c_int;
    let mut col__state: ForEachState = {
        let mut init = ForEachState {
            l: (*rtf).columns,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(col__state.l).is_null() && col__state.i < (*col__state.l).length {
        col = &mut *((*col__state.l).elements).offset(col__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        col = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut rawc: *mut RangeTableFuncCol = (*col).ptr_value as *mut RangeTableFuncCol;
        let mut typid: Oid = 0;
        let mut typmod: i32 = 0;
        let mut colexpr: *mut Node = 0 as *mut Node;
        let mut coldefexpr: *mut Node = 0 as *mut Node;
        let mut j: libc::c_int = 0;
        (*tf).colnames = lappend(
            (*tf).colnames,
            makeString(pstrdup((*rawc).colname)) as *mut libc::c_void,
        );
        if (*rawc).for_ordinality != 0 {
            if (*tf).ordinalitycol != -(1 as libc::c_int) {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            typmod = -(1 as libc::c_int);
            (*tf).ordinalitycol = colno;
        } else {
            if (*(*rawc).typeName).setof != 0 {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            typenameTypeIdAndMod(pstate, (*rawc).typeName, &mut typid, &mut typmod);
        }
        (*tf).coltypes = lappend_oid((*tf).coltypes, typid);
        (*tf).coltypmods = lappend_int((*tf).coltypmods, typmod);
        (*tf).colcollations = lappend_oid((*tf).colcollations, get_typcollation(typid));
        if !((*rawc).colexpr).is_null() {
            assign_expr_collations(pstate, colexpr);
        } else {
            colexpr = 0 as *mut Node;
        }
        if !((*rawc).coldefexpr).is_null() {
            coldefexpr = coerce_to_specific_type_typmod(
                pstate,
                transformExpr(pstate, (*rawc).coldefexpr, EXPR_KIND_FROM_FUNCTION),
                typid,
                typmod,
                constructName,
            );
            assign_expr_collations(pstate, coldefexpr);
        } else {
            coldefexpr = 0 as *mut Node;
        }
        (*tf).colexprs = lappend((*tf).colexprs, colexpr as *mut libc::c_void);
        (*tf).coldefexprs = lappend((*tf).coldefexprs, coldefexpr as *mut libc::c_void);
        if (*rawc).is_not_null != 0 {
            (*tf).notnulls = bms_add_member((*tf).notnulls, colno);
        }
        j = 0 as libc::c_int;
        while j < colno {
            if strcmp(*names.offset(j as isize), (*rawc).colname) == 0 as libc::c_int {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
            j += 1;
            j;
        }
        let ref mut fresh0 = *names.offset(colno as isize);
        *fresh0 = (*rawc).colname;
        colno += 1;
        colno;
        col__state.i += 1;
        col__state.i;
    }
    pfree(names as *mut libc::c_void);
    if !((*rtf).namespaces).is_null() {
        let mut ns: *mut ListCell = 0 as *mut ListCell;
        let mut lc2: *mut ListCell = 0 as *mut ListCell;
        let mut ns_uris: *mut List = 0 as *mut libc::c_void as *mut List;
        let mut ns_names: *mut List = 0 as *mut libc::c_void as *mut List;
        let mut default_ns_seen: bool = false;
        let mut ns__state: ForEachState = {
            let mut init = ForEachState {
                l: (*rtf).namespaces,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(ns__state.l).is_null() && ns__state.i < (*ns__state.l).length {
            ns = &mut *((*ns__state.l).elements).offset(ns__state.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            ns = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut r: *mut ResTarget = (*ns).ptr_value as *mut ResTarget;
            let mut ns_uri: *mut Node = 0 as *mut Node;
            ns_uri = transformExpr(pstate, (*r).val, EXPR_KIND_FROM_FUNCTION);
            assign_expr_collations(pstate, ns_uri);
            ns_uris = lappend(ns_uris, ns_uri as *mut libc::c_void);
            if !((*r).name).is_null() {
                let mut lc2__state: ForEachState = {
                    let mut init = ForEachState {
                        l: ns_names,
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
                    let mut ns_node: *mut Value = (*lc2).ptr_value as *mut Value;
                    if !ns_node.is_null() {
                        if strcmp((*ns_node).val.str_0, (*r).name) == 0 as libc::c_int {
                            let elevel__2: libc::c_int = 21 as libc::c_int;
                            let mut __error_2: libc::c_int = 0;
                            if elevel__2 >= 21 as libc::c_int {
                                abort();
                            }
                        }
                    }
                    lc2__state.i += 1;
                    lc2__state.i;
                }
            } else {
                if default_ns_seen != 0 {
                    let elevel__3: libc::c_int = 21 as libc::c_int;
                    let mut __error_3: libc::c_int = 0;
                    if elevel__3 >= 21 as libc::c_int {
                        abort();
                    }
                }
                default_ns_seen = true;
            }
            ns_names = lappend(
                ns_names,
                (if !((*r).name).is_null() {
                    makeString((*r).name)
                } else {
                    0 as *mut Value
                }) as *mut libc::c_void,
            );
            ns__state.i += 1;
            ns__state.i;
        }
        (*tf).ns_uris = ns_uris;
        (*tf).ns_names = ns_names;
    }
    (*tf).location = (*rtf).location;
    (*pstate).p_lateral_active = false;
    is_lateral = ((*rtf).lateral as libc::c_int != 0
        || contain_vars_of_level(tf as *mut Node, 0 as libc::c_int) as libc::c_int != 0)
        as libc::c_int as bool;
    return addRangeTableEntryForTableFunc(pstate, tf, (*rtf).alias, is_lateral, true);
}
unsafe extern "C" fn getNSItemForSpecialRelationTypes(
    mut pstate: *mut ParseState,
    mut rv: *mut RangeVar,
) -> *mut ParseNamespaceItem {
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    let mut cte: *mut CommonTableExpr = 0 as *mut CommonTableExpr;
    let mut levelsup: Index = 0;
    if !((*rv).schemaname).is_null() {
        return 0 as *mut ParseNamespaceItem;
    }
    cte = scanNameSpaceForCTE(pstate, (*rv).relname, &mut levelsup);
    if !cte.is_null() {
        nsitem = addRangeTableEntryForCTE(pstate, cte, levelsup, rv, true);
    } else if scanNameSpaceForENR(pstate, (*rv).relname) != 0 {
        nsitem = addRangeTableEntryForENR(pstate, rv, true);
    } else {
        nsitem = 0 as *mut ParseNamespaceItem;
    }
    return nsitem;
}
unsafe extern "C" fn buildVarFromNSColumn(mut nscol: *mut ParseNamespaceColumn) -> *mut Var {
    let mut var: *mut Var = 0 as *mut Var;
    var = makeVar(
        (*nscol).p_varno,
        (*nscol).p_varattno,
        (*nscol).p_vartype,
        (*nscol).p_vartypmod,
        (*nscol).p_varcollid,
        0 as libc::c_int as Index,
    );
    (*var).varnosyn = (*nscol).p_varnosyn;
    (*var).varattnosyn = (*nscol).p_varattnosyn;
    return var;
}
unsafe extern "C" fn buildMergedJoinVar(
    mut pstate: *mut ParseState,
    mut jointype: JoinType,
    mut l_colvar: *mut Var,
    mut r_colvar: *mut Var,
) -> *mut Node {
    let mut outcoltype: Oid = 0;
    let mut outcoltypmod: i32 = 0;
    let mut l_node: *mut Node = 0 as *mut Node;
    let mut r_node: *mut Node = 0 as *mut Node;
    let mut res_node: *mut Node = 0 as *mut Node;
    outcoltype = select_common_type(
        pstate,
        list_make2_impl(
            T_List,
            ListCell {
                ptr_value: l_colvar as *mut libc::c_void,
            },
            ListCell {
                ptr_value: r_colvar as *mut libc::c_void,
            },
        ),
        b"JOIN/USING\0" as *const u8 as *const libc::c_char,
        0 as *mut *mut Node,
    );
    outcoltypmod = select_common_typmod(
        pstate,
        list_make2_impl(
            T_List,
            ListCell {
                ptr_value: l_colvar as *mut libc::c_void,
            },
            ListCell {
                ptr_value: r_colvar as *mut libc::c_void,
            },
        ),
        outcoltype,
    );
    if (*l_colvar).vartype != outcoltype {
        l_node = coerce_type(
            pstate,
            l_colvar as *mut Node,
            (*l_colvar).vartype,
            outcoltype,
            outcoltypmod,
            COERCION_IMPLICIT,
            COERCE_IMPLICIT_CAST,
            -(1 as libc::c_int),
        );
    } else if (*l_colvar).vartypmod != outcoltypmod {
        l_node = makeRelabelType(
            l_colvar as *mut Expr,
            outcoltype,
            outcoltypmod,
            0 as libc::c_int as Oid,
            COERCE_IMPLICIT_CAST,
        ) as *mut Node;
    } else {
        l_node = l_colvar as *mut Node;
    }
    if (*r_colvar).vartype != outcoltype {
        r_node = coerce_type(
            pstate,
            r_colvar as *mut Node,
            (*r_colvar).vartype,
            outcoltype,
            outcoltypmod,
            COERCION_IMPLICIT,
            COERCE_IMPLICIT_CAST,
            -(1 as libc::c_int),
        );
    } else if (*r_colvar).vartypmod != outcoltypmod {
        r_node = makeRelabelType(
            r_colvar as *mut Expr,
            outcoltype,
            outcoltypmod,
            0 as libc::c_int as Oid,
            COERCE_IMPLICIT_CAST,
        ) as *mut Node;
    } else {
        r_node = r_colvar as *mut Node;
    }
    match jointype as libc::c_uint {
        0 => {
            if (*(l_node as *const Node)).type_0 as libc::c_uint
                == T_Var as libc::c_int as libc::c_uint
            {
                res_node = l_node;
            } else if (*(r_node as *const Node)).type_0 as libc::c_uint
                == T_Var as libc::c_int as libc::c_uint
            {
                res_node = r_node;
            } else {
                res_node = l_node;
            }
        }
        1 => {
            res_node = l_node;
        }
        3 => {
            res_node = r_node;
        }
        2 => {
            let mut c: *mut CoalesceExpr = ({
                let mut _result: *mut Node = 0 as *mut Node;
                (*_result).type_0 = T_CoalesceExpr;
                _result
            }) as *mut CoalesceExpr;
            (*c).coalescetype = outcoltype;
            (*c).args = list_make2_impl(
                T_List,
                ListCell {
                    ptr_value: l_node as *mut libc::c_void,
                },
                ListCell {
                    ptr_value: r_node as *mut libc::c_void,
                },
            );
            (*c).location = -(1 as libc::c_int);
            res_node = c as *mut Node;
        }
        _ => {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unrecognized join type: %d\0" as *const u8 as *const libc::c_char,
                    jointype as libc::c_int,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_clause.c\0"
                        as *const u8 as *const libc::c_char,
                    1648 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
            res_node = 0 as *mut Node;
        }
    }
    assign_expr_collations(pstate, res_node);
    return res_node;
}
unsafe extern "C" fn setNamespaceColumnVisibility(
    mut namespace: *mut List,
    mut cols_visible: bool,
) {
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: namespace,
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
        let mut nsitem: *mut ParseNamespaceItem = (*lc).ptr_value as *mut ParseNamespaceItem;
        (*nsitem).p_cols_visible = cols_visible;
        lc__state.i += 1;
        lc__state.i;
    }
}
unsafe extern "C" fn setNamespaceLateralState(
    mut namespace: *mut List,
    mut lateral_only: bool,
    mut lateral_ok: bool,
) {
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: namespace,
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
        let mut nsitem: *mut ParseNamespaceItem = (*lc).ptr_value as *mut ParseNamespaceItem;
        (*nsitem).p_lateral_only = lateral_only;
        (*nsitem).p_lateral_ok = lateral_ok;
        lc__state.i += 1;
        lc__state.i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn transformWhereClause(
    mut pstate: *mut ParseState,
    mut clause: *mut Node,
    mut exprKind: ParseExprKind,
    mut constructName: *const libc::c_char,
) -> *mut Node {
    let mut qual: *mut Node = 0 as *mut Node;
    if clause.is_null() {
        return 0 as *mut Node;
    }
    qual = transformExpr(pstate, clause, exprKind);
    qual = coerce_to_boolean(pstate, qual, constructName);
    return qual;
}
#[no_mangle]
pub unsafe extern "C" fn transformLimitClause(
    mut pstate: *mut ParseState,
    mut clause: *mut Node,
    mut exprKind: ParseExprKind,
    mut constructName: *const libc::c_char,
    mut limitOption: LimitOption,
) -> *mut Node {
    let mut qual: *mut Node = 0 as *mut Node;
    if clause.is_null() {
        return 0 as *mut Node;
    }
    qual = transformExpr(pstate, clause, exprKind);
    checkExprIsVarFree(pstate, qual, constructName);
    if exprKind as libc::c_uint == EXPR_KIND_LIMIT as libc::c_int as libc::c_uint
        && limitOption as libc::c_uint == LIMIT_OPTION_WITH_TIES as libc::c_int as libc::c_uint
        && (*(clause as *const Node)).type_0 as libc::c_uint
            == T_A_Const as libc::c_int as libc::c_uint
        && (*(clause as *mut A_Const)).val.type_0 as libc::c_uint
            == T_Null as libc::c_int as libc::c_uint
    {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return qual;
}
unsafe extern "C" fn checkExprIsVarFree(
    mut pstate: *mut ParseState,
    mut n: *mut Node,
    mut constructName: *const libc::c_char,
) {
    if contain_vars_of_level(n, 0 as libc::c_int) != 0 {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
}
unsafe extern "C" fn checkTargetlistEntrySQL92(
    mut pstate: *mut ParseState,
    mut tle: *mut TargetEntry,
    mut exprKind: ParseExprKind,
) {
    match exprKind as libc::c_uint {
        18 => {
            if (*pstate).p_hasAggs as libc::c_int != 0
                && contain_aggs_of_level((*tle).expr as *mut Node, 0 as libc::c_int) as libc::c_int
                    != 0
            {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            if (*pstate).p_hasWindowFuncs as libc::c_int != 0
                && contain_windowfuncs((*tle).expr as *mut Node) as libc::c_int != 0
            {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        19 => {}
        20 => {}
        _ => {
            let elevel__1: libc::c_int = 21 as libc::c_int;
            let mut __error_1: libc::c_int = 0;
            if errstart(elevel__1, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unexpected exprKind in checkTargetlistEntrySQL92\0" as *const u8
                        as *const libc::c_char,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_clause.c\0"
                        as *const u8 as *const libc::c_char,
                    1836 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel__1 >= 21 as libc::c_int {
                abort();
            }
        }
    };
}
unsafe extern "C" fn findTargetlistEntrySQL92(
    mut pstate: *mut ParseState,
    mut node: *mut Node,
    mut tlist: *mut *mut List,
    mut exprKind: ParseExprKind,
) -> *mut TargetEntry {
    let mut tl: *mut ListCell = 0 as *mut ListCell;
    if (*(node as *const Node)).type_0 as libc::c_uint == T_ColumnRef as libc::c_int as libc::c_uint
        && list_length((*(node as *mut ColumnRef)).fields) == 1 as libc::c_int
        && (*((*list_nth_cell((*(node as *mut ColumnRef)).fields, 0 as libc::c_int)).ptr_value
            as *const Node))
            .type_0 as libc::c_uint
            == T_String as libc::c_int as libc::c_uint
    {
        let mut name: *mut libc::c_char =
            (*((*list_nth_cell((*(node as *mut ColumnRef)).fields, 0 as libc::c_int)).ptr_value
                as *mut Value))
                .val
                .str_0;
        let mut location: libc::c_int = (*(node as *mut ColumnRef)).location;
        if exprKind as libc::c_uint == EXPR_KIND_GROUP_BY as libc::c_int as libc::c_uint {
            if !(colNameToVar(pstate, name, true, location)).is_null() {
                name = 0 as *mut libc::c_char;
            }
        }
        if !name.is_null() {
            let mut target_result: *mut TargetEntry = 0 as *mut TargetEntry;
            let mut tl__state: ForEachState = {
                let mut init = ForEachState {
                    l: *tlist,
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
                let mut tle: *mut TargetEntry = (*tl).ptr_value as *mut TargetEntry;
                if (*tle).resjunk == 0 && strcmp((*tle).resname, name) == 0 as libc::c_int {
                    if !target_result.is_null() {
                        if equal(
                            (*target_result).expr as *const libc::c_void,
                            (*tle).expr as *const libc::c_void,
                        ) == 0
                        {
                            let elevel_: libc::c_int = 21 as libc::c_int;
                            let mut __error: libc::c_int = 0;
                            if elevel_ >= 21 as libc::c_int {
                                abort();
                            }
                        }
                    } else {
                        target_result = tle;
                    }
                }
                tl__state.i += 1;
                tl__state.i;
            }
            if !target_result.is_null() {
                checkTargetlistEntrySQL92(pstate, target_result, exprKind);
                return target_result;
            }
        }
    }
    if (*(node as *const Node)).type_0 as libc::c_uint == T_A_Const as libc::c_int as libc::c_uint {
        let mut val: *mut Value = &mut (*(node as *mut A_Const)).val;
        let mut location_0: libc::c_int = (*(node as *mut A_Const)).location;
        let mut targetlist_pos: libc::c_int = 0 as libc::c_int;
        let mut target_pos: libc::c_int = 0;
        if !((*(val as *const Node)).type_0 as libc::c_uint
            == T_Integer as libc::c_int as libc::c_uint)
        {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
        target_pos = (*val).val.ival;
        let mut tl__state_0: ForEachState = {
            let mut init = ForEachState {
                l: *tlist,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(tl__state_0.l).is_null() && tl__state_0.i < (*tl__state_0.l).length {
            tl = &mut *((*tl__state_0.l).elements).offset(tl__state_0.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            tl = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut tle_0: *mut TargetEntry = (*tl).ptr_value as *mut TargetEntry;
            if (*tle_0).resjunk == 0 {
                targetlist_pos += 1;
                if targetlist_pos == target_pos {
                    checkTargetlistEntrySQL92(pstate, tle_0, exprKind);
                    return tle_0;
                }
            }
            tl__state_0.i += 1;
            tl__state_0.i;
        }
        let elevel__1: libc::c_int = 21 as libc::c_int;
        let mut __error_1: libc::c_int = 0;
        if elevel__1 >= 21 as libc::c_int {
            abort();
        }
    }
    return findTargetlistEntrySQL99(pstate, node, tlist, exprKind);
}
unsafe extern "C" fn findTargetlistEntrySQL99(
    mut pstate: *mut ParseState,
    mut node: *mut Node,
    mut tlist: *mut *mut List,
    mut exprKind: ParseExprKind,
) -> *mut TargetEntry {
    let mut target_result: *mut TargetEntry = 0 as *mut TargetEntry;
    let mut tl: *mut ListCell = 0 as *mut ListCell;
    let mut expr: *mut Node = 0 as *mut Node;
    expr = transformExpr(pstate, node, exprKind);
    let mut tl__state: ForEachState = {
        let mut init = ForEachState {
            l: *tlist,
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
        let mut tle: *mut TargetEntry = (*tl).ptr_value as *mut TargetEntry;
        let mut texpr: *mut Node = 0 as *mut Node;
        texpr = strip_implicit_coercions((*tle).expr as *mut Node);
        if equal(expr as *const libc::c_void, texpr as *const libc::c_void) != 0 {
            return tle;
        }
        tl__state.i += 1;
        tl__state.i;
    }
    target_result =
        transformTargetEntry(pstate, node, expr, exprKind, 0 as *mut libc::c_char, true);
    *tlist = lappend(*tlist, target_result as *mut libc::c_void);
    return target_result;
}
unsafe extern "C" fn flatten_grouping_sets(
    mut expr: *mut Node,
    mut toplevel: bool,
    mut hasGroupingSets: *mut bool,
) -> *mut Node {
    check_stack_depth();
    if expr.is_null() {
        return 0 as *mut libc::c_void as *mut List as *mut Node;
    }
    match (*expr).type_0 as libc::c_uint {
        136 => {
            let mut r: *mut RowExpr = expr as *mut RowExpr;
            if (*r).row_format as libc::c_uint
                == COERCE_IMPLICIT_CAST as libc::c_int as libc::c_uint
            {
                return flatten_grouping_sets((*r).args as *mut Node, false, 0 as *mut bool);
            }
        }
        378 => {
            let mut gset: *mut GroupingSet = expr as *mut GroupingSet;
            let mut l2: *mut ListCell = 0 as *mut ListCell;
            let mut result_set: *mut List = 0 as *mut libc::c_void as *mut List;
            if !hasGroupingSets.is_null() {
                *hasGroupingSets = true;
            }
            if toplevel as libc::c_int != 0
                && (*gset).kind as libc::c_uint == GROUPING_SET_EMPTY as libc::c_int as libc::c_uint
            {
                return 0 as *mut libc::c_void as *mut List as *mut Node;
            }
            let mut l2__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*gset).content,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(l2__state.l).is_null() && l2__state.i < (*l2__state.l).length {
                l2 = &mut *((*l2__state.l).elements).offset(l2__state.i as isize) as *mut ListCell;
                true as libc::c_int
            } else {
                l2 = 0 as *mut ListCell;
                false as libc::c_int
            } != 0
            {
                let mut n1: *mut Node = (*l2).ptr_value as *mut Node;
                let mut n2: *mut Node = flatten_grouping_sets(n1, false, 0 as *mut bool);
                if (*(n1 as *const Node)).type_0 as libc::c_uint
                    == T_GroupingSet as libc::c_int as libc::c_uint
                    && (*(n1 as *mut GroupingSet)).kind as libc::c_uint
                        == GROUPING_SET_SETS as libc::c_int as libc::c_uint
                {
                    result_set = list_concat(result_set, n2 as *mut List);
                } else {
                    result_set = lappend(result_set, n2 as *mut libc::c_void);
                }
                l2__state.i += 1;
                l2__state.i;
            }
            if toplevel as libc::c_int != 0
                || (*gset).kind as libc::c_uint != GROUPING_SET_SETS as libc::c_int as libc::c_uint
            {
                return makeGroupingSet((*gset).kind, result_set, (*gset).location) as *mut Node;
            } else {
                return result_set as *mut Node;
            }
        }
        227 => {
            let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
            let mut l: *mut ListCell = 0 as *mut ListCell;
            let mut l__state: ForEachState = {
                let mut init = ForEachState {
                    l: expr as *mut List,
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
                let mut n: *mut Node =
                    flatten_grouping_sets((*l).ptr_value as *mut Node, toplevel, hasGroupingSets);
                if !n.is_null() {
                    if (*(n as *const Node)).type_0 as libc::c_uint
                        == T_List as libc::c_int as libc::c_uint
                    {
                        result = list_concat(result, n as *mut List);
                    } else {
                        result = lappend(result, n as *mut libc::c_void);
                    }
                }
                l__state.i += 1;
                l__state.i;
            }
            return result as *mut Node;
        }
        _ => {}
    }
    return expr;
}
unsafe extern "C" fn transformGroupClauseExpr(
    mut flatresult: *mut *mut List,
    mut seen_local: *mut Bitmapset,
    mut pstate: *mut ParseState,
    mut gexpr: *mut Node,
    mut targetlist: *mut *mut List,
    mut sortClause: *mut List,
    mut exprKind: ParseExprKind,
    mut useSQL99: bool,
    mut toplevel: bool,
) -> Index {
    let mut tle: *mut TargetEntry = 0 as *mut TargetEntry;
    let mut found: bool = false;
    if useSQL99 != 0 {
        tle = findTargetlistEntrySQL99(pstate, gexpr, targetlist, exprKind);
    } else {
        tle = findTargetlistEntrySQL92(pstate, gexpr, targetlist, exprKind);
    }
    if (*tle).ressortgroupref > 0 as libc::c_int as Index {
        let mut sl: *mut ListCell = 0 as *mut ListCell;
        if bms_is_member((*tle).ressortgroupref as libc::c_int, seen_local) != 0 {
            return 0 as libc::c_int as Index;
        }
        found = targetIsInSortList(tle, 0 as libc::c_int as Oid, *flatresult);
        if found != 0 {
            return (*tle).ressortgroupref;
        }
        let mut sl__state: ForEachState = {
            let mut init = ForEachState {
                l: sortClause,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(sl__state.l).is_null() && sl__state.i < (*sl__state.l).length {
            sl = &mut *((*sl__state.l).elements).offset(sl__state.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            sl = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut sc: *mut SortGroupClause = (*sl).ptr_value as *mut SortGroupClause;
            if (*sc).tleSortGroupRef == (*tle).ressortgroupref {
                let mut grpc: *mut SortGroupClause =
                    copyObjectImpl(sc as *const libc::c_void) as *mut SortGroupClause;
                if toplevel == 0 {
                    (*grpc).nulls_first = false;
                }
                *flatresult = lappend(*flatresult, grpc as *mut libc::c_void);
                found = true;
                break;
            } else {
                sl__state.i += 1;
                sl__state.i;
            }
        }
    }
    if found == 0 {
        *flatresult =
            addTargetToGroupList(pstate, tle, *flatresult, *targetlist, exprLocation(gexpr));
    }
    return (*tle).ressortgroupref;
}
unsafe extern "C" fn transformGroupClauseList(
    mut flatresult: *mut *mut List,
    mut pstate: *mut ParseState,
    mut list: *mut List,
    mut targetlist: *mut *mut List,
    mut sortClause: *mut List,
    mut exprKind: ParseExprKind,
    mut useSQL99: bool,
    mut toplevel: bool,
) -> *mut List {
    let mut seen_local: *mut Bitmapset = 0 as *mut Bitmapset;
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut gl: *mut ListCell = 0 as *mut ListCell;
    let mut gl__state: ForEachState = {
        let mut init = ForEachState {
            l: list,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(gl__state.l).is_null() && gl__state.i < (*gl__state.l).length {
        gl = &mut *((*gl__state.l).elements).offset(gl__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        gl = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut gexpr: *mut Node = (*gl).ptr_value as *mut Node;
        let mut ref_0: Index = transformGroupClauseExpr(
            flatresult, seen_local, pstate, gexpr, targetlist, sortClause, exprKind, useSQL99,
            toplevel,
        );
        if ref_0 > 0 as libc::c_int as Index {
            seen_local = bms_add_member(seen_local, ref_0 as libc::c_int);
            result = lappend_int(result, ref_0 as libc::c_int);
        }
        gl__state.i += 1;
        gl__state.i;
    }
    return result;
}
unsafe extern "C" fn transformGroupingSet(
    mut flatresult: *mut *mut List,
    mut pstate: *mut ParseState,
    mut gset: *mut GroupingSet,
    mut targetlist: *mut *mut List,
    mut sortClause: *mut List,
    mut exprKind: ParseExprKind,
    mut useSQL99: bool,
    mut toplevel: bool,
) -> *mut Node {
    let mut gl: *mut ListCell = 0 as *mut ListCell;
    let mut content: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut gl__state: ForEachState = {
        let mut init = ForEachState {
            l: (*gset).content,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(gl__state.l).is_null() && gl__state.i < (*gl__state.l).length {
        gl = &mut *((*gl__state.l).elements).offset(gl__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        gl = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut n: *mut Node = (*gl).ptr_value as *mut Node;
        if (*(n as *const Node)).type_0 as libc::c_uint == T_List as libc::c_int as libc::c_uint {
            let mut l: *mut List = transformGroupClauseList(
                flatresult,
                pstate,
                n as *mut List,
                targetlist,
                sortClause,
                exprKind,
                useSQL99,
                false,
            );
            content = lappend(
                content,
                makeGroupingSet(GROUPING_SET_SIMPLE, l, exprLocation(n)) as *mut libc::c_void,
            );
        } else if (*(n as *const Node)).type_0 as libc::c_uint
            == T_GroupingSet as libc::c_int as libc::c_uint
        {
            let mut gset2: *mut GroupingSet = (*gl).ptr_value as *mut GroupingSet;
            content = lappend(
                content,
                transformGroupingSet(
                    flatresult, pstate, gset2, targetlist, sortClause, exprKind, useSQL99, false,
                ) as *mut libc::c_void,
            );
        } else {
            let mut ref_0: Index = transformGroupClauseExpr(
                flatresult,
                0 as *mut Bitmapset,
                pstate,
                n,
                targetlist,
                sortClause,
                exprKind,
                useSQL99,
                false,
            );
            content = lappend(
                content,
                makeGroupingSet(
                    GROUPING_SET_SIMPLE,
                    list_make1_impl(
                        T_IntList,
                        ListCell {
                            int_value: ref_0 as libc::c_int,
                        },
                    ),
                    exprLocation(n),
                ) as *mut libc::c_void,
            );
        }
        gl__state.i += 1;
        gl__state.i;
    }
    if (*gset).kind as libc::c_uint == GROUPING_SET_CUBE as libc::c_int as libc::c_uint {
        if list_length(content) > 12 as libc::c_int {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
    }
    return makeGroupingSet((*gset).kind, content, (*gset).location) as *mut Node;
}
#[no_mangle]
pub unsafe extern "C" fn transformGroupClause(
    mut pstate: *mut ParseState,
    mut grouplist: *mut List,
    mut groupingSets: *mut *mut List,
    mut targetlist: *mut *mut List,
    mut sortClause: *mut List,
    mut exprKind: ParseExprKind,
    mut useSQL99: bool,
) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut flat_grouplist: *mut List = 0 as *mut List;
    let mut gsets: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut gl: *mut ListCell = 0 as *mut ListCell;
    let mut hasGroupingSets: bool = false;
    let mut seen_local: *mut Bitmapset = 0 as *mut Bitmapset;
    flat_grouplist =
        flatten_grouping_sets(grouplist as *mut Node, true, &mut hasGroupingSets) as *mut List;
    if flat_grouplist.is_null() && hasGroupingSets as libc::c_int != 0 {
        flat_grouplist = list_make1_impl(
            T_List,
            ListCell {
                ptr_value: makeGroupingSet(
                    GROUPING_SET_EMPTY,
                    0 as *mut libc::c_void as *mut List,
                    exprLocation(grouplist as *mut Node),
                ) as *mut libc::c_void,
            },
        );
    }
    let mut gl__state: ForEachState = {
        let mut init = ForEachState {
            l: flat_grouplist,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(gl__state.l).is_null() && gl__state.i < (*gl__state.l).length {
        gl = &mut *((*gl__state.l).elements).offset(gl__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        gl = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut gexpr: *mut Node = (*gl).ptr_value as *mut Node;
        if (*(gexpr as *const Node)).type_0 as libc::c_uint
            == T_GroupingSet as libc::c_int as libc::c_uint
        {
            let mut gset: *mut GroupingSet = gexpr as *mut GroupingSet;
            match (*gset).kind as libc::c_uint {
                0 => {
                    gsets = lappend(gsets, gset as *mut libc::c_void);
                }
                4 | 3 | 2 => {
                    gsets = lappend(
                        gsets,
                        transformGroupingSet(
                            &mut result,
                            pstate,
                            gset,
                            targetlist,
                            sortClause,
                            exprKind,
                            useSQL99,
                            true,
                        ) as *mut libc::c_void,
                    );
                }
                1 | _ => {}
            }
        } else {
            let mut ref_0: Index = transformGroupClauseExpr(
                &mut result,
                seen_local,
                pstate,
                gexpr,
                targetlist,
                sortClause,
                exprKind,
                useSQL99,
                true,
            );
            if ref_0 > 0 as libc::c_int as Index {
                seen_local = bms_add_member(seen_local, ref_0 as libc::c_int);
                if hasGroupingSets != 0 {
                    gsets = lappend(
                        gsets,
                        makeGroupingSet(
                            GROUPING_SET_SIMPLE,
                            list_make1_impl(
                                T_IntList,
                                ListCell {
                                    int_value: ref_0 as libc::c_int,
                                },
                            ),
                            exprLocation(gexpr),
                        ) as *mut libc::c_void,
                    );
                }
            }
        }
        gl__state.i += 1;
        gl__state.i;
    }
    if !groupingSets.is_null() {
        *groupingSets = gsets;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn transformSortClause(
    mut pstate: *mut ParseState,
    mut orderlist: *mut List,
    mut targetlist: *mut *mut List,
    mut exprKind: ParseExprKind,
    mut useSQL99: bool,
) -> *mut List {
    let mut sortlist: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut olitem: *mut ListCell = 0 as *mut ListCell;
    let mut olitem__state: ForEachState = {
        let mut init = ForEachState {
            l: orderlist,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(olitem__state.l).is_null() && olitem__state.i < (*olitem__state.l).length {
        olitem =
            &mut *((*olitem__state.l).elements).offset(olitem__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        olitem = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut sortby: *mut SortBy = (*olitem).ptr_value as *mut SortBy;
        let mut tle: *mut TargetEntry = 0 as *mut TargetEntry;
        if useSQL99 != 0 {
            tle = findTargetlistEntrySQL99(pstate, (*sortby).node, targetlist, exprKind);
        } else {
            tle = findTargetlistEntrySQL92(pstate, (*sortby).node, targetlist, exprKind);
        }
        sortlist = addTargetToSortList(pstate, tle, sortlist, *targetlist, sortby);
        olitem__state.i += 1;
        olitem__state.i;
    }
    return sortlist;
}
#[no_mangle]
pub unsafe extern "C" fn transformWindowDefinitions(
    mut pstate: *mut ParseState,
    mut windowdefs: *mut List,
    mut targetlist: *mut *mut List,
) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut winref: Index = 0 as libc::c_int as Index;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: windowdefs,
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
        let mut windef: *mut WindowDef = (*lc).ptr_value as *mut WindowDef;
        let mut refwc: *mut WindowClause = 0 as *mut WindowClause;
        let mut partitionClause: *mut List = 0 as *mut List;
        let mut orderClause: *mut List = 0 as *mut List;
        let mut rangeopfamily: Oid = 0 as libc::c_int as Oid;
        let mut rangeopcintype: Oid = 0 as libc::c_int as Oid;
        let mut wc: *mut WindowClause = 0 as *mut WindowClause;
        winref = winref.wrapping_add(1);
        winref;
        if !((*windef).name).is_null() && !(findWindowClause(result, (*windef).name)).is_null() {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        if !((*windef).refname).is_null() {
            refwc = findWindowClause(result, (*windef).refname);
            if refwc.is_null() {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        orderClause = transformSortClause(
            pstate,
            (*windef).orderClause,
            targetlist,
            EXPR_KIND_WINDOW_ORDER,
            true,
        );
        partitionClause = transformGroupClause(
            pstate,
            (*windef).partitionClause,
            0 as *mut *mut List,
            targetlist,
            orderClause,
            EXPR_KIND_WINDOW_PARTITION,
            true,
        );
        wc = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_WindowClause;
            _result
        }) as *mut WindowClause;
        (*wc).name = (*windef).name;
        (*wc).refname = (*windef).refname;
        if !refwc.is_null() {
            if !partitionClause.is_null() {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
            (*wc).partitionClause =
                copyObjectImpl((*refwc).partitionClause as *const libc::c_void) as *mut List;
        } else {
            (*wc).partitionClause = partitionClause;
        }
        if !refwc.is_null() {
            if !orderClause.is_null() && !((*refwc).orderClause).is_null() {
                let elevel__2: libc::c_int = 21 as libc::c_int;
                let mut __error_2: libc::c_int = 0;
                if elevel__2 >= 21 as libc::c_int {
                    abort();
                }
            }
            if !orderClause.is_null() {
                (*wc).orderClause = orderClause;
                (*wc).copiedOrder = false;
            } else {
                (*wc).orderClause =
                    copyObjectImpl((*refwc).orderClause as *const libc::c_void) as *mut List;
                (*wc).copiedOrder = true;
            }
        } else {
            (*wc).orderClause = orderClause;
            (*wc).copiedOrder = false;
        }
        if !refwc.is_null()
            && (*refwc).frameOptions
                != 0x2 as libc::c_int | 0x20 as libc::c_int | 0x400 as libc::c_int
        {
            if !((*windef).name).is_null()
                || !orderClause.is_null()
                || (*windef).frameOptions
                    != 0x2 as libc::c_int | 0x20 as libc::c_int | 0x400 as libc::c_int
            {
                let elevel__3: libc::c_int = 21 as libc::c_int;
                let mut __error_3: libc::c_int = 0;
                if elevel__3 >= 21 as libc::c_int {
                    abort();
                }
            }
            let elevel__4: libc::c_int = 21 as libc::c_int;
            let mut __error_4: libc::c_int = 0;
            if elevel__4 >= 21 as libc::c_int {
                abort();
            }
        }
        (*wc).frameOptions = (*windef).frameOptions;
        if (*wc).frameOptions & 0x2 as libc::c_int != 0
            && (*wc).frameOptions
                & (0x800 as libc::c_int
                    | 0x2000 as libc::c_int
                    | (0x1000 as libc::c_int | 0x4000 as libc::c_int))
                != 0
        {
            let mut sortcl: *mut SortGroupClause = 0 as *mut SortGroupClause;
            let mut sortkey: *mut Node = 0 as *mut Node;
            let mut rangestrategy: i16 = 0;
            if list_length((*wc).orderClause) != 1 as libc::c_int {
                let elevel__5: libc::c_int = 21 as libc::c_int;
                let mut __error_5: libc::c_int = 0;
                if elevel__5 >= 21 as libc::c_int {
                    abort();
                }
            }
            sortcl = (*list_nth_cell((*wc).orderClause, 0 as libc::c_int)).ptr_value
                as *mut SortGroupClause;
            sortkey = get_sortgroupclause_expr(sortcl, *targetlist);
            if get_ordering_op_properties(
                (*sortcl).sortop,
                &mut rangeopfamily,
                &mut rangeopcintype,
                &mut rangestrategy,
            ) == 0
            {
                let elevel__6: libc::c_int = 21 as libc::c_int;
                let mut __error_6: libc::c_int = 0;
                if errstart(elevel__6, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"operator %u is not a valid ordering operator\0" as *const u8
                            as *const libc::c_char,
                        (*sortcl).sortop,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_clause.c\0"
                            as *const u8 as *const libc::c_char,
                        2787 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel__6 >= 21 as libc::c_int {
                    abort();
                }
            }
            (*wc).inRangeColl = exprCollation(sortkey);
            (*wc).inRangeAsc =
                (rangestrategy as libc::c_int == 1 as libc::c_int) as libc::c_int as bool;
            (*wc).inRangeNullsFirst = (*sortcl).nulls_first;
        }
        if (*wc).frameOptions & 0x8 as libc::c_int != 0 {
            if ((*wc).orderClause).is_null() {
                let elevel__7: libc::c_int = 21 as libc::c_int;
                let mut __error_7: libc::c_int = 0;
                if elevel__7 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        (*wc).startOffset = transformFrameOffset(
            pstate,
            (*wc).frameOptions,
            rangeopfamily,
            rangeopcintype,
            &mut (*wc).startInRangeFunc,
            (*windef).startOffset,
        );
        (*wc).endOffset = transformFrameOffset(
            pstate,
            (*wc).frameOptions,
            rangeopfamily,
            rangeopcintype,
            &mut (*wc).endInRangeFunc,
            (*windef).endOffset,
        );
        (*wc).winref = winref;
        result = lappend(result, wc as *mut libc::c_void);
        lc__state.i += 1;
        lc__state.i;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn transformDistinctClause(
    mut pstate: *mut ParseState,
    mut targetlist: *mut *mut List,
    mut sortClause: *mut List,
    mut is_agg: bool,
) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut slitem: *mut ListCell = 0 as *mut ListCell;
    let mut tlitem: *mut ListCell = 0 as *mut ListCell;
    let mut slitem__state: ForEachState = {
        let mut init = ForEachState {
            l: sortClause,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(slitem__state.l).is_null() && slitem__state.i < (*slitem__state.l).length {
        slitem =
            &mut *((*slitem__state.l).elements).offset(slitem__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        slitem = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut scl: *mut SortGroupClause = (*slitem).ptr_value as *mut SortGroupClause;
        let mut tle: *mut TargetEntry = get_sortgroupclause_tle(scl, *targetlist);
        if (*tle).resjunk != 0 {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        result = lappend(result, copyObjectImpl(scl as *const libc::c_void));
        slitem__state.i += 1;
        slitem__state.i;
    }
    let mut tlitem__state: ForEachState = {
        let mut init = ForEachState {
            l: *targetlist,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(tlitem__state.l).is_null() && tlitem__state.i < (*tlitem__state.l).length {
        tlitem =
            &mut *((*tlitem__state.l).elements).offset(tlitem__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        tlitem = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut tle_0: *mut TargetEntry = (*tlitem).ptr_value as *mut TargetEntry;
        if !((*tle_0).resjunk != 0) {
            result = addTargetToGroupList(
                pstate,
                tle_0,
                result,
                *targetlist,
                exprLocation((*tle_0).expr as *mut Node),
            );
        }
        tlitem__state.i += 1;
        tlitem__state.i;
    }
    if result.is_null() {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn transformDistinctOnClause(
    mut pstate: *mut ParseState,
    mut distinctlist: *mut List,
    mut targetlist: *mut *mut List,
    mut sortClause: *mut List,
) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut sortgrouprefs: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut skipped_sortitem: bool = false;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc2: *mut ListCell = 0 as *mut ListCell;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: distinctlist,
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
        let mut dexpr: *mut Node = (*lc).ptr_value as *mut Node;
        let mut sortgroupref: libc::c_int = 0;
        let mut tle: *mut TargetEntry = 0 as *mut TargetEntry;
        tle = findTargetlistEntrySQL92(pstate, dexpr, targetlist, EXPR_KIND_DISTINCT_ON);
        sortgroupref = assignSortGroupRef(tle, *targetlist) as libc::c_int;
        sortgrouprefs = lappend_int(sortgrouprefs, sortgroupref);
        lc__state.i += 1;
        lc__state.i;
    }
    skipped_sortitem = false;
    let mut lc__state_0: ForEachState = {
        let mut init = ForEachState {
            l: sortClause,
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
        let mut scl: *mut SortGroupClause = (*lc).ptr_value as *mut SortGroupClause;
        if list_member_int(sortgrouprefs, (*scl).tleSortGroupRef as libc::c_int) != 0 {
            if skipped_sortitem != 0 {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            } else {
                result = lappend(result, copyObjectImpl(scl as *const libc::c_void));
            }
        } else {
            skipped_sortitem = true;
        }
        lc__state_0.i += 1;
        lc__state_0.i;
    }
    let mut lc__state_1: ForBothState = {
        let mut init = ForBothState {
            l1: distinctlist,
            l2: sortgrouprefs,
            i: 0 as libc::c_int,
        };
        init
    };
    loop {
        lc = (if !(lc__state_1.l1).is_null() && lc__state_1.i < (*lc__state_1.l1).length {
            &mut *((*lc__state_1.l1).elements).offset(lc__state_1.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        lc2 = (if !(lc__state_1.l2).is_null() && lc__state_1.i < (*lc__state_1.l2).length {
            &mut *((*lc__state_1.l2).elements).offset(lc__state_1.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        if !(!lc.is_null() && !lc2.is_null()) {
            break;
        }
        let mut dexpr_0: *mut Node = (*lc).ptr_value as *mut Node;
        let mut sortgroupref_0: libc::c_int = (*lc2).int_value;
        let mut tle_0: *mut TargetEntry =
            get_sortgroupref_tle(sortgroupref_0 as Index, *targetlist);
        if !(targetIsInSortList(tle_0, 0 as libc::c_int as Oid, result) != 0) {
            if skipped_sortitem != 0 {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            result =
                addTargetToGroupList(pstate, tle_0, result, *targetlist, exprLocation(dexpr_0));
        }
        lc__state_1.i += 1;
        lc__state_1.i;
    }
    return result;
}
unsafe extern "C" fn resolve_unique_index_expr(
    mut pstate: *mut ParseState,
    mut infer: *mut InferClause,
    mut heapRel: Relation,
) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: (*infer).indexElems,
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
        let mut ielem: *mut IndexElem = (*l).ptr_value as *mut IndexElem;
        let mut pInfer: *mut InferenceElem = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_InferenceElem;
            _result
        }) as *mut InferenceElem;
        let mut parse: *mut Node = 0 as *mut Node;
        if (*ielem).ordering as libc::c_uint != SORTBY_DEFAULT as libc::c_int as libc::c_uint {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        if (*ielem).nulls_ordering as libc::c_uint
            != SORTBY_NULLS_DEFAULT as libc::c_int as libc::c_uint
        {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
        if ((*ielem).expr).is_null() {
            let mut n: *mut ColumnRef = 0 as *mut ColumnRef;
            n = ({
                let mut _result: *mut Node = 0 as *mut Node;
                (*_result).type_0 = T_ColumnRef;
                _result
            }) as *mut ColumnRef;
            (*n).fields = list_make1_impl(
                T_List,
                ListCell {
                    ptr_value: makeString((*ielem).name) as *mut libc::c_void,
                },
            );
            (*n).location = (*infer).location;
            parse = n as *mut Node;
        } else {
            parse = (*ielem).expr;
        }
        (*pInfer).expr = transformExpr(pstate, parse, EXPR_KIND_INDEX_EXPRESSION);
        if ((*ielem).collation).is_null() {
            (*pInfer).infercollid = 0 as libc::c_int as Oid;
        } else {
            (*pInfer).infercollid =
                LookupCollation(pstate, (*ielem).collation, exprLocation((*pInfer).expr));
        }
        if ((*ielem).opclass).is_null() {
            (*pInfer).inferopclass = 0 as libc::c_int as Oid;
        }
        result = lappend(result, pInfer as *mut libc::c_void);
        l__state.i += 1;
        l__state.i;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn assignSortGroupRef(
    mut tle: *mut TargetEntry,
    mut tlist: *mut List,
) -> Index {
    let mut maxRef: Index = 0;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    if (*tle).ressortgroupref != 0 {
        return (*tle).ressortgroupref;
    }
    maxRef = 0 as libc::c_int as Index;
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: tlist,
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
        let mut ref_0: Index = (*((*l).ptr_value as *mut TargetEntry)).ressortgroupref;
        if ref_0 > maxRef {
            maxRef = ref_0;
        }
        l__state.i += 1;
        l__state.i;
    }
    (*tle).ressortgroupref = maxRef.wrapping_add(1 as libc::c_int as Index);
    return (*tle).ressortgroupref;
}
#[no_mangle]
pub unsafe extern "C" fn targetIsInSortList(
    mut tle: *mut TargetEntry,
    mut sortop: Oid,
    mut sortList: *mut List,
) -> bool {
    let mut ref_0: Index = (*tle).ressortgroupref;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    if ref_0 == 0 as libc::c_int as Index {
        return false;
    }
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: sortList,
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
        let mut scl: *mut SortGroupClause = (*l).ptr_value as *mut SortGroupClause;
        if (*scl).tleSortGroupRef == ref_0
            && (sortop == 0 as libc::c_int as Oid
                || sortop == (*scl).sortop
                || sortop == get_commutator((*scl).sortop))
        {
            return true;
        }
        l__state.i += 1;
        l__state.i;
    }
    return false;
}
unsafe extern "C" fn findWindowClause(
    mut wclist: *mut List,
    mut name: *const libc::c_char,
) -> *mut WindowClause {
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: wclist,
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
        let mut wc: *mut WindowClause = (*l).ptr_value as *mut WindowClause;
        if !((*wc).name).is_null() && strcmp((*wc).name, name) == 0 as libc::c_int {
            return wc;
        }
        l__state.i += 1;
        l__state.i;
    }
    return 0 as *mut WindowClause;
}
unsafe extern "C" fn transformFrameOffset(
    mut pstate: *mut ParseState,
    mut frameOptions: libc::c_int,
    mut rangeopfamily: Oid,
    mut rangeopcintype: Oid,
    mut inRangeFunc: *mut Oid,
    mut clause: *mut Node,
) -> *mut Node {
    let mut constructName: *const libc::c_char = 0 as *const libc::c_char;
    let mut node: *mut Node = 0 as *mut Node;
    *inRangeFunc = 0 as libc::c_int as Oid;
    if clause.is_null() {
        return 0 as *mut Node;
    }
    if frameOptions & 0x4 as libc::c_int != 0 {
        node = transformExpr(pstate, clause, EXPR_KIND_WINDOW_FRAME_ROWS);
        constructName = b"ROWS\0" as *const u8 as *const libc::c_char;
    } else if frameOptions & 0x2 as libc::c_int != 0 {
        let mut nodeType: Oid = 0;
        let mut preferredType: Oid = 0;
        let mut nfuncs: libc::c_int = 0 as libc::c_int;
        let mut nmatches: libc::c_int = 0 as libc::c_int;
        let mut selectedType: Oid = 0 as libc::c_int as Oid;
        let mut selectedFunc: Oid = 0 as libc::c_int as Oid;
        let mut proclist: *mut CatCList = 0 as *mut CatCList;
        let mut i: libc::c_int = 0;
        node = transformExpr(pstate, clause, EXPR_KIND_WINDOW_FRAME_RANGE);
        nodeType = exprType(node);
        proclist = SearchSysCacheList(
            AMPROCNUM as libc::c_int,
            2 as libc::c_int,
            rangeopfamily as Datum,
            rangeopcintype as Datum,
            0 as libc::c_int as Datum,
        );
        i = 0 as libc::c_int;
        while i < (*proclist).n_members {
            let mut proctup: HeapTuple =
                &mut (**((*proclist).members).as_mut_ptr().offset(i as isize)).tuple;
            let mut procform: Form_pg_amproc = ((*proctup).t_data as *mut libc::c_char)
                .offset((*(*proctup).t_data).t_hoff as libc::c_int as isize)
                as Form_pg_amproc;
            if !((*procform).amprocnum as libc::c_int != 3 as libc::c_int) {
                nfuncs += 1;
                nfuncs;
                if !(can_coerce_type(
                    1 as libc::c_int,
                    &mut nodeType,
                    &mut (*procform).amprocrighttype,
                    COERCION_IMPLICIT,
                ) == 0)
                {
                    nmatches += 1;
                    nmatches;
                    if selectedType != preferredType {
                        selectedType = (*procform).amprocrighttype;
                        selectedFunc = (*procform).amproc;
                    }
                }
            }
            i += 1;
            i;
        }
        ReleaseCatCacheList(proclist);
        if nfuncs == 0 as libc::c_int {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        if nmatches == 0 as libc::c_int {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
        if nmatches != 1 as libc::c_int && selectedType != preferredType {
            let elevel__1: libc::c_int = 21 as libc::c_int;
            let mut __error_1: libc::c_int = 0;
            if elevel__1 >= 21 as libc::c_int {
                abort();
            }
        }
        constructName = b"RANGE\0" as *const u8 as *const libc::c_char;
        node = coerce_to_specific_type(pstate, node, selectedType, constructName);
        *inRangeFunc = selectedFunc;
    } else if frameOptions & 0x8 as libc::c_int != 0 {
        node = transformExpr(pstate, clause, EXPR_KIND_WINDOW_FRAME_GROUPS);
        constructName = b"GROUPS\0" as *const u8 as *const libc::c_char;
    } else {
        node = 0 as *mut Node;
    }
    checkExprIsVarFree(pstate, node, constructName);
    return node;
}
