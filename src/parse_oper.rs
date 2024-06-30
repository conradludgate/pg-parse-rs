#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// #![feature(extern_types)]
// extern "C" {
//     pub type MemoryContextData;
//     pub type AttrMissing;
//     pub type RelationData;
//     pub type QueryEnvironment;
//     pub type HASHHDR;
//     pub type HTAB;
//     pub type TypeCacheEnumData;
//     pub type DomainConstraintCache;
//     fn abort() -> !;
//     fn memset(
//         _: *mut libc::c_void,
//         _: libc::c_int,
//         _: libc::c_ulong,
//     ) -> *mut libc::c_void;
//     fn strlcpy(
//         _: *mut libc::c_char,
//         _: *const libc::c_char,
//         _: libc::c_ulong,
//     ) -> libc::c_ulong;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
//     fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
//     fn exprType(expr: *const Node) -> Oid;
//     fn IsBinaryCoercible(srctype: Oid, targettype: Oid) -> bool;
//     fn enforce_generic_type_consistency(
//         actual_arg_types: *const Oid,
//         declared_arg_types: *mut Oid,
//         nargs: libc::c_int,
//         rettype: Oid,
//         allow_poly: bool,
//     ) -> Oid;
//     fn setup_parser_errposition_callback(
//         pcbstate: *mut ParseCallbackState,
//         pstate: *mut ParseState,
//         location: libc::c_int,
//     );
//     fn cancel_parser_errposition_callback(pcbstate: *mut ParseCallbackState);
//     fn hash_create(
//         tabname: *const libc::c_char,
//         nelem: libc::c_long,
//         info: *const HASHCTL,
//         flags: libc::c_int,
//     ) -> *mut HTAB;
//     fn hash_search(
//         hashp: *mut HTAB,
//         keyPtr: *const libc::c_void,
//         action: HASHACTION,
//         foundPtr: *mut bool,
//     ) -> *mut libc::c_void;
//     fn hash_seq_init(status: *mut HASH_SEQ_STATUS, hashp: *mut HTAB);
//     fn hash_seq_search(status: *mut HASH_SEQ_STATUS) -> *mut libc::c_void;
//     fn OpernameGetOprid(names: *mut List, oprleft: Oid, oprright: Oid) -> Oid;
//     fn OpernameGetCandidates(
//         names: *mut List,
//         oprkind: libc::c_char,
//         missing_schema_ok: bool,
//     ) -> FuncCandidateList;
//     fn DeconstructQualifiedName(
//         names: *mut List,
//         nspname_p: *mut *mut libc::c_char,
//         objname_p: *mut *mut libc::c_char,
//     );
//     fn LookupExplicitNamespace(nspname: *const libc::c_char, missing_ok: bool) -> Oid;
//     fn fetch_search_path_array(sarray: *mut Oid, sarray_len: libc::c_int) -> libc::c_int;
//     fn func_match_argtypes(
//         nargs: libc::c_int,
//         input_typeids: *mut Oid,
//         raw_candidates: FuncCandidateList,
//         candidates: *mut FuncCandidateList,
//     ) -> libc::c_int;
//     fn func_select_candidate(
//         nargs: libc::c_int,
//         input_typeids: *mut Oid,
//         candidates: FuncCandidateList,
//     ) -> FuncCandidateList;
//     fn make_fn_arguments(
//         pstate: *mut ParseState,
//         fargs: *mut List,
//         actual_arg_types: *mut Oid,
//         declared_arg_types: *mut Oid,
//     );
//     fn check_srf_call_placement(
//         pstate: *mut ParseState,
//         last_srf: *mut Node,
//         location: libc::c_int,
//     );
//     fn LookupTypeNameOid(
//         pstate: *mut ParseState,
//         typeName: *const TypeName,
//         missing_ok: bool,
//     ) -> Oid;
//     fn CacheRegisterSyscacheCallback(
//         cacheid: libc::c_int,
//         func: SyscacheCallbackFunction,
//         arg: Datum,
//     );
//     fn get_func_retset(funcid: Oid) -> bool;
//     fn get_array_type(typid: Oid) -> Oid;
//     fn get_base_element_type(typid: Oid) -> Oid;
//     fn getBaseType(typid: Oid) -> Oid;
//     fn SearchSysCache1(cacheId: libc::c_int, key1: Datum) -> HeapTuple;
//     fn ReleaseSysCache(tuple: HeapTuple);
//     fn lookup_type_cache(type_id: Oid, flags: libc::c_int) -> *mut TypeCacheEntry;
// }
use super::*;
pub type Oid = libc::c_uint;
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
pub type Size = isize;
pub type Index = libc::c_uint;
pub type regproc = Oid;
pub type TransactionId = u32;
pub type CommandId = u32;
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
    pub callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
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
const ItemPointerData_PADDING: usize = ::core::mem::size_of::<ItemPointerData>()
    - ::core::mem::size_of::<ItemPointerData_Inner>();
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bitmapset {
    pub nwords: libc::c_int,
    pub words: [bitmapword; 0],
}
pub type bitmapword = u32;
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
pub union ListCell {
    pub ptr_value: *mut libc::c_void,
    pub int_value: libc::c_int,
    pub oid_value: Oid,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Expr {
    pub type_0: NodeTag,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScalarArrayOpExpr {
    pub xpr: Expr,
    pub opno: Oid,
    pub opfuncid: Oid,
    pub useOr: bool,
    pub inputcollid: Oid,
    pub args: *mut List,
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
pub struct TypeName {
    pub type_0: NodeTag,
    pub names: *mut List,
    pub typeOid: Oid,
    pub setof: bool,
    pub pct_type: bool,
    pub typmods: *mut List,
    pub typemod: i32,
    pub arrayBounds: *mut List,
    pub location: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectWithArgs {
    pub type_0: NodeTag,
    pub objname: *mut List,
    pub objargs: *mut List,
    pub args_unspecified: bool,
}
pub type Relation = *mut RelationData;
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

pub type CoerceParamHook = Option::<
    unsafe extern "C" fn(
        *mut ParseState,
        *mut Param,
        Oid,
        i32,
        libc::c_int,
    ) -> *mut Node,
>;
pub type ParseParamRefHook = Option::<
    unsafe extern "C" fn(*mut ParseState, *mut ParamRef) -> *mut Node,
>;
pub type PostParseColumnRefHook = Option::<
    unsafe extern "C" fn(*mut ParseState, *mut ColumnRef, *mut Node) -> *mut Node,
>;
pub type PreParseColumnRefHook = Option::<
    unsafe extern "C" fn(*mut ParseState, *mut ColumnRef) -> *mut Node,
>;
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
pub type HashValueFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, Size) -> u32,
>;
pub type HashCompareFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void, Size) -> libc::c_int,
>;
pub type HashCopyFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_void,
        Size,
    ) -> *mut libc::c_void,
>;
pub type HashAllocFunc = Option::<unsafe extern "C" fn(Size) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASHELEMENT {
    pub link: *mut HASHELEMENT,
    pub hashvalue: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASHCTL {
    pub num_partitions: libc::c_long,
    pub ssize: libc::c_long,
    pub dsize: libc::c_long,
    pub max_dsize: libc::c_long,
    pub keysize: Size,
    pub entrysize: Size,
    pub hash: HashValueFunc,
    pub match_0: HashCompareFunc,
    pub keycopy: HashCopyFunc,
    pub alloc: HashAllocFunc,
    pub hcxt: MemoryContext,
    pub hctl: *mut HASHHDR,
}
pub type HASHACTION = libc::c_uint;
pub const HASH_ENTER_NULL: HASHACTION = 3;
pub const HASH_REMOVE: HASHACTION = 2;
pub const HASH_ENTER: HASHACTION = 1;
pub const HASH_FIND: HASHACTION = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HASH_SEQ_STATUS {
    pub hashp: *mut HTAB,
    pub curBucket: u32,
    pub curEntry: *mut HASHELEMENT,
}
pub type fmNodePtr = *mut Node;
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
pub type PGFunction = Option::<unsafe extern "C" fn(FunctionCallInfo) -> Datum>;
pub type FunctionCallInfo = *mut FunctionCallInfoBaseData;
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
pub type Operator = HeapTuple;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OprCacheKey {
    pub oprname: [libc::c_char; 64],
    pub left_arg: Oid,
    pub right_arg: Oid,
    pub search_path: [Oid; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OprCacheEntry {
    pub key: OprCacheKey,
    pub opr_oid: Oid,
}
pub const OPEROID: SysCacheIdentifier = 38;
pub const CASTSOURCETARGET: SysCacheIdentifier = 12;
pub type SyscacheCallbackFunction = Option::<
    unsafe extern "C" fn(Datum, libc::c_int, u32) -> (),
>;
pub const OPERNAMENSP: SysCacheIdentifier = 37;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TypeCacheEntry {
    pub type_id: Oid,
    pub type_id_hash: u32,
    pub typlen: i16,
    pub typbyval: bool,
    pub typalign: libc::c_char,
    pub typstorage: libc::c_char,
    pub typtype: libc::c_char,
    pub typrelid: Oid,
    pub typsubscript: Oid,
    pub typelem: Oid,
    pub typcollation: Oid,
    pub btree_opf: Oid,
    pub btree_opintype: Oid,
    pub hash_opf: Oid,
    pub hash_opintype: Oid,
    pub eq_opr: Oid,
    pub lt_opr: Oid,
    pub gt_opr: Oid,
    pub cmp_proc: Oid,
    pub hash_proc: Oid,
    pub hash_extended_proc: Oid,
    pub eq_opr_finfo: FmgrInfo,
    pub cmp_proc_finfo: FmgrInfo,
    pub hash_proc_finfo: FmgrInfo,
    pub hash_extended_proc_finfo: FmgrInfo,
    pub tupDesc: TupleDesc,
    pub tupDesc_identifier: uint64,
    pub rngelemtype: *mut TypeCacheEntry,
    pub rng_collation: Oid,
    pub rng_cmp_proc_finfo: FmgrInfo,
    pub rng_canonical_finfo: FmgrInfo,
    pub rng_subdiff_finfo: FmgrInfo,
    pub rngtype: *mut TypeCacheEntry,
    pub domainBaseType: Oid,
    pub domainBaseTypmod: i32,
    pub domainData: *mut DomainConstraintCache,
    pub flags: libc::c_int,
    pub enumData: *mut TypeCacheEnumData,
    pub nextDomain: *mut TypeCacheEntry,
}
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
unsafe extern "C" fn list_nth_cell(
    mut list: *const List,
    mut n: libc::c_int,
) -> *mut ListCell {
    return &mut *((*list).elements).offset(n as isize) as *mut ListCell;
}
#[no_mangle]
pub unsafe extern "C" fn LookupOperName(
    mut pstate: *mut ParseState,
    mut opername: *mut List,
    mut oprleft: Oid,
    mut oprright: Oid,
    mut noError: bool,
    mut location: libc::c_int,
) -> Oid {
    let mut result: Oid = 0;
    result = OpernameGetOprid(opername, oprleft, oprright);
    if (result != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
        return result;
    }
    if noError == 0 {
        let mut oprkind: libc::c_char = 0;
        if (oprleft != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
            oprkind = 'l' as i32 as libc::c_char;
        } else if (oprright != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
            oprkind = 'b' as i32 as libc::c_char;
        } else {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
            oprkind = 0 as libc::c_int as libc::c_char;
        }
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    return 0 as libc::c_int as Oid;
}
#[no_mangle]
pub unsafe extern "C" fn LookupOperWithArgs(
    mut oper_0: *mut ObjectWithArgs,
    mut noError: bool,
) -> Oid {
    let mut oprleft: *mut TypeName = 0 as *mut TypeName;
    let mut oprright: *mut TypeName = 0 as *mut TypeName;
    let mut leftoid: Oid = 0;
    let mut rightoid: Oid = 0;
    oprleft = (*list_nth_cell((*oper_0).objargs, 0 as libc::c_int)).ptr_value
        as *mut TypeName;
    oprright = (*list_nth_cell((*oper_0).objargs, 1 as libc::c_int)).ptr_value
        as *mut TypeName;
    if oprleft.is_null() {
        leftoid = 0 as libc::c_int as Oid;
    } else {
        leftoid = LookupTypeNameOid(0 as *mut ParseState, oprleft, noError);
    }
    if oprright.is_null() {
        rightoid = 0 as libc::c_int as Oid;
    } else {
        rightoid = LookupTypeNameOid(0 as *mut ParseState, oprright, noError);
    }
    return LookupOperName(
        0 as *mut ParseState,
        (*oper_0).objname,
        leftoid,
        rightoid,
        noError,
        -(1 as libc::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn get_sort_group_operators(
    mut argtype: Oid,
    mut needLT: bool,
    mut needEQ: bool,
    mut needGT: bool,
    mut ltOpr: *mut Oid,
    mut eqOpr: *mut Oid,
    mut gtOpr: *mut Oid,
    mut isHashable: *mut bool,
) {
    let mut typentry: *mut TypeCacheEntry = 0 as *mut TypeCacheEntry;
    let mut cache_flags: libc::c_int = 0;
    let mut lt_opr: Oid = 0;
    let mut eq_opr: Oid = 0;
    let mut gt_opr: Oid = 0;
    let mut hashable: bool = 0;
    if !isHashable.is_null() {
        cache_flags = 0x2 as libc::c_int | 0x1 as libc::c_int | 0x4 as libc::c_int
            | 0x10 as libc::c_int;
    } else {
        cache_flags = 0x2 as libc::c_int | 0x1 as libc::c_int | 0x4 as libc::c_int;
    }
    typentry = lookup_type_cache(argtype, cache_flags);
    lt_opr = (*typentry).lt_opr;
    eq_opr = (*typentry).eq_opr;
    gt_opr = (*typentry).gt_opr;
    hashable = ((*typentry).hash_proc != 0 as libc::c_int as Oid) as libc::c_int
        as bool;
    if needLT as libc::c_int != 0
        && (lt_opr != 0 as libc::c_int as Oid) as libc::c_int as bool == 0
        || needGT as libc::c_int != 0
            && (gt_opr != 0 as libc::c_int as Oid) as libc::c_int as bool == 0
    {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if needEQ as libc::c_int != 0
        && (eq_opr != 0 as libc::c_int as Oid) as libc::c_int as bool == 0
    {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    if !ltOpr.is_null() {
        *ltOpr = lt_opr;
    }
    if !eqOpr.is_null() {
        *eqOpr = eq_opr;
    }
    if !gtOpr.is_null() {
        *gtOpr = gt_opr;
    }
    if !isHashable.is_null() {
        *isHashable = hashable;
    }
}
#[no_mangle]
pub unsafe extern "C" fn oprid(mut op: Operator) -> Oid {
    return (*(((*op).t_data as *mut libc::c_char)
        .offset((*(*op).t_data).t_hoff as libc::c_int as isize) as Form_pg_operator))
        .oid;
}
#[no_mangle]
pub unsafe extern "C" fn oprfuncid(mut op: Operator) -> Oid {
    let mut pgopform: Form_pg_operator = ((*op).t_data as *mut libc::c_char)
        .offset((*(*op).t_data).t_hoff as libc::c_int as isize) as Form_pg_operator;
    return (*pgopform).oprcode;
}
unsafe extern "C" fn oper_select_candidate(
    mut nargs: libc::c_int,
    mut input_typeids: *mut Oid,
    mut candidates: FuncCandidateList,
    mut operOid: *mut Oid,
) -> FuncDetailCode {
    let mut ncandidates: libc::c_int = 0;
    ncandidates = func_match_argtypes(nargs, input_typeids, candidates, &mut candidates);
    if ncandidates == 0 as libc::c_int {
        *operOid = 0 as libc::c_int as Oid;
        return FUNCDETAIL_NOTFOUND;
    }
    if ncandidates == 1 as libc::c_int {
        *operOid = (*candidates).oid;
        return FUNCDETAIL_NORMAL;
    }
    candidates = func_select_candidate(nargs, input_typeids, candidates);
    if !candidates.is_null() {
        *operOid = (*candidates).oid;
        return FUNCDETAIL_NORMAL;
    }
    *operOid = 0 as libc::c_int as Oid;
    return FUNCDETAIL_MULTIPLE;
}
#[no_mangle]
pub unsafe extern "C" fn oper(
    mut pstate: *mut ParseState,
    mut opname: *mut List,
    mut ltypeId: Oid,
    mut rtypeId: Oid,
    mut noError: bool,
    mut location: libc::c_int,
) -> Operator {
    let mut operOid: Oid = 0;
    let mut key: OprCacheKey = OprCacheKey {
        oprname: [0; 64],
        left_arg: 0,
        right_arg: 0,
        search_path: [0; 16],
    };
    let mut key_ok: bool = 0;
    let mut fdresult: FuncDetailCode = FUNCDETAIL_NOTFOUND;
    let mut tup: HeapTuple = 0 as HeapTuple;
    key_ok = make_oper_cache_key(pstate, &mut key, opname, ltypeId, rtypeId, location);
    if key_ok != 0 {
        operOid = find_oper_cache_entry(&mut key);
        if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
            tup = SearchSysCache1(OPEROID as libc::c_int, operOid as Datum);
            if !(tup as *const libc::c_void).is_null() {
                return tup;
            }
        }
    }
    operOid = binary_oper_exact(opname, ltypeId, rtypeId);
    if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
        let mut clist: FuncCandidateList = 0 as *mut _FuncCandidateList;
        clist = OpernameGetCandidates(
            opname,
            'b' as i32 as libc::c_char,
            false,
        );
        if !clist.is_null() {
            let mut inputOids: [Oid; 2] = [0; 2];
            if rtypeId == 0 as libc::c_int as Oid {
                rtypeId = ltypeId;
            } else if ltypeId == 0 as libc::c_int as Oid {
                ltypeId = rtypeId;
            }
            inputOids[0 as libc::c_int as usize] = ltypeId;
            inputOids[1 as libc::c_int as usize] = rtypeId;
            fdresult = oper_select_candidate(
                2 as libc::c_int,
                inputOids.as_mut_ptr(),
                clist,
                &mut operOid,
            );
        }
    }
    if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
        tup = SearchSysCache1(OPEROID as libc::c_int, operOid as Datum);
    }
    if !(tup as *const libc::c_void).is_null() {
        if key_ok != 0 {
            make_oper_cache_entry(&mut key, operOid);
        }
    } else if noError == 0 {
        op_error(
            pstate,
            opname,
            'b' as i32 as libc::c_char,
            ltypeId,
            rtypeId,
            fdresult,
            location,
        );
    }
    return tup;
}
#[no_mangle]
pub unsafe extern "C" fn compatible_oper(
    mut pstate: *mut ParseState,
    mut op: *mut List,
    mut arg1: Oid,
    mut arg2: Oid,
    mut noError: bool,
    mut location: libc::c_int,
) -> Operator {
    let mut optup: Operator = 0 as *mut HeapTupleData;
    let mut opform: Form_pg_operator = 0 as *mut FormData_pg_operator;
    optup = oper(pstate, op, arg1, arg2, noError, location);
    if optup.is_null() {
        return 0 as *mut libc::c_void as Operator;
    }
    opform = ((*optup).t_data as *mut libc::c_char)
        .offset((*(*optup).t_data).t_hoff as libc::c_int as isize) as Form_pg_operator;
    if IsBinaryCoercible(arg1, (*opform).oprleft) as libc::c_int != 0
        && IsBinaryCoercible(arg2, (*opform).oprright) as libc::c_int != 0
    {
        return optup;
    }
    ReleaseSysCache(optup);
    if noError == 0 {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return 0 as *mut libc::c_void as Operator;
}
#[no_mangle]
pub unsafe extern "C" fn compatible_oper_opid(
    mut op: *mut List,
    mut arg1: Oid,
    mut arg2: Oid,
    mut noError: bool,
) -> Oid {
    let mut optup: Operator = 0 as *mut HeapTupleData;
    let mut result: Oid = 0;
    optup = compatible_oper(
        0 as *mut ParseState,
        op,
        arg1,
        arg2,
        noError,
        -(1 as libc::c_int),
    );
    if !optup.is_null() {
        result = oprid(optup);
        ReleaseSysCache(optup);
        return result;
    }
    return 0 as libc::c_int as Oid;
}
#[no_mangle]
pub unsafe extern "C" fn left_oper(
    mut pstate: *mut ParseState,
    mut op: *mut List,
    mut arg: Oid,
    mut noError: bool,
    mut location: libc::c_int,
) -> Operator {
    let mut operOid: Oid = 0;
    let mut key: OprCacheKey = OprCacheKey {
        oprname: [0; 64],
        left_arg: 0,
        right_arg: 0,
        search_path: [0; 16],
    };
    let mut key_ok: bool = 0;
    let mut fdresult: FuncDetailCode = FUNCDETAIL_NOTFOUND;
    let mut tup: HeapTuple = 0 as HeapTuple;
    key_ok = make_oper_cache_key(
        pstate,
        &mut key,
        op,
        0 as libc::c_int as Oid,
        arg,
        location,
    );
    if key_ok != 0 {
        operOid = find_oper_cache_entry(&mut key);
        if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
            tup = SearchSysCache1(OPEROID as libc::c_int, operOid as Datum);
            if !(tup as *const libc::c_void).is_null() {
                return tup;
            }
        }
    }
    operOid = OpernameGetOprid(op, 0 as libc::c_int as Oid, arg);
    if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
        let mut clist: FuncCandidateList = 0 as *mut _FuncCandidateList;
        clist = OpernameGetCandidates(
            op,
            'l' as i32 as libc::c_char,
            false,
        );
        if !clist.is_null() {
            let mut clisti: FuncCandidateList = 0 as *mut _FuncCandidateList;
            clisti = clist;
            while !clisti.is_null() {
                *((*clisti).args)
                    .as_mut_ptr()
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *((*clisti).args).as_mut_ptr().offset(1 as libc::c_int as isize);
                clisti = (*clisti).next;
            }
            fdresult = oper_select_candidate(
                1 as libc::c_int,
                &mut arg,
                clist,
                &mut operOid,
            );
        }
    }
    if (operOid != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
        tup = SearchSysCache1(OPEROID as libc::c_int, operOid as Datum);
    }
    if !(tup as *const libc::c_void).is_null() {
        if key_ok != 0 {
            make_oper_cache_entry(&mut key, operOid);
        }
    } else if noError == 0 {
        op_error(
            pstate,
            op,
            'l' as i32 as libc::c_char,
            0 as libc::c_int as Oid,
            arg,
            fdresult,
            location,
        );
    }
    return tup;
}
unsafe extern "C" fn op_error(
    mut pstate: *mut ParseState,
    mut op: *mut List,
    mut oprkind: libc::c_char,
    mut arg1: Oid,
    mut arg2: Oid,
    mut fdresult: FuncDetailCode,
    mut location: libc::c_int,
) {
    if fdresult as libc::c_uint == FUNCDETAIL_MULTIPLE as libc::c_int as libc::c_uint {
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
    };
}
#[no_mangle]
pub unsafe extern "C" fn make_op(
    mut pstate: *mut ParseState,
    mut opname: *mut List,
    mut ltree: *mut Node,
    mut rtree: *mut Node,
    mut last_srf: *mut Node,
    mut location: libc::c_int,
) -> *mut Expr {
    let mut ltypeId: Oid = 0;
    let mut rtypeId: Oid = 0;
    let mut tup: Operator = 0 as *mut HeapTupleData;
    let mut opform: Form_pg_operator = 0 as *mut FormData_pg_operator;
    let mut actual_arg_types: [Oid; 2] = [0; 2];
    let mut declared_arg_types: [Oid; 2] = [0; 2];
    let mut nargs: libc::c_int = 0;
    let mut args: *mut List = 0 as *mut List;
    let mut rettype: Oid = 0;
    let mut result: *mut OpExpr = 0 as *mut OpExpr;
    if rtree.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if ltree.is_null() {
        rtypeId = exprType(rtree);
        ltypeId = 0 as libc::c_int as Oid;
        tup = left_oper(pstate, opname, rtypeId, false, location);
    } else {
        ltypeId = exprType(ltree);
        rtypeId = exprType(rtree);
        tup = oper(
            pstate,
            opname,
            ltypeId,
            rtypeId,
            false,
            location,
        );
    }
    opform = ((*tup).t_data as *mut libc::c_char)
        .offset((*(*tup).t_data).t_hoff as libc::c_int as isize) as Form_pg_operator;
    if ((*opform).oprcode != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    if ltree.is_null() {
        args = list_make1_impl(
            T_List,
            ListCell {
                ptr_value: rtree as *mut libc::c_void,
            },
        );
        actual_arg_types[0 as libc::c_int as usize] = rtypeId;
        declared_arg_types[0 as libc::c_int as usize] = (*opform).oprright;
        nargs = 1 as libc::c_int;
    } else {
        args = list_make2_impl(
            T_List,
            ListCell {
                ptr_value: ltree as *mut libc::c_void,
            },
            ListCell {
                ptr_value: rtree as *mut libc::c_void,
            },
        );
        actual_arg_types[0 as libc::c_int as usize] = ltypeId;
        actual_arg_types[1 as libc::c_int as usize] = rtypeId;
        declared_arg_types[0 as libc::c_int as usize] = (*opform).oprleft;
        declared_arg_types[1 as libc::c_int as usize] = (*opform).oprright;
        nargs = 2 as libc::c_int;
    }
    rettype = enforce_generic_type_consistency(
        actual_arg_types.as_mut_ptr(),
        declared_arg_types.as_mut_ptr(),
        nargs,
        (*opform).oprresult,
        false,
    );
    make_fn_arguments(
        pstate,
        args,
        actual_arg_types.as_mut_ptr(),
        declared_arg_types.as_mut_ptr(),
    );
    result = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_OpExpr;
        _result
    }) as *mut OpExpr;
    (*result).opno = oprid(tup);
    (*result).opfuncid = (*opform).oprcode;
    (*result).opresulttype = rettype;
    (*result).opretset = get_func_retset((*opform).oprcode);
    (*result).args = args;
    (*result).location = location;
    if (*result).opretset != 0 {
        check_srf_call_placement(pstate, last_srf, location);
        (*pstate).p_last_srf = result as *mut Node;
    }
    ReleaseSysCache(tup);
    return result as *mut Expr;
}
static mut OprCacheHash: *mut HTAB = 0 as *const HTAB as *mut HTAB;
unsafe extern "C" fn find_oper_cache_entry(mut key: *mut OprCacheKey) -> Oid {
    let mut oprentry: *mut OprCacheEntry = 0 as *mut OprCacheEntry;
    if OprCacheHash.is_null() {
        let mut ctl: HASHCTL = HASHCTL {
            num_partitions: 0,
            ssize: 0,
            dsize: 0,
            max_dsize: 0,
            keysize: 0,
            entrysize: 0,
            hash: None,
            match_0: None,
            keycopy: None,
            alloc: None,
            hcxt: 0 as *mut MemoryContextData,
            hctl: 0 as *mut HASHHDR,
        };
        ctl.keysize = ::core::mem::size_of::<OprCacheKey>() as libc::c_ulong;
        ctl.entrysize = ::core::mem::size_of::<OprCacheEntry>() as libc::c_ulong;
        OprCacheHash = hash_create(
            b"Operator lookup cache\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int as libc::c_long,
            &mut ctl,
            0x8 as libc::c_int | 0x20 as libc::c_int,
        );
        CacheRegisterSyscacheCallback(
            OPERNAMENSP as libc::c_int,
            Some(
                InvalidateOprCacheCallBack
                    as unsafe extern "C" fn(Datum, libc::c_int, u32) -> (),
            ),
            0 as libc::c_int as Datum,
        );
        CacheRegisterSyscacheCallback(
            CASTSOURCETARGET as libc::c_int,
            Some(
                InvalidateOprCacheCallBack
                    as unsafe extern "C" fn(Datum, libc::c_int, u32) -> (),
            ),
            0 as libc::c_int as Datum,
        );
    }
    oprentry = hash_search(
        OprCacheHash,
        key as *mut libc::c_void,
        HASH_FIND,
        0 as *mut bool,
    ) as *mut OprCacheEntry;
    if oprentry.is_null() {
        return 0 as libc::c_int as Oid;
    }
    return (*oprentry).opr_oid;
}
unsafe extern "C" fn make_oper_cache_entry(mut key: *mut OprCacheKey, mut opr_oid: Oid) {
    let mut oprentry: *mut OprCacheEntry = 0 as *mut OprCacheEntry;
    oprentry = hash_search(
        OprCacheHash,
        key as *mut libc::c_void,
        HASH_ENTER,
        0 as *mut bool,
    ) as *mut OprCacheEntry;
    (*oprentry).opr_oid = opr_oid;
}
unsafe extern "C" fn InvalidateOprCacheCallBack(
    mut arg: Datum,
    mut cacheid: libc::c_int,
    mut hashvalue: u32,
) {
    let mut status: HASH_SEQ_STATUS = HASH_SEQ_STATUS {
        hashp: 0 as *mut HTAB,
        curBucket: 0,
        curEntry: 0 as *mut HASHELEMENT,
    };
    let mut hentry: *mut OprCacheEntry = 0 as *mut OprCacheEntry;
    hash_seq_init(&mut status, OprCacheHash);
    loop {
        hentry = hash_seq_search(&mut status) as *mut OprCacheEntry;
        if hentry.is_null() {
            break;
        }
        if (hash_search(
            OprCacheHash,
            &mut (*hentry).key as *mut OprCacheKey as *mut libc::c_void,
            HASH_REMOVE,
            0 as *mut bool,
        ))
            .is_null()
        {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"hash table corrupted\0" as *const u8 as *const libc::c_char,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_oper.c\0"
                        as *const u8 as *const libc::c_char,
                    1063 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
    };
}
