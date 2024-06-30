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
//     fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
//     fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
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
//     fn palloc0(size: usize) -> *mut libc::c_void;
//     fn bms_add_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
//     fn copyObjectImpl(obj: *const libc::c_void) -> *mut libc::c_void;
//     fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
//     fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn lappend_int(list: *mut List, datum: libc::c_int) -> *mut List;
//     fn lappend_oid(list: *mut List, datum: Oid) -> *mut List;
//     fn list_truncate(list: *mut List, new_size: libc::c_int) -> *mut List;
//     fn list_delete_first(list: *mut List) -> *mut List;
//     fn list_free(list: *mut List);
//     fn list_copy(list: *const List) -> *mut List;
//     fn makeString(str: *mut libc::c_char) -> *mut Value;
//     fn check_stack_depth();
//     fn makeAlias(aliasname: *const libc::c_char, colnames: *mut List) -> *mut Alias;
//     fn makeFromExpr(fromlist: *mut List, quals: *mut Node) -> *mut FromExpr;
//     fn makeTargetEntry(
//         expr: *mut Expr,
//         resno: AttrNumber,
//         resname: *mut libc::c_char,
//         resjunk: bool,
//     ) -> *mut TargetEntry;
//     fn makeVarFromTargetEntry(varno: Index, tle: *mut TargetEntry) -> *mut Var;
//     fn makeVar(
//         varno: Index,
//         varattno: AttrNumber,
//         vartype: Oid,
//         vartypmod: i32,
//         varcollid: Oid,
//         varlevelsup: Index,
//     ) -> *mut Var;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprTypmod(expr: *const Node) -> i32;
//     fn exprCollation(expr: *const Node) -> Oid;
//     fn exprLocation(expr: *const Node) -> libc::c_int;
//     fn contain_vars_of_level(node: *mut Node, levelsup: libc::c_int) -> bool;
//     fn make_parsestate(parentParseState: *mut ParseState) -> *mut ParseState;
//     fn free_parsestate(pstate: *mut ParseState);
//     fn setup_parser_errposition_callback(
//         pcbstate: *mut ParseCallbackState,
//         pstate: *mut ParseState,
//         location: libc::c_int,
//     );
//     fn cancel_parser_errposition_callback(pcbstate: *mut ParseCallbackState);
//     fn parseCheckAggregates(pstate: *mut ParseState, qry: *mut Query);
//     fn transformFromClause(pstate: *mut ParseState, frmList: *mut List);
//     fn setTargetTable(
//         pstate: *mut ParseState,
//         relation: *mut RangeVar,
//         inh: bool,
//         alsoSource: bool,
//         requiredPerms: AclMode,
//     ) -> libc::c_int;
//     fn transformWhereClause(
//         pstate: *mut ParseState,
//         clause: *mut Node,
//         exprKind: ParseExprKind,
//         constructName: *const libc::c_char,
//     ) -> *mut Node;
//     fn transformLimitClause(
//         pstate: *mut ParseState,
//         clause: *mut Node,
//         exprKind: ParseExprKind,
//         constructName: *const libc::c_char,
//         limitOption: LimitOption,
//     ) -> *mut Node;
//     fn transformGroupClause(
//         pstate: *mut ParseState,
//         grouplist: *mut List,
//         groupingSets: *mut *mut List,
//         targetlist: *mut *mut List,
//         sortClause: *mut List,
//         exprKind: ParseExprKind,
//         useSQL99: bool,
//     ) -> *mut List;
//     fn transformSortClause(
//         pstate: *mut ParseState,
//         orderlist: *mut List,
//         targetlist: *mut *mut List,
//         exprKind: ParseExprKind,
//         useSQL99: bool,
//     ) -> *mut List;
//     fn transformWindowDefinitions(
//         pstate: *mut ParseState,
//         windowdefs: *mut List,
//         targetlist: *mut *mut List,
//     ) -> *mut List;
//     fn transformDistinctClause(
//         pstate: *mut ParseState,
//         targetlist: *mut *mut List,
//         sortClause: *mut List,
//         is_agg: bool,
//     ) -> *mut List;
//     fn transformDistinctOnClause(
//         pstate: *mut ParseState,
//         distinctlist: *mut List,
//         targetlist: *mut *mut List,
//         sortClause: *mut List,
//     ) -> *mut List;
//     fn transformOnConflictArbiter(
//         pstate: *mut ParseState,
//         onConflictClause: *mut OnConflictClause,
//         arbiterExpr: *mut *mut List,
//         arbiterWhere: *mut *mut Node,
//         constraint: *mut Oid,
//     );
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
//     fn assign_query_collations(pstate: *mut ParseState, query: *mut Query);
//     fn assign_list_collations(pstate: *mut ParseState, exprs: *mut List);
//     fn assign_expr_collations(pstate: *mut ParseState, expr: *mut Node);
//     fn select_common_collation(
//         pstate: *mut ParseState,
//         exprs: *mut List,
//         none_ok: bool,
//     ) -> Oid;
//     fn transformWithClause(
//         pstate: *mut ParseState,
//         withClause: *mut WithClause,
//     ) -> *mut List;
//     fn analyzeCTETargetList(
//         pstate: *mut ParseState,
//         cte: *mut CommonTableExpr,
//         tlist: *mut List,
//     );
//     fn transformExpr(
//         pstate: *mut ParseState,
//         expr: *mut Node,
//         exprKind: ParseExprKind,
//     ) -> *mut Node;
//     fn ParseFuncOrColumn(
//         pstate: *mut ParseState,
//         funcname: *mut List,
//         fargs: *mut List,
//         last_srf: *mut Node,
//         fn_0: *mut FuncCall,
//         proc_call: bool,
//         location: libc::c_int,
//     ) -> *mut Node;
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
//     fn parse_fixed_parameters(
//         pstate: *mut ParseState,
//         paramTypes: *mut Oid,
//         numParams: libc::c_int,
//     );
//     fn parse_variable_parameters(
//         pstate: *mut ParseState,
//         paramTypes: *mut *mut Oid,
//         numParams: *mut libc::c_int,
//     );
//     fn check_variable_parameters(pstate: *mut ParseState, query: *mut Query);
//     fn query_contains_extern_params(query: *mut Query) -> bool;
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
//     fn addRangeTableEntryForValues(
//         pstate: *mut ParseState,
//         exprs: *mut List,
//         coltypes: *mut List,
//         coltypmods: *mut List,
//         colcollations: *mut List,
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
//     fn addNSItemToQuery(
//         pstate: *mut ParseState,
//         nsitem: *mut ParseNamespaceItem,
//         addToJoinList: bool,
//         addToRelNameSpace: bool,
//         addToVarNameSpace: bool,
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
//     fn isQueryUsingTempRelation(query: *mut Query) -> bool;
//     fn transformTargetList(
//         pstate: *mut ParseState,
//         targetlist: *mut List,
//         exprKind: ParseExprKind,
//     ) -> *mut List;
//     fn transformExpressionList(
//         pstate: *mut ParseState,
//         exprlist: *mut List,
//         exprKind: ParseExprKind,
//         allowDefault: bool,
//     ) -> *mut List;
//     fn resolveTargetListUnknowns(pstate: *mut ParseState, targetlist: *mut List);
//     fn markTargetListOrigins(pstate: *mut ParseState, targetlist: *mut List);
//     fn transformAssignedExpr(
//         pstate: *mut ParseState,
//         expr: *mut Expr,
//         exprKind: ParseExprKind,
//         colname: *const libc::c_char,
//         attrno: libc::c_int,
//         indirection: *mut List,
//         location: libc::c_int,
//     ) -> *mut Expr;
//     fn updateTargetListEntry(
//         pstate: *mut ParseState,
//         tle: *mut TargetEntry,
//         colname: *mut libc::c_char,
//         attrno: libc::c_int,
//         indirection: *mut List,
//         location: libc::c_int,
//     );
//     fn transformAssignmentIndirection(
//         pstate: *mut ParseState,
//         basenode: *mut Node,
//         targetName: *const libc::c_char,
//         targetIsSubscripting: bool,
//         targetTypeId: Oid,
//         targetTypMod: i32,
//         targetCollation: Oid,
//         indirection: *mut List,
//         indirection_cell: *mut ListCell,
//         rhs: *mut Node,
//         ccontext: CoercionContext,
//         location: libc::c_int,
//     ) -> *mut Node;
//     fn checkInsertTargets(
//         pstate: *mut ParseState,
//         cols: *mut List,
//         attrnos: *mut *mut List,
//     ) -> *mut List;
//     fn get_parse_rowmark(qry: *mut Query, rtindex: Index) -> *mut RowMarkClause;
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
pub type Index = libc::c_uint;
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
pub struct ErrorContextCallback {
    pub previous: *mut ErrorContextCallback,
    pub callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
pub type MemoryContext = *mut MemoryContextData;
pub type Datum = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct List {
    pub type_0: NodeTag,
    pub length: libc::c_int,
    pub max_length: libc::c_int,
    pub elements: *mut ListCell,
    pub initial_elements: [ListCell; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ListCell {
    pub ptr_value: *mut libc::c_void,
    pub int_value: libc::c_int,
    pub oid_value: Oid,
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
pub type bitmapword = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bitmapset {
    pub nwords: libc::c_int,
    pub words: [bitmapword; 0],
}
pub type LockClauseStrength = libc::c_uint;
pub const LCS_FORUPDATE: LockClauseStrength = 4;
pub const LCS_FORNOKEYUPDATE: LockClauseStrength = 3;
pub const LCS_FORSHARE: LockClauseStrength = 2;
pub const LCS_FORKEYSHARE: LockClauseStrength = 1;
pub const LCS_NONE: LockClauseStrength = 0;
pub type LockWaitPolicy = libc::c_uint;
pub const LockWaitError: LockWaitPolicy = 2;
pub const LockWaitSkip: LockWaitPolicy = 1;
pub const LockWaitBlock: LockWaitPolicy = 0;
pub type AttrNumber = i16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub type_0: NodeTag,
}
pub type CmdType = libc::c_uint;
pub const CMD_NOTHING: CmdType = 6;
pub const CMD_UTILITY: CmdType = 5;
pub const CMD_DELETE: CmdType = 4;
pub const CMD_INSERT: CmdType = 3;
pub const CMD_UPDATE: CmdType = 2;
pub const CMD_SELECT: CmdType = 1;
pub const CMD_UNKNOWN: CmdType = 0;
pub type JoinType = libc::c_uint;
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
pub struct ForThreeState {
    pub l1: *const List,
    pub l2: *const List,
    pub l3: *const List,
    pub i: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ForFourState {
    pub l1: *const List,
    pub l2: *const List,
    pub l3: *const List,
    pub l4: *const List,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr {
    pub type_0: NodeTag,
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
pub struct FieldStore {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub newvals: *mut List,
    pub fieldnums: *mut List,
    pub resulttype: Oid,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TargetEntry {
    pub xpr: Expr,
    pub expr: *mut Expr,
    pub resno: AttrNumber,
    pub resname: *mut libc::c_char,
    pub ressortgroupref: Index,
    pub resorigtbl: Oid,
    pub resorigcol: AttrNumber,
    pub resjunk: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RangeTblRef {
    pub type_0: NodeTag,
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
pub struct LockingClause {
    pub type_0: NodeTag,
    pub lockedRels: *mut List,
    pub strength: LockClauseStrength,
    pub waitPolicy: LockWaitPolicy,
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
pub struct SortGroupClause {
    pub type_0: NodeTag,
    pub tleSortGroupRef: Index,
    pub eqop: Oid,
    pub sortop: Oid,
    pub nulls_first: bool,
    pub hashable: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RowMarkClause {
    pub type_0: NodeTag,
    pub rti: Index,
    pub strength: LockClauseStrength,
    pub waitPolicy: LockWaitPolicy,
    pub pushedDown: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WithClause {
    pub type_0: NodeTag,
    pub ctes: *mut List,
    pub recursive: bool,
    pub location: libc::c_int,
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
pub struct RawStmt {
    pub type_0: NodeTag,
    pub stmt: *mut Node,
    pub stmt_location: libc::c_int,
    pub stmt_len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InsertStmt {
    pub type_0: NodeTag,
    pub relation: *mut RangeVar,
    pub cols: *mut List,
    pub selectStmt: *mut Node,
    pub onConflictClause: *mut OnConflictClause,
    pub returningList: *mut List,
    pub withClause: *mut WithClause,
    pub override_0: OverridingKind,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeleteStmt {
    pub type_0: NodeTag,
    pub relation: *mut RangeVar,
    pub usingClause: *mut List,
    pub whereClause: *mut Node,
    pub returningList: *mut List,
    pub withClause: *mut WithClause,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpdateStmt {
    pub type_0: NodeTag,
    pub relation: *mut RangeVar,
    pub targetList: *mut List,
    pub whereClause: *mut Node,
    pub fromClause: *mut List,
    pub returningList: *mut List,
    pub withClause: *mut WithClause,
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
pub struct PLAssignStmt {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub indirection: *mut List,
    pub nnames: libc::c_int,
    pub val: *mut SelectStmt,
    pub location: libc::c_int,
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
pub struct DeclareCursorStmt {
    pub type_0: NodeTag,
    pub portalname: *mut libc::c_char,
    pub options: libc::c_int,
    pub query: *mut Node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallStmt {
    pub type_0: NodeTag,
    pub funccall: *mut FuncCall,
    pub funcexpr: *mut FuncExpr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExplainStmt {
    pub type_0: NodeTag,
    pub query: *mut Node,
    pub options: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreateTableAsStmt {
    pub type_0: NodeTag,
    pub query: *mut Node,
    pub into: *mut IntoClause,
    pub objtype: ObjectType,
    pub is_select_into: bool,
    pub if_not_exists: bool,
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
pub struct RelationData {
    pub rd_node: RelFileNode,
    pub rd_smgr: *mut SMgrRelationData,
    pub rd_refcnt: libc::c_int,
    pub rd_backend: BackendId,
    pub rd_islocaltemp: bool,
    pub rd_isnailed: bool,
    pub rd_isvalid: bool,
    pub rd_indexvalid: bool,
    pub rd_statvalid: bool,
    pub rd_version_checked: bool,
    pub rd_createSubid: SubTransactionId,
    pub rd_newRelfilenodeSubid: SubTransactionId,
    pub rd_firstRelfilenodeSubid: SubTransactionId,
    pub rd_droppedSubid: SubTransactionId,
    pub rd_rel: Form_pg_class,
    pub rd_att: TupleDesc,
    pub rd_id: Oid,
    pub rd_lockInfo: LockInfoData,
    pub rd_rules: *mut RuleLock,
    pub rd_rulescxt: MemoryContext,
    pub trigdesc: *mut TriggerDesc,
    pub rd_rsdesc: *mut RowSecurityDesc,
    pub rd_fkeylist: *mut List,
    pub rd_fkeyvalid: bool,
    pub rd_partkey: PartitionKey,
    pub rd_partkeycxt: MemoryContext,
    pub rd_partdesc: PartitionDesc,
    pub rd_pdcxt: MemoryContext,
    pub rd_partcheck: *mut List,
    pub rd_partcheckvalid: bool,
    pub rd_partcheckcxt: MemoryContext,
    pub rd_indexlist: *mut List,
    pub rd_pkindex: Oid,
    pub rd_replidindex: Oid,
    pub rd_statlist: *mut List,
    pub rd_indexattr: *mut Bitmapset,
    pub rd_keyattr: *mut Bitmapset,
    pub rd_pkattr: *mut Bitmapset,
    pub rd_idattr: *mut Bitmapset,
    pub rd_pubactions: *mut PublicationActions,
    pub rd_options: *mut bytea,
    pub rd_amhandler: Oid,
    pub rd_tableam: *const TableAmRoutine,
    pub rd_index: Form_pg_index,
    pub rd_indextuple: *mut HeapTupleData,
    pub rd_indexcxt: MemoryContext,
    pub rd_indam: *mut IndexAmRoutine,
    pub rd_opfamily: *mut Oid,
    pub rd_opcintype: *mut Oid,
    pub rd_support: *mut RegProcedure,
    pub rd_supportinfo: *mut FmgrInfo,
    pub rd_indoption: *mut i16,
    pub rd_indexprs: *mut List,
    pub rd_indpred: *mut List,
    pub rd_exclops: *mut Oid,
    pub rd_exclprocs: *mut Oid,
    pub rd_exclstrats: *mut u16,
    pub rd_indcollation: *mut Oid,
    pub rd_opcoptions: *mut *mut bytea,
    pub rd_amcache: *mut libc::c_void,
    pub rd_fdwroutine: *mut FdwRoutine,
    pub rd_toastoid: Oid,
    pub pgstat_info: *mut PgStat_TableStatus,
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
pub type Relation = *mut RelationData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseState {
    pub parentParseState: *mut ParseState,
    pub p_sourcetext: *const libc::c_char,
    pub p_rtable: *mut List,
    pub p_joinexprs: *mut List,
    pub p_joinlist: *mut List,
    pub p_namespace: *mut List,
    pub p_lateral_active: bool,
    pub p_ctenamespace: *mut List,
    pub p_future_ctes: *mut List,
    pub p_parent_cte: *mut CommonTableExpr,
    pub p_target_relation: Relation,
    pub p_target_nsitem: *mut ParseNamespaceItem,
    pub p_is_insert: bool,
    pub p_windowdefs: *mut List,
    pub p_expr_kind: ParseExprKind,
    pub p_next_resno: libc::c_int,
    pub p_multiassign_exprs: *mut List,
    pub p_locking_clause: *mut List,
    pub p_locked_from_parent: bool,
    pub p_resolve_unknowns: bool,
    pub p_queryEnv: *mut QueryEnvironment,
    pub p_hasAggs: bool,
    pub p_hasWindowFuncs: bool,
    pub p_hasTargetSRFs: bool,
    pub p_hasSubLinks: bool,
    pub p_hasModifyingCTE: bool,
    pub p_last_srf: *mut Node,
    pub p_pre_columnref_hook: PreParseColumnRefHook,
    pub p_post_columnref_hook: PostParseColumnRefHook,
    pub p_paramref_hook: ParseParamRefHook,
    pub p_coerce_param_hook: CoerceParamHook,
    pub p_ref_hook_state: *mut libc::c_void,
}
pub type CoerceParamHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut Param, Oid, i32, libc::c_int) -> *mut Node>;
pub type ParseParamRefHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut ParamRef) -> *mut Node>;
pub type PostParseColumnRefHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut ColumnRef, *mut Node) -> *mut Node>;
pub type PreParseColumnRefHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut ColumnRef) -> *mut Node>;
pub type ParseExprKind = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseNamespaceItem {
    pub p_rte: *mut RangeTblEntry,
    pub p_rtindex: libc::c_int,
    pub p_nscolumns: *mut ParseNamespaceColumn,
    pub p_rel_visible: bool,
    pub p_cols_visible: bool,
    pub p_lateral_only: bool,
    pub p_lateral_ok: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseNamespaceColumn {
    pub p_varno: Index,
    pub p_varattno: AttrNumber,
    pub p_vartype: Oid,
    pub p_vartypmod: i32,
    pub p_varcollid: Oid,
    pub p_varnosyn: Index,
    pub p_varattnosyn: AttrNumber,
    pub p_dontexpand: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseCallbackState {
    pub pstate: *mut ParseState,
    pub location: libc::c_int,
    pub errcallback: ErrorContextCallback,
}
pub type post_parse_analyze_hook_type =
    Option<unsafe extern "C" fn(*mut ParseState, *mut Query) -> ()>;
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
unsafe extern "C" fn list_nth(mut list: *const List, mut n: libc::c_int) -> *mut libc::c_void {
    return (*list_nth_cell(list, n)).ptr_value;
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
pub static mut post_parse_analyze_hook: post_parse_analyze_hook_type = None;
#[no_mangle]
pub unsafe extern "C" fn parse_analyze(
    mut parseTree: *mut RawStmt,
    mut sourceText: *const libc::c_char,
    mut paramTypes: *mut Oid,
    mut numParams: libc::c_int,
    mut queryEnv: *mut QueryEnvironment,
) -> *mut Query {
    let mut pstate: *mut ParseState = make_parsestate(0 as *mut ParseState);
    let mut query: *mut Query = 0 as *mut Query;
    (*pstate).p_sourcetext = sourceText;
    if numParams > 0 as libc::c_int {
        parse_fixed_parameters(pstate, paramTypes, numParams);
    }
    (*pstate).p_queryEnv = queryEnv;
    query = transformTopLevelStmt(pstate, parseTree);
    if post_parse_analyze_hook.is_some() {
        (Some(post_parse_analyze_hook.expect("non-null function pointer")))
            .expect("non-null function pointer")(pstate, query);
    }
    free_parsestate(pstate);
    return query;
}
#[no_mangle]
pub unsafe extern "C" fn parse_analyze_varparams(
    mut parseTree: *mut RawStmt,
    mut sourceText: *const libc::c_char,
    mut paramTypes: *mut *mut Oid,
    mut numParams: *mut libc::c_int,
) -> *mut Query {
    let mut pstate: *mut ParseState = make_parsestate(0 as *mut ParseState);
    let mut query: *mut Query = 0 as *mut Query;
    (*pstate).p_sourcetext = sourceText;
    parse_variable_parameters(pstate, paramTypes, numParams);
    query = transformTopLevelStmt(pstate, parseTree);
    check_variable_parameters(pstate, query);
    if post_parse_analyze_hook.is_some() {
        (Some(post_parse_analyze_hook.expect("non-null function pointer")))
            .expect("non-null function pointer")(pstate, query);
    }
    free_parsestate(pstate);
    return query;
}
#[no_mangle]
pub unsafe extern "C" fn parse_sub_analyze(
    mut parseTree: *mut Node,
    mut parentParseState: *mut ParseState,
    mut parentCTE: *mut CommonTableExpr,
    mut locked_from_parent: bool,
    mut resolve_unknowns: bool,
) -> *mut Query {
    let mut pstate: *mut ParseState = make_parsestate(parentParseState);
    let mut query: *mut Query = 0 as *mut Query;
    (*pstate).p_parent_cte = parentCTE;
    (*pstate).p_locked_from_parent = locked_from_parent;
    (*pstate).p_resolve_unknowns = resolve_unknowns;
    query = transformStmt(pstate, parseTree);
    free_parsestate(pstate);
    return query;
}
#[no_mangle]
pub unsafe extern "C" fn transformTopLevelStmt(
    mut pstate: *mut ParseState,
    mut parseTree: *mut RawStmt,
) -> *mut Query {
    let mut result: *mut Query = 0 as *mut Query;
    result = transformOptionalSelectInto(pstate, (*parseTree).stmt);
    (*result).stmt_location = (*parseTree).stmt_location;
    (*result).stmt_len = (*parseTree).stmt_len;
    return result;
}
unsafe extern "C" fn transformOptionalSelectInto(
    mut pstate: *mut ParseState,
    mut parseTree: *mut Node,
) -> *mut Query {
    if (*(parseTree as *const Node)).type_0 as libc::c_uint
        == T_SelectStmt as libc::c_int as libc::c_uint
    {
        let mut stmt: *mut SelectStmt = parseTree as *mut SelectStmt;
        while !stmt.is_null()
            && (*stmt).op as libc::c_uint != SETOP_NONE as libc::c_int as libc::c_uint
        {
            stmt = (*stmt).larg;
        }
        if !((*stmt).intoClause).is_null() {
            let mut ctas: *mut CreateTableAsStmt = ({
                let mut _result: *mut Node = 0 as *mut Node;
                (*_result).type_0 = T_CreateTableAsStmt;
                _result
            }) as *mut CreateTableAsStmt;
            (*ctas).query = parseTree;
            (*ctas).into = (*stmt).intoClause;
            (*ctas).objtype = OBJECT_TABLE;
            (*ctas).is_select_into = true;
            (*stmt).intoClause = 0 as *mut IntoClause;
            parseTree = ctas as *mut Node;
        }
    }
    return transformStmt(pstate, parseTree);
}
#[no_mangle]
pub unsafe extern "C" fn transformStmt(
    mut pstate: *mut ParseState,
    mut parseTree: *mut Node,
) -> *mut Query {
    let mut result: *mut Query = 0 as *mut Query;
    match (*(parseTree as *const Node)).type_0 as libc::c_uint {
        234 => {
            result = transformInsertStmt(pstate, parseTree as *mut InsertStmt);
        }
        235 => {
            result = transformDeleteStmt(pstate, parseTree as *mut DeleteStmt);
        }
        236 => {
            result = transformUpdateStmt(pstate, parseTree as *mut UpdateStmt);
        }
        237 => {
            let mut n: *mut SelectStmt = parseTree as *mut SelectStmt;
            if !((*n).valuesLists).is_null() {
                result = transformValuesClause(pstate, n);
            } else if (*n).op as libc::c_uint == SETOP_NONE as libc::c_int as libc::c_uint {
                result = transformSelectStmt(pstate, n);
            } else {
                result = transformSetOperationStmt(pstate, n);
            }
        }
        238 => {
            result = transformPLAssignStmt(pstate, parseTree as *mut PLAssignStmt);
        }
        299 => {
            result = transformDeclareCursorStmt(pstate, parseTree as *mut DeclareCursorStmt);
        }
        271 => {
            result = transformExplainStmt(pstate, parseTree as *mut ExplainStmt);
        }
        272 => {
            result = transformCreateTableAsStmt(pstate, parseTree as *mut CreateTableAsStmt);
        }
        346 => {
            result = transformCallStmt(pstate, parseTree as *mut CallStmt);
        }
        _ => {
            result = ({
                let mut _result: *mut Node = 0 as *mut Node;
                (*_result).type_0 = T_Query;
                _result
            }) as *mut Query;
            (*result).commandType = CMD_UTILITY;
            (*result).utilityStmt = parseTree;
        }
    }
    (*result).querySource = QSRC_ORIGINAL;
    (*result).canSetTag = true;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn analyze_requires_snapshot(mut parseTree: *mut RawStmt) -> bool {
    let mut result: bool = false;
    match (*((*parseTree).stmt as *const Node)).type_0 as libc::c_uint {
        234 | 235 | 236 | 237 | 238 => {
            result = true;
        }
        299 | 271 | 272 => {
            result = true;
        }
        _ => {
            result = false;
        }
    }
    return result;
}
unsafe extern "C" fn transformDeleteStmt(
    mut pstate: *mut ParseState,
    mut stmt: *mut DeleteStmt,
) -> *mut Query {
    let mut qry: *mut Query = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_Query;
        _result
    }) as *mut Query;
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    let mut qual: *mut Node = 0 as *mut Node;
    (*qry).commandType = CMD_DELETE;
    if !((*stmt).withClause).is_null() {
        (*qry).hasRecursive = (*(*stmt).withClause).recursive;
        (*qry).cteList = transformWithClause(pstate, (*stmt).withClause);
        (*qry).hasModifyingCTE = (*pstate).p_hasModifyingCTE;
    }
    (*qry).resultRelation = setTargetTable(
        pstate,
        (*stmt).relation,
        (*(*stmt).relation).inh,
        true,
        ((1 as libc::c_int) << 3 as libc::c_int) as AclMode,
    );
    nsitem = (*pstate).p_target_nsitem;
    (*qry).distinctClause = 0 as *mut libc::c_void as *mut List;
    (*nsitem).p_lateral_only = true;
    (*nsitem).p_lateral_ok = false;
    transformFromClause(pstate, (*stmt).usingClause);
    (*nsitem).p_lateral_only = false;
    (*nsitem).p_lateral_ok = true;
    qual = transformWhereClause(
        pstate,
        (*stmt).whereClause,
        EXPR_KIND_WHERE,
        b"WHERE\0" as *const u8 as *const libc::c_char,
    );
    (*qry).returningList = transformReturningList(pstate, (*stmt).returningList);
    (*qry).rtable = (*pstate).p_rtable;
    (*qry).jointree = makeFromExpr((*pstate).p_joinlist, qual);
    (*qry).hasSubLinks = (*pstate).p_hasSubLinks;
    (*qry).hasWindowFuncs = (*pstate).p_hasWindowFuncs;
    (*qry).hasTargetSRFs = (*pstate).p_hasTargetSRFs;
    (*qry).hasAggs = (*pstate).p_hasAggs;
    assign_query_collations(pstate, qry);
    if (*pstate).p_hasAggs != 0 {
        parseCheckAggregates(pstate, qry);
    }
    return qry;
}
unsafe extern "C" fn transformInsertRow(
    mut pstate: *mut ParseState,
    mut exprlist: *mut List,
    mut stmtcols: *mut List,
    mut icolumns: *mut List,
    mut attrnos: *mut List,
    mut strip_indirection: bool,
) -> *mut List {
    let mut result: *mut List = 0 as *mut List;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut icols: *mut ListCell = 0 as *mut ListCell;
    let mut attnos: *mut ListCell = 0 as *mut ListCell;
    if list_length(exprlist) > list_length(icolumns) {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if !stmtcols.is_null() && list_length(exprlist) < list_length(icolumns) {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    result = 0 as *mut libc::c_void as *mut List;
    let mut lc__state: ForThreeState = {
        let mut init = ForThreeState {
            l1: exprlist,
            l2: icolumns,
            l3: attrnos,
            i: 0 as libc::c_int,
        };
        init
    };
    loop {
        lc = (if !(lc__state.l1).is_null() && lc__state.i < (*lc__state.l1).length {
            &mut *((*lc__state.l1).elements).offset(lc__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        icols = (if !(lc__state.l2).is_null() && lc__state.i < (*lc__state.l2).length {
            &mut *((*lc__state.l2).elements).offset(lc__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        attnos = (if !(lc__state.l3).is_null() && lc__state.i < (*lc__state.l3).length {
            &mut *((*lc__state.l3).elements).offset(lc__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        if !(!lc.is_null() && !icols.is_null() && !attnos.is_null()) {
            break;
        }
        let mut expr: *mut Expr = (*lc).ptr_value as *mut Expr;
        let mut col: *mut ResTarget = (*icols).ptr_value as *mut ResTarget;
        let mut attno: libc::c_int = (*attnos).int_value;
        expr = transformAssignedExpr(
            pstate,
            expr,
            EXPR_KIND_INSERT_TARGET,
            (*col).name,
            attno,
            (*col).indirection,
            (*col).location,
        );
        if strip_indirection != 0 {
            while !expr.is_null() {
                if (*(expr as *const Node)).type_0 as libc::c_uint
                    == T_FieldStore as libc::c_int as libc::c_uint
                {
                    let mut fstore: *mut FieldStore = expr as *mut FieldStore;
                    expr = (*list_nth_cell((*fstore).newvals, 0 as libc::c_int)).ptr_value
                        as *mut Expr;
                } else {
                    if !((*(expr as *const Node)).type_0 as libc::c_uint
                        == T_SubscriptingRef as libc::c_int as libc::c_uint)
                    {
                        break;
                    }
                    let mut sbsref: *mut SubscriptingRef = expr as *mut SubscriptingRef;
                    if ((*sbsref).refassgnexpr).is_null() {
                        break;
                    }
                    expr = (*sbsref).refassgnexpr;
                }
            }
        }
        result = lappend(result, expr as *mut libc::c_void);
        lc__state.i += 1;
        lc__state.i;
    }
    return result;
}
unsafe extern "C" fn transformOnConflictClause(
    mut pstate: *mut ParseState,
    mut onConflictClause: *mut OnConflictClause,
) -> *mut OnConflictExpr {
    let mut arbiterElems: *mut List = 0 as *mut List;
    let mut arbiterWhere: *mut Node = 0 as *mut Node;
    let mut arbiterConstraint: Oid = 0;
    let mut onConflictSet: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut onConflictWhere: *mut Node = 0 as *mut Node;
    let mut exclRelIndex: libc::c_int = 0 as libc::c_int;
    let mut exclRelTlist: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut result: *mut OnConflictExpr = 0 as *mut OnConflictExpr;
    transformOnConflictArbiter(
        pstate,
        onConflictClause,
        &mut arbiterElems,
        &mut arbiterWhere,
        &mut arbiterConstraint,
    );
    if (*onConflictClause).action as libc::c_uint
        == ONCONFLICT_UPDATE as libc::c_int as libc::c_uint
    {
        let mut targetrel: Relation = (*pstate).p_target_relation;
        let mut exclNSItem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
        let mut exclRte: *mut RangeTblEntry = 0 as *mut RangeTblEntry;
        (*pstate).p_is_insert = false;
        exclNSItem = addRangeTableEntryForRelation(
            pstate,
            targetrel,
            3 as libc::c_int,
            makeAlias(
                b"excluded\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *mut List,
            ),
            false,
            false,
        );
        exclRte = (*exclNSItem).p_rte;
        exclRelIndex = (*exclNSItem).p_rtindex;
        (*exclRte).requiredPerms = 0 as libc::c_int as AclMode;
        exclRelTlist = BuildOnConflictExcludedTargetlist(targetrel, exclRelIndex as Index);
        addNSItemToQuery(pstate, exclNSItem, false, true, true);
        addNSItemToQuery(pstate, (*pstate).p_target_nsitem, false, true, true);
        onConflictSet = transformUpdateTargetList(pstate, (*onConflictClause).targetList);
        onConflictWhere = transformWhereClause(
            pstate,
            (*onConflictClause).whereClause,
            EXPR_KIND_WHERE,
            b"WHERE\0" as *const u8 as *const libc::c_char,
        );
    }
    result = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_OnConflictExpr;
        _result
    }) as *mut OnConflictExpr;
    (*result).action = (*onConflictClause).action;
    (*result).arbiterElems = arbiterElems;
    (*result).arbiterWhere = arbiterWhere;
    (*result).constraint = arbiterConstraint;
    (*result).onConflictSet = onConflictSet;
    (*result).onConflictWhere = onConflictWhere;
    (*result).exclRelIndex = exclRelIndex;
    (*result).exclRelTlist = exclRelTlist;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn BuildOnConflictExcludedTargetlist(
    mut targetrel: Relation,
    mut exclRelIndex: Index,
) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut attno: libc::c_int = 0;
    let mut var: *mut Var = 0 as *mut Var;
    let mut te: *mut TargetEntry = 0 as *mut TargetEntry;
    attno = 0 as libc::c_int;
    while attno < (*(*targetrel).rd_rel).relnatts as libc::c_int {
        let mut attr: Form_pg_attribute = &mut *((*(*targetrel).rd_att).attrs)
            .as_mut_ptr()
            .offset(attno as isize)
            as *mut FormData_pg_attribute;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        if (*attr).attisdropped != 0 {
            name = 0 as *mut libc::c_char;
        } else {
            var = makeVar(
                exclRelIndex,
                (attno + 1 as libc::c_int) as AttrNumber,
                (*attr).atttypid,
                (*attr).atttypmod,
                (*attr).attcollation,
                0 as libc::c_int as Index,
            );
            name = pstrdup(((*attr).attname.data).as_mut_ptr());
        }
        te = makeTargetEntry(
            var as *mut Expr,
            (attno + 1 as libc::c_int) as AttrNumber,
            name,
            false,
        );
        result = lappend(result, te as *mut libc::c_void);
        attno += 1;
        attno;
    }
    var = makeVar(
        exclRelIndex,
        0 as libc::c_int as AttrNumber,
        (*(*targetrel).rd_rel).reltype,
        -(1 as libc::c_int),
        0 as libc::c_int as Oid,
        0 as libc::c_int as Index,
    );
    te = makeTargetEntry(
        var as *mut Expr,
        0 as libc::c_int as AttrNumber,
        0 as *mut libc::c_char,
        true,
    );
    result = lappend(result, te as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn transformSelectStmt(
    mut pstate: *mut ParseState,
    mut stmt: *mut SelectStmt,
) -> *mut Query {
    let mut qry: *mut Query = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_Query;
        _result
    }) as *mut Query;
    let mut qual: *mut Node = 0 as *mut Node;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    (*qry).commandType = CMD_SELECT;
    if !((*stmt).withClause).is_null() {
        (*qry).hasRecursive = (*(*stmt).withClause).recursive;
        (*qry).cteList = transformWithClause(pstate, (*stmt).withClause);
        (*qry).hasModifyingCTE = (*pstate).p_hasModifyingCTE;
    }
    if !((*stmt).intoClause).is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    (*pstate).p_locking_clause = (*stmt).lockingClause;
    (*pstate).p_windowdefs = (*stmt).windowClause;
    transformFromClause(pstate, (*stmt).fromClause);
    (*qry).targetList = transformTargetList(pstate, (*stmt).targetList, EXPR_KIND_SELECT_TARGET);
    markTargetListOrigins(pstate, (*qry).targetList);
    qual = transformWhereClause(
        pstate,
        (*stmt).whereClause,
        EXPR_KIND_WHERE,
        b"WHERE\0" as *const u8 as *const libc::c_char,
    );
    (*qry).havingQual = transformWhereClause(
        pstate,
        (*stmt).havingClause,
        EXPR_KIND_HAVING,
        b"HAVING\0" as *const u8 as *const libc::c_char,
    );
    (*qry).sortClause = transformSortClause(
        pstate,
        (*stmt).sortClause,
        &mut (*qry).targetList,
        EXPR_KIND_ORDER_BY,
        false,
    );
    (*qry).groupClause = transformGroupClause(
        pstate,
        (*stmt).groupClause,
        &mut (*qry).groupingSets,
        &mut (*qry).targetList,
        (*qry).sortClause,
        EXPR_KIND_GROUP_BY,
        false,
    );
    if ((*stmt).distinctClause).is_null() {
        (*qry).distinctClause = 0 as *mut libc::c_void as *mut List;
        (*qry).hasDistinctOn = false;
    } else if ((*list_nth_cell((*stmt).distinctClause, 0 as libc::c_int)).ptr_value).is_null() {
        (*qry).distinctClause =
            transformDistinctClause(pstate, &mut (*qry).targetList, (*qry).sortClause, false);
        (*qry).hasDistinctOn = false;
    } else {
        (*qry).distinctClause = transformDistinctOnClause(
            pstate,
            (*stmt).distinctClause,
            &mut (*qry).targetList,
            (*qry).sortClause,
        );
        (*qry).hasDistinctOn = true;
    }
    (*qry).limitOffset = transformLimitClause(
        pstate,
        (*stmt).limitOffset,
        EXPR_KIND_OFFSET,
        b"OFFSET\0" as *const u8 as *const libc::c_char,
        (*stmt).limitOption,
    );
    (*qry).limitCount = transformLimitClause(
        pstate,
        (*stmt).limitCount,
        EXPR_KIND_LIMIT,
        b"LIMIT\0" as *const u8 as *const libc::c_char,
        (*stmt).limitOption,
    );
    (*qry).limitOption = (*stmt).limitOption;
    (*qry).windowClause =
        transformWindowDefinitions(pstate, (*pstate).p_windowdefs, &mut (*qry).targetList);
    if (*pstate).p_resolve_unknowns != 0 {
        resolveTargetListUnknowns(pstate, (*qry).targetList);
    }
    (*qry).rtable = (*pstate).p_rtable;
    (*qry).jointree = makeFromExpr((*pstate).p_joinlist, qual);
    (*qry).hasSubLinks = (*pstate).p_hasSubLinks;
    (*qry).hasWindowFuncs = (*pstate).p_hasWindowFuncs;
    (*qry).hasTargetSRFs = (*pstate).p_hasTargetSRFs;
    (*qry).hasAggs = (*pstate).p_hasAggs;
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: (*stmt).lockingClause,
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
        transformLockingClause(pstate, qry, (*l).ptr_value as *mut LockingClause, false);
        l__state.i += 1;
        l__state.i;
    }
    assign_query_collations(pstate, qry);
    if (*pstate).p_hasAggs as libc::c_int != 0
        || !((*qry).groupClause).is_null()
        || !((*qry).groupingSets).is_null()
        || !((*qry).havingQual).is_null()
    {
        parseCheckAggregates(pstate, qry);
    }
    return qry;
}
unsafe extern "C" fn transformValuesClause(
    mut pstate: *mut ParseState,
    mut stmt: *mut SelectStmt,
) -> *mut Query {
    let mut qry: *mut Query = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_Query;
        _result
    }) as *mut Query;
    let mut exprsLists: *mut List = 0 as *mut List;
    let mut coltypes: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut coltypmods: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut colcollations: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut colexprs: *mut *mut List = 0 as *mut *mut List;
    let mut sublist_length: libc::c_int = -(1 as libc::c_int);
    let mut lateral: bool = false;
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc2: *mut ListCell = 0 as *mut ListCell;
    let mut i: libc::c_int = 0;
    (*qry).commandType = CMD_SELECT;
    if !((*stmt).withClause).is_null() {
        (*qry).hasRecursive = (*(*stmt).withClause).recursive;
        (*qry).cteList = transformWithClause(pstate, (*stmt).withClause);
        (*qry).hasModifyingCTE = (*pstate).p_hasModifyingCTE;
    }
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*stmt).valuesLists,
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
        let mut sublist: *mut List = (*lc).ptr_value as *mut List;
        sublist = transformExpressionList(pstate, sublist, EXPR_KIND_VALUES, false);
        if sublist_length < 0 as libc::c_int {
            sublist_length = list_length(sublist);
            colexprs = palloc0(
                (sublist_length as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut List>() as libc::c_ulong),
            ) as *mut *mut List;
        } else if sublist_length != list_length(sublist) {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        i = 0 as libc::c_int;
        let mut lc2__state: ForEachState = {
            let mut init = ForEachState {
                l: sublist,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(lc2__state.l).is_null() && lc2__state.i < (*lc2__state.l).length {
            lc2 = &mut *((*lc2__state.l).elements).offset(lc2__state.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            lc2 = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut col: *mut Node = (*lc2).ptr_value as *mut Node;
            let ref mut fresh0 = *colexprs.offset(i as isize);
            *fresh0 = lappend(*colexprs.offset(i as isize), col as *mut libc::c_void);
            i += 1;
            i;
            lc2__state.i += 1;
            lc2__state.i;
        }
        list_free(sublist);
        lc__state.i += 1;
        lc__state.i;
    }
    i = 0 as libc::c_int;
    while i < sublist_length {
        let mut coltype: Oid = 0;
        let mut coltypmod: i32 = 0;
        let mut colcoll: Oid = 0;
        coltype = select_common_type(
            pstate,
            *colexprs.offset(i as isize),
            b"VALUES\0" as *const u8 as *const libc::c_char,
            0 as *mut *mut Node,
        );
        let mut lc__state_0: ForEachState = {
            let mut init = ForEachState {
                l: *colexprs.offset(i as isize),
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
            let mut col_0: *mut Node = (*lc).ptr_value as *mut Node;
            col_0 = coerce_to_common_type(
                pstate,
                col_0,
                coltype,
                b"VALUES\0" as *const u8 as *const libc::c_char,
            );
            (*lc).ptr_value = col_0 as *mut libc::c_void;
            lc__state_0.i += 1;
            lc__state_0.i;
        }
        coltypmod = select_common_typmod(pstate, *colexprs.offset(i as isize), coltype);
        colcoll = select_common_collation(pstate, *colexprs.offset(i as isize), true);
        coltypes = lappend_oid(coltypes, coltype);
        coltypmods = lappend_int(coltypmods, coltypmod);
        colcollations = lappend_oid(colcollations, colcoll);
        i += 1;
        i;
    }
    exprsLists = 0 as *mut libc::c_void as *mut List;
    let mut lc__state_1: ForEachState = {
        let mut init = ForEachState {
            l: *colexprs.offset(0 as libc::c_int as isize),
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
        let mut col_1: *mut Node = (*lc).ptr_value as *mut Node;
        let mut sublist_0: *mut List = 0 as *mut List;
        sublist_0 = list_make1_impl(
            T_List,
            ListCell {
                ptr_value: col_1 as *mut libc::c_void,
            },
        );
        exprsLists = lappend(exprsLists, sublist_0 as *mut libc::c_void);
        lc__state_1.i += 1;
        lc__state_1.i;
    }
    list_free(*colexprs.offset(0 as libc::c_int as isize));
    i = 1 as libc::c_int;
    while i < sublist_length {
        let mut lc__state_2: ForBothState = {
            let mut init = ForBothState {
                l1: *colexprs.offset(i as isize),
                l2: exprsLists,
                i: 0 as libc::c_int,
            };
            init
        };
        loop {
            lc = (if !(lc__state_2.l1).is_null() && lc__state_2.i < (*lc__state_2.l1).length {
                &mut *((*lc__state_2.l1).elements).offset(lc__state_2.i as isize) as *mut ListCell
            } else {
                0 as *mut ListCell
            });
            lc2 = (if !(lc__state_2.l2).is_null() && lc__state_2.i < (*lc__state_2.l2).length {
                &mut *((*lc__state_2.l2).elements).offset(lc__state_2.i as isize) as *mut ListCell
            } else {
                0 as *mut ListCell
            });
            if !(!lc.is_null() && !lc2.is_null()) {
                break;
            }
            let mut col_2: *mut Node = (*lc).ptr_value as *mut Node;
            let mut sublist_1: *mut List = (*lc2).ptr_value as *mut List;
            sublist_1 = lappend(sublist_1, col_2 as *mut libc::c_void);
            lc__state_2.i += 1;
            lc__state_2.i;
        }
        list_free(*colexprs.offset(i as isize));
        i += 1;
        i;
    }
    if !((*pstate).p_rtable).is_null()
        && contain_vars_of_level(exprsLists as *mut Node, 0 as libc::c_int) as libc::c_int != 0
    {
        lateral = true;
    }
    nsitem = addRangeTableEntryForValues(
        pstate,
        exprsLists,
        coltypes,
        coltypmods,
        colcollations,
        0 as *mut Alias,
        lateral,
        true,
    );
    addNSItemToQuery(pstate, nsitem, true, true, true);
    (*qry).targetList = expandNSItemAttrs(pstate, nsitem, 0 as libc::c_int, -(1 as libc::c_int));
    (*qry).sortClause = transformSortClause(
        pstate,
        (*stmt).sortClause,
        &mut (*qry).targetList,
        EXPR_KIND_ORDER_BY,
        false,
    );
    (*qry).limitOffset = transformLimitClause(
        pstate,
        (*stmt).limitOffset,
        EXPR_KIND_OFFSET,
        b"OFFSET\0" as *const u8 as *const libc::c_char,
        (*stmt).limitOption,
    );
    (*qry).limitCount = transformLimitClause(
        pstate,
        (*stmt).limitCount,
        EXPR_KIND_LIMIT,
        b"LIMIT\0" as *const u8 as *const libc::c_char,
        (*stmt).limitOption,
    );
    (*qry).limitOption = (*stmt).limitOption;
    if !((*stmt).lockingClause).is_null() {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    (*qry).rtable = (*pstate).p_rtable;
    (*qry).jointree = makeFromExpr((*pstate).p_joinlist, 0 as *mut Node);
    (*qry).hasSubLinks = (*pstate).p_hasSubLinks;
    assign_query_collations(pstate, qry);
    return qry;
}
unsafe extern "C" fn transformSetOperationStmt(
    mut pstate: *mut ParseState,
    mut stmt: *mut SelectStmt,
) -> *mut Query {
    let mut qry: *mut Query = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_Query;
        _result
    }) as *mut Query;
    let mut leftmostSelect: *mut SelectStmt = 0 as *mut SelectStmt;
    let mut leftmostRTI: libc::c_int = 0;
    let mut leftmostQuery: *mut Query = 0 as *mut Query;
    let mut sostmt: *mut SetOperationStmt = 0 as *mut SetOperationStmt;
    let mut sortClause: *mut List = 0 as *mut List;
    let mut limitOffset: *mut Node = 0 as *mut Node;
    let mut limitCount: *mut Node = 0 as *mut Node;
    let mut lockingClause: *mut List = 0 as *mut List;
    let mut withClause: *mut WithClause = 0 as *mut WithClause;
    let mut node: *mut Node = 0 as *mut Node;
    let mut left_tlist: *mut ListCell = 0 as *mut ListCell;
    let mut lct: *mut ListCell = 0 as *mut ListCell;
    let mut lcm: *mut ListCell = 0 as *mut ListCell;
    let mut lcc: *mut ListCell = 0 as *mut ListCell;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut targetvars: *mut List = 0 as *mut List;
    let mut targetnames: *mut List = 0 as *mut List;
    let mut sv_namespace: *mut List = 0 as *mut List;
    let mut sv_rtable_length: libc::c_int = 0;
    let mut jnsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    let mut sortnscolumns: *mut ParseNamespaceColumn = 0 as *mut ParseNamespaceColumn;
    let mut sortcolindex: libc::c_int = 0;
    let mut tllen: libc::c_int = 0;
    (*qry).commandType = CMD_SELECT;
    leftmostSelect = (*stmt).larg;
    while !leftmostSelect.is_null()
        && (*leftmostSelect).op as libc::c_uint != SETOP_NONE as libc::c_int as libc::c_uint
    {
        leftmostSelect = (*leftmostSelect).larg;
    }
    if !((*leftmostSelect).intoClause).is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    sortClause = (*stmt).sortClause;
    limitOffset = (*stmt).limitOffset;
    limitCount = (*stmt).limitCount;
    lockingClause = (*stmt).lockingClause;
    withClause = (*stmt).withClause;
    (*stmt).sortClause = 0 as *mut libc::c_void as *mut List;
    (*stmt).limitOffset = 0 as *mut Node;
    (*stmt).limitCount = 0 as *mut Node;
    (*stmt).lockingClause = 0 as *mut libc::c_void as *mut List;
    (*stmt).withClause = 0 as *mut WithClause;
    if !lockingClause.is_null() {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    if !withClause.is_null() {
        (*qry).hasRecursive = (*withClause).recursive;
        (*qry).cteList = transformWithClause(pstate, withClause);
        (*qry).hasModifyingCTE = (*pstate).p_hasModifyingCTE;
    }
    sostmt =
        transformSetOperationTree(pstate, stmt, true, 0 as *mut *mut List) as *mut SetOperationStmt;
    (*qry).setOperations = sostmt as *mut Node;
    node = (*sostmt).larg;
    while !node.is_null()
        && (*(node as *const Node)).type_0 as libc::c_uint
            == T_SetOperationStmt as libc::c_int as libc::c_uint
    {
        node = (*(node as *mut SetOperationStmt)).larg;
    }
    leftmostRTI = (*(node as *mut RangeTblRef)).rtindex;
    leftmostQuery = (*(list_nth((*pstate).p_rtable, leftmostRTI - 1 as libc::c_int)
        as *mut RangeTblEntry))
        .subquery;
    (*qry).targetList = 0 as *mut libc::c_void as *mut List;
    targetvars = 0 as *mut libc::c_void as *mut List;
    targetnames = 0 as *mut libc::c_void as *mut List;
    sortnscolumns = palloc0(
        (list_length((*sostmt).colTypes) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<ParseNamespaceColumn>() as libc::c_ulong),
    ) as *mut ParseNamespaceColumn;
    sortcolindex = 0 as libc::c_int;
    let mut lct__state: ForFourState = {
        let mut init = ForFourState {
            l1: (*sostmt).colTypes,
            l2: (*sostmt).colTypmods,
            l3: (*sostmt).colCollations,
            l4: (*leftmostQuery).targetList,
            i: 0 as libc::c_int,
        };
        init
    };
    loop {
        lct = (if !(lct__state.l1).is_null() && lct__state.i < (*lct__state.l1).length {
            &mut *((*lct__state.l1).elements).offset(lct__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        lcm = (if !(lct__state.l2).is_null() && lct__state.i < (*lct__state.l2).length {
            &mut *((*lct__state.l2).elements).offset(lct__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        lcc = (if !(lct__state.l3).is_null() && lct__state.i < (*lct__state.l3).length {
            &mut *((*lct__state.l3).elements).offset(lct__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        left_tlist = (if !(lct__state.l4).is_null() && lct__state.i < (*lct__state.l4).length {
            &mut *((*lct__state.l4).elements).offset(lct__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        if !(!lct.is_null() && !lcm.is_null() && !lcc.is_null() && !left_tlist.is_null()) {
            break;
        }
        let mut colType: Oid = (*lct).oid_value;
        let mut colTypmod: i32 = (*lcm).int_value;
        let mut colCollation: Oid = (*lcc).oid_value;
        let mut lefttle: *mut TargetEntry = (*left_tlist).ptr_value as *mut TargetEntry;
        let mut colName: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tle: *mut TargetEntry = 0 as *mut TargetEntry;
        let mut var: *mut Var = 0 as *mut Var;
        colName = pstrdup((*lefttle).resname);
        var = makeVar(
            leftmostRTI as Index,
            (*lefttle).resno,
            colType,
            colTypmod,
            colCollation,
            0 as libc::c_int as Index,
        );
        (*var).location = exprLocation((*lefttle).expr as *mut Node);
        let fresh1 = (*pstate).p_next_resno;
        (*pstate).p_next_resno = (*pstate).p_next_resno + 1;
        tle = makeTargetEntry(var as *mut Expr, fresh1 as AttrNumber, colName, false);
        (*qry).targetList = lappend((*qry).targetList, tle as *mut libc::c_void);
        targetvars = lappend(targetvars, var as *mut libc::c_void);
        targetnames = lappend(targetnames, makeString(colName) as *mut libc::c_void);
        (*sortnscolumns.offset(sortcolindex as isize)).p_varno = leftmostRTI as Index;
        (*sortnscolumns.offset(sortcolindex as isize)).p_varattno = (*lefttle).resno;
        (*sortnscolumns.offset(sortcolindex as isize)).p_vartype = colType;
        (*sortnscolumns.offset(sortcolindex as isize)).p_vartypmod = colTypmod;
        (*sortnscolumns.offset(sortcolindex as isize)).p_varcollid = colCollation;
        (*sortnscolumns.offset(sortcolindex as isize)).p_varnosyn = leftmostRTI as Index;
        (*sortnscolumns.offset(sortcolindex as isize)).p_varattnosyn = (*lefttle).resno;
        sortcolindex += 1;
        sortcolindex;
        lct__state.i += 1;
        lct__state.i;
    }
    sv_rtable_length = list_length((*pstate).p_rtable);
    jnsitem = addRangeTableEntryForJoin(
        pstate,
        targetnames,
        sortnscolumns,
        JOIN_INNER,
        0 as libc::c_int,
        targetvars,
        0 as *mut libc::c_void as *mut List,
        0 as *mut libc::c_void as *mut List,
        0 as *mut Alias,
        false,
    );
    sv_namespace = (*pstate).p_namespace;
    (*pstate).p_namespace = 0 as *mut libc::c_void as *mut List;
    addNSItemToQuery(pstate, jnsitem, false, false, true);
    tllen = list_length((*qry).targetList);
    (*qry).sortClause = transformSortClause(
        pstate,
        sortClause,
        &mut (*qry).targetList,
        EXPR_KIND_ORDER_BY,
        false,
    );
    (*pstate).p_namespace = sv_namespace;
    (*pstate).p_rtable = list_truncate((*pstate).p_rtable, sv_rtable_length);
    if tllen != list_length((*qry).targetList) {
        let elevel__1: libc::c_int = 21 as libc::c_int;
        let mut __error_1: libc::c_int = 0;
        if elevel__1 >= 21 as libc::c_int {
            abort();
        }
    }
    (*qry).limitOffset = transformLimitClause(
        pstate,
        limitOffset,
        EXPR_KIND_OFFSET,
        b"OFFSET\0" as *const u8 as *const libc::c_char,
        (*stmt).limitOption,
    );
    (*qry).limitCount = transformLimitClause(
        pstate,
        limitCount,
        EXPR_KIND_LIMIT,
        b"LIMIT\0" as *const u8 as *const libc::c_char,
        (*stmt).limitOption,
    );
    (*qry).limitOption = (*stmt).limitOption;
    (*qry).rtable = (*pstate).p_rtable;
    (*qry).jointree = makeFromExpr((*pstate).p_joinlist, 0 as *mut Node);
    (*qry).hasSubLinks = (*pstate).p_hasSubLinks;
    (*qry).hasWindowFuncs = (*pstate).p_hasWindowFuncs;
    (*qry).hasTargetSRFs = (*pstate).p_hasTargetSRFs;
    (*qry).hasAggs = (*pstate).p_hasAggs;
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: lockingClause,
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
        transformLockingClause(pstate, qry, (*l).ptr_value as *mut LockingClause, false);
        l__state.i += 1;
        l__state.i;
    }
    assign_query_collations(pstate, qry);
    if (*pstate).p_hasAggs as libc::c_int != 0
        || !((*qry).groupClause).is_null()
        || !((*qry).groupingSets).is_null()
        || !((*qry).havingQual).is_null()
    {
        parseCheckAggregates(pstate, qry);
    }
    return qry;
}
#[no_mangle]
pub unsafe extern "C" fn makeSortGroupClauseForSetOp(mut rescoltype: Oid) -> *mut SortGroupClause {
    let mut grpcl: *mut SortGroupClause = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_SortGroupClause;
        _result
    }) as *mut SortGroupClause;
    let mut sortop: Oid = 0;
    let mut eqop: Oid = 0;
    let mut hashable: bool = false;
    get_sort_group_operators(
        rescoltype,
        false,
        true,
        false,
        &mut sortop,
        &mut eqop,
        0 as *mut Oid,
        &mut hashable,
    );
    (*grpcl).tleSortGroupRef = 0 as libc::c_int as Index;
    (*grpcl).eqop = eqop;
    (*grpcl).sortop = sortop;
    (*grpcl).nulls_first = false;
    (*grpcl).hashable = hashable;
    return grpcl;
}
unsafe extern "C" fn determineRecursiveColTypes(
    mut pstate: *mut ParseState,
    mut larg: *mut Node,
    mut nrtargetlist: *mut List,
) {
    let mut node: *mut Node = 0 as *mut Node;
    let mut leftmostRTI: libc::c_int = 0;
    let mut leftmostQuery: *mut Query = 0 as *mut Query;
    let mut targetList: *mut List = 0 as *mut List;
    let mut left_tlist: *mut ListCell = 0 as *mut ListCell;
    let mut nrtl: *mut ListCell = 0 as *mut ListCell;
    let mut next_resno: libc::c_int = 0;
    node = larg;
    while !node.is_null()
        && (*(node as *const Node)).type_0 as libc::c_uint
            == T_SetOperationStmt as libc::c_int as libc::c_uint
    {
        node = (*(node as *mut SetOperationStmt)).larg;
    }
    leftmostRTI = (*(node as *mut RangeTblRef)).rtindex;
    leftmostQuery = (*(list_nth((*pstate).p_rtable, leftmostRTI - 1 as libc::c_int)
        as *mut RangeTblEntry))
        .subquery;
    targetList = 0 as *mut libc::c_void as *mut List;
    next_resno = 1 as libc::c_int;
    let mut nrtl__state: ForBothState = {
        let mut init = ForBothState {
            l1: nrtargetlist,
            l2: (*leftmostQuery).targetList,
            i: 0 as libc::c_int,
        };
        init
    };
    loop {
        nrtl = (if !(nrtl__state.l1).is_null() && nrtl__state.i < (*nrtl__state.l1).length {
            &mut *((*nrtl__state.l1).elements).offset(nrtl__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        left_tlist = (if !(nrtl__state.l2).is_null() && nrtl__state.i < (*nrtl__state.l2).length {
            &mut *((*nrtl__state.l2).elements).offset(nrtl__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        if !(!nrtl.is_null() && !left_tlist.is_null()) {
            break;
        }
        let mut nrtle: *mut TargetEntry = (*nrtl).ptr_value as *mut TargetEntry;
        let mut lefttle: *mut TargetEntry = (*left_tlist).ptr_value as *mut TargetEntry;
        let mut colName: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut tle: *mut TargetEntry = 0 as *mut TargetEntry;
        colName = pstrdup((*lefttle).resname);
        let fresh2 = next_resno;
        next_resno = next_resno + 1;
        tle = makeTargetEntry((*nrtle).expr, fresh2 as AttrNumber, colName, false);
        targetList = lappend(targetList, tle as *mut libc::c_void);
        nrtl__state.i += 1;
        nrtl__state.i;
    }
    analyzeCTETargetList(pstate, (*pstate).p_parent_cte, targetList);
}
unsafe extern "C" fn transformUpdateStmt(
    mut pstate: *mut ParseState,
    mut stmt: *mut UpdateStmt,
) -> *mut Query {
    let mut qry: *mut Query = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_Query;
        _result
    }) as *mut Query;
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    let mut qual: *mut Node = 0 as *mut Node;
    (*qry).commandType = CMD_UPDATE;
    (*pstate).p_is_insert = false;
    if !((*stmt).withClause).is_null() {
        (*qry).hasRecursive = (*(*stmt).withClause).recursive;
        (*qry).cteList = transformWithClause(pstate, (*stmt).withClause);
        (*qry).hasModifyingCTE = (*pstate).p_hasModifyingCTE;
    }
    (*qry).resultRelation = setTargetTable(
        pstate,
        (*stmt).relation,
        (*(*stmt).relation).inh,
        true,
        ((1 as libc::c_int) << 2 as libc::c_int) as AclMode,
    );
    nsitem = (*pstate).p_target_nsitem;
    (*nsitem).p_lateral_only = true;
    (*nsitem).p_lateral_ok = false;
    transformFromClause(pstate, (*stmt).fromClause);
    (*nsitem).p_lateral_only = false;
    (*nsitem).p_lateral_ok = true;
    qual = transformWhereClause(
        pstate,
        (*stmt).whereClause,
        EXPR_KIND_WHERE,
        b"WHERE\0" as *const u8 as *const libc::c_char,
    );
    (*qry).returningList = transformReturningList(pstate, (*stmt).returningList);
    (*qry).targetList = transformUpdateTargetList(pstate, (*stmt).targetList);
    (*qry).rtable = (*pstate).p_rtable;
    (*qry).jointree = makeFromExpr((*pstate).p_joinlist, qual);
    (*qry).hasTargetSRFs = (*pstate).p_hasTargetSRFs;
    (*qry).hasSubLinks = (*pstate).p_hasSubLinks;
    assign_query_collations(pstate, qry);
    return qry;
}
unsafe extern "C" fn transformUpdateTargetList(
    mut pstate: *mut ParseState,
    mut origTlist: *mut List,
) -> *mut List {
    let mut tlist: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut target_rte: *mut RangeTblEntry = 0 as *mut RangeTblEntry;
    let mut orig_tl: *mut ListCell = 0 as *mut ListCell;
    let mut tl: *mut ListCell = 0 as *mut ListCell;
    tlist = transformTargetList(pstate, origTlist, EXPR_KIND_UPDATE_SOURCE);
    if (*pstate).p_next_resno <= (*(*(*pstate).p_target_relation).rd_rel).relnatts as libc::c_int {
        (*pstate).p_next_resno =
            (*(*(*pstate).p_target_relation).rd_rel).relnatts as libc::c_int + 1 as libc::c_int;
    }
    target_rte = (*(*pstate).p_target_nsitem).p_rte;
    orig_tl = list_head(origTlist);
    let mut tl__state: ForEachState = {
        let mut init = ForEachState {
            l: tlist,
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
        let mut origTarget: *mut ResTarget = 0 as *mut ResTarget;
        let mut attrno: libc::c_int = 0;
        if (*tle).resjunk != 0 {
            let fresh3 = (*pstate).p_next_resno;
            (*pstate).p_next_resno = (*pstate).p_next_resno + 1;
            (*tle).resno = fresh3 as AttrNumber;
            (*tle).resname = 0 as *mut libc::c_char;
        } else {
            if orig_tl.is_null() {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"UPDATE target count mismatch --- internal error\0" as *const u8
                            as *const libc::c_char,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/analyze.c\0"
                            as *const u8 as *const libc::c_char,
                        2336 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            origTarget = (*orig_tl).ptr_value as *mut ResTarget;
            attrno = attnameAttNum((*pstate).p_target_relation, (*origTarget).name, true);
            if attrno == 0 as libc::c_int {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            updateTargetListEntry(
                pstate,
                tle,
                (*origTarget).name,
                attrno,
                (*origTarget).indirection,
                (*origTarget).location,
            );
            (*target_rte).updatedCols =
                bms_add_member((*target_rte).updatedCols, attrno - -(7 as libc::c_int));
            orig_tl = lnext(origTlist, orig_tl);
        }
        tl__state.i += 1;
        tl__state.i;
    }
    if !orig_tl.is_null() {
        let elevel__1: libc::c_int = 21 as libc::c_int;
        let mut __error_1: libc::c_int = 0;
        if errstart(elevel__1, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"UPDATE target count mismatch --- internal error\0" as *const u8
                    as *const libc::c_char,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/analyze.c\0" as *const u8
                    as *const libc::c_char,
                2361 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel__1 >= 21 as libc::c_int {
            abort();
        }
    }
    return tlist;
}
unsafe extern "C" fn transformReturningList(
    mut pstate: *mut ParseState,
    mut returningList: *mut List,
) -> *mut List {
    let mut rlist: *mut List = 0 as *mut List;
    let mut save_next_resno: libc::c_int = 0;
    if returningList.is_null() {
        return 0 as *mut libc::c_void as *mut List;
    }
    save_next_resno = (*pstate).p_next_resno;
    (*pstate).p_next_resno = 1 as libc::c_int;
    rlist = transformTargetList(pstate, returningList, EXPR_KIND_RETURNING);
    if rlist.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    markTargetListOrigins(pstate, rlist);
    if (*pstate).p_resolve_unknowns != 0 {
        resolveTargetListUnknowns(pstate, rlist);
    }
    (*pstate).p_next_resno = save_next_resno;
    return rlist;
}
unsafe extern "C" fn transformDeclareCursorStmt(
    mut pstate: *mut ParseState,
    mut stmt: *mut DeclareCursorStmt,
) -> *mut Query {
    let mut result: *mut Query = 0 as *mut Query;
    let mut query: *mut Query = 0 as *mut Query;
    if (*stmt).options & 0x2 as libc::c_int != 0 && (*stmt).options & 0x4 as libc::c_int != 0 {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    query = transformStmt(pstate, (*stmt).query);
    (*stmt).query = query as *mut Node;
    if !((*(query as *const Node)).type_0 as libc::c_uint == T_Query as libc::c_int as libc::c_uint)
        || (*query).commandType as libc::c_uint != CMD_SELECT as libc::c_int as libc::c_uint
    {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"unexpected non-SELECT command in DECLARE CURSOR\0" as *const u8
                    as *const libc::c_char,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/analyze.c\0" as *const u8
                    as *const libc::c_char,
                2698 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    if (*query).hasModifyingCTE != 0 {
        let elevel__1: libc::c_int = 21 as libc::c_int;
        let mut __error_1: libc::c_int = 0;
        if elevel__1 >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*query).rowMarks).is_null() && (*stmt).options & 0x10 as libc::c_int != 0 {
        let elevel__2: libc::c_int = 21 as libc::c_int;
        let mut __error_2: libc::c_int = 0;
        if elevel__2 >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*query).rowMarks).is_null() && (*stmt).options & 0x2 as libc::c_int != 0 {
        let elevel__3: libc::c_int = 21 as libc::c_int;
        let mut __error_3: libc::c_int = 0;
        if elevel__3 >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*query).rowMarks).is_null() && (*stmt).options & 0x8 as libc::c_int != 0 {
        let elevel__4: libc::c_int = 21 as libc::c_int;
        let mut __error_4: libc::c_int = 0;
        if elevel__4 >= 21 as libc::c_int {
            abort();
        }
    }
    result = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_Query;
        _result
    }) as *mut Query;
    (*result).commandType = CMD_UTILITY;
    (*result).utilityStmt = stmt as *mut Node;
    return result;
}
unsafe extern "C" fn transformExplainStmt(
    mut pstate: *mut ParseState,
    mut stmt: *mut ExplainStmt,
) -> *mut Query {
    let mut result: *mut Query = 0 as *mut Query;
    (*stmt).query = transformOptionalSelectInto(pstate, (*stmt).query) as *mut Node;
    result = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_Query;
        _result
    }) as *mut Query;
    (*result).commandType = CMD_UTILITY;
    (*result).utilityStmt = stmt as *mut Node;
    return result;
}
unsafe extern "C" fn transformCallStmt(
    mut pstate: *mut ParseState,
    mut stmt: *mut CallStmt,
) -> *mut Query {
    let mut targs: *mut List = 0 as *mut List;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut node: *mut Node = 0 as *mut Node;
    let mut result: *mut Query = 0 as *mut Query;
    targs = 0 as *mut libc::c_void as *mut List;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*(*stmt).funccall).args,
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
        targs = lappend(
            targs,
            transformExpr(
                pstate,
                (*lc).ptr_value as *mut Node,
                EXPR_KIND_CALL_ARGUMENT,
            ) as *mut libc::c_void,
        );
        lc__state.i += 1;
        lc__state.i;
    }
    node = ParseFuncOrColumn(
        pstate,
        (*(*stmt).funccall).funcname,
        targs,
        (*pstate).p_last_srf,
        (*stmt).funccall,
        true,
        (*(*stmt).funccall).location,
    );
    assign_expr_collations(pstate, node);
    (*stmt).funcexpr = node as *mut FuncExpr;
    result = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_Query;
        _result
    }) as *mut Query;
    (*result).commandType = CMD_UTILITY;
    (*result).utilityStmt = stmt as *mut Node;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn LCS_asString(mut strength: LockClauseStrength) -> *const libc::c_char {
    match strength as libc::c_uint {
        1 => return b"FOR KEY SHARE\0" as *const u8 as *const libc::c_char,
        2 => return b"FOR SHARE\0" as *const u8 as *const libc::c_char,
        3 => return b"FOR NO KEY UPDATE\0" as *const u8 as *const libc::c_char,
        4 => return b"FOR UPDATE\0" as *const u8 as *const libc::c_char,
        0 | _ => {}
    }
    return b"FOR some\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn CheckSelectLocking(mut qry: *mut Query, mut strength: LockClauseStrength) {
    if !((*qry).setOperations).is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*qry).distinctClause).is_null() {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*qry).groupClause).is_null() {
        let elevel__1: libc::c_int = 21 as libc::c_int;
        let mut __error_1: libc::c_int = 0;
        if elevel__1 >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*qry).havingQual).is_null() {
        let elevel__2: libc::c_int = 21 as libc::c_int;
        let mut __error_2: libc::c_int = 0;
        if elevel__2 >= 21 as libc::c_int {
            abort();
        }
    }
    if (*qry).hasAggs != 0 {
        let elevel__3: libc::c_int = 21 as libc::c_int;
        let mut __error_3: libc::c_int = 0;
        if elevel__3 >= 21 as libc::c_int {
            abort();
        }
    }
    if (*qry).hasWindowFuncs != 0 {
        let elevel__4: libc::c_int = 21 as libc::c_int;
        let mut __error_4: libc::c_int = 0;
        if elevel__4 >= 21 as libc::c_int {
            abort();
        }
    }
    if (*qry).hasTargetSRFs != 0 {
        let elevel__5: libc::c_int = 21 as libc::c_int;
        let mut __error_5: libc::c_int = 0;
        if elevel__5 >= 21 as libc::c_int {
            abort();
        }
    }
}
unsafe extern "C" fn transformLockingClause(
    mut pstate: *mut ParseState,
    mut qry: *mut Query,
    mut lc: *mut LockingClause,
    mut pushedDown: bool,
) {
    let mut lockedRels: *mut List = (*lc).lockedRels;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut rt: *mut ListCell = 0 as *mut ListCell;
    let mut i: Index = 0;
    let mut allrels: *mut LockingClause = 0 as *mut LockingClause;
    CheckSelectLocking(qry, (*lc).strength);
    allrels = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_LockingClause;
        _result
    }) as *mut LockingClause;
    (*allrels).lockedRels = 0 as *mut libc::c_void as *mut List;
    (*allrels).strength = (*lc).strength;
    (*allrels).waitPolicy = (*lc).waitPolicy;
    if lockedRels.is_null() {
        i = 0 as libc::c_int as Index;
        let mut rt__state: ForEachState = {
            let mut init = ForEachState {
                l: (*qry).rtable,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(rt__state.l).is_null() && rt__state.i < (*rt__state.l).length {
            rt = &mut *((*rt__state.l).elements).offset(rt__state.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            rt = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut rte: *mut RangeTblEntry = (*rt).ptr_value as *mut RangeTblEntry;
            i = i.wrapping_add(1);
            i;
            match (*rte).rtekind as libc::c_uint {
                0 => {
                    applyLockingClause(qry, i, (*lc).strength, (*lc).waitPolicy, pushedDown);
                    (*rte).requiredPerms |= ((1 as libc::c_int) << 2 as libc::c_int) as AclMode;
                }
                1 => {
                    applyLockingClause(qry, i, (*lc).strength, (*lc).waitPolicy, pushedDown);
                    transformLockingClause(pstate, (*rte).subquery, allrels, true);
                }
                _ => {}
            }
            rt__state.i += 1;
            rt__state.i;
        }
    } else {
        let mut l__state: ForEachState = {
            let mut init = ForEachState {
                l: lockedRels,
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
            let mut thisrel: *mut RangeVar = (*l).ptr_value as *mut RangeVar;
            if !((*thisrel).catalogname).is_null() || !((*thisrel).schemaname).is_null() {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            i = 0 as libc::c_int as Index;
            let mut rt__state_0: ForEachState = {
                let mut init = ForEachState {
                    l: (*qry).rtable,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(rt__state_0.l).is_null() && rt__state_0.i < (*rt__state_0.l).length {
                rt = &mut *((*rt__state_0.l).elements).offset(rt__state_0.i as isize)
                    as *mut ListCell;
                true as libc::c_int
            } else {
                rt = 0 as *mut ListCell;
                false as libc::c_int
            } != 0
            {
                let mut rte_0: *mut RangeTblEntry = (*rt).ptr_value as *mut RangeTblEntry;
                i = i.wrapping_add(1);
                i;
                if strcmp((*(*rte_0).eref).aliasname, (*thisrel).relname) == 0 as libc::c_int {
                    match (*rte_0).rtekind as libc::c_uint {
                        0 => {
                            applyLockingClause(
                                qry,
                                i,
                                (*lc).strength,
                                (*lc).waitPolicy,
                                pushedDown,
                            );
                            (*rte_0).requiredPerms |=
                                ((1 as libc::c_int) << 2 as libc::c_int) as AclMode;
                        }
                        1 => {
                            applyLockingClause(
                                qry,
                                i,
                                (*lc).strength,
                                (*lc).waitPolicy,
                                pushedDown,
                            );
                            transformLockingClause(pstate, (*rte_0).subquery, allrels, true);
                        }
                        2 => {
                            let elevel__0: libc::c_int = 21 as libc::c_int;
                            let mut __error_0: libc::c_int = 0;
                            if elevel__0 >= 21 as libc::c_int {
                                abort();
                            }
                        }
                        3 => {
                            let elevel__1: libc::c_int = 21 as libc::c_int;
                            let mut __error_1: libc::c_int = 0;
                            if elevel__1 >= 21 as libc::c_int {
                                abort();
                            }
                        }
                        4 => {
                            let elevel__2: libc::c_int = 21 as libc::c_int;
                            let mut __error_2: libc::c_int = 0;
                            if elevel__2 >= 21 as libc::c_int {
                                abort();
                            }
                        }
                        5 => {
                            let elevel__3: libc::c_int = 21 as libc::c_int;
                            let mut __error_3: libc::c_int = 0;
                            if elevel__3 >= 21 as libc::c_int {
                                abort();
                            }
                        }
                        6 => {
                            let elevel__4: libc::c_int = 21 as libc::c_int;
                            let mut __error_4: libc::c_int = 0;
                            if elevel__4 >= 21 as libc::c_int {
                                abort();
                            }
                        }
                        7 => {
                            let elevel__5: libc::c_int = 21 as libc::c_int;
                            let mut __error_5: libc::c_int = 0;
                            if elevel__5 >= 21 as libc::c_int {
                                abort();
                            }
                        }
                        _ => {
                            let elevel__6: libc::c_int = 21 as libc::c_int;
                            let mut __error_6: libc::c_int = 0;
                            if errstart(elevel__6, 0 as *const libc::c_char) != 0 {
                                errmsg_internal(
                                    b"unrecognized RTE type: %d\0" as *const u8
                                        as *const libc::c_char,
                                    (*rte_0).rtekind as libc::c_int,
                                );
                                errfinish(
                                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/analyze.c\0"
                                        as *const u8 as *const libc::c_char,
                                    3143 as libc::c_int,
                                    0 as *const libc::c_char,
                                );
                            }
                            if elevel__6 >= 21 as libc::c_int {
                                abort();
                            }
                        }
                    }
                    break;
                } else {
                    rt__state_0.i += 1;
                    rt__state_0.i;
                }
            }
            if rt.is_null() {
                let elevel__7: libc::c_int = 21 as libc::c_int;
                let mut __error_7: libc::c_int = 0;
                if elevel__7 >= 21 as libc::c_int {
                    abort();
                }
            }
            l__state.i += 1;
            l__state.i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn applyLockingClause(
    mut qry: *mut Query,
    mut rtindex: Index,
    mut strength: LockClauseStrength,
    mut waitPolicy: LockWaitPolicy,
    mut pushedDown: bool,
) {
    let mut rc: *mut RowMarkClause = 0 as *mut RowMarkClause;
    if pushedDown == 0 {
        (*qry).hasForUpdate = true;
    }
    rc = get_parse_rowmark(qry, rtindex);
    if !rc.is_null() {
        (*rc).strength = (if (*rc).strength as libc::c_uint > strength as libc::c_uint {
            (*rc).strength as libc::c_uint
        } else {
            strength as libc::c_uint
        }) as LockClauseStrength;
        (*rc).waitPolicy = (if (*rc).waitPolicy as libc::c_uint > waitPolicy as libc::c_uint {
            (*rc).waitPolicy as libc::c_uint
        } else {
            waitPolicy as libc::c_uint
        }) as LockWaitPolicy;
        (*rc).pushedDown = ((*rc).pushedDown as libc::c_int & pushedDown as libc::c_int) as bool;
        return;
    }
    rc = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_RowMarkClause;
        _result
    }) as *mut RowMarkClause;
    (*rc).rti = rtindex;
    (*rc).strength = strength;
    (*rc).waitPolicy = waitPolicy;
    (*rc).pushedDown = pushedDown;
    (*qry).rowMarks = lappend((*qry).rowMarks, rc as *mut libc::c_void);
}
