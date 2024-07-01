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
//     pub type dsa_area;
//     pub type TIDBitmap;
//     pub type AttrMissing;
//     pub type PgStat_TableStatus;
//     pub type FdwRoutine;
//     pub type IndexScanDescData;
//     pub type IndexPath;
//     pub type PlannerInfo;
//     pub type BufferAccessStrategyData;
//     pub type QueryEnvironment;
//     pub type JitInstrumentation;
//     pub type JitContext;
//     pub type PartitionDirectoryData;
//     pub type CopyMultiInsertBuffer;
//     pub type GlobalVisState;
//     pub type SharedJitInstrumentation;
//     pub type ExprEvalStep;
//     pub type TableAmRoutine;
//     pub type PartitionDescData;
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
//     fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
//     fn pg_detoast_datum(datum: *mut varlena) -> *mut varlena;
//     fn nodeToString(obj: *const libc::c_void) -> *mut libc::c_char;
//     fn stringToNode(str: *const libc::c_char) -> *mut libc::c_void;
//     fn copyObjectImpl(obj: *const libc::c_void) -> *mut libc::c_void;
//     fn equal(a: *const libc::c_void, b: *const libc::c_void) -> bool;
//     fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
//     fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
//     fn list_make3_impl(
//         t: NodeTag,
//         datum1: ListCell,
//         datum2: ListCell,
//         datum3: ListCell,
//     ) -> *mut List;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn lcons(datum: *mut libc::c_void, list: *mut List) -> *mut List;
//     fn list_concat(list1: *mut List, list2: *const List) -> *mut List;
//     fn list_delete_nth_cell(list: *mut List, n: libc::c_int) -> *mut List;
//     fn list_free(list: *mut List);
//     fn DecrTupleDescRefCount(tupdesc: TupleDesc);
//     fn index_close(relation: Relation, lockmode: LOCKMODE);
//     fn index_open(relationId: Oid, lockmode: LOCKMODE) -> Relation;
//     fn RelationGetIndexList(relation: Relation) -> *mut List;
//     fn RelationGetStatExtList(relation: Relation) -> *mut List;
//     fn RelationGetIndexExpressions(relation: Relation) -> *mut List;
//     fn RelationGetIndexPredicate(relation: Relation) -> *mut List;
//     fn relation_close(relation: Relation, lockmode: LOCKMODE);
//     fn relation_open(relationId: Oid, lockmode: LOCKMODE) -> Relation;
//     fn relation_openrv(relation: *const RangeVar, lockmode: LOCKMODE) -> Relation;
//     fn untransformRelOptions(options: Datum) -> *mut List;
//     fn table_openrv(relation: *const RangeVar, lockmode: LOCKMODE) -> Relation;
//     fn table_close(relation: Relation, lockmode: LOCKMODE);
//     fn get_relkind_objtype(relkind: libc::c_char) -> ObjectType;
//     fn makeString(str: *mut libc::c_char) -> *mut Value;
//     fn getIdentitySequence(relid: Oid, attnum: AttrNumber, missing_ok: bool) -> Oid;
//     fn get_index_constraint(indexId: Oid) -> Oid;
//     fn build_attrmap_by_name(indesc: TupleDesc, outdesc: TupleDesc) -> *mut AttrMap;
//     fn SystemAttributeByName(
//         attname: *const libc::c_char,
//     ) -> *const FormData_pg_attribute;
//     fn SystemAttributeDefinition(attno: AttrNumber) -> *const FormData_pg_attribute;
//     fn cancel_parser_errposition_callback(pcbstate: *mut ParseCallbackState);
//     fn setup_parser_errposition_callback(
//         pcbstate: *mut ParseCallbackState,
//         pstate: *mut ParseState,
//         location: libc::c_int,
//     );
//     fn free_parsestate(pstate: *mut ParseState);
//     fn make_parsestate(parentParseState: *mut ParseState) -> *mut ParseState;
//     fn RangeVarGetCreationNamespace(newRelation: *const RangeVar) -> Oid;
//     fn RangeVarGetAndCheckCreationNamespace(
//         newRelation: *mut RangeVar,
//         lockmode: LOCKMODE,
//         existing_relation_id: *mut Oid,
//     ) -> Oid;
//     fn RangeVarAdjustRelationPersistence(newRelation: *mut RangeVar, nspid: Oid);
//     fn makeRangeVarFromNameList(names: *mut List) -> *mut RangeVar;
//     fn ChooseRelationName(
//         name1: *const libc::c_char,
//         name2: *const libc::c_char,
//         label: *const libc::c_char,
//         namespaceid: Oid,
//         isconstraint: bool,
//     ) -> *mut libc::c_char;
//     fn GetDefaultOpClass(type_id: Oid, am_id: Oid) -> Oid;
//     fn get_index_am_oid(amname: *const libc::c_char, missing_ok: bool) -> Oid;
//     fn sequence_options(relid: Oid) -> *mut List;
//     fn check_of_type(typetuple: HeapTuple);
//     fn get_tablespace_name(spc_oid: Oid) -> *mut libc::c_char;
//     fn GetUserId() -> Oid;
//     fn makeFromExpr(fromlist: *mut List, quals: *mut Node) -> *mut FromExpr;
//     fn makeAlias(aliasname: *const libc::c_char, colnames: *mut List) -> *mut Alias;
//     fn makeRangeVar(
//         schemaname: *mut libc::c_char,
//         relname: *mut libc::c_char,
//         location: libc::c_int,
//     ) -> *mut RangeVar;
//     fn makeTypeNameFromOid(typeOid: Oid, typmod: i32) -> *mut TypeName;
//     fn makeFuncCall(
//         name: *mut List,
//         args: *mut List,
//         funcformat: CoercionForm,
//         location: libc::c_int,
//     ) -> *mut FuncCall;
//     fn makeDefElem(
//         name: *mut libc::c_char,
//         arg: *mut Node,
//         location: libc::c_int,
//     ) -> *mut DefElem;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprLocation(expr: *const Node) -> libc::c_int;
//     fn expression_planner(expr: *mut Expr) -> *mut Expr;
//     fn evaluate_expr(
//         expr: *mut Expr,
//         result_type: Oid,
//         result_typmod: i32,
//         result_collation: Oid,
//     ) -> *mut Expr;
//     fn transformStmt(pstate: *mut ParseState, parseTree: *mut Node) -> *mut Query;
//     fn transformWhereClause(
//         pstate: *mut ParseState,
//         clause: *mut Node,
//         exprKind: ParseExprKind,
//         constructName: *const libc::c_char,
//     ) -> *mut Node;
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
//     fn assign_expr_collations(pstate: *mut ParseState, expr: *mut Node);
//     fn transformExpr(
//         pstate: *mut ParseState,
//         expr: *mut Node,
//         exprKind: ParseExprKind,
//     ) -> *mut Node;
//     fn addRangeTableEntryForRelation(
//         pstate: *mut ParseState,
//         rel: Relation,
//         lockmode: libc::c_int,
//         alias: *mut Alias,
//         inh: bool,
//         inFromCl: bool,
//     ) -> *mut ParseNamespaceItem;
//     fn addNSItemToQuery(
//         pstate: *mut ParseState,
//         nsitem: *mut ParseNamespaceItem,
//         addToJoinList: bool,
//         addToRelNameSpace: bool,
//         addToVarNameSpace: bool,
//     );
//     fn FigureIndexColname(node: *mut Node) -> *mut libc::c_char;
//     fn typenameType(
//         pstate: *mut ParseState,
//         typeName: *const TypeName,
//         typmod_p: *mut i32,
//     ) -> Type;
//     fn typenameTypeId(pstate: *mut ParseState, typeName: *const TypeName) -> Oid;
//     fn LookupCollation(
//         pstate: *mut ParseState,
//         collnames: *mut List,
//         location: libc::c_int,
//     ) -> Oid;
//     fn SystemFuncName(name: *mut libc::c_char) -> *mut List;
//     fn SystemTypeName(name: *mut libc::c_char) -> *mut TypeName;
//     fn rangeTableEntry_used(
//         node: *mut Node,
//         rt_index: libc::c_int,
//         sublevels_up: libc::c_int,
//     ) -> bool;
//     fn getInsertSelectQuery(
//         parsetree: *mut Query,
//         subquery_ptr: *mut *mut *mut Query,
//     ) -> *mut Query;
//     fn map_variable_attnos(
//         node: *mut Node,
//         target_varno: libc::c_int,
//         sublevels_up: libc::c_int,
//         attno_map: *const AttrMap,
//         to_rowtype: Oid,
//         found_whole_row: *mut bool,
//     ) -> *mut Node;
//     fn pg_class_aclcheck(table_oid: Oid, roleid: Oid, mode: AclMode) -> AclResult;
//     fn pg_type_aclcheck(type_oid: Oid, roleid: Oid, mode: AclMode) -> AclResult;
//     fn aclcheck_error(
//         aclerr: AclResult,
//         objtype: ObjectType,
//         objectname: *const libc::c_char,
//     );
//     fn quote_qualified_identifier(
//         qualifier: *const libc::c_char,
//         ident: *const libc::c_char,
//     ) -> *mut libc::c_char;
//     fn text_to_cstring(t: *const text) -> *mut libc::c_char;
//     fn get_attname(
//         relid: Oid,
//         attnum: AttrNumber,
//         missing_ok: bool,
//     ) -> *mut libc::c_char;
//     fn get_attnum(relid: Oid, attname: *const libc::c_char) -> AttrNumber;
//     fn get_atttype(relid: Oid, attnum: AttrNumber) -> Oid;
//     fn get_attoptions(relid: Oid, attnum: i16) -> Datum;
//     fn get_relname_relid(relname: *const libc::c_char, relnamespace: Oid) -> Oid;
//     fn get_rel_name(relid: Oid) -> *mut libc::c_char;
//     fn get_rel_namespace(relid: Oid) -> Oid;
//     fn get_typcollation(typid: Oid) -> Oid;
//     fn get_namespace_name(nspid: Oid) -> *mut libc::c_char;
//     fn RelationGetPartitionKey(rel: Relation) -> PartitionKey;
//     fn deparse_expression(
//         expr: *mut Node,
//         dpcontext: *mut List,
//         forceprefix: bool,
//         showimplicit: bool,
//     ) -> *mut libc::c_char;
//     fn deparse_context_for(aliasname: *const libc::c_char, relid: Oid) -> *mut List;
//     fn SearchSysCache1(cacheId: libc::c_int, key1: Datum) -> HeapTuple;
//     fn ReleaseSysCache(tuple: HeapTuple);
//     fn lookup_rowtype_tupdesc(type_id: Oid, typmod: i32) -> TupleDesc;
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
pub type Pointer = *mut libc::c_char;
// pub type i16 = libc::c_short;
// pub type i32 = libc::c_int;
// pub type u8 = libc::c_uchar;
// pub type u16 = libc::c_ushort;
// pub type u32 = libc::c_uint;
pub type bits8 = u8;
pub type bits32 = u32;
// pub type i64 = libc::c_long;
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
pub type text = varlena;
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
pub type MemoryContext = *mut MemoryContextData;
// pub type Datum = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NullableDatum {
    pub value: Datum,
    pub isnull: bool,
}
pub type ScanDirection = libc::c_int;
pub const ForwardScanDirection: ScanDirection = 1;
pub const NoMovementScanDirection: ScanDirection = 0;
pub const BackwardScanDirection: ScanDirection = -1;
pub type AttrNumber = i16;
pub type StrategyNumber = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub type_0: NodeTag,
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
pub type fmNodePtr = *mut Node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr {
    pub type_0: NodeTag,
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
pub type LOCKMODE = libc::c_int;
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
pub struct Bitmapset {
    pub nwords: libc::c_int,
    pub words: [bitmapword; 0],
}
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
// pub type Relation = *mut RelationData;
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
pub type ParamFetchHook = Option<
    unsafe extern "C" fn(
        ParamListInfo,
        libc::c_int,
        bool,
        *mut ParamExternData,
    ) -> *mut ParamExternData,
>;
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
pub type MinimalTuple = *mut MinimalTupleData;
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
pub type HeapTuple = *mut HeapTupleData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapTupleData {
    pub t_len: u32,
    pub t_self: ItemPointerData,
    pub t_tableOid: Oid,
    pub t_data: HeapTupleHeader,
}
pub type HeapTupleHeader = *mut HeapTupleHeaderData;
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
pub type PartitionDirectory = *mut PartitionDirectoryData;
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
pub type XLogRecPtr = uint64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WorkerInstrumentation {
    pub num_workers: libc::c_int,
    pub instrument: [Instrumentation; 0],
}
pub type ExecProcNodeMtd = Option<unsafe extern "C" fn(*mut PlanState) -> *mut TupleTableSlot>;
pub type ExprStateEvalFunc =
    Option<unsafe extern "C" fn(*mut ExprState, *mut ExprContext, *mut bool) -> Datum>;
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
pub struct PartitionKeyData {
    pub strategy: libc::c_char,
    pub partnatts: i16,
    pub partattrs: *mut AttrNumber,
    pub partexprs: *mut List,
    pub partopfamily: *mut Oid,
    pub partopcintype: *mut Oid,
    pub partsupfunc: *mut FmgrInfo,
    pub partcollation: *mut Oid,
    pub parttypid: *mut Oid,
    pub parttypmod: *mut i32,
    pub parttyplen: *mut i16,
    pub parttypbyval: *mut bool,
    pub parttypalign: *mut libc::c_char,
    pub parttypcoll: *mut Oid,
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
pub struct RangeTblRef {
    pub type_0: NodeTag,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionBoundSpec {
    pub type_0: NodeTag,
    pub strategy: libc::c_char,
    pub is_default: bool,
    pub modulus: libc::c_int,
    pub remainder: libc::c_int,
    pub listdatums: *mut List,
    pub lowerdatums: *mut List,
    pub upperdatums: *mut List,
    pub location: libc::c_int,
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
pub type RoleSpecType = libc::c_uint;
pub const ROLESPEC_PUBLIC: RoleSpecType = 4;
pub const ROLESPEC_SESSION_USER: RoleSpecType = 3;
pub const ROLESPEC_CURRENT_USER: RoleSpecType = 2;
pub const ROLESPEC_CURRENT_ROLE: RoleSpecType = 1;
pub const ROLESPEC_CSTRING: RoleSpecType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RoleSpec {
    pub type_0: NodeTag,
    pub roletype: RoleSpecType,
    pub rolename: *mut libc::c_char,
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
pub struct ColumnDef {
    pub type_0: NodeTag,
    pub colname: *mut libc::c_char,
    pub typeName: *mut TypeName,
    pub inhcount: libc::c_int,
    pub is_local: bool,
    pub is_not_null: bool,
    pub is_from_type: bool,
    pub storage: libc::c_char,
    pub raw_default: *mut Node,
    pub cooked_default: *mut Node,
    pub identity: libc::c_char,
    pub identitySequence: *mut RangeVar,
    pub generated: libc::c_char,
    pub collClause: *mut CollateClause,
    pub collOid: Oid,
    pub constraints: *mut List,
    pub fdwoptions: *mut List,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableLikeClause {
    pub type_0: NodeTag,
    pub relation: *mut RangeVar,
    pub options: bits32,
    pub relationOid: Oid,
}
pub type TableLikeOption = libc::c_uint;
pub const CREATE_TABLE_LIKE_ALL: TableLikeOption = 2147483647;
pub const CREATE_TABLE_LIKE_STORAGE: TableLikeOption = 128;
pub const CREATE_TABLE_LIKE_STATISTICS: TableLikeOption = 64;
pub const CREATE_TABLE_LIKE_INDEXES: TableLikeOption = 32;
pub const CREATE_TABLE_LIKE_IDENTITY: TableLikeOption = 16;
pub const CREATE_TABLE_LIKE_GENERATED: TableLikeOption = 8;
pub const CREATE_TABLE_LIKE_DEFAULTS: TableLikeOption = 4;
pub const CREATE_TABLE_LIKE_CONSTRAINTS: TableLikeOption = 2;
pub const CREATE_TABLE_LIKE_COMMENTS: TableLikeOption = 1;
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
pub type DefElemAction = libc::c_uint;
pub const DEFELEM_DROP: DefElemAction = 3;
pub const DEFELEM_ADD: DefElemAction = 2;
pub const DEFELEM_SET: DefElemAction = 1;
pub const DEFELEM_UNSPEC: DefElemAction = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DefElem {
    pub type_0: NodeTag,
    pub defnamespace: *mut libc::c_char,
    pub defname: *mut libc::c_char,
    pub arg: *mut Node,
    pub defaction: DefElemAction,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionSpec {
    pub type_0: NodeTag,
    pub strategy: *mut libc::c_char,
    pub partParams: *mut List,
    pub location: libc::c_int,
}
pub type PartitionRangeDatumKind = libc::c_int;
pub const PARTITION_RANGE_DATUM_MAXVALUE: PartitionRangeDatumKind = 1;
pub const PARTITION_RANGE_DATUM_VALUE: PartitionRangeDatumKind = 0;
pub const PARTITION_RANGE_DATUM_MINVALUE: PartitionRangeDatumKind = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionRangeDatum {
    pub type_0: NodeTag,
    pub kind: PartitionRangeDatumKind,
    pub value: *mut Node,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionCmd {
    pub type_0: NodeTag,
    pub name: *mut RangeVar,
    pub bound: *mut PartitionBoundSpec,
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
pub struct CreateSchemaStmt {
    pub type_0: NodeTag,
    pub schemaname: *mut libc::c_char,
    pub authrole: *mut RoleSpec,
    pub schemaElts: *mut List,
    pub if_not_exists: bool,
}
pub type DropBehavior = libc::c_uint;
pub const DROP_CASCADE: DropBehavior = 1;
pub const DROP_RESTRICT: DropBehavior = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AlterTableStmt {
    pub type_0: NodeTag,
    pub relation: *mut RangeVar,
    pub cmds: *mut List,
    pub objtype: ObjectType,
    pub missing_ok: bool,
}
pub type AlterTableType = libc::c_uint;
pub const AT_AlterCollationRefreshVersion: AlterTableType = 67;
pub const AT_DropIdentity: AlterTableType = 66;
pub const AT_SetIdentity: AlterTableType = 65;
pub const AT_AddIdentity: AlterTableType = 64;
pub const AT_DetachPartition: AlterTableType = 63;
pub const AT_AttachPartition: AlterTableType = 62;
pub const AT_GenericOptions: AlterTableType = 61;
pub const AT_NoForceRowSecurity: AlterTableType = 60;
pub const AT_ForceRowSecurity: AlterTableType = 59;
pub const AT_DisableRowSecurity: AlterTableType = 58;
pub const AT_EnableRowSecurity: AlterTableType = 57;
pub const AT_ReplicaIdentity: AlterTableType = 56;
pub const AT_DropOf: AlterTableType = 55;
pub const AT_AddOf: AlterTableType = 54;
pub const AT_DropInherit: AlterTableType = 53;
pub const AT_AddInherit: AlterTableType = 52;
pub const AT_DisableRule: AlterTableType = 51;
pub const AT_EnableReplicaRule: AlterTableType = 50;
pub const AT_EnableAlwaysRule: AlterTableType = 49;
pub const AT_EnableRule: AlterTableType = 48;
pub const AT_DisableTrigUser: AlterTableType = 47;
pub const AT_EnableTrigUser: AlterTableType = 46;
pub const AT_DisableTrigAll: AlterTableType = 45;
pub const AT_EnableTrigAll: AlterTableType = 44;
pub const AT_DisableTrig: AlterTableType = 43;
pub const AT_EnableReplicaTrig: AlterTableType = 42;
pub const AT_EnableAlwaysTrig: AlterTableType = 41;
pub const AT_EnableTrig: AlterTableType = 40;
pub const AT_ReplaceRelOptions: AlterTableType = 39;
pub const AT_ResetRelOptions: AlterTableType = 38;
pub const AT_SetRelOptions: AlterTableType = 37;
pub const AT_SetTableSpace: AlterTableType = 36;
pub const AT_DropOids: AlterTableType = 35;
pub const AT_SetUnLogged: AlterTableType = 34;
pub const AT_SetLogged: AlterTableType = 33;
pub const AT_DropCluster: AlterTableType = 32;
pub const AT_ClusterOn: AlterTableType = 31;
pub const AT_ChangeOwner: AlterTableType = 30;
pub const AT_AlterColumnGenericOptions: AlterTableType = 29;
pub const AT_AlterColumnType: AlterTableType = 28;
pub const AT_ReAddComment: AlterTableType = 27;
pub const AT_DropConstraintRecurse: AlterTableType = 26;
pub const AT_DropConstraint: AlterTableType = 25;
pub const AT_AddIndexConstraint: AlterTableType = 24;
pub const AT_ValidateConstraintRecurse: AlterTableType = 23;
pub const AT_ValidateConstraint: AlterTableType = 22;
pub const AT_AlterConstraint: AlterTableType = 21;
pub const AT_ReAddDomainConstraint: AlterTableType = 20;
pub const AT_ReAddConstraint: AlterTableType = 19;
pub const AT_AddConstraintRecurse: AlterTableType = 18;
pub const AT_AddConstraint: AlterTableType = 17;
pub const AT_ReAddIndex: AlterTableType = 16;
pub const AT_AddIndex: AlterTableType = 15;
pub const AT_DropColumnRecurse: AlterTableType = 14;
pub const AT_DropColumn: AlterTableType = 13;
pub const AT_SetStorage: AlterTableType = 12;
pub const AT_ResetOptions: AlterTableType = 11;
pub const AT_SetOptions: AlterTableType = 10;
pub const AT_SetStatistics: AlterTableType = 9;
pub const AT_CheckNotNull: AlterTableType = 8;
pub const AT_DropExpression: AlterTableType = 7;
pub const AT_SetNotNull: AlterTableType = 6;
pub const AT_DropNotNull: AlterTableType = 5;
pub const AT_CookedColumnDefault: AlterTableType = 4;
pub const AT_ColumnDefault: AlterTableType = 3;
pub const AT_AddColumnToView: AlterTableType = 2;
pub const AT_AddColumnRecurse: AlterTableType = 1;
pub const AT_AddColumn: AlterTableType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AlterTableCmd {
    pub type_0: NodeTag,
    pub subtype: AlterTableType,
    pub name: *mut libc::c_char,
    pub object: *mut List,
    pub num: i16,
    pub newowner: *mut RoleSpec,
    pub def: *mut Node,
    pub behavior: DropBehavior,
    pub missing_ok: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreateStmt {
    pub type_0: NodeTag,
    pub relation: *mut RangeVar,
    pub tableElts: *mut List,
    pub inhRelations: *mut List,
    pub partbound: *mut PartitionBoundSpec,
    pub partspec: *mut PartitionSpec,
    pub ofTypename: *mut TypeName,
    pub constraints: *mut List,
    pub options: *mut List,
    pub oncommit: OnCommitAction,
    pub tablespacename: *mut libc::c_char,
    pub accessMethod: *mut libc::c_char,
    pub if_not_exists: bool,
}
pub type ConstrType = libc::c_uint;
pub const CONSTR_ATTR_IMMEDIATE: ConstrType = 13;
pub const CONSTR_ATTR_DEFERRED: ConstrType = 12;
pub const CONSTR_ATTR_NOT_DEFERRABLE: ConstrType = 11;
pub const CONSTR_ATTR_DEFERRABLE: ConstrType = 10;
pub const CONSTR_FOREIGN: ConstrType = 9;
pub const CONSTR_EXCLUSION: ConstrType = 8;
pub const CONSTR_UNIQUE: ConstrType = 7;
pub const CONSTR_PRIMARY: ConstrType = 6;
pub const CONSTR_CHECK: ConstrType = 5;
pub const CONSTR_GENERATED: ConstrType = 4;
pub const CONSTR_IDENTITY: ConstrType = 3;
pub const CONSTR_DEFAULT: ConstrType = 2;
pub const CONSTR_NOTNULL: ConstrType = 1;
pub const CONSTR_NULL: ConstrType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Constraint {
    pub type_0: NodeTag,
    pub contype: ConstrType,
    pub conname: *mut libc::c_char,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub location: libc::c_int,
    pub is_no_inherit: bool,
    pub raw_expr: *mut Node,
    pub cooked_expr: *mut libc::c_char,
    pub generated_when: libc::c_char,
    pub keys: *mut List,
    pub including: *mut List,
    pub exclusions: *mut List,
    pub options: *mut List,
    pub indexname: *mut libc::c_char,
    pub indexspace: *mut libc::c_char,
    pub reset_default_tblspc: bool,
    pub access_method: *mut libc::c_char,
    pub where_clause: *mut Node,
    pub pktable: *mut RangeVar,
    pub fk_attrs: *mut List,
    pub pk_attrs: *mut List,
    pub fk_matchtype: libc::c_char,
    pub fk_upd_action: libc::c_char,
    pub fk_del_action: libc::c_char,
    pub old_conpfeqop: *mut List,
    pub old_pktable_oid: Oid,
    pub skip_validation: bool,
    pub initially_valid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreateTrigStmt {
    pub type_0: NodeTag,
    pub replace: bool,
    pub isconstraint: bool,
    pub trigname: *mut libc::c_char,
    pub relation: *mut RangeVar,
    pub funcname: *mut List,
    pub args: *mut List,
    pub row: bool,
    pub timing: i16,
    pub events: i16,
    pub columns: *mut List,
    pub whenClause: *mut Node,
    pub transitionRels: *mut List,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub constrrel: *mut RangeVar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreateSeqStmt {
    pub type_0: NodeTag,
    pub sequence: *mut RangeVar,
    pub options: *mut List,
    pub ownerId: Oid,
    pub for_identity: bool,
    pub if_not_exists: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AlterSeqStmt {
    pub type_0: NodeTag,
    pub sequence: *mut RangeVar,
    pub options: *mut List,
    pub for_identity: bool,
    pub missing_ok: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommentStmt {
    pub type_0: NodeTag,
    pub objtype: ObjectType,
    pub object: *mut Node,
    pub comment: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexStmt {
    pub type_0: NodeTag,
    pub idxname: *mut libc::c_char,
    pub relation: *mut RangeVar,
    pub accessMethod: *mut libc::c_char,
    pub tableSpace: *mut libc::c_char,
    pub indexParams: *mut List,
    pub indexIncludingParams: *mut List,
    pub options: *mut List,
    pub whereClause: *mut Node,
    pub excludeOpNames: *mut List,
    pub idxcomment: *mut libc::c_char,
    pub indexOid: Oid,
    pub oldNode: Oid,
    pub oldCreateSubid: SubTransactionId,
    pub oldFirstRelfilenodeSubid: SubTransactionId,
    pub unique: bool,
    pub primary: bool,
    pub isconstraint: bool,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub transformed: bool,
    pub concurrent: bool,
    pub if_not_exists: bool,
    pub reset_default_tblspc: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreateStatsStmt {
    pub type_0: NodeTag,
    pub defnames: *mut List,
    pub stat_types: *mut List,
    pub exprs: *mut List,
    pub relations: *mut List,
    pub stxcomment: *mut libc::c_char,
    pub if_not_exists: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RuleStmt {
    pub type_0: NodeTag,
    pub relation: *mut RangeVar,
    pub rulename: *mut libc::c_char,
    pub whereClause: *mut Node,
    pub event: CmdType,
    pub instead: bool,
    pub actions: *mut List,
    pub replace: bool,
}
pub type ViewCheckOption = libc::c_uint;
pub const CASCADED_CHECK_OPTION: ViewCheckOption = 2;
pub const LOCAL_CHECK_OPTION: ViewCheckOption = 1;
pub const NO_CHECK_OPTION: ViewCheckOption = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ViewStmt {
    pub type_0: NodeTag,
    pub view: *mut RangeVar,
    pub aliases: *mut List,
    pub query: *mut Node,
    pub replace: bool,
    pub options: *mut List,
    pub withCheckOption: ViewCheckOption,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ArrayType {
    pub vl_len_: i32,
    pub ndim: libc::c_int,
    pub dataoffset: i32,
    pub elemtype: Oid,
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
pub struct FormData_pg_am {
    pub oid: Oid,
    pub amname: NameData,
    pub amhandler: regproc,
    pub amtype: libc::c_char,
}
pub type Form_pg_am = *mut FormData_pg_am;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_collation {
    pub oid: Oid,
    pub collname: NameData,
    pub collnamespace: Oid,
    pub collowner: Oid,
    pub collprovider: libc::c_char,
    pub collisdeterministic: bool,
    pub collencoding: i32,
    pub collcollate: NameData,
    pub collctype: NameData,
}
pub type Form_pg_collation = *mut FormData_pg_collation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_constraint {
    pub oid: Oid,
    pub conname: NameData,
    pub connamespace: Oid,
    pub contype: libc::c_char,
    pub condeferrable: bool,
    pub condeferred: bool,
    pub convalidated: bool,
    pub conrelid: Oid,
    pub contypid: Oid,
    pub conindid: Oid,
    pub conparentid: Oid,
    pub confrelid: Oid,
    pub confupdtype: libc::c_char,
    pub confdeltype: libc::c_char,
    pub confmatchtype: libc::c_char,
    pub conislocal: bool,
    pub coninhcount: i32,
    pub connoinherit: bool,
}
pub type Form_pg_constraint = *mut FormData_pg_constraint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_opclass {
    pub oid: Oid,
    pub opcmethod: Oid,
    pub opcname: NameData,
    pub opcnamespace: Oid,
    pub opcowner: Oid,
    pub opcfamily: Oid,
    pub opcintype: Oid,
    pub opcdefault: bool,
    pub opckeytype: Oid,
}
pub type Form_pg_opclass = *mut FormData_pg_opclass;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_operator {
    pub oid: Oid,
    pub oprname: NameData,
    pub oprnamespace: Oid,
    pub oprowner: Oid,
    pub oprkind: libc::c_char,
    pub oprcanmerge: bool,
    pub oprcanhash: bool,
    pub oprleft: Oid,
    pub oprright: Oid,
    pub oprresult: Oid,
    pub oprcom: Oid,
    pub oprnegate: Oid,
    pub oprcode: regproc,
    pub oprrest: regproc,
    pub oprjoin: regproc,
}
pub type Form_pg_operator = *mut FormData_pg_operator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_statistic_ext {
    pub oid: Oid,
    pub stxrelid: Oid,
    pub stxname: NameData,
    pub stxnamespace: Oid,
    pub stxowner: Oid,
    pub stxstattarget: i32,
    pub stxkeys: int2vector,
}
pub type Form_pg_statistic_ext = *mut FormData_pg_statistic_ext;
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
pub type Type = HeapTuple;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreateStmtContext {
    pub pstate: *mut ParseState,
    pub stmtType: *const libc::c_char,
    pub relation: *mut RangeVar,
    pub rel: Relation,
    pub inhRelations: *mut List,
    pub isforeign: bool,
    pub isalter: bool,
    pub columns: *mut List,
    pub ckconstraints: *mut List,
    pub fkconstraints: *mut List,
    pub ixconstraints: *mut List,
    pub likeclauses: *mut List,
    pub extstats: *mut List,
    pub blist: *mut List,
    pub alist: *mut List,
    pub pkey: *mut IndexStmt,
    pub ispartitioned: bool,
    pub partbound: *mut PartitionBoundSpec,
    pub ofType: bool,
}
pub const STATEXTOID: SysCacheIdentifier = 58;
pub type AclResult = libc::c_uint;
pub const ACLCHECK_NOT_OWNER: AclResult = 2;
pub const ACLCHECK_NO_PRIV: AclResult = 1;
pub const ACLCHECK_OK: AclResult = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CreateSchemaStmtContext {
    pub stmtType: *const libc::c_char,
    pub schemaname: *mut libc::c_char,
    pub authrole: *mut RoleSpec,
    pub sequences: *mut List,
    pub tables: *mut List,
    pub views: *mut List,
    pub indexes: *mut List,
    pub triggers: *mut List,
    pub grants: *mut List,
}
pub const CLAOID: SysCacheIdentifier = 14;
pub const COLLOID: SysCacheIdentifier = 16;
pub const OPEROID: SysCacheIdentifier = 38;
pub const CONSTROID: SysCacheIdentifier = 19;
pub const AMOID: SysCacheIdentifier = 2;
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
pub const PROCOID: SysCacheIdentifier = 43;
pub const PROCNAMEARGSNSP: SysCacheIdentifier = 42;
pub const PARTRELID: SysCacheIdentifier = 41;
pub const OPFAMILYOID: SysCacheIdentifier = 40;
pub const OPFAMILYAMNAMENSP: SysCacheIdentifier = 39;
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
pub const CONNAMENSP: SysCacheIdentifier = 18;
pub const CONDEFAULT: SysCacheIdentifier = 17;
pub const COLLNAMEENCNSP: SysCacheIdentifier = 15;
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
pub const AMNAME: SysCacheIdentifier = 1;
pub const AGGFNOID: SysCacheIdentifier = 0;
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
#[inline]
unsafe extern "C" fn get_partition_strategy(mut key: PartitionKey) -> libc::c_int {
    return (*key).strategy as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_partition_natts(mut key: PartitionKey) -> libc::c_int {
    return (*key).partnatts as libc::c_int;
}
#[inline]
unsafe extern "C" fn get_partition_exprs(mut key: PartitionKey) -> *mut List {
    return (*key).partexprs;
}
#[inline]
unsafe extern "C" fn get_partition_col_typid(mut key: PartitionKey, mut col: libc::c_int) -> Oid {
    return *((*key).parttypid).offset(col as isize);
}
#[inline]
unsafe extern "C" fn get_partition_col_typmod(mut key: PartitionKey, mut col: libc::c_int) -> i32 {
    return *((*key).parttypmod).offset(col as isize);
}
#[inline]
unsafe extern "C" fn get_partition_col_collation(
    mut key: PartitionKey,
    mut col: libc::c_int,
) -> Oid {
    return *((*key).partcollation).offset(col as isize);
}
unsafe extern "C" fn generateSerialExtraStmts(
    mut cxt: *mut CreateStmtContext,
    mut column: *mut ColumnDef,
    mut seqtypid: Oid,
    mut seqoptions: *mut List,
    mut for_identity: bool,
    mut col_exists: bool,
    mut snamespace_p: *mut *mut libc::c_char,
    mut sname_p: *mut *mut libc::c_char,
) {
    let mut option: *mut ListCell = 0 as *mut ListCell;
    let mut nameEl: *mut DefElem = 0 as *mut DefElem;
    let mut snamespaceid: Oid = 0;
    let mut snamespace: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut seqstmt: *mut CreateSeqStmt = 0 as *mut CreateSeqStmt;
    let mut altseqstmt: *mut AlterSeqStmt = 0 as *mut AlterSeqStmt;
    let mut attnamelist: *mut List = 0 as *mut List;
    let mut nameEl_idx: libc::c_int = -(1 as libc::c_int);
    let mut option__state: ForEachState = {
        let mut init = ForEachState {
            l: seqoptions,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(option__state.l).is_null() && option__state.i < (*option__state.l).length {
        option =
            &mut *((*option__state.l).elements).offset(option__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        option = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut defel: *mut DefElem = (*option).ptr_value as *mut DefElem;
        if strcmp(
            (*defel).defname,
            b"sequence_name\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if !nameEl.is_null() {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            nameEl = defel;
            nameEl_idx = option__state.i;
        }
        option__state.i += 1;
        option__state.i;
    }
    if !nameEl.is_null() {
        let mut rv: *mut RangeVar = makeRangeVarFromNameList((*nameEl).arg as *mut List);
        snamespace = (*rv).schemaname;
        if snamespace.is_null() {
            if !((*cxt).rel).is_null() {
                snamespaceid = (*(*(*cxt).rel).rd_rel).relnamespace;
            } else {
                snamespaceid = RangeVarGetCreationNamespace((*cxt).relation);
            }
            snamespace = get_namespace_name(snamespaceid);
        }
        sname = (*rv).relname;
        seqoptions = list_delete_nth_cell(seqoptions, nameEl_idx);
    } else {
        if !((*cxt).rel).is_null() {
            snamespaceid = (*(*(*cxt).rel).rd_rel).relnamespace;
        } else {
            snamespaceid = RangeVarGetCreationNamespace((*cxt).relation);
            RangeVarAdjustRelationPersistence((*cxt).relation, snamespaceid);
        }
        snamespace = get_namespace_name(snamespaceid);
        sname = ChooseRelationName(
            (*(*cxt).relation).relname,
            (*column).colname,
            b"seq\0" as *const u8 as *const libc::c_char,
            snamespaceid,
            false,
        );
    }
    let elevel__0: libc::c_int = 14 as libc::c_int;
    let mut __error_0: libc::c_int = 0;
    if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
        errmsg_internal(
            b"%s will create implicit sequence \"%s\" for serial column \"%s.%s\"\0" as *const u8
                as *const libc::c_char,
            (*cxt).stmtType,
            sname,
            (*(*cxt).relation).relname,
            (*column).colname,
        );
        errfinish(
            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                as *const u8 as *const libc::c_char,
            448 as libc::c_int,
            0 as *const libc::c_char,
        );
    }
    if elevel__0 >= 21 as libc::c_int {
        abort();
    }
    seqstmt = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_CreateSeqStmt;
        _result
    }) as *mut CreateSeqStmt;
    (*seqstmt).for_identity = for_identity;
    (*seqstmt).sequence = makeRangeVar(snamespace, sname, -(1 as libc::c_int));
    (*seqstmt).options = seqoptions;
    if seqtypid != 0 {
        (*seqstmt).options = lcons(
            makeDefElem(
                b"as\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                makeTypeNameFromOid(seqtypid, -(1 as libc::c_int)) as *mut Node,
                -(1 as libc::c_int),
            ) as *mut libc::c_void,
            (*seqstmt).options,
        );
    }
    if !((*cxt).rel).is_null() {
        (*seqstmt).ownerId = (*(*(*cxt).rel).rd_rel).relowner;
    } else {
        (*seqstmt).ownerId = 0 as libc::c_int as Oid;
    }
    (*cxt).blist = lappend((*cxt).blist, seqstmt as *mut libc::c_void);
    (*column).identitySequence = (*seqstmt).sequence;
    altseqstmt = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_AlterSeqStmt;
        _result
    }) as *mut AlterSeqStmt;
    (*altseqstmt).sequence = makeRangeVar(snamespace, sname, -(1 as libc::c_int));
    attnamelist = list_make3_impl(
        T_List,
        ListCell {
            ptr_value: makeString(snamespace) as *mut libc::c_void,
        },
        ListCell {
            ptr_value: makeString((*(*cxt).relation).relname) as *mut libc::c_void,
        },
        ListCell {
            ptr_value: makeString((*column).colname) as *mut libc::c_void,
        },
    );
    (*altseqstmt).options = list_make1_impl(
        T_List,
        ListCell {
            ptr_value: makeDefElem(
                b"owned_by\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                attnamelist as *mut Node,
                -(1 as libc::c_int),
            ) as *mut libc::c_void,
        },
    );
    (*altseqstmt).for_identity = for_identity;
    if col_exists != 0 {
        (*cxt).blist = lappend((*cxt).blist, altseqstmt as *mut libc::c_void);
    } else {
        (*cxt).alist = lappend((*cxt).alist, altseqstmt as *mut libc::c_void);
    }
    if !snamespace_p.is_null() {
        *snamespace_p = snamespace;
    }
    if !sname_p.is_null() {
        *sname_p = sname;
    }
}
unsafe extern "C" fn transformColumnDefinition(
    mut cxt: *mut CreateStmtContext,
    mut column: *mut ColumnDef,
) {
    let mut is_serial: bool = false;
    let mut saw_nullable: bool = false;
    let mut saw_default: bool = false;
    let mut saw_identity: bool = false;
    let mut saw_generated: bool = false;
    let mut clist: *mut ListCell = 0 as *mut ListCell;
    (*cxt).columns = lappend((*cxt).columns, column as *mut libc::c_void);
    is_serial = false;
    if !((*column).typeName).is_null()
        && list_length((*(*column).typeName).names) == 1 as libc::c_int
        && (*(*column).typeName).pct_type == 0
    {
        let mut typname: *mut libc::c_char =
            (*((*list_nth_cell((*(*column).typeName).names, 0 as libc::c_int)).ptr_value
                as *mut Value))
                .val
                .str_0;
        if strcmp(
            typname,
            b"smallserial\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(typname, b"serial2\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            is_serial = true;
            (*(*column).typeName).names = 0 as *mut libc::c_void as *mut List;
        } else if strcmp(typname, b"serial\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(typname, b"serial4\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            is_serial = true;
            (*(*column).typeName).names = 0 as *mut libc::c_void as *mut List;
        } else if strcmp(typname, b"bigserial\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
            || strcmp(typname, b"serial8\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            is_serial = true;
            (*(*column).typeName).names = 0 as *mut libc::c_void as *mut List;
        }
        if is_serial as libc::c_int != 0 && !((*(*column).typeName).arrayBounds).is_null() {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
    }
    if !((*column).typeName).is_null() {
        transformColumnType(cxt, column);
    }
    if is_serial != 0 {
        let mut snamespace: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut sname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut qstring: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut snamenode: *mut A_Const = 0 as *mut A_Const;
        let mut castnode: *mut TypeCast = 0 as *mut TypeCast;
        let mut funccallnode: *mut FuncCall = 0 as *mut FuncCall;
        let mut constraint: *mut Constraint = 0 as *mut Constraint;
        generateSerialExtraStmts(
            cxt,
            column,
            (*(*column).typeName).typeOid,
            0 as *mut libc::c_void as *mut List,
            false,
            false,
            &mut snamespace,
            &mut sname,
        );
        qstring = quote_qualified_identifier(snamespace, sname);
        snamenode = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_A_Const;
            _result
        }) as *mut A_Const;
        (*snamenode).val.type_0 = T_String;
        (*snamenode).val.val.str_0 = qstring;
        (*snamenode).location = -(1 as libc::c_int);
        castnode = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_TypeCast;
            _result
        }) as *mut TypeCast;
        (*castnode).typeName =
            SystemTypeName(b"regclass\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        (*castnode).arg = snamenode as *mut Node;
        (*castnode).location = -(1 as libc::c_int);
        funccallnode = makeFuncCall(
            SystemFuncName(b"nextval\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
            list_make1_impl(
                T_List,
                ListCell {
                    ptr_value: castnode as *mut libc::c_void,
                },
            ),
            COERCE_EXPLICIT_CALL,
            -(1 as libc::c_int),
        );
        constraint = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_Constraint;
            _result
        }) as *mut Constraint;
        (*constraint).contype = CONSTR_DEFAULT;
        (*constraint).location = -(1 as libc::c_int);
        (*constraint).raw_expr = funccallnode as *mut Node;
        (*constraint).cooked_expr = 0 as *mut libc::c_char;
        (*column).constraints = lappend((*column).constraints, constraint as *mut libc::c_void);
        constraint = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_Constraint;
            _result
        }) as *mut Constraint;
        (*constraint).contype = CONSTR_NOTNULL;
        (*constraint).location = -(1 as libc::c_int);
        (*column).constraints = lappend((*column).constraints, constraint as *mut libc::c_void);
    }
    transformConstraintAttrs(cxt, (*column).constraints);
    saw_nullable = false;
    saw_default = false;
    saw_identity = false;
    saw_generated = false;
    let mut clist__state: ForEachState = {
        let mut init = ForEachState {
            l: (*column).constraints,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(clist__state.l).is_null() && clist__state.i < (*clist__state.l).length {
        clist = &mut *((*clist__state.l).elements).offset(clist__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        clist = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut constraint_0: *mut Constraint = (*clist).ptr_value as *mut Constraint;
        let mut current_block_145: u64;
        match (*constraint_0).contype as libc::c_uint {
            0 => {
                if saw_nullable as libc::c_int != 0 && (*column).is_not_null as libc::c_int != 0 {
                    let elevel__0: libc::c_int = 21 as libc::c_int;
                    let mut __error_0: libc::c_int = 0;
                    if elevel__0 >= 21 as libc::c_int {
                        abort();
                    }
                }
                (*column).is_not_null = false;
                saw_nullable = true;
                current_block_145 = 11957990509374275265;
            }
            1 => {
                if saw_nullable as libc::c_int != 0 && (*column).is_not_null == 0 {
                    let elevel__1: libc::c_int = 21 as libc::c_int;
                    let mut __error_1: libc::c_int = 0;
                    if elevel__1 >= 21 as libc::c_int {
                        abort();
                    }
                }
                (*column).is_not_null = true;
                saw_nullable = true;
                current_block_145 = 11957990509374275265;
            }
            2 => {
                if saw_default != 0 {
                    let elevel__2: libc::c_int = 21 as libc::c_int;
                    let mut __error_2: libc::c_int = 0;
                    if elevel__2 >= 21 as libc::c_int {
                        abort();
                    }
                }
                (*column).raw_default = (*constraint_0).raw_expr;
                saw_default = true;
                current_block_145 = 11957990509374275265;
            }
            3 => {
                let mut ctype: Type = 0 as *mut HeapTupleData;
                let mut typeOid: Oid = 0;
                if (*cxt).ofType != 0 {
                    let elevel__3: libc::c_int = 21 as libc::c_int;
                    let mut __error_3: libc::c_int = 0;
                    if elevel__3 >= 21 as libc::c_int {
                        abort();
                    }
                }
                if !((*cxt).partbound).is_null() {
                    let elevel__4: libc::c_int = 21 as libc::c_int;
                    let mut __error_4: libc::c_int = 0;
                    if elevel__4 >= 21 as libc::c_int {
                        abort();
                    }
                }
                ctype = typenameType((*cxt).pstate, (*column).typeName, 0 as *mut i32);
                typeOid = (*(((*ctype).t_data as *mut libc::c_char)
                    .offset((*(*ctype).t_data).t_hoff as libc::c_int as isize)
                    as Form_pg_type))
                    .oid;
                ReleaseSysCache(ctype);
                if saw_identity != 0 {
                    let elevel__5: libc::c_int = 21 as libc::c_int;
                    let mut __error_5: libc::c_int = 0;
                    if elevel__5 >= 21 as libc::c_int {
                        abort();
                    }
                }
                generateSerialExtraStmts(
                    cxt,
                    column,
                    typeOid,
                    (*constraint_0).options,
                    true,
                    false,
                    0 as *mut *mut libc::c_char,
                    0 as *mut *mut libc::c_char,
                );
                (*column).identity = (*constraint_0).generated_when;
                saw_identity = true;
                (*column).is_not_null = true;
                current_block_145 = 11957990509374275265;
            }
            4 => {
                if (*cxt).ofType != 0 {
                    let elevel__6: libc::c_int = 21 as libc::c_int;
                    let mut __error_6: libc::c_int = 0;
                    if elevel__6 >= 21 as libc::c_int {
                        abort();
                    }
                }
                if !((*cxt).partbound).is_null() {
                    let elevel__7: libc::c_int = 21 as libc::c_int;
                    let mut __error_7: libc::c_int = 0;
                    if elevel__7 >= 21 as libc::c_int {
                        abort();
                    }
                }
                if saw_generated != 0 {
                    let elevel__8: libc::c_int = 21 as libc::c_int;
                    let mut __error_8: libc::c_int = 0;
                    if elevel__8 >= 21 as libc::c_int {
                        abort();
                    }
                }
                (*column).raw_default = (*constraint_0).raw_expr;
                saw_generated = true;
                current_block_145 = 11957990509374275265;
            }
            5 => {
                (*cxt).ckconstraints =
                    lappend((*cxt).ckconstraints, constraint_0 as *mut libc::c_void);
                current_block_145 = 11957990509374275265;
            }
            6 => {
                if (*cxt).isforeign != 0 {
                    let elevel__9: libc::c_int = 21 as libc::c_int;
                    let mut __error_9: libc::c_int = 0;
                    if elevel__9 >= 21 as libc::c_int {
                        abort();
                    }
                }
                current_block_145 = 1003796805849494897;
            }
            7 => {
                current_block_145 = 1003796805849494897;
            }
            8 => {
                let elevel__11: libc::c_int = 21 as libc::c_int;
                let mut __error_11: libc::c_int = 0;
                if errstart(elevel__11, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"column exclusion constraints are not supported\0" as *const u8
                            as *const libc::c_char,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                            as *const u8 as *const libc::c_char,
                        776 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel__11 >= 21 as libc::c_int {
                    abort();
                }
                current_block_145 = 11957990509374275265;
            }
            9 => {
                if (*cxt).isforeign != 0 {
                    let elevel__12: libc::c_int = 21 as libc::c_int;
                    let mut __error_12: libc::c_int = 0;
                    if elevel__12 >= 21 as libc::c_int {
                        abort();
                    }
                }
                (*constraint_0).fk_attrs = list_make1_impl(
                    T_List,
                    ListCell {
                        ptr_value: makeString((*column).colname) as *mut libc::c_void,
                    },
                );
                (*cxt).fkconstraints =
                    lappend((*cxt).fkconstraints, constraint_0 as *mut libc::c_void);
                current_block_145 = 11957990509374275265;
            }
            10 | 11 | 12 | 13 => {
                current_block_145 = 11957990509374275265;
            }
            _ => {
                let elevel__13: libc::c_int = 21 as libc::c_int;
                let mut __error_13: libc::c_int = 0;
                if errstart(elevel__13, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"unrecognized constraint type: %d\0" as *const u8 as *const libc::c_char,
                        (*constraint_0).contype as libc::c_uint,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                            as *const u8 as *const libc::c_char,
                        804 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel__13 >= 21 as libc::c_int {
                    abort();
                }
                current_block_145 = 11957990509374275265;
            }
        }
        match current_block_145 {
            1003796805849494897 => {
                if (*cxt).isforeign != 0 {
                    let elevel__10: libc::c_int = 21 as libc::c_int;
                    let mut __error_10: libc::c_int = 0;
                    if elevel__10 >= 21 as libc::c_int {
                        abort();
                    }
                }
                if ((*constraint_0).keys).is_null() {
                    (*constraint_0).keys = list_make1_impl(
                        T_List,
                        ListCell {
                            ptr_value: makeString((*column).colname) as *mut libc::c_void,
                        },
                    );
                }
                (*cxt).ixconstraints =
                    lappend((*cxt).ixconstraints, constraint_0 as *mut libc::c_void);
            }
            _ => {}
        }
        if saw_default as libc::c_int != 0 && saw_identity as libc::c_int != 0 {
            let elevel__14: libc::c_int = 21 as libc::c_int;
            let mut __error_14: libc::c_int = 0;
            if elevel__14 >= 21 as libc::c_int {
                abort();
            }
        }
        if saw_default as libc::c_int != 0 && saw_generated as libc::c_int != 0 {
            let elevel__15: libc::c_int = 21 as libc::c_int;
            let mut __error_15: libc::c_int = 0;
            if elevel__15 >= 21 as libc::c_int {
                abort();
            }
        }
        if saw_identity as libc::c_int != 0 && saw_generated as libc::c_int != 0 {
            let elevel__16: libc::c_int = 21 as libc::c_int;
            let mut __error_16: libc::c_int = 0;
            if elevel__16 >= 21 as libc::c_int {
                abort();
            }
        }
        clist__state.i += 1;
        clist__state.i;
    }
    if !((*column).fdwoptions).is_null() {
        let mut stmt: *mut AlterTableStmt = 0 as *mut AlterTableStmt;
        let mut cmd: *mut AlterTableCmd = 0 as *mut AlterTableCmd;
        cmd = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_AlterTableCmd;
            _result
        }) as *mut AlterTableCmd;
        (*cmd).subtype = AT_AlterColumnGenericOptions;
        (*cmd).name = (*column).colname;
        (*cmd).def = (*column).fdwoptions as *mut Node;
        (*cmd).behavior = DROP_RESTRICT;
        (*cmd).missing_ok = false;
        stmt = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_AlterTableStmt;
            _result
        }) as *mut AlterTableStmt;
        (*stmt).relation = (*cxt).relation;
        (*stmt).cmds = 0 as *mut libc::c_void as *mut List;
        (*stmt).objtype = OBJECT_FOREIGN_TABLE;
        (*stmt).cmds = lappend((*stmt).cmds, cmd as *mut libc::c_void);
        (*cxt).alist = lappend((*cxt).alist, stmt as *mut libc::c_void);
    }
}
unsafe extern "C" fn transformTableConstraint(
    mut cxt: *mut CreateStmtContext,
    mut constraint: *mut Constraint,
) {
    match (*constraint).contype as libc::c_uint {
        6 => {
            if (*cxt).isforeign != 0 {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            (*cxt).ixconstraints = lappend((*cxt).ixconstraints, constraint as *mut libc::c_void);
        }
        7 => {
            if (*cxt).isforeign != 0 {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            (*cxt).ixconstraints = lappend((*cxt).ixconstraints, constraint as *mut libc::c_void);
        }
        8 => {
            if (*cxt).isforeign != 0 {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
            if (*cxt).ispartitioned != 0 {
                let elevel__2: libc::c_int = 21 as libc::c_int;
                let mut __error_2: libc::c_int = 0;
                if elevel__2 >= 21 as libc::c_int {
                    abort();
                }
            }
            (*cxt).ixconstraints = lappend((*cxt).ixconstraints, constraint as *mut libc::c_void);
        }
        5 => {
            (*cxt).ckconstraints = lappend((*cxt).ckconstraints, constraint as *mut libc::c_void);
        }
        9 => {
            if (*cxt).isforeign != 0 {
                let elevel__3: libc::c_int = 21 as libc::c_int;
                let mut __error_3: libc::c_int = 0;
                if elevel__3 >= 21 as libc::c_int {
                    abort();
                }
            }
            (*cxt).fkconstraints = lappend((*cxt).fkconstraints, constraint as *mut libc::c_void);
        }
        0 | 1 | 2 | 10 | 11 | 12 | 13 => {
            let elevel__4: libc::c_int = 21 as libc::c_int;
            let mut __error_4: libc::c_int = 0;
            if errstart(elevel__4, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"invalid context for constraint type %d\0" as *const u8 as *const libc::c_char,
                    (*constraint).contype as libc::c_uint,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                        as *const u8 as *const libc::c_char,
                    926 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel__4 >= 21 as libc::c_int {
                abort();
            }
        }
        _ => {
            let elevel__5: libc::c_int = 21 as libc::c_int;
            let mut __error_5: libc::c_int = 0;
            if errstart(elevel__5, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unrecognized constraint type: %d\0" as *const u8 as *const libc::c_char,
                    (*constraint).contype as libc::c_uint,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                        as *const u8 as *const libc::c_char,
                    931 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel__5 >= 21 as libc::c_int {
                abort();
            }
        }
    };
}
unsafe extern "C" fn transformOfType(
    mut cxt: *mut CreateStmtContext,
    mut ofTypename: *mut TypeName,
) {
    let mut tuple: HeapTuple = 0 as *mut HeapTupleData;
    let mut tupdesc: TupleDesc = 0 as *mut TupleDescData;
    let mut i: libc::c_int = 0;
    let mut ofTypeId: Oid = 0;
    tuple = typenameType(0 as *mut ParseState, ofTypename, 0 as *mut i32);
    check_of_type(tuple);
    ofTypeId = (*(((*tuple).t_data as *mut libc::c_char)
        .offset((*(*tuple).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_type))
        .oid;
    (*ofTypename).typeOid = ofTypeId;
    tupdesc = lookup_rowtype_tupdesc(ofTypeId, -(1 as libc::c_int));
    i = 0 as libc::c_int;
    while i < (*tupdesc).natts {
        let mut attr: Form_pg_attribute =
            &mut *((*tupdesc).attrs).as_mut_ptr().offset(i as isize) as *mut FormData_pg_attribute;
        let mut n: *mut ColumnDef = 0 as *mut ColumnDef;
        if !((*attr).attisdropped != 0) {
            n = ({
                let mut _result: *mut Node = 0 as *mut Node;
                (*_result).type_0 = T_ColumnDef;
                _result
            }) as *mut ColumnDef;
            (*n).colname = pstrdup(((*attr).attname.data).as_mut_ptr());
            (*n).typeName = makeTypeNameFromOid((*attr).atttypid, (*attr).atttypmod);
            (*n).inhcount = 0 as libc::c_int;
            (*n).is_local = true;
            (*n).is_not_null = false;
            (*n).is_from_type = true;
            (*n).storage = 0 as libc::c_int as libc::c_char;
            (*n).raw_default = 0 as *mut Node;
            (*n).cooked_default = 0 as *mut Node;
            (*n).collClause = 0 as *mut CollateClause;
            (*n).collOid = (*attr).attcollation;
            (*n).constraints = 0 as *mut libc::c_void as *mut List;
            (*n).location = -(1 as libc::c_int);
            (*cxt).columns = lappend((*cxt).columns, n as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    DecrTupleDescRefCount(tupdesc);
    ReleaseSysCache(tuple);
}
unsafe extern "C" fn get_collation(mut collation: Oid, mut actual_datatype: Oid) -> *mut List {
    let mut result: *mut List = 0 as *mut List;
    let mut ht_coll: HeapTuple = 0 as *mut HeapTupleData;
    let mut coll_rec: Form_pg_collation = 0 as *mut FormData_pg_collation;
    let mut nsp_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut coll_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if (collation != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
        return 0 as *mut libc::c_void as *mut List;
    }
    if collation == get_typcollation(actual_datatype) {
        return 0 as *mut libc::c_void as *mut List;
    }
    ht_coll = SearchSysCache1(COLLOID as libc::c_int, collation as Datum);
    if (ht_coll as *const libc::c_void).is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"cache lookup failed for collation %u\0" as *const u8 as *const libc::c_char,
                collation,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                    as *const u8 as *const libc::c_char,
                1955 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    coll_rec = ((*ht_coll).t_data as *mut libc::c_char)
        .offset((*(*ht_coll).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_collation;
    nsp_name = get_namespace_name((*coll_rec).collnamespace);
    coll_name = pstrdup(((*coll_rec).collname.data).as_mut_ptr());
    result = list_make2_impl(
        T_List,
        ListCell {
            ptr_value: makeString(nsp_name) as *mut libc::c_void,
        },
        ListCell {
            ptr_value: makeString(coll_name) as *mut libc::c_void,
        },
    );
    ReleaseSysCache(ht_coll);
    return result;
}
unsafe extern "C" fn get_opclass(mut opclass: Oid, mut actual_datatype: Oid) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut ht_opc: HeapTuple = 0 as *mut HeapTupleData;
    let mut opc_rec: Form_pg_opclass = 0 as *mut FormData_pg_opclass;
    ht_opc = SearchSysCache1(CLAOID as libc::c_int, opclass as Datum);
    if (ht_opc as *const libc::c_void).is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"cache lookup failed for opclass %u\0" as *const u8 as *const libc::c_char,
                opclass,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                    as *const u8 as *const libc::c_char,
                1982 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    opc_rec = ((*ht_opc).t_data as *mut libc::c_char)
        .offset((*(*ht_opc).t_data).t_hoff as libc::c_int as isize)
        as Form_pg_opclass;
    if GetDefaultOpClass(actual_datatype, (*opc_rec).opcmethod) != opclass {
        let mut nsp_name: *mut libc::c_char = get_namespace_name((*opc_rec).opcnamespace);
        let mut opc_name: *mut libc::c_char = pstrdup(((*opc_rec).opcname.data).as_mut_ptr());
        result = list_make2_impl(
            T_List,
            ListCell {
                ptr_value: makeString(nsp_name) as *mut libc::c_void,
            },
            ListCell {
                ptr_value: makeString(opc_name) as *mut libc::c_void,
            },
        );
    }
    ReleaseSysCache(ht_opc);
    return result;
}
unsafe extern "C" fn transformIndexConstraints(mut cxt: *mut CreateStmtContext) {
    let mut index: *mut IndexStmt = 0 as *mut IndexStmt;
    let mut indexlist: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut finalindexlist: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*cxt).ixconstraints,
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
        let mut constraint: *mut Constraint = (*lc).ptr_value as *mut Constraint;
        index = transformIndexConstraint(constraint, cxt);
        indexlist = lappend(indexlist, index as *mut libc::c_void);
        lc__state.i += 1;
        lc__state.i;
    }
    if !((*cxt).pkey).is_null() {
        finalindexlist = list_make1_impl(
            T_List,
            ListCell {
                ptr_value: (*cxt).pkey as *mut libc::c_void,
            },
        );
    }
    let mut lc__state_0: ForEachState = {
        let mut init = ForEachState {
            l: indexlist,
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
        let mut keep: bool = true;
        let mut k: *mut ListCell = 0 as *mut ListCell;
        index = (*lc).ptr_value as *mut IndexStmt;
        if !(index == (*cxt).pkey) {
            let mut k__state: ForEachState = {
                let mut init = ForEachState {
                    l: finalindexlist,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(k__state.l).is_null() && k__state.i < (*k__state.l).length {
                k = &mut *((*k__state.l).elements).offset(k__state.i as isize) as *mut ListCell;
                true as libc::c_int
            } else {
                k = 0 as *mut ListCell;
                false as libc::c_int
            } != 0
            {
                let mut priorindex: *mut IndexStmt = (*k).ptr_value as *mut IndexStmt;
                if equal(
                    (*index).indexParams as *const libc::c_void,
                    (*priorindex).indexParams as *const libc::c_void,
                ) as libc::c_int
                    != 0
                    && equal(
                        (*index).indexIncludingParams as *const libc::c_void,
                        (*priorindex).indexIncludingParams as *const libc::c_void,
                    ) as libc::c_int
                        != 0
                    && equal(
                        (*index).whereClause as *const libc::c_void,
                        (*priorindex).whereClause as *const libc::c_void,
                    ) as libc::c_int
                        != 0
                    && equal(
                        (*index).excludeOpNames as *const libc::c_void,
                        (*priorindex).excludeOpNames as *const libc::c_void,
                    ) as libc::c_int
                        != 0
                    && strcmp((*index).accessMethod, (*priorindex).accessMethod) == 0 as libc::c_int
                    && (*index).deferrable as libc::c_int == (*priorindex).deferrable as libc::c_int
                    && (*index).initdeferred as libc::c_int
                        == (*priorindex).initdeferred as libc::c_int
                {
                    (*priorindex).unique = ((*priorindex).unique as libc::c_int
                        | (*index).unique as libc::c_int)
                        as bool;
                    if ((*priorindex).idxname).is_null() {
                        (*priorindex).idxname = (*index).idxname;
                    }
                    keep = false;
                    break;
                } else {
                    k__state.i += 1;
                    k__state.i;
                }
            }
            if keep != 0 {
                finalindexlist = lappend(finalindexlist, index as *mut libc::c_void);
            }
        }
        lc__state_0.i += 1;
        lc__state_0.i;
    }
    (*cxt).alist = list_concat((*cxt).alist, finalindexlist);
}
unsafe extern "C" fn transformExtendedStatistics(mut cxt: *mut CreateStmtContext) {
    (*cxt).alist = list_concat((*cxt).alist, (*cxt).extstats);
}
unsafe extern "C" fn transformCheckConstraints(
    mut cxt: *mut CreateStmtContext,
    mut skipValidation: bool,
) {
    let mut ckclist: *mut ListCell = 0 as *mut ListCell;
    if ((*cxt).ckconstraints).is_null() {
        return;
    }
    if skipValidation != 0 {
        let mut ckclist__state: ForEachState = {
            let mut init = ForEachState {
                l: (*cxt).ckconstraints,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(ckclist__state.l).is_null() && ckclist__state.i < (*ckclist__state.l).length {
            ckclist = &mut *((*ckclist__state.l).elements).offset(ckclist__state.i as isize)
                as *mut ListCell;
            true as libc::c_int
        } else {
            ckclist = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut constraint: *mut Constraint = (*ckclist).ptr_value as *mut Constraint;
            (*constraint).skip_validation = true;
            (*constraint).initially_valid = true;
            ckclist__state.i += 1;
            ckclist__state.i;
        }
    }
}
unsafe extern "C" fn transformFKConstraints(
    mut cxt: *mut CreateStmtContext,
    mut skipValidation: bool,
    mut isAddConstraint: bool,
) {
    let mut fkclist: *mut ListCell = 0 as *mut ListCell;
    if ((*cxt).fkconstraints).is_null() {
        return;
    }
    if skipValidation != 0 {
        let mut fkclist__state: ForEachState = {
            let mut init = ForEachState {
                l: (*cxt).fkconstraints,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(fkclist__state.l).is_null() && fkclist__state.i < (*fkclist__state.l).length {
            fkclist = &mut *((*fkclist__state.l).elements).offset(fkclist__state.i as isize)
                as *mut ListCell;
            true as libc::c_int
        } else {
            fkclist = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut constraint: *mut Constraint = (*fkclist).ptr_value as *mut Constraint;
            (*constraint).skip_validation = true;
            (*constraint).initially_valid = true;
            fkclist__state.i += 1;
            fkclist__state.i;
        }
    }
    if isAddConstraint == 0 {
        let mut alterstmt: *mut AlterTableStmt = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_AlterTableStmt;
            _result
        }) as *mut AlterTableStmt;
        (*alterstmt).relation = (*cxt).relation;
        (*alterstmt).cmds = 0 as *mut libc::c_void as *mut List;
        (*alterstmt).objtype = OBJECT_TABLE;
        let mut fkclist__state_0: ForEachState = {
            let mut init = ForEachState {
                l: (*cxt).fkconstraints,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(fkclist__state_0.l).is_null()
            && fkclist__state_0.i < (*fkclist__state_0.l).length
        {
            fkclist = &mut *((*fkclist__state_0.l).elements).offset(fkclist__state_0.i as isize)
                as *mut ListCell;
            true as libc::c_int
        } else {
            fkclist = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut constraint_0: *mut Constraint = (*fkclist).ptr_value as *mut Constraint;
            let mut altercmd: *mut AlterTableCmd = ({
                let mut _result: *mut Node = 0 as *mut Node;
                (*_result).type_0 = T_AlterTableCmd;
                _result
            }) as *mut AlterTableCmd;
            (*altercmd).subtype = AT_AddConstraint;
            (*altercmd).name = 0 as *mut libc::c_char;
            (*altercmd).def = constraint_0 as *mut Node;
            (*alterstmt).cmds = lappend((*alterstmt).cmds, altercmd as *mut libc::c_void);
            fkclist__state_0.i += 1;
            fkclist__state_0.i;
        }
        (*cxt).alist = lappend((*cxt).alist, alterstmt as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn transformIndexStmt(
    mut relid: Oid,
    mut stmt: *mut IndexStmt,
    mut queryString: *const libc::c_char,
) -> *mut IndexStmt {
    let mut pstate: *mut ParseState = 0 as *mut ParseState;
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut rel: Relation = 0 as *mut RelationData;
    if (*stmt).transformed != 0 {
        return stmt;
    }
    stmt = copyObjectImpl(stmt as *const libc::c_void) as *mut IndexStmt;
    pstate = make_parsestate(0 as *mut ParseState);
    (*pstate).p_sourcetext = queryString;
    rel = relation_open(relid, 0 as libc::c_int);
    nsitem =
        addRangeTableEntryForRelation(pstate, rel, 1 as libc::c_int, 0 as *mut Alias, false, true);
    addNSItemToQuery(pstate, nsitem, false, true, true);
    if !((*stmt).whereClause).is_null() {
        (*stmt).whereClause = transformWhereClause(
            pstate,
            (*stmt).whereClause,
            EXPR_KIND_INDEX_PREDICATE,
            b"WHERE\0" as *const u8 as *const libc::c_char,
        );
        assign_expr_collations(pstate, (*stmt).whereClause);
    }
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: (*stmt).indexParams,
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
        if !((*ielem).expr).is_null() {
            if ((*ielem).indexcolname).is_null() {
                (*ielem).indexcolname = FigureIndexColname((*ielem).expr);
            }
            (*ielem).expr = transformExpr(pstate, (*ielem).expr, EXPR_KIND_INDEX_EXPRESSION);
            assign_expr_collations(pstate, (*ielem).expr);
        }
        l__state.i += 1;
        l__state.i;
    }
    if list_length((*pstate).p_rtable) != 1 as libc::c_int {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    free_parsestate(pstate);
    table_close(rel, 0 as libc::c_int);
    (*stmt).transformed = true;
    return stmt;
}
unsafe extern "C" fn transformConstraintAttrs(
    mut cxt: *mut CreateStmtContext,
    mut constraintList: *mut List,
) {
    let mut lastprimarycon: *mut Constraint = 0 as *mut Constraint;
    let mut saw_deferrability: bool = false;
    let mut saw_initially: bool = false;
    let mut clist: *mut ListCell = 0 as *mut ListCell;
    let mut clist__state: ForEachState = {
        let mut init = ForEachState {
            l: constraintList,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(clist__state.l).is_null() && clist__state.i < (*clist__state.l).length {
        clist = &mut *((*clist__state.l).elements).offset(clist__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        clist = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut con: *mut Constraint = (*clist).ptr_value as *mut Constraint;
        if !((*(con as *const Node)).type_0 as libc::c_uint
            == T_Constraint as libc::c_int as libc::c_uint)
        {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unrecognized node type: %d\0" as *const u8 as *const libc::c_char,
                    (*(con as *const Node)).type_0 as libc::c_int,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                        as *const u8 as *const libc::c_char,
                    3562 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        match (*con).contype as libc::c_uint {
            10 => {
                if !(!lastprimarycon.is_null()
                    && ((*lastprimarycon).contype as libc::c_uint
                        == CONSTR_PRIMARY as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_UNIQUE as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_EXCLUSION as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_FOREIGN as libc::c_int as libc::c_uint))
                {
                    let elevel__0: libc::c_int = 21 as libc::c_int;
                    let mut __error_0: libc::c_int = 0;
                    if elevel__0 >= 21 as libc::c_int {
                        abort();
                    }
                }
                if saw_deferrability != 0 {
                    let elevel__1: libc::c_int = 21 as libc::c_int;
                    let mut __error_1: libc::c_int = 0;
                    if elevel__1 >= 21 as libc::c_int {
                        abort();
                    }
                }
                saw_deferrability = true;
                (*lastprimarycon).deferrable = true;
            }
            11 => {
                if !(!lastprimarycon.is_null()
                    && ((*lastprimarycon).contype as libc::c_uint
                        == CONSTR_PRIMARY as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_UNIQUE as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_EXCLUSION as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_FOREIGN as libc::c_int as libc::c_uint))
                {
                    let elevel__2: libc::c_int = 21 as libc::c_int;
                    let mut __error_2: libc::c_int = 0;
                    if elevel__2 >= 21 as libc::c_int {
                        abort();
                    }
                }
                if saw_deferrability != 0 {
                    let elevel__3: libc::c_int = 21 as libc::c_int;
                    let mut __error_3: libc::c_int = 0;
                    if elevel__3 >= 21 as libc::c_int {
                        abort();
                    }
                }
                saw_deferrability = true;
                (*lastprimarycon).deferrable = false;
                if saw_initially as libc::c_int != 0
                    && (*lastprimarycon).initdeferred as libc::c_int != 0
                {
                    let elevel__4: libc::c_int = 21 as libc::c_int;
                    let mut __error_4: libc::c_int = 0;
                    if elevel__4 >= 21 as libc::c_int {
                        abort();
                    }
                }
            }
            12 => {
                if !(!lastprimarycon.is_null()
                    && ((*lastprimarycon).contype as libc::c_uint
                        == CONSTR_PRIMARY as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_UNIQUE as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_EXCLUSION as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_FOREIGN as libc::c_int as libc::c_uint))
                {
                    let elevel__5: libc::c_int = 21 as libc::c_int;
                    let mut __error_5: libc::c_int = 0;
                    if elevel__5 >= 21 as libc::c_int {
                        abort();
                    }
                }
                if saw_initially != 0 {
                    let elevel__6: libc::c_int = 21 as libc::c_int;
                    let mut __error_6: libc::c_int = 0;
                    if elevel__6 >= 21 as libc::c_int {
                        abort();
                    }
                }
                saw_initially = true;
                (*lastprimarycon).initdeferred = true;
                if saw_deferrability == 0 {
                    (*lastprimarycon).deferrable = true;
                } else if (*lastprimarycon).deferrable == 0 {
                    let elevel__7: libc::c_int = 21 as libc::c_int;
                    let mut __error_7: libc::c_int = 0;
                    if elevel__7 >= 21 as libc::c_int {
                        abort();
                    }
                }
            }
            13 => {
                if !(!lastprimarycon.is_null()
                    && ((*lastprimarycon).contype as libc::c_uint
                        == CONSTR_PRIMARY as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_UNIQUE as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_EXCLUSION as libc::c_int as libc::c_uint
                        || (*lastprimarycon).contype as libc::c_uint
                            == CONSTR_FOREIGN as libc::c_int as libc::c_uint))
                {
                    let elevel__8: libc::c_int = 21 as libc::c_int;
                    let mut __error_8: libc::c_int = 0;
                    if elevel__8 >= 21 as libc::c_int {
                        abort();
                    }
                }
                if saw_initially != 0 {
                    let elevel__9: libc::c_int = 21 as libc::c_int;
                    let mut __error_9: libc::c_int = 0;
                    if elevel__9 >= 21 as libc::c_int {
                        abort();
                    }
                }
                saw_initially = true;
                (*lastprimarycon).initdeferred = false;
            }
            _ => {
                lastprimarycon = con;
                saw_deferrability = false;
                saw_initially = false;
            }
        }
        clist__state.i += 1;
        clist__state.i;
    }
}
unsafe extern "C" fn transformColumnType(
    mut cxt: *mut CreateStmtContext,
    mut column: *mut ColumnDef,
) {
    let mut ctype: Type = typenameType((*cxt).pstate, (*column).typeName, 0 as *mut i32);
    if !((*column).collClause).is_null() {
        let mut typtup: Form_pg_type = ((*ctype).t_data as *mut libc::c_char)
            .offset((*(*ctype).t_data).t_hoff as libc::c_int as isize)
            as Form_pg_type;
        LookupCollation(
            (*cxt).pstate,
            (*(*column).collClause).collname,
            (*(*column).collClause).location,
        );
        if ((*typtup).typcollation != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
    }
    ReleaseSysCache(ctype);
}
#[no_mangle]
pub unsafe extern "C" fn transformCreateSchemaStmt(mut stmt: *mut CreateSchemaStmt) -> *mut List {
    let mut cxt: CreateSchemaStmtContext = CreateSchemaStmtContext {
        stmtType: 0 as *const libc::c_char,
        schemaname: 0 as *mut libc::c_char,
        authrole: 0 as *mut RoleSpec,
        sequences: 0 as *mut List,
        tables: 0 as *mut List,
        views: 0 as *mut List,
        indexes: 0 as *mut List,
        triggers: 0 as *mut List,
        grants: 0 as *mut List,
    };
    let mut result: *mut List = 0 as *mut List;
    let mut elements: *mut ListCell = 0 as *mut ListCell;
    cxt.stmtType = b"CREATE SCHEMA\0" as *const u8 as *const libc::c_char;
    cxt.schemaname = (*stmt).schemaname;
    cxt.authrole = (*stmt).authrole;
    cxt.sequences = 0 as *mut libc::c_void as *mut List;
    cxt.tables = 0 as *mut libc::c_void as *mut List;
    cxt.views = 0 as *mut libc::c_void as *mut List;
    cxt.indexes = 0 as *mut libc::c_void as *mut List;
    cxt.triggers = 0 as *mut libc::c_void as *mut List;
    cxt.grants = 0 as *mut libc::c_void as *mut List;
    let mut elements__state: ForEachState = {
        let mut init = ForEachState {
            l: (*stmt).schemaElts,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(elements__state.l).is_null() && elements__state.i < (*elements__state.l).length {
        elements = &mut *((*elements__state.l).elements).offset(elements__state.i as isize)
            as *mut ListCell;
        true as libc::c_int
    } else {
        elements = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut element: *mut Node = (*elements).ptr_value as *mut Node;
        match (*(element as *const Node)).type_0 as libc::c_uint {
            273 => {
                let mut elp: *mut CreateSeqStmt = element as *mut CreateSeqStmt;
                setSchemaName(cxt.schemaname, &mut (*(*elp).sequence).schemaname);
                cxt.sequences = lappend(cxt.sequences, element as *mut libc::c_void);
            }
            249 => {
                let mut elp_0: *mut CreateStmt = element as *mut CreateStmt;
                setSchemaName(cxt.schemaname, &mut (*(*elp_0).relation).schemaname);
                cxt.tables = lappend(cxt.tables, element as *mut libc::c_void);
            }
            265 => {
                let mut elp_1: *mut ViewStmt = element as *mut ViewStmt;
                setSchemaName(cxt.schemaname, &mut (*(*elp_1).view).schemaname);
                cxt.views = lappend(cxt.views, element as *mut libc::c_void);
            }
            255 => {
                let mut elp_2: *mut IndexStmt = element as *mut IndexStmt;
                setSchemaName(cxt.schemaname, &mut (*(*elp_2).relation).schemaname);
                cxt.indexes = lappend(cxt.indexes, element as *mut libc::c_void);
            }
            278 => {
                let mut elp_3: *mut CreateTrigStmt = element as *mut CreateTrigStmt;
                setSchemaName(cxt.schemaname, &mut (*(*elp_3).relation).schemaname);
                cxt.triggers = lappend(cxt.triggers, element as *mut libc::c_void);
            }
            243 => {
                cxt.grants = lappend(cxt.grants, element as *mut libc::c_void);
            }
            _ => {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"unrecognized node type: %d\0" as *const u8 as *const libc::c_char,
                        (*(element as *const Node)).type_0 as libc::c_int,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                            as *const u8 as *const libc::c_char,
                        3795 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
        }
        elements__state.i += 1;
        elements__state.i;
    }
    result = 0 as *mut libc::c_void as *mut List;
    result = list_concat(result, cxt.sequences);
    result = list_concat(result, cxt.tables);
    result = list_concat(result, cxt.views);
    result = list_concat(result, cxt.indexes);
    result = list_concat(result, cxt.triggers);
    result = list_concat(result, cxt.grants);
    return result;
}
unsafe extern "C" fn setSchemaName(
    mut context_schema: *mut libc::c_char,
    mut stmt_schema_name: *mut *mut libc::c_char,
) {
    if (*stmt_schema_name).is_null() {
        *stmt_schema_name = context_schema;
    } else if strcmp(context_schema, *stmt_schema_name) != 0 as libc::c_int {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
}
unsafe extern "C" fn transformPartitionCmd(
    mut cxt: *mut CreateStmtContext,
    mut cmd: *mut PartitionCmd,
) {
    let mut parentRel: Relation = (*cxt).rel;
    match (*(*parentRel).rd_rel).relkind as libc::c_int {
        _ => {}
    }
    let elevel__2: libc::c_int = 21 as libc::c_int;
    let mut __error_2: libc::c_int = 0;
    if errstart(elevel__2, 0 as *const libc::c_char) != 0 {
        errmsg_internal(
            b"\"%s\" is not a partitioned table or index\0" as *const u8 as *const libc::c_char,
            ((*(*parentRel).rd_rel).relname.data).as_mut_ptr(),
        );
        errfinish(
            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                as *const u8 as *const libc::c_char,
            3877 as libc::c_int,
            0 as *const libc::c_char,
        );
    }
    if elevel__2 >= 21 as libc::c_int {
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn transformPartitionBound(
    mut pstate: *mut ParseState,
    mut parent: Relation,
    mut spec: *mut PartitionBoundSpec,
) -> *mut PartitionBoundSpec {
    let mut result_spec: *mut PartitionBoundSpec = 0 as *mut PartitionBoundSpec;
    let mut key: PartitionKey = RelationGetPartitionKey(parent);
    let mut strategy: libc::c_char = get_partition_strategy(key) as libc::c_char;
    let mut partnatts: libc::c_int = get_partition_natts(key);
    let mut partexprs: *mut List = get_partition_exprs(key);
    result_spec = copyObjectImpl(spec as *const libc::c_void) as *mut PartitionBoundSpec;
    if (*spec).is_default != 0 {
        if strategy as libc::c_int == 'h' as i32 {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        (*result_spec).strategy = strategy;
        return result_spec;
    }
    if strategy as libc::c_int == 'h' as i32 {
        if (*spec).strategy as libc::c_int != 'h' as i32 {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
        if (*spec).modulus <= 0 as libc::c_int {
            let elevel__1: libc::c_int = 21 as libc::c_int;
            let mut __error_1: libc::c_int = 0;
            if elevel__1 >= 21 as libc::c_int {
                abort();
            }
        }
        if (*spec).remainder >= (*spec).modulus {
            let elevel__2: libc::c_int = 21 as libc::c_int;
            let mut __error_2: libc::c_int = 0;
            if elevel__2 >= 21 as libc::c_int {
                abort();
            }
        }
    } else if strategy as libc::c_int == 'l' as i32 {
        let mut cell: *mut ListCell = 0 as *mut ListCell;
        let mut colname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut coltype: Oid = 0;
        let mut coltypmod: i32 = 0;
        let mut partcollation: Oid = 0;
        if (*spec).strategy as libc::c_int != 'l' as i32 {
            let elevel__3: libc::c_int = 21 as libc::c_int;
            let mut __error_3: libc::c_int = 0;
            if elevel__3 >= 21 as libc::c_int {
                abort();
            }
        }
        if *((*key).partattrs).offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
        {
            colname = get_attname(
                (*parent).rd_id,
                *((*key).partattrs).offset(0 as libc::c_int as isize),
                false,
            );
        } else {
            colname = deparse_expression(
                (*list_nth_cell(partexprs, 0 as libc::c_int)).ptr_value as *mut Node,
                deparse_context_for(
                    ((*(*parent).rd_rel).relname.data).as_mut_ptr(),
                    (*parent).rd_id,
                ),
                false,
                false,
            );
        }
        coltype = get_partition_col_typid(key, 0 as libc::c_int);
        coltypmod = get_partition_col_typmod(key, 0 as libc::c_int);
        partcollation = get_partition_col_collation(key, 0 as libc::c_int);
        (*result_spec).listdatums = 0 as *mut libc::c_void as *mut List;
        let mut cell__state: ForEachState = {
            let mut init = ForEachState {
                l: (*spec).listdatums,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(cell__state.l).is_null() && cell__state.i < (*cell__state.l).length {
            cell =
                &mut *((*cell__state.l).elements).offset(cell__state.i as isize) as *mut ListCell;
            true as libc::c_int
        } else {
            cell = 0 as *mut ListCell;
            false as libc::c_int
        } != 0
        {
            let mut expr: *mut Node = (*cell).ptr_value as *mut Node;
            let mut value: *mut Const = 0 as *mut Const;
            let mut cell2: *mut ListCell = 0 as *mut ListCell;
            let mut duplicate: bool = false;
            value = transformPartitionBoundValue(
                pstate,
                expr,
                colname,
                coltype,
                coltypmod,
                partcollation,
            );
            duplicate = false;
            let mut cell2__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*result_spec).listdatums,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(cell2__state.l).is_null() && cell2__state.i < (*cell2__state.l).length {
                cell2 = &mut *((*cell2__state.l).elements).offset(cell2__state.i as isize)
                    as *mut ListCell;
                true as libc::c_int
            } else {
                cell2 = 0 as *mut ListCell;
                false as libc::c_int
            } != 0
            {
                let mut value2: *mut Const = (*cell2).ptr_value as *mut Const;
                if equal(value as *const libc::c_void, value2 as *const libc::c_void) != 0 {
                    duplicate = true;
                    break;
                } else {
                    cell2__state.i += 1;
                    cell2__state.i;
                }
            }
            if !(duplicate != 0) {
                (*result_spec).listdatums =
                    lappend((*result_spec).listdatums, value as *mut libc::c_void);
            }
            cell__state.i += 1;
            cell__state.i;
        }
    } else if strategy as libc::c_int == 'r' as i32 {
        if (*spec).strategy as libc::c_int != 'r' as i32 {
            let elevel__4: libc::c_int = 21 as libc::c_int;
            let mut __error_4: libc::c_int = 0;
            if elevel__4 >= 21 as libc::c_int {
                abort();
            }
        }
        if list_length((*spec).lowerdatums) != partnatts {
            let elevel__5: libc::c_int = 21 as libc::c_int;
            let mut __error_5: libc::c_int = 0;
            if elevel__5 >= 21 as libc::c_int {
                abort();
            }
        }
        if list_length((*spec).upperdatums) != partnatts {
            let elevel__6: libc::c_int = 21 as libc::c_int;
            let mut __error_6: libc::c_int = 0;
            if elevel__6 >= 21 as libc::c_int {
                abort();
            }
        }
        (*result_spec).lowerdatums =
            transformPartitionRangeBounds(pstate, (*spec).lowerdatums, parent);
        (*result_spec).upperdatums =
            transformPartitionRangeBounds(pstate, (*spec).upperdatums, parent);
    } else {
        let elevel__7: libc::c_int = 21 as libc::c_int;
        let mut __error_7: libc::c_int = 0;
        if errstart(elevel__7, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"unexpected partition strategy: %d\0" as *const u8 as *const libc::c_char,
                strategy as libc::c_int,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                    as *const u8 as *const libc::c_char,
                4031 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel__7 >= 21 as libc::c_int {
            abort();
        }
    }
    return result_spec;
}
unsafe extern "C" fn transformPartitionRangeBounds(
    mut pstate: *mut ParseState,
    mut blist: *mut List,
    mut parent: Relation,
) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut key: PartitionKey = RelationGetPartitionKey(parent);
    let mut partexprs: *mut List = get_partition_exprs(key);
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    i = j;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: blist,
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
        let mut prd: *mut PartitionRangeDatum = 0 as *mut PartitionRangeDatum;
        if (*(expr as *const Node)).type_0 as libc::c_uint
            == T_ColumnRef as libc::c_int as libc::c_uint
        {
            let mut cref: *mut ColumnRef = expr as *mut ColumnRef;
            let mut cname: *mut libc::c_char = 0 as *mut libc::c_char;
            if list_length((*cref).fields) == 1 as libc::c_int
                && (*((*list_nth_cell((*cref).fields, 0 as libc::c_int)).ptr_value as *const Node))
                    .type_0 as libc::c_uint
                    == T_String as libc::c_int as libc::c_uint
            {
                cname = (*((*list_nth_cell((*cref).fields, 0 as libc::c_int)).ptr_value
                    as *mut Value))
                    .val
                    .str_0;
            }
            if !cname.is_null() {
                if strcmp(b"minvalue\0" as *const u8 as *const libc::c_char, cname)
                    == 0 as libc::c_int
                {
                    prd = ({
                        let mut _result: *mut Node = 0 as *mut Node;
                        (*_result).type_0 = T_PartitionRangeDatum;
                        _result
                    }) as *mut PartitionRangeDatum;
                    (*prd).kind = PARTITION_RANGE_DATUM_MINVALUE;
                    (*prd).value = 0 as *mut Node;
                } else if strcmp(b"maxvalue\0" as *const u8 as *const libc::c_char, cname)
                    == 0 as libc::c_int
                {
                    prd = ({
                        let mut _result: *mut Node = 0 as *mut Node;
                        (*_result).type_0 = T_PartitionRangeDatum;
                        _result
                    }) as *mut PartitionRangeDatum;
                    (*prd).kind = PARTITION_RANGE_DATUM_MAXVALUE;
                    (*prd).value = 0 as *mut Node;
                }
            }
        }
        if prd.is_null() {
            let mut colname: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut coltype: Oid = 0;
            let mut coltypmod: i32 = 0;
            let mut partcollation: Oid = 0;
            let mut value: *mut Const = 0 as *mut Const;
            if *((*key).partattrs).offset(i as isize) as libc::c_int != 0 as libc::c_int {
                colname = get_attname(
                    (*parent).rd_id,
                    *((*key).partattrs).offset(i as isize),
                    false,
                );
            } else {
                colname = deparse_expression(
                    list_nth(partexprs, j) as *mut Node,
                    deparse_context_for(
                        ((*(*parent).rd_rel).relname.data).as_mut_ptr(),
                        (*parent).rd_id,
                    ),
                    false,
                    false,
                );
                j += 1;
                j;
            }
            coltype = get_partition_col_typid(key, i);
            coltypmod = get_partition_col_typmod(key, i);
            partcollation = get_partition_col_collation(key, i);
            value = transformPartitionBoundValue(
                pstate,
                expr,
                colname,
                coltype,
                coltypmod,
                partcollation,
            );
            if (*value).constisnull != 0 {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            prd = ({
                let mut _result: *mut Node = 0 as *mut Node;
                (*_result).type_0 = T_PartitionRangeDatum;
                _result
            }) as *mut PartitionRangeDatum;
            (*prd).kind = PARTITION_RANGE_DATUM_VALUE;
            (*prd).value = value as *mut Node;
            i += 1;
            i;
        }
        (*prd).location = exprLocation(expr);
        result = lappend(result, prd as *mut libc::c_void);
        lc__state.i += 1;
        lc__state.i;
    }
    validateInfiniteBounds(pstate, result);
    return result;
}
unsafe extern "C" fn validateInfiniteBounds(mut pstate: *mut ParseState, mut blist: *mut List) {
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut kind: PartitionRangeDatumKind = PARTITION_RANGE_DATUM_VALUE;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: blist,
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
        let mut prd: *mut PartitionRangeDatum = (*lc).ptr_value as *mut PartitionRangeDatum;
        if !(kind as libc::c_int == (*prd).kind as libc::c_int) {
            match kind as libc::c_int {
                0 => {
                    kind = (*prd).kind;
                }
                1 => {
                    let elevel_: libc::c_int = 21 as libc::c_int;
                    let mut __error: libc::c_int = 0;
                    if elevel_ >= 21 as libc::c_int {
                        abort();
                    }
                }
                -1 => {
                    let elevel__0: libc::c_int = 21 as libc::c_int;
                    let mut __error_0: libc::c_int = 0;
                    if elevel__0 >= 21 as libc::c_int {
                        abort();
                    }
                }
                _ => {}
            }
        }
        lc__state.i += 1;
        lc__state.i;
    }
}
unsafe extern "C" fn transformPartitionBoundValue(
    mut pstate: *mut ParseState,
    mut val: *mut Node,
    mut colName: *const libc::c_char,
    mut colType: Oid,
    mut colTypmod: i32,
    mut partCollation: Oid,
) -> *mut Const {
    let mut value: *mut Node = 0 as *mut Node;
    value = transformExpr(pstate, val, EXPR_KIND_PARTITION_BOUND);
    value = coerce_to_target_type(
        pstate,
        value,
        exprType(value),
        colType,
        colTypmod,
        COERCION_ASSIGNMENT,
        COERCE_IMPLICIT_CAST,
        -(1 as libc::c_int),
    );
    if value.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*(value as *const Node)).type_0 as libc::c_uint == T_Const as libc::c_int as libc::c_uint)
    {
        assign_expr_collations(pstate, value);
        value = expression_planner(value as *mut Expr) as *mut Node;
        value = evaluate_expr(value as *mut Expr, colType, colTypmod, partCollation) as *mut Node;
        if !((*(value as *const Node)).type_0 as libc::c_uint
            == T_Const as libc::c_int as libc::c_uint)
        {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"could not evaluate partition bound expression\0" as *const u8
                        as *const libc::c_char,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_utilcmd.c\0"
                        as *const u8 as *const libc::c_char,
                    4245 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
    } else {
        (*(value as *mut Const)).constcollid = partCollation;
    }
    (*(value as *mut Const)).location = exprLocation(val);
    return value as *mut Const;
}
