#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// #![feature(extern_types)]
// extern "C" {
//     pub type RelationData;
//     pub type QueryEnvironment;
//     fn abort() -> !;
//     fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
//     fn pfree(pointer: *mut libc::c_void);
//     fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
//     fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
//     fn equal(a: *const libc::c_void, b: *const libc::c_void) -> bool_0;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn list_member_int(list: *const List, datum: libc::c_int) -> bool_0;
//     fn list_truncate(list: *mut List, new_size: libc::c_int) -> *mut List;
//     fn list_concat(list1: *mut List, list2: *const List) -> *mut List;
//     fn lappend_oid(list: *mut List, datum: Oid) -> *mut List;
//     fn lappend_int(list: *mut List, datum: libc::c_int) -> *mut List;
//     fn list_intersection_int(list1: *const List, list2: *const List) -> *mut List;
//     fn list_union_int(list1: *const List, list2: *const List) -> *mut List;
//     fn list_copy_tail(list: *const List, nskip: libc::c_int) -> *mut List;
//     fn list_sort(list: *mut List, cmp: list_sort_comparator);
//     fn check_functional_grouping(
//         relid: Oid,
//         varno: Index,
//         varlevelsup: Index,
//         grouping_columns: *mut List,
//         constraintDeps: *mut *mut List,
//     ) -> bool_0;
//     fn makeFuncExpr(
//         funcid: Oid,
//         rettype: Oid,
//         args: *mut List,
//         funccollid: Oid,
//         inputcollid: Oid,
//         fformat: CoercionForm,
//     ) -> *mut FuncExpr;
//     fn makeTargetEntry(
//         expr: *mut Expr,
//         resno: AttrNumber,
//         resname: *mut libc::c_char,
//         resjunk: bool_0,
//     ) -> *mut TargetEntry;
//     fn exprType(expr: *const Node) -> Oid;
//     fn expression_tree_walker(
//         node: *mut Node,
//         walker: Option::<unsafe extern "C" fn() -> bool_0>,
//         context: *mut libc::c_void,
//     ) -> bool_0;
//     fn query_tree_walker(
//         query: *mut Query,
//         walker: Option::<unsafe extern "C" fn() -> bool_0>,
//         context: *mut libc::c_void,
//         flags: libc::c_int,
//     ) -> bool_0;
//     fn get_sortgroupclause_tle(
//         sgClause: *mut SortGroupClause,
//         targetList: *mut List,
//     ) -> *mut TargetEntry;
//     fn get_sortgroupclause_expr(
//         sgClause: *mut SortGroupClause,
//         targetList: *mut List,
//     ) -> *mut Node;
//     fn flatten_join_alias_vars(query: *mut Query, node: *mut Node) -> *mut Node;
//     fn transformSortClause(
//         pstate: *mut ParseState,
//         orderlist: *mut List,
//         targetlist: *mut *mut List,
//         exprKind: ParseExprKind,
//         useSQL99: bool_0,
//     ) -> *mut List;
//     fn transformDistinctClause(
//         pstate: *mut ParseState,
//         targetlist: *mut *mut List,
//         sortClause: *mut List,
//         is_agg: bool_0,
//     ) -> *mut List;
//     fn addTargetToSortList(
//         pstate: *mut ParseState,
//         tle: *mut TargetEntry,
//         sortlist: *mut List,
//         targetlist: *mut List,
//         sortby: *mut SortBy,
//     ) -> *mut List;
//     fn enforce_generic_type_consistency(
//         actual_arg_types: *const Oid,
//         declared_arg_types: *mut Oid,
//         nargs: libc::c_int,
//         rettype: Oid,
//         allow_poly: bool_0,
//     ) -> Oid;
//     fn transformExpr(
//         pstate: *mut ParseState,
//         expr: *mut Node,
//         exprKind: ParseExprKind,
//     ) -> *mut Node;
//     fn get_rte_attribute_name(
//         rte: *mut RangeTblEntry,
//         attnum: AttrNumber,
//     ) -> *mut libc::c_char;
//     fn locate_agg_of_level(node: *mut Node, levelsup: libc::c_int) -> libc::c_int;
//     fn contain_windowfuncs(node: *mut Node) -> bool_0;
//     fn get_func_signature(
//         funcid: Oid,
//         argtypes: *mut *mut Oid,
//         nargs: *mut libc::c_int,
//     ) -> Oid;
// }
use super::*;
pub type Oid = libc::c_uint;
pub type bool_0 = libc::c_uchar;
pub type int16 = libc::c_short;
pub type int32 = libc::c_int;
pub type uint32 = libc::c_uint;
pub type uint64 = libc::c_ulong;
pub type Index = libc::c_uint;
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
pub type bitmapword = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bitmapset {
    pub nwords: libc::c_int,
    pub words: [bitmapword; 0],
}
pub type AttrNumber = int16;
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
pub type list_sort_comparator = Option::<
    unsafe extern "C" fn(*const ListCell, *const ListCell) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Var {
    pub xpr: Expr,
    pub varno: Index,
    pub varattno: AttrNumber,
    pub vartype: Oid,
    pub vartypmod: int32,
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
    pub paramtypmod: int32,
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
    pub aggstar: bool_0,
    pub aggvariadic: bool_0,
    pub aggkind: libc::c_char,
    pub agglevelsup: Index,
    pub aggsplit: AggSplit,
    pub aggno: libc::c_int,
    pub aggtransno: libc::c_int,
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
pub struct WindowFunc {
    pub xpr: Expr,
    pub winfnoid: Oid,
    pub wintype: Oid,
    pub wincollid: Oid,
    pub inputcollid: Oid,
    pub args: *mut List,
    pub aggfilter: *mut Expr,
    pub winref: Index,
    pub winstar: bool_0,
    pub winagg: bool_0,
    pub location: libc::c_int,
}
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
    pub funcretset: bool_0,
    pub funcvariadic: bool_0,
    pub funcformat: CoercionForm,
    pub funccollid: Oid,
    pub inputcollid: Oid,
    pub args: *mut List,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OpExpr {
    pub xpr: Expr,
    pub opno: Oid,
    pub opfuncid: Oid,
    pub opresulttype: Oid,
    pub opretset: bool_0,
    pub opcollid: Oid,
    pub inputcollid: Oid,
    pub args: *mut List,
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
    pub resjunk: bool_0,
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
pub type SortByDir = libc::c_uint;
pub const SORTBY_USING: SortByDir = 3;
pub const SORTBY_DESC: SortByDir = 2;
pub const SORTBY_ASC: SortByDir = 1;
pub const SORTBY_DEFAULT: SortByDir = 0;
pub type SortByNulls = libc::c_uint;
pub const SORTBY_NULLS_LAST: SortByNulls = 2;
pub const SORTBY_NULLS_FIRST: SortByNulls = 1;
pub const SORTBY_NULLS_DEFAULT: SortByNulls = 0;
pub type AclMode = uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Query {
    pub type_0: NodeTag,
    pub commandType: CmdType,
    pub querySource: QuerySource,
    pub queryId: uint64,
    pub canSetTag: bool_0,
    pub utilityStmt: *mut Node,
    pub resultRelation: libc::c_int,
    pub hasAggs: bool_0,
    pub hasWindowFuncs: bool_0,
    pub hasTargetSRFs: bool_0,
    pub hasSubLinks: bool_0,
    pub hasDistinctOn: bool_0,
    pub hasRecursive: bool_0,
    pub hasModifyingCTE: bool_0,
    pub hasForUpdate: bool_0,
    pub hasRowSecurity: bool_0,
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
pub struct SortBy {
    pub type_0: NodeTag,
    pub node: *mut Node,
    pub sortby_dir: SortByDir,
    pub sortby_nulls: SortByNulls,
    pub useOp: *mut List,
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
    pub security_barrier: bool_0,
    pub jointype: JoinType,
    pub joinmergedcols: libc::c_int,
    pub joinaliasvars: *mut List,
    pub joinleftcols: *mut List,
    pub joinrightcols: *mut List,
    pub functions: *mut List,
    pub funcordinality: bool_0,
    pub tablefunc: *mut TableFunc,
    pub values_lists: *mut List,
    pub ctename: *mut libc::c_char,
    pub ctelevelsup: Index,
    pub self_reference: bool_0,
    pub coltypes: *mut List,
    pub coltypmods: *mut List,
    pub colcollations: *mut List,
    pub enrname: *mut libc::c_char,
    pub enrtuples: libc::c_double,
    pub alias: *mut Alias,
    pub eref: *mut Alias,
    pub lateral: bool_0,
    pub inh: bool_0,
    pub inFromCl: bool_0,
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
    pub nulls_first: bool_0,
    pub hashable: bool_0,
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
pub type CTEMaterialize = libc::c_uint;
pub const CTEMaterializeNever: CTEMaterialize = 2;
pub const CTEMaterializeAlways: CTEMaterialize = 1;
pub const CTEMaterializeDefault: CTEMaterialize = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CTESearchClause {
    pub type_0: NodeTag,
    pub search_col_list: *mut List,
    pub search_breadth_first: bool_0,
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
    pub cterecursive: bool_0,
    pub cterefcount: libc::c_int,
    pub ctecolnames: *mut List,
    pub ctecoltypes: *mut List,
    pub ctecoltypmods: *mut List,
    pub ctecolcollations: *mut List,
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
    pub p_lateral_active: bool_0,
    pub p_ctenamespace: *mut List,
    pub p_future_ctes: *mut List,
    pub p_parent_cte: *mut CommonTableExpr,
    pub p_target_relation: Relation,
    pub p_target_nsitem: *mut ParseNamespaceItem,
    pub p_is_insert: bool_0,
    pub p_windowdefs: *mut List,
    pub p_expr_kind: ParseExprKind,
    pub p_next_resno: libc::c_int,
    pub p_multiassign_exprs: *mut List,
    pub p_locking_clause: *mut List,
    pub p_locked_from_parent: bool_0,
    pub p_resolve_unknowns: bool_0,
    pub p_queryEnv: *mut QueryEnvironment,
    pub p_hasAggs: bool_0,
    pub p_hasWindowFuncs: bool_0,
    pub p_hasTargetSRFs: bool_0,
    pub p_hasSubLinks: bool_0,
    pub p_hasModifyingCTE: bool_0,
    pub p_last_srf: *mut Node,
    pub p_pre_columnref_hook: PreParseColumnRefHook,
    pub p_post_columnref_hook: PostParseColumnRefHook,
    pub p_paramref_hook: ParseParamRefHook,
    pub p_coerce_param_hook: CoerceParamHook,
    pub p_ref_hook_state: *mut libc::c_void,
}
pub type CoerceParamHook = Option::<
    unsafe extern "C" fn(
        *mut ParseState,
        *mut Param,
        Oid,
        int32,
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
    pub p_rel_visible: bool_0,
    pub p_cols_visible: bool_0,
    pub p_lateral_only: bool_0,
    pub p_lateral_ok: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParseNamespaceColumn {
    pub p_varno: Index,
    pub p_varattno: AttrNumber,
    pub p_vartype: Oid,
    pub p_vartypmod: int32,
    pub p_varcollid: Oid,
    pub p_varnosyn: Index,
    pub p_varattnosyn: AttrNumber,
    pub p_dontexpand: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct check_agg_arguments_context {
    pub pstate: *mut ParseState,
    pub min_varlevel: libc::c_int,
    pub min_agglevel: libc::c_int,
    pub sublevels_up: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct check_ungrouped_columns_context {
    pub pstate: *mut ParseState,
    pub qry: *mut Query,
    pub hasJoinRTEs: bool_0,
    pub groupClauses: *mut List,
    pub groupClauseCommonVars: *mut List,
    pub have_non_var_grouping: bool_0,
    pub func_grouped_rels: *mut *mut List,
    pub sublevels_up: libc::c_int,
    pub in_agg_direct_args: bool_0,
}
#[inline]
unsafe extern "C" fn for_each_from_setup(
    mut lst: *const List,
    mut N: libc::c_int,
) -> ForEachState {
    let mut r: ForEachState = {
        let mut init = ForEachState { l: lst, i: N };
        init
    };
    return r;
}
#[inline]
unsafe extern "C" fn list_nth(
    mut list: *const List,
    mut n: libc::c_int,
) -> *mut libc::c_void {
    return (*list_nth_cell(list, n)).ptr_value;
}
#[inline]
unsafe extern "C" fn list_nth_cell(
    mut list: *const List,
    mut n: libc::c_int,
) -> *mut ListCell {
    return &mut *((*list).elements).offset(n as isize) as *mut ListCell;
}
#[inline]
unsafe extern "C" fn list_length(mut l: *const List) -> libc::c_int {
    return if !l.is_null() { (*l).length } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn transformAggregateCall(
    mut pstate: *mut ParseState,
    mut agg: *mut Aggref,
    mut args: *mut List,
    mut aggorder: *mut List,
    mut agg_distinct: bool_0,
) {
    let mut argtypes: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut tlist: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut torder: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut tdistinct: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut attno: AttrNumber = 1 as libc::c_int as AttrNumber;
    let mut save_next_resno: libc::c_int = 0;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: args,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length {
        lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize)
            as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        lc = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut arg: *mut Expr = (*lc).ptr_value as *mut Expr;
        argtypes = lappend_oid(argtypes, exprType(arg as *mut Node));
        lc__state.i += 1;
        lc__state.i;
    }
    (*agg).aggargtypes = argtypes;
    if AGGKIND_IS_ORDERED_SET((*agg).aggkind as libc::c_int) != 0 {
        let mut numDirectArgs: libc::c_int = list_length(args) - list_length(aggorder);
        let mut aargs: *mut List = 0 as *mut List;
        let mut lc2: *mut ListCell = 0 as *mut ListCell;
        aargs = list_copy_tail(args, numDirectArgs);
        (*agg).aggdirectargs = list_truncate(args, numDirectArgs);
        let mut lc__state_0: ForBothState = {
            let mut init = ForBothState {
                l1: aargs,
                l2: aggorder,
                i: 0 as libc::c_int,
            };
            init
        };
        loop {
            lc = (if !(lc__state_0.l1).is_null()
                && lc__state_0.i < (*lc__state_0.l1).length
            {
                &mut *((*lc__state_0.l1).elements).offset(lc__state_0.i as isize)
                    as *mut ListCell
            } else {
                0 as *mut ListCell
            });
            lc2 = (if !(lc__state_0.l2).is_null()
                && lc__state_0.i < (*lc__state_0.l2).length
            {
                &mut *((*lc__state_0.l2).elements).offset(lc__state_0.i as isize)
                    as *mut ListCell
            } else {
                0 as *mut ListCell
            });
            if !(!lc.is_null() && !lc2.is_null()) {
                break;
            }
            let mut arg_0: *mut Expr = (*lc).ptr_value as *mut Expr;
            let mut sortby: *mut SortBy = (*lc2).ptr_value as *mut SortBy;
            let mut tle: *mut TargetEntry = 0 as *mut TargetEntry;
            let fresh0 = attno;
            attno = attno + 1;
            tle = makeTargetEntry(
                arg_0,
                fresh0,
                0 as *mut libc::c_char,
                0 as libc::c_int as bool_0,
            );
            tlist = lappend(tlist, tle as *mut libc::c_void);
            torder = addTargetToSortList(pstate, tle, torder, tlist, sortby);
            lc__state_0.i += 1;
            lc__state_0.i;
        }
    } else {
        (*agg).aggdirectargs = 0 as *mut libc::c_void as *mut List;
        let mut lc__state_1: ForEachState = {
            let mut init = ForEachState {
                l: args,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(lc__state_1.l).is_null() && lc__state_1.i < (*lc__state_1.l).length {
            lc = &mut *((*lc__state_1.l).elements).offset(lc__state_1.i as isize)
                as *mut ListCell;
            1 as libc::c_int as bool_0 as libc::c_int
        } else {
            lc = 0 as *mut ListCell;
            0 as libc::c_int as bool_0 as libc::c_int
        } != 0
        {
            let mut arg_1: *mut Expr = (*lc).ptr_value as *mut Expr;
            let mut tle_0: *mut TargetEntry = 0 as *mut TargetEntry;
            let fresh1 = attno;
            attno = attno + 1;
            tle_0 = makeTargetEntry(
                arg_1,
                fresh1,
                0 as *mut libc::c_char,
                0 as libc::c_int as bool_0,
            );
            tlist = lappend(tlist, tle_0 as *mut libc::c_void);
            lc__state_1.i += 1;
            lc__state_1.i;
        }
        save_next_resno = (*pstate).p_next_resno;
        (*pstate).p_next_resno = attno as libc::c_int;
        torder = transformSortClause(
            pstate,
            aggorder,
            &mut tlist,
            EXPR_KIND_ORDER_BY,
            1 as libc::c_int as bool_0,
        );
        if agg_distinct != 0 {
            tdistinct = transformDistinctClause(
                pstate,
                &mut tlist,
                torder,
                1 as libc::c_int as bool_0,
            );
            let mut lc__state_2: ForEachState = {
                let mut init = ForEachState {
                    l: tdistinct,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(lc__state_2.l).is_null()
                && lc__state_2.i < (*lc__state_2.l).length
            {
                lc = &mut *((*lc__state_2.l).elements).offset(lc__state_2.i as isize)
                    as *mut ListCell;
                1 as libc::c_int as bool_0 as libc::c_int
            } else {
                lc = 0 as *mut ListCell;
                0 as libc::c_int as bool_0 as libc::c_int
            } != 0
            {
                let mut sortcl: *mut SortGroupClause = (*lc).ptr_value
                    as *mut SortGroupClause;
                if ((*sortcl).sortop != 0 as libc::c_int as Oid) as libc::c_int as bool_0
                    == 0
                {
                    let mut expr: *mut Node = get_sortgroupclause_expr(sortcl, tlist);
                    let elevel_: libc::c_int = 21 as libc::c_int;
                    let mut __error: libc::c_int = 0;
                    if elevel_ >= 21 as libc::c_int {
                        abort();
                    }
                }
                lc__state_2.i += 1;
                lc__state_2.i;
            }
        }
        (*pstate).p_next_resno = save_next_resno;
    }
    (*agg).args = tlist;
    (*agg).aggorder = torder;
    (*agg).aggdistinct = tdistinct;
    check_agglevels_and_constraints(pstate, agg as *mut Node);
}
#[no_mangle]
pub unsafe extern "C" fn transformGroupingFunc(
    mut pstate: *mut ParseState,
    mut p: *mut GroupingFunc,
) -> *mut Node {
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut args: *mut List = (*p).args;
    let mut result_list: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut result: *mut GroupingFunc = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_GroupingFunc;
        _result
    }) as *mut GroupingFunc;
    if list_length(args) > 31 as libc::c_int {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: args,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length {
        lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize)
            as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        lc = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut current_result: *mut Node = 0 as *mut Node;
        current_result = transformExpr(
            pstate,
            (*lc).ptr_value as *mut Node,
            (*pstate).p_expr_kind,
        );
        result_list = lappend(result_list, current_result as *mut libc::c_void);
        lc__state.i += 1;
        lc__state.i;
    }
    (*result).args = result_list;
    (*result).location = (*p).location;
    check_agglevels_and_constraints(pstate, result as *mut Node);
    return result as *mut Node;
}
unsafe extern "C" fn check_agglevels_and_constraints(
    mut pstate: *mut ParseState,
    mut expr: *mut Node,
) {
    let mut directargs: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut args: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut filter: *mut Expr = 0 as *mut Expr;
    let mut min_varlevel: libc::c_int = 0;
    let mut location: libc::c_int = -(1 as libc::c_int);
    let mut p_levelsup: *mut Index = 0 as *mut Index;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    let mut errkind: bool_0 = 0;
    let mut isAgg: bool_0 = ((*(expr as *const Node)).type_0 as libc::c_uint
        == T_Aggref as libc::c_int as libc::c_uint) as libc::c_int as bool_0;
    if isAgg != 0 {
        let mut agg: *mut Aggref = expr as *mut Aggref;
        directargs = (*agg).aggdirectargs;
        args = (*agg).args;
        filter = (*agg).aggfilter;
        location = (*agg).location;
        p_levelsup = &mut (*agg).agglevelsup;
    } else {
        let mut grp: *mut GroupingFunc = expr as *mut GroupingFunc;
        args = (*grp).args;
        location = (*grp).location;
        p_levelsup = &mut (*grp).agglevelsup;
    }
    min_varlevel = check_agg_arguments(pstate, directargs, args, filter);
    *p_levelsup = min_varlevel as Index;
    loop {
        let fresh2 = min_varlevel;
        min_varlevel = min_varlevel - 1;
        if !(fresh2 > 0 as libc::c_int) {
            break;
        }
        pstate = (*pstate).parentParseState;
    }
    (*pstate).p_hasAggs = 1 as libc::c_int as bool_0;
    err = 0 as *const libc::c_char;
    errkind = 0 as libc::c_int as bool_0;
    match (*pstate).p_expr_kind as libc::c_uint {
        1 => {}
        2 | 3 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in JOIN conditions\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in JOIN conditions\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        4 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in FROM clause of their own query level\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in FROM clause of their own query level\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        5 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in functions in FROM\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in functions in FROM\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        6 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        35 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in policy expressions\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in policy expressions\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        7 => {}
        8 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        9 => {}
        10 => {}
        11 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in window RANGE\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in window RANGE\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        12 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in window ROWS\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in window ROWS\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        13 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in window GROUPS\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in window GROUPS\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        14 => {}
        15 | 16 | 17 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        18 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        19 => {}
        20 => {}
        21 | 22 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        23 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        24 | 25 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        26 | 27 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in check constraints\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in check constraints\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        28 | 29 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in DEFAULT expressions\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in DEFAULT expressions\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        30 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in index expressions\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in index expressions\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        31 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in index predicates\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in index predicates\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        32 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in transform expressions\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in transform expressions\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        33 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in EXECUTE parameters\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in EXECUTE parameters\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        34 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in trigger WHEN conditions\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in trigger WHEN conditions\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        36 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in partition bound\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in partition bound\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        37 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in partition key expressions\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in partition key expressions\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        40 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in column generation expressions\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in column generation expressions\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        38 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in CALL arguments\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in CALL arguments\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        39 => {
            if isAgg != 0 {
                err = b"aggregate functions are not allowed in COPY FROM WHERE conditions\0"
                    as *const u8 as *const libc::c_char;
            } else {
                err = b"grouping operations are not allowed in COPY FROM WHERE conditions\0"
                    as *const u8 as *const libc::c_char;
            }
        }
        41 => {
            errkind = 1 as libc::c_int as bool_0;
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
    if errkind != 0 {
        if isAgg != 0 {
            err = b"aggregate functions are not allowed in %s\0" as *const u8
                as *const libc::c_char;
        } else {
            err = b"grouping operations are not allowed in %s\0" as *const u8
                as *const libc::c_char;
        }
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
}
unsafe extern "C" fn check_agg_arguments(
    mut pstate: *mut ParseState,
    mut directargs: *mut List,
    mut args: *mut List,
    mut filter: *mut Expr,
) -> libc::c_int {
    let mut agglevel: libc::c_int = 0;
    let mut context: check_agg_arguments_context = check_agg_arguments_context {
        pstate: 0 as *mut ParseState,
        min_varlevel: 0,
        min_agglevel: 0,
        sublevels_up: 0,
    };
    context.pstate = pstate;
    context.min_varlevel = -(1 as libc::c_int);
    context.min_agglevel = -(1 as libc::c_int);
    context.sublevels_up = 0 as libc::c_int;
    expression_tree_walker(
        args as *mut Node,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Node,
                    *mut check_agg_arguments_context,
                ) -> bool_0,
            >,
            Option::<unsafe extern "C" fn() -> bool_0>,
        >(
            Some(
                check_agg_arguments_walker
                    as unsafe extern "C" fn(
                        *mut Node,
                        *mut check_agg_arguments_context,
                    ) -> bool_0,
            ),
        ),
        &mut context as *mut check_agg_arguments_context as *mut libc::c_void,
    );
    expression_tree_walker(
        filter as *mut Node,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Node,
                    *mut check_agg_arguments_context,
                ) -> bool_0,
            >,
            Option::<unsafe extern "C" fn() -> bool_0>,
        >(
            Some(
                check_agg_arguments_walker
                    as unsafe extern "C" fn(
                        *mut Node,
                        *mut check_agg_arguments_context,
                    ) -> bool_0,
            ),
        ),
        &mut context as *mut check_agg_arguments_context as *mut libc::c_void,
    );
    if context.min_varlevel < 0 as libc::c_int {
        if context.min_agglevel < 0 as libc::c_int {
            agglevel = 0 as libc::c_int;
        } else {
            agglevel = context.min_agglevel;
        }
    } else if context.min_agglevel < 0 as libc::c_int {
        agglevel = context.min_varlevel;
    } else {
        agglevel = if context.min_varlevel < context.min_agglevel {
            context.min_varlevel
        } else {
            context.min_agglevel
        };
    }
    if agglevel == context.min_agglevel {
        let mut aggloc: libc::c_int = 0;
        aggloc = locate_agg_of_level(args as *mut Node, agglevel);
        if aggloc < 0 as libc::c_int {
            aggloc = locate_agg_of_level(filter as *mut Node, agglevel);
        }
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if !directargs.is_null() {
        context.min_varlevel = -(1 as libc::c_int);
        context.min_agglevel = -(1 as libc::c_int);
        expression_tree_walker(
            directargs as *mut Node,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Node,
                        *mut check_agg_arguments_context,
                    ) -> bool_0,
                >,
                Option::<unsafe extern "C" fn() -> bool_0>,
            >(
                Some(
                    check_agg_arguments_walker
                        as unsafe extern "C" fn(
                            *mut Node,
                            *mut check_agg_arguments_context,
                        ) -> bool_0,
                ),
            ),
            &mut context as *mut check_agg_arguments_context as *mut libc::c_void,
        );
        if context.min_varlevel >= 0 as libc::c_int && context.min_varlevel < agglevel {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
        if context.min_agglevel >= 0 as libc::c_int && context.min_agglevel <= agglevel {
            let elevel__1: libc::c_int = 21 as libc::c_int;
            let mut __error_1: libc::c_int = 0;
            if elevel__1 >= 21 as libc::c_int {
                abort();
            }
        }
    }
    return agglevel;
}
unsafe extern "C" fn check_agg_arguments_walker(
    mut node: *mut Node,
    mut context: *mut check_agg_arguments_context,
) -> bool_0 {
    if node.is_null() {
        return 0 as libc::c_int as bool_0;
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_Var as libc::c_int as libc::c_uint
    {
        let mut varlevelsup: libc::c_int = (*(node as *mut Var)).varlevelsup
            as libc::c_int;
        varlevelsup -= (*context).sublevels_up;
        if varlevelsup >= 0 as libc::c_int {
            if (*context).min_varlevel < 0 as libc::c_int
                || (*context).min_varlevel > varlevelsup
            {
                (*context).min_varlevel = varlevelsup;
            }
        }
        return 0 as libc::c_int as bool_0;
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_Aggref as libc::c_int as libc::c_uint
    {
        let mut agglevelsup: libc::c_int = (*(node as *mut Aggref)).agglevelsup
            as libc::c_int;
        agglevelsup -= (*context).sublevels_up;
        if agglevelsup >= 0 as libc::c_int {
            if (*context).min_agglevel < 0 as libc::c_int
                || (*context).min_agglevel > agglevelsup
            {
                (*context).min_agglevel = agglevelsup;
            }
        }
        return 0 as libc::c_int as bool_0;
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_GroupingFunc as libc::c_int as libc::c_uint
    {
        let mut agglevelsup_0: libc::c_int = (*(node as *mut GroupingFunc)).agglevelsup
            as libc::c_int;
        agglevelsup_0 -= (*context).sublevels_up;
        if agglevelsup_0 >= 0 as libc::c_int {
            if (*context).min_agglevel < 0 as libc::c_int
                || (*context).min_agglevel > agglevelsup_0
            {
                (*context).min_agglevel = agglevelsup_0;
            }
        }
    }
    if (*context).sublevels_up == 0 as libc::c_int {
        if (*(node as *const Node)).type_0 as libc::c_uint
            == T_FuncExpr as libc::c_int as libc::c_uint
            && (*(node as *mut FuncExpr)).funcretset as libc::c_int != 0
            || (*(node as *const Node)).type_0 as libc::c_uint
                == T_OpExpr as libc::c_int as libc::c_uint
                && (*(node as *mut OpExpr)).opretset as libc::c_int != 0
        {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        if (*(node as *const Node)).type_0 as libc::c_uint
            == T_WindowFunc as libc::c_int as libc::c_uint
        {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_Query as libc::c_int as libc::c_uint
    {
        let mut result: bool_0 = 0;
        (*context).sublevels_up += 1;
        (*context).sublevels_up;
        result = query_tree_walker(
            node as *mut Query,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Node,
                        *mut check_agg_arguments_context,
                    ) -> bool_0,
                >,
                Option::<unsafe extern "C" fn() -> bool_0>,
            >(
                Some(
                    check_agg_arguments_walker
                        as unsafe extern "C" fn(
                            *mut Node,
                            *mut check_agg_arguments_context,
                        ) -> bool_0,
                ),
            ),
            context as *mut libc::c_void,
            0 as libc::c_int,
        );
        (*context).sublevels_up -= 1;
        (*context).sublevels_up;
        return result;
    }
    return expression_tree_walker(
        node,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Node,
                    *mut check_agg_arguments_context,
                ) -> bool_0,
            >,
            Option::<unsafe extern "C" fn() -> bool_0>,
        >(
            Some(
                check_agg_arguments_walker
                    as unsafe extern "C" fn(
                        *mut Node,
                        *mut check_agg_arguments_context,
                    ) -> bool_0,
            ),
        ),
        context as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn transformWindowFuncCall(
    mut pstate: *mut ParseState,
    mut wfunc: *mut WindowFunc,
    mut windef: *mut WindowDef,
) {
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    let mut errkind: bool_0 = 0;
    if (*pstate).p_hasWindowFuncs as libc::c_int != 0
        && contain_windowfuncs((*wfunc).args as *mut Node) as libc::c_int != 0
    {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    err = 0 as *const libc::c_char;
    errkind = 0 as libc::c_int as bool_0;
    match (*pstate).p_expr_kind as libc::c_uint {
        1 => {}
        2 | 3 => {
            err = b"window functions are not allowed in JOIN conditions\0" as *const u8
                as *const libc::c_char;
        }
        4 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        5 => {
            err = b"window functions are not allowed in functions in FROM\0" as *const u8
                as *const libc::c_char;
        }
        6 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        35 => {
            err = b"window functions are not allowed in policy expressions\0"
                as *const u8 as *const libc::c_char;
        }
        7 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        8 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        9 | 10 | 11 | 12 | 13 => {
            err = b"window functions are not allowed in window definitions\0"
                as *const u8 as *const libc::c_char;
        }
        14 => {}
        15 | 16 | 17 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        18 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        19 => {}
        20 => {}
        21 | 22 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        23 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        24 | 25 => {
            errkind = 1 as libc::c_int as bool_0;
        }
        26 | 27 => {
            err = b"window functions are not allowed in check constraints\0" as *const u8
                as *const libc::c_char;
        }
        28 | 29 => {
            err = b"window functions are not allowed in DEFAULT expressions\0"
                as *const u8 as *const libc::c_char;
        }
        30 => {
            err = b"window functions are not allowed in index expressions\0" as *const u8
                as *const libc::c_char;
        }
        31 => {
            err = b"window functions are not allowed in index predicates\0" as *const u8
                as *const libc::c_char;
        }
        32 => {
            err = b"window functions are not allowed in transform expressions\0"
                as *const u8 as *const libc::c_char;
        }
        33 => {
            err = b"window functions are not allowed in EXECUTE parameters\0"
                as *const u8 as *const libc::c_char;
        }
        34 => {
            err = b"window functions are not allowed in trigger WHEN conditions\0"
                as *const u8 as *const libc::c_char;
        }
        36 => {
            err = b"window functions are not allowed in partition bound\0" as *const u8
                as *const libc::c_char;
        }
        37 => {
            err = b"window functions are not allowed in partition key expressions\0"
                as *const u8 as *const libc::c_char;
        }
        38 => {
            err = b"window functions are not allowed in CALL arguments\0" as *const u8
                as *const libc::c_char;
        }
        39 => {
            err = b"window functions are not allowed in COPY FROM WHERE conditions\0"
                as *const u8 as *const libc::c_char;
        }
        40 => {
            err = b"window functions are not allowed in column generation expressions\0"
                as *const u8 as *const libc::c_char;
        }
        41 => {
            errkind = 1 as libc::c_int as bool_0;
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
    if errkind != 0 {
        let elevel__1: libc::c_int = 21 as libc::c_int;
        let mut __error_1: libc::c_int = 0;
        if elevel__1 >= 21 as libc::c_int {
            abort();
        }
    }
    if !((*windef).name).is_null() {
        let mut winref: Index = 0 as libc::c_int as Index;
        let mut lc: *mut ListCell = 0 as *mut ListCell;
        let mut lc__state: ForEachState = {
            let mut init = ForEachState {
                l: (*pstate).p_windowdefs,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length {
            lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize)
                as *mut ListCell;
            1 as libc::c_int as bool_0 as libc::c_int
        } else {
            lc = 0 as *mut ListCell;
            0 as libc::c_int as bool_0 as libc::c_int
        } != 0
        {
            let mut refwin: *mut WindowDef = (*lc).ptr_value as *mut WindowDef;
            winref = winref.wrapping_add(1);
            winref;
            if !((*refwin).name).is_null()
                && strcmp((*refwin).name, (*windef).name) == 0 as libc::c_int
            {
                (*wfunc).winref = winref;
                break;
            } else {
                lc__state.i += 1;
                lc__state.i;
            }
        }
        if lc.is_null() {
            let elevel__2: libc::c_int = 21 as libc::c_int;
            let mut __error_2: libc::c_int = 0;
            if elevel__2 >= 21 as libc::c_int {
                abort();
            }
        }
    } else {
        let mut winref_0: Index = 0 as libc::c_int as Index;
        let mut lc_0: *mut ListCell = 0 as *mut ListCell;
        let mut current_block_58: u64;
        let mut lc__state_0: ForEachState = {
            let mut init = ForEachState {
                l: (*pstate).p_windowdefs,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(lc__state_0.l).is_null() && lc__state_0.i < (*lc__state_0.l).length {
            lc_0 = &mut *((*lc__state_0.l).elements).offset(lc__state_0.i as isize)
                as *mut ListCell;
            1 as libc::c_int as bool_0 as libc::c_int
        } else {
            lc_0 = 0 as *mut ListCell;
            0 as libc::c_int as bool_0 as libc::c_int
        } != 0
        {
            let mut refwin_0: *mut WindowDef = (*lc_0).ptr_value as *mut WindowDef;
            winref_0 = winref_0.wrapping_add(1);
            winref_0;
            if !((*refwin_0).refname).is_null() && !((*windef).refname).is_null()
                && strcmp((*refwin_0).refname, (*windef).refname) == 0 as libc::c_int
            {
                current_block_58 = 8716029205547827362;
            } else if ((*refwin_0).refname).is_null() && ((*windef).refname).is_null() {
                current_block_58 = 8716029205547827362;
            } else {
                current_block_58 = 7318352876044315808;
            }
            match current_block_58 {
                8716029205547827362 => {
                    if equal(
                        (*refwin_0).partitionClause as *const libc::c_void,
                        (*windef).partitionClause as *const libc::c_void,
                    ) as libc::c_int != 0
                        && equal(
                            (*refwin_0).orderClause as *const libc::c_void,
                            (*windef).orderClause as *const libc::c_void,
                        ) as libc::c_int != 0
                        && (*refwin_0).frameOptions == (*windef).frameOptions
                        && equal(
                            (*refwin_0).startOffset as *const libc::c_void,
                            (*windef).startOffset as *const libc::c_void,
                        ) as libc::c_int != 0
                        && equal(
                            (*refwin_0).endOffset as *const libc::c_void,
                            (*windef).endOffset as *const libc::c_void,
                        ) as libc::c_int != 0
                    {
                        (*wfunc).winref = winref_0;
                        break;
                    }
                }
                _ => {}
            }
            lc__state_0.i += 1;
            lc__state_0.i;
        }
        if lc_0.is_null() {
            (*pstate)
                .p_windowdefs = lappend(
                (*pstate).p_windowdefs,
                windef as *mut libc::c_void,
            );
            (*wfunc).winref = list_length((*pstate).p_windowdefs) as Index;
        }
    }
    (*pstate).p_hasWindowFuncs = 1 as libc::c_int as bool_0;
}
#[no_mangle]
pub unsafe extern "C" fn parseCheckAggregates(
    mut pstate: *mut ParseState,
    mut qry: *mut Query,
) {
    let mut gset_common: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut groupClauses: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut groupClauseCommonVars: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut have_non_var_grouping: bool_0 = 0;
    let mut func_grouped_rels: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut hasJoinRTEs: bool_0 = 0;
    let mut hasSelfRefRTEs: bool_0 = 0;
    let mut clause: *mut Node = 0 as *mut Node;
    if !((*qry).groupingSets).is_null() {
        let mut gsets: *mut List = expand_grouping_sets(
            (*qry).groupingSets,
            4096 as libc::c_int,
        );
        if gsets.is_null() {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        gset_common = (*list_nth_cell(gsets, 0 as libc::c_int)).ptr_value as *mut List;
        if !gset_common.is_null() {
            let mut l__state: ForEachState = for_each_from_setup(
                gsets,
                1 as libc::c_int,
            );
            while if !(l__state.l).is_null() && l__state.i < (*l__state.l).length {
                l = &mut *((*l__state.l).elements).offset(l__state.i as isize)
                    as *mut ListCell;
                1 as libc::c_int as bool_0 as libc::c_int
            } else {
                l = 0 as *mut ListCell;
                0 as libc::c_int as bool_0 as libc::c_int
            } != 0
            {
                gset_common = list_intersection_int(
                    gset_common,
                    (*l).ptr_value as *const List,
                );
                if gset_common.is_null() {
                    break;
                }
                l__state.i += 1;
                l__state.i;
            }
        }
        if list_length(gsets) == 1 as libc::c_int && !((*qry).groupClause).is_null() {
            (*qry).groupingSets = 0 as *mut libc::c_void as *mut List;
        }
    }
    hasSelfRefRTEs = 0 as libc::c_int as bool_0;
    hasJoinRTEs = hasSelfRefRTEs;
    let mut l__state_0: ForEachState = {
        let mut init = ForEachState {
            l: (*pstate).p_rtable,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(l__state_0.l).is_null() && l__state_0.i < (*l__state_0.l).length {
        l = &mut *((*l__state_0.l).elements).offset(l__state_0.i as isize)
            as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        l = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut rte: *mut RangeTblEntry = (*l).ptr_value as *mut RangeTblEntry;
        if (*rte).rtekind as libc::c_uint == RTE_JOIN as libc::c_int as libc::c_uint {
            hasJoinRTEs = 1 as libc::c_int as bool_0;
        } else if (*rte).rtekind as libc::c_uint
            == RTE_CTE as libc::c_int as libc::c_uint
            && (*rte).self_reference as libc::c_int != 0
        {
            hasSelfRefRTEs = 1 as libc::c_int as bool_0;
        }
        l__state_0.i += 1;
        l__state_0.i;
    }
    let mut l__state_1: ForEachState = {
        let mut init = ForEachState {
            l: (*qry).groupClause,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(l__state_1.l).is_null() && l__state_1.i < (*l__state_1.l).length {
        l = &mut *((*l__state_1.l).elements).offset(l__state_1.i as isize)
            as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        l = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut grpcl: *mut SortGroupClause = (*l).ptr_value as *mut SortGroupClause;
        let mut expr: *mut TargetEntry = 0 as *mut TargetEntry;
        expr = get_sortgroupclause_tle(grpcl, (*qry).targetList);
        if !expr.is_null() {
            groupClauses = lappend(groupClauses, expr as *mut libc::c_void);
        }
        l__state_1.i += 1;
        l__state_1.i;
    }
    if hasJoinRTEs != 0 {
        groupClauses = flatten_join_alias_vars(qry, groupClauses as *mut Node)
            as *mut List;
    }
    have_non_var_grouping = 0 as libc::c_int as bool_0;
    let mut l__state_2: ForEachState = {
        let mut init = ForEachState {
            l: groupClauses,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(l__state_2.l).is_null() && l__state_2.i < (*l__state_2.l).length {
        l = &mut *((*l__state_2.l).elements).offset(l__state_2.i as isize)
            as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        l = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut tle: *mut TargetEntry = (*l).ptr_value as *mut TargetEntry;
        if !((*((*tle).expr as *const Node)).type_0 as libc::c_uint
            == T_Var as libc::c_int as libc::c_uint)
        {
            have_non_var_grouping = 1 as libc::c_int as bool_0;
        } else if ((*qry).groupingSets).is_null()
            || list_member_int(gset_common, (*tle).ressortgroupref as libc::c_int)
                as libc::c_int != 0
        {
            groupClauseCommonVars = lappend(
                groupClauseCommonVars,
                (*tle).expr as *mut libc::c_void,
            );
        }
        l__state_2.i += 1;
        l__state_2.i;
    }
    clause = (*qry).targetList as *mut Node;
    finalize_grouping_exprs(
        clause,
        pstate,
        qry,
        groupClauses,
        hasJoinRTEs,
        have_non_var_grouping,
    );
    if hasJoinRTEs != 0 {
        clause = flatten_join_alias_vars(qry, clause);
    }
    check_ungrouped_columns(
        clause,
        pstate,
        qry,
        groupClauses,
        groupClauseCommonVars,
        have_non_var_grouping,
        &mut func_grouped_rels,
    );
    clause = (*qry).havingQual;
    finalize_grouping_exprs(
        clause,
        pstate,
        qry,
        groupClauses,
        hasJoinRTEs,
        have_non_var_grouping,
    );
    if hasJoinRTEs != 0 {
        clause = flatten_join_alias_vars(qry, clause);
    }
    check_ungrouped_columns(
        clause,
        pstate,
        qry,
        groupClauses,
        groupClauseCommonVars,
        have_non_var_grouping,
        &mut func_grouped_rels,
    );
    if (*pstate).p_hasAggs as libc::c_int != 0 && hasSelfRefRTEs as libc::c_int != 0 {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
}
unsafe extern "C" fn check_ungrouped_columns(
    mut node: *mut Node,
    mut pstate: *mut ParseState,
    mut qry: *mut Query,
    mut groupClauses: *mut List,
    mut groupClauseCommonVars: *mut List,
    mut have_non_var_grouping: bool_0,
    mut func_grouped_rels: *mut *mut List,
) {
    let mut context: check_ungrouped_columns_context = check_ungrouped_columns_context {
        pstate: 0 as *mut ParseState,
        qry: 0 as *mut Query,
        hasJoinRTEs: 0,
        groupClauses: 0 as *mut List,
        groupClauseCommonVars: 0 as *mut List,
        have_non_var_grouping: 0,
        func_grouped_rels: 0 as *mut *mut List,
        sublevels_up: 0,
        in_agg_direct_args: 0,
    };
    context.pstate = pstate;
    context.qry = qry;
    context.hasJoinRTEs = 0 as libc::c_int as bool_0;
    context.groupClauses = groupClauses;
    context.groupClauseCommonVars = groupClauseCommonVars;
    context.have_non_var_grouping = have_non_var_grouping;
    context.func_grouped_rels = func_grouped_rels;
    context.sublevels_up = 0 as libc::c_int;
    context.in_agg_direct_args = 0 as libc::c_int as bool_0;
    check_ungrouped_columns_walker(node, &mut context);
}
unsafe extern "C" fn check_ungrouped_columns_walker(
    mut node: *mut Node,
    mut context: *mut check_ungrouped_columns_context,
) -> bool_0 {
    let mut gl: *mut ListCell = 0 as *mut ListCell;
    if node.is_null() {
        return 0 as libc::c_int as bool_0;
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_Const as libc::c_int as libc::c_uint
        || (*(node as *const Node)).type_0 as libc::c_uint
            == T_Param as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int as bool_0;
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_Aggref as libc::c_int as libc::c_uint
    {
        let mut agg: *mut Aggref = node as *mut Aggref;
        if (*agg).agglevelsup as libc::c_int == (*context).sublevels_up {
            let mut result: bool_0 = 0;
            (*context).in_agg_direct_args = 1 as libc::c_int as bool_0;
            result = check_ungrouped_columns_walker(
                (*agg).aggdirectargs as *mut Node,
                context,
            );
            (*context).in_agg_direct_args = 0 as libc::c_int as bool_0;
            return result;
        }
        if (*agg).agglevelsup as libc::c_int > (*context).sublevels_up {
            return 0 as libc::c_int as bool_0;
        }
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_GroupingFunc as libc::c_int as libc::c_uint
    {
        let mut grp: *mut GroupingFunc = node as *mut GroupingFunc;
        if (*grp).agglevelsup as libc::c_int >= (*context).sublevels_up {
            return 0 as libc::c_int as bool_0;
        }
    }
    if (*context).have_non_var_grouping as libc::c_int != 0
        && (*context).sublevels_up == 0 as libc::c_int
    {
        let mut gl__state: ForEachState = {
            let mut init = ForEachState {
                l: (*context).groupClauses,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(gl__state.l).is_null() && gl__state.i < (*gl__state.l).length {
            gl = &mut *((*gl__state.l).elements).offset(gl__state.i as isize)
                as *mut ListCell;
            1 as libc::c_int as bool_0 as libc::c_int
        } else {
            gl = 0 as *mut ListCell;
            0 as libc::c_int as bool_0 as libc::c_int
        } != 0
        {
            let mut tle: *mut TargetEntry = (*gl).ptr_value as *mut TargetEntry;
            if equal(node as *const libc::c_void, (*tle).expr as *const libc::c_void)
                != 0
            {
                return 0 as libc::c_int as bool_0;
            }
            gl__state.i += 1;
            gl__state.i;
        }
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_Var as libc::c_int as libc::c_uint
    {
        let mut var: *mut Var = node as *mut Var;
        let mut rte: *mut RangeTblEntry = 0 as *mut RangeTblEntry;
        let mut attname: *mut libc::c_char = 0 as *mut libc::c_char;
        if (*var).varlevelsup != (*context).sublevels_up as Index {
            return 0 as libc::c_int as bool_0;
        }
        if (*context).have_non_var_grouping == 0
            || (*context).sublevels_up != 0 as libc::c_int
        {
            let mut gl__state_0: ForEachState = {
                let mut init = ForEachState {
                    l: (*context).groupClauses,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(gl__state_0.l).is_null()
                && gl__state_0.i < (*gl__state_0.l).length
            {
                gl = &mut *((*gl__state_0.l).elements).offset(gl__state_0.i as isize)
                    as *mut ListCell;
                1 as libc::c_int as bool_0 as libc::c_int
            } else {
                gl = 0 as *mut ListCell;
                0 as libc::c_int as bool_0 as libc::c_int
            } != 0
            {
                let mut gvar: *mut Var = (*((*gl).ptr_value as *mut TargetEntry)).expr
                    as *mut Var;
                if (*(gvar as *const Node)).type_0 as libc::c_uint
                    == T_Var as libc::c_int as libc::c_uint
                    && (*gvar).varno == (*var).varno
                    && (*gvar).varattno as libc::c_int == (*var).varattno as libc::c_int
                    && (*gvar).varlevelsup == 0 as libc::c_int as Index
                {
                    return 0 as libc::c_int as bool_0;
                }
                gl__state_0.i += 1;
                gl__state_0.i;
            }
        }
        if list_member_int(*(*context).func_grouped_rels, (*var).varno as libc::c_int)
            != 0
        {
            return 0 as libc::c_int as bool_0;
        }
        rte = list_nth(
            (*(*context).pstate).p_rtable,
            ((*var).varno).wrapping_sub(1 as libc::c_int as Index) as libc::c_int,
        ) as *mut RangeTblEntry;
        if (*rte).rtekind as libc::c_uint == RTE_RELATION as libc::c_int as libc::c_uint
        {
            if check_functional_grouping(
                (*rte).relid,
                (*var).varno,
                0 as libc::c_int as Index,
                (*context).groupClauseCommonVars,
                &mut (*(*context).qry).constraintDeps,
            ) != 0
            {
                *(*context)
                    .func_grouped_rels = lappend_int(
                    *(*context).func_grouped_rels,
                    (*var).varno as libc::c_int,
                );
                return 0 as libc::c_int as bool_0;
            }
        }
        attname = get_rte_attribute_name(rte, (*var).varattno);
        if (*context).sublevels_up == 0 as libc::c_int {
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
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_Query as libc::c_int as libc::c_uint
    {
        let mut result_0: bool_0 = 0;
        (*context).sublevels_up += 1;
        (*context).sublevels_up;
        result_0 = query_tree_walker(
            node as *mut Query,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Node,
                        *mut check_ungrouped_columns_context,
                    ) -> bool_0,
                >,
                Option::<unsafe extern "C" fn() -> bool_0>,
            >(
                Some(
                    check_ungrouped_columns_walker
                        as unsafe extern "C" fn(
                            *mut Node,
                            *mut check_ungrouped_columns_context,
                        ) -> bool_0,
                ),
            ),
            context as *mut libc::c_void,
            0 as libc::c_int,
        );
        (*context).sublevels_up -= 1;
        (*context).sublevels_up;
        return result_0;
    }
    return expression_tree_walker(
        node,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Node,
                    *mut check_ungrouped_columns_context,
                ) -> bool_0,
            >,
            Option::<unsafe extern "C" fn() -> bool_0>,
        >(
            Some(
                check_ungrouped_columns_walker
                    as unsafe extern "C" fn(
                        *mut Node,
                        *mut check_ungrouped_columns_context,
                    ) -> bool_0,
            ),
        ),
        context as *mut libc::c_void,
    );
}
unsafe extern "C" fn finalize_grouping_exprs(
    mut node: *mut Node,
    mut pstate: *mut ParseState,
    mut qry: *mut Query,
    mut groupClauses: *mut List,
    mut hasJoinRTEs: bool_0,
    mut have_non_var_grouping: bool_0,
) {
    let mut context: check_ungrouped_columns_context = check_ungrouped_columns_context {
        pstate: 0 as *mut ParseState,
        qry: 0 as *mut Query,
        hasJoinRTEs: 0,
        groupClauses: 0 as *mut List,
        groupClauseCommonVars: 0 as *mut List,
        have_non_var_grouping: 0,
        func_grouped_rels: 0 as *mut *mut List,
        sublevels_up: 0,
        in_agg_direct_args: 0,
    };
    context.pstate = pstate;
    context.qry = qry;
    context.hasJoinRTEs = hasJoinRTEs;
    context.groupClauses = groupClauses;
    context.groupClauseCommonVars = 0 as *mut libc::c_void as *mut List;
    context.have_non_var_grouping = have_non_var_grouping;
    context.func_grouped_rels = 0 as *mut *mut List;
    context.sublevels_up = 0 as libc::c_int;
    context.in_agg_direct_args = 0 as libc::c_int as bool_0;
    finalize_grouping_exprs_walker(node, &mut context);
}
unsafe extern "C" fn finalize_grouping_exprs_walker(
    mut node: *mut Node,
    mut context: *mut check_ungrouped_columns_context,
) -> bool_0 {
    let mut gl: *mut ListCell = 0 as *mut ListCell;
    if node.is_null() {
        return 0 as libc::c_int as bool_0;
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_Const as libc::c_int as libc::c_uint
        || (*(node as *const Node)).type_0 as libc::c_uint
            == T_Param as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int as bool_0;
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_Aggref as libc::c_int as libc::c_uint
    {
        let mut agg: *mut Aggref = node as *mut Aggref;
        if (*agg).agglevelsup as libc::c_int == (*context).sublevels_up {
            let mut result: bool_0 = 0;
            (*context).in_agg_direct_args = 1 as libc::c_int as bool_0;
            result = finalize_grouping_exprs_walker(
                (*agg).aggdirectargs as *mut Node,
                context,
            );
            (*context).in_agg_direct_args = 0 as libc::c_int as bool_0;
            return result;
        }
        if (*agg).agglevelsup as libc::c_int > (*context).sublevels_up {
            return 0 as libc::c_int as bool_0;
        }
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_GroupingFunc as libc::c_int as libc::c_uint
    {
        let mut grp: *mut GroupingFunc = node as *mut GroupingFunc;
        if (*grp).agglevelsup as libc::c_int == (*context).sublevels_up {
            let mut lc: *mut ListCell = 0 as *mut ListCell;
            let mut ref_list: *mut List = 0 as *mut libc::c_void as *mut List;
            let mut lc__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*grp).args,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length {
                lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize)
                    as *mut ListCell;
                1 as libc::c_int as bool_0 as libc::c_int
            } else {
                lc = 0 as *mut ListCell;
                0 as libc::c_int as bool_0 as libc::c_int
            } != 0
            {
                let mut expr: *mut Node = (*lc).ptr_value as *mut Node;
                let mut ref_0: Index = 0 as libc::c_int as Index;
                if (*context).hasJoinRTEs != 0 {
                    expr = flatten_join_alias_vars((*context).qry, expr);
                }
                if (*(expr as *const Node)).type_0 as libc::c_uint
                    == T_Var as libc::c_int as libc::c_uint
                {
                    let mut var: *mut Var = expr as *mut Var;
                    if (*var).varlevelsup == (*context).sublevels_up as Index {
                        let mut gl__state: ForEachState = {
                            let mut init = ForEachState {
                                l: (*context).groupClauses,
                                i: 0 as libc::c_int,
                            };
                            init
                        };
                        while if !(gl__state.l).is_null()
                            && gl__state.i < (*gl__state.l).length
                        {
                            gl = &mut *((*gl__state.l).elements)
                                .offset(gl__state.i as isize) as *mut ListCell;
                            1 as libc::c_int as bool_0 as libc::c_int
                        } else {
                            gl = 0 as *mut ListCell;
                            0 as libc::c_int as bool_0 as libc::c_int
                        } != 0
                        {
                            let mut tle: *mut TargetEntry = (*gl).ptr_value
                                as *mut TargetEntry;
                            let mut gvar: *mut Var = (*tle).expr as *mut Var;
                            if (*(gvar as *const Node)).type_0 as libc::c_uint
                                == T_Var as libc::c_int as libc::c_uint
                                && (*gvar).varno == (*var).varno
                                && (*gvar).varattno as libc::c_int
                                    == (*var).varattno as libc::c_int
                                && (*gvar).varlevelsup == 0 as libc::c_int as Index
                            {
                                ref_0 = (*tle).ressortgroupref;
                                break;
                            } else {
                                gl__state.i += 1;
                                gl__state.i;
                            }
                        }
                    }
                } else if (*context).have_non_var_grouping as libc::c_int != 0
                    && (*context).sublevels_up == 0 as libc::c_int
                {
                    let mut gl__state_0: ForEachState = {
                        let mut init = ForEachState {
                            l: (*context).groupClauses,
                            i: 0 as libc::c_int,
                        };
                        init
                    };
                    while if !(gl__state_0.l).is_null()
                        && gl__state_0.i < (*gl__state_0.l).length
                    {
                        gl = &mut *((*gl__state_0.l).elements)
                            .offset(gl__state_0.i as isize) as *mut ListCell;
                        1 as libc::c_int as bool_0 as libc::c_int
                    } else {
                        gl = 0 as *mut ListCell;
                        0 as libc::c_int as bool_0 as libc::c_int
                    } != 0
                    {
                        let mut tle_0: *mut TargetEntry = (*gl).ptr_value
                            as *mut TargetEntry;
                        if equal(
                            expr as *const libc::c_void,
                            (*tle_0).expr as *const libc::c_void,
                        ) != 0
                        {
                            ref_0 = (*tle_0).ressortgroupref;
                            break;
                        } else {
                            gl__state_0.i += 1;
                            gl__state_0.i;
                        }
                    }
                }
                if ref_0 == 0 as libc::c_int as Index {
                    let elevel_: libc::c_int = 21 as libc::c_int;
                    let mut __error: libc::c_int = 0;
                    if elevel_ >= 21 as libc::c_int {
                        abort();
                    }
                }
                ref_list = lappend_int(ref_list, ref_0 as libc::c_int);
                lc__state.i += 1;
                lc__state.i;
            }
            (*grp).refs = ref_list;
        }
        if (*grp).agglevelsup as libc::c_int > (*context).sublevels_up {
            return 0 as libc::c_int as bool_0;
        }
    }
    if (*(node as *const Node)).type_0 as libc::c_uint
        == T_Query as libc::c_int as libc::c_uint
    {
        let mut result_0: bool_0 = 0;
        (*context).sublevels_up += 1;
        (*context).sublevels_up;
        result_0 = query_tree_walker(
            node as *mut Query,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut Node,
                        *mut check_ungrouped_columns_context,
                    ) -> bool_0,
                >,
                Option::<unsafe extern "C" fn() -> bool_0>,
            >(
                Some(
                    finalize_grouping_exprs_walker
                        as unsafe extern "C" fn(
                            *mut Node,
                            *mut check_ungrouped_columns_context,
                        ) -> bool_0,
                ),
            ),
            context as *mut libc::c_void,
            0 as libc::c_int,
        );
        (*context).sublevels_up -= 1;
        (*context).sublevels_up;
        return result_0;
    }
    return expression_tree_walker(
        node,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut Node,
                    *mut check_ungrouped_columns_context,
                ) -> bool_0,
            >,
            Option::<unsafe extern "C" fn() -> bool_0>,
        >(
            Some(
                finalize_grouping_exprs_walker
                    as unsafe extern "C" fn(
                        *mut Node,
                        *mut check_ungrouped_columns_context,
                    ) -> bool_0,
            ),
        ),
        context as *mut libc::c_void,
    );
}
unsafe extern "C" fn expand_groupingset_node(mut gs: *mut GroupingSet) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    match (*gs).kind as libc::c_uint {
        0 => {
            result = list_make1_impl(
                T_List,
                ListCell {
                    ptr_value: 0 as *mut libc::c_void as *mut List as *mut libc::c_void,
                },
            );
        }
        1 => {
            result = list_make1_impl(
                T_List,
                ListCell {
                    ptr_value: (*gs).content as *mut libc::c_void,
                },
            );
        }
        2 => {
            let mut rollup_val: *mut List = (*gs).content;
            let mut lc: *mut ListCell = 0 as *mut ListCell;
            let mut curgroup_size: libc::c_int = list_length((*gs).content);
            while curgroup_size > 0 as libc::c_int {
                let mut current_result: *mut List = 0 as *mut libc::c_void as *mut List;
                let mut i: libc::c_int = curgroup_size;
                let mut lc__state: ForEachState = {
                    let mut init = ForEachState {
                        l: rollup_val,
                        i: 0 as libc::c_int,
                    };
                    init
                };
                while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length
                {
                    lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize)
                        as *mut ListCell;
                    1 as libc::c_int as bool_0 as libc::c_int
                } else {
                    lc = 0 as *mut ListCell;
                    0 as libc::c_int as bool_0 as libc::c_int
                } != 0
                {
                    let mut gs_current: *mut GroupingSet = (*lc).ptr_value
                        as *mut GroupingSet;
                    current_result = list_concat(current_result, (*gs_current).content);
                    i -= 1;
                    if i == 0 as libc::c_int {
                        break;
                    }
                    lc__state.i += 1;
                    lc__state.i;
                }
                result = lappend(result, current_result as *mut libc::c_void);
                curgroup_size -= 1;
                curgroup_size;
            }
            result = lappend(
                result,
                0 as *mut libc::c_void as *mut List as *mut libc::c_void,
            );
        }
        3 => {
            let mut cube_list: *mut List = (*gs).content;
            let mut number_bits: libc::c_int = list_length(cube_list);
            let mut num_sets: uint32 = 0;
            let mut i_0: uint32 = 0;
            num_sets = (1 as libc::c_uint) << number_bits;
            i_0 = 0 as libc::c_int as uint32;
            while i_0 < num_sets {
                let mut current_result_0: *mut List = 0 as *mut libc::c_void
                    as *mut List;
                let mut lc_0: *mut ListCell = 0 as *mut ListCell;
                let mut mask: uint32 = 1 as libc::c_uint;
                let mut lc__state_0: ForEachState = {
                    let mut init = ForEachState {
                        l: cube_list,
                        i: 0 as libc::c_int,
                    };
                    init
                };
                while if !(lc__state_0.l).is_null()
                    && lc__state_0.i < (*lc__state_0.l).length
                {
                    lc_0 = &mut *((*lc__state_0.l).elements)
                        .offset(lc__state_0.i as isize) as *mut ListCell;
                    1 as libc::c_int as bool_0 as libc::c_int
                } else {
                    lc_0 = 0 as *mut ListCell;
                    0 as libc::c_int as bool_0 as libc::c_int
                } != 0
                {
                    let mut gs_current_0: *mut GroupingSet = (*lc_0).ptr_value
                        as *mut GroupingSet;
                    if mask & i_0 != 0 {
                        current_result_0 = list_concat(
                            current_result_0,
                            (*gs_current_0).content,
                        );
                    }
                    mask <<= 1 as libc::c_int;
                    lc__state_0.i += 1;
                    lc__state_0.i;
                }
                result = lappend(result, current_result_0 as *mut libc::c_void);
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        }
        4 => {
            let mut lc_1: *mut ListCell = 0 as *mut ListCell;
            let mut lc__state_1: ForEachState = {
                let mut init = ForEachState {
                    l: (*gs).content,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(lc__state_1.l).is_null()
                && lc__state_1.i < (*lc__state_1.l).length
            {
                lc_1 = &mut *((*lc__state_1.l).elements).offset(lc__state_1.i as isize)
                    as *mut ListCell;
                1 as libc::c_int as bool_0 as libc::c_int
            } else {
                lc_1 = 0 as *mut ListCell;
                0 as libc::c_int as bool_0 as libc::c_int
            } != 0
            {
                let mut current_result_1: *mut List = expand_groupingset_node(
                    (*lc_1).ptr_value as *mut GroupingSet,
                );
                result = list_concat(result, current_result_1);
                lc__state_1.i += 1;
                lc__state_1.i;
            }
        }
        _ => {}
    }
    return result;
}
unsafe extern "C" fn cmp_list_len_asc(
    mut a: *const ListCell,
    mut b: *const ListCell,
) -> libc::c_int {
    let mut la: libc::c_int = list_length((*a).ptr_value as *const List);
    let mut lb: libc::c_int = list_length((*b).ptr_value as *const List);
    return if la > lb {
        1 as libc::c_int
    } else if la == lb {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn expand_grouping_sets(
    mut groupingSets: *mut List,
    mut limit: libc::c_int,
) -> *mut List {
    let mut expanded_groups: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut numsets: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    if groupingSets.is_null() {
        return 0 as *mut libc::c_void as *mut List;
    }
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: groupingSets,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length {
        lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize)
            as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        lc = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut current_result: *mut List = 0 as *mut libc::c_void as *mut List;
        let mut gs: *mut GroupingSet = (*lc).ptr_value as *mut GroupingSet;
        current_result = expand_groupingset_node(gs);
        numsets *= list_length(current_result) as libc::c_double;
        if limit >= 0 as libc::c_int && numsets > limit as libc::c_double {
            return 0 as *mut libc::c_void as *mut List;
        }
        expanded_groups = lappend(expanded_groups, current_result as *mut libc::c_void);
        lc__state.i += 1;
        lc__state.i;
    }
    let mut lc__state_0: ForEachState = {
        let mut init = ForEachState {
            l: (*list_nth_cell(expanded_groups, 0 as libc::c_int)).ptr_value
                as *mut List,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(lc__state_0.l).is_null() && lc__state_0.i < (*lc__state_0.l).length {
        lc = &mut *((*lc__state_0.l).elements).offset(lc__state_0.i as isize)
            as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        lc = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        result = lappend(
            result,
            list_union_int(
                0 as *mut libc::c_void as *mut List,
                (*lc).ptr_value as *mut List,
            ) as *mut libc::c_void,
        );
        lc__state_0.i += 1;
        lc__state_0.i;
    }
    let mut lc__state_1: ForEachState = for_each_from_setup(
        expanded_groups,
        1 as libc::c_int,
    );
    while if !(lc__state_1.l).is_null() && lc__state_1.i < (*lc__state_1.l).length {
        lc = &mut *((*lc__state_1.l).elements).offset(lc__state_1.i as isize)
            as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        lc = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut p: *mut List = (*lc).ptr_value as *mut List;
        let mut new_result: *mut List = 0 as *mut libc::c_void as *mut List;
        let mut lc2: *mut ListCell = 0 as *mut ListCell;
        let mut lc2__state: ForEachState = {
            let mut init = ForEachState {
                l: result,
                i: 0 as libc::c_int,
            };
            init
        };
        while if !(lc2__state.l).is_null() && lc2__state.i < (*lc2__state.l).length {
            lc2 = &mut *((*lc2__state.l).elements).offset(lc2__state.i as isize)
                as *mut ListCell;
            1 as libc::c_int as bool_0 as libc::c_int
        } else {
            lc2 = 0 as *mut ListCell;
            0 as libc::c_int as bool_0 as libc::c_int
        } != 0
        {
            let mut q: *mut List = (*lc2).ptr_value as *mut List;
            let mut lc3: *mut ListCell = 0 as *mut ListCell;
            let mut lc3__state: ForEachState = {
                let mut init = ForEachState {
                    l: p,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(lc3__state.l).is_null() && lc3__state.i < (*lc3__state.l).length {
                lc3 = &mut *((*lc3__state.l).elements).offset(lc3__state.i as isize)
                    as *mut ListCell;
                1 as libc::c_int as bool_0 as libc::c_int
            } else {
                lc3 = 0 as *mut ListCell;
                0 as libc::c_int as bool_0 as libc::c_int
            } != 0
            {
                new_result = lappend(
                    new_result,
                    list_union_int(q, (*lc3).ptr_value as *mut List) as *mut libc::c_void,
                );
                lc3__state.i += 1;
                lc3__state.i;
            }
            lc2__state.i += 1;
            lc2__state.i;
        }
        result = new_result;
        lc__state_1.i += 1;
        lc__state_1.i;
    }
    list_sort(
        result,
        Some(
            cmp_list_len_asc
                as unsafe extern "C" fn(*const ListCell, *const ListCell) -> libc::c_int,
        ),
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn get_aggregate_argtypes(
    mut aggref: *mut Aggref,
    mut inputTypes: *mut Oid,
) -> libc::c_int {
    let mut numArguments: libc::c_int = 0 as libc::c_int;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*aggref).aggargtypes,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(lc__state.l).is_null() && lc__state.i < (*lc__state.l).length {
        lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize)
            as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        lc = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let fresh3 = numArguments;
        numArguments = numArguments + 1;
        *inputTypes.offset(fresh3 as isize) = (*lc).oid_value;
        lc__state.i += 1;
        lc__state.i;
    }
    return numArguments;
}
#[no_mangle]
pub unsafe extern "C" fn resolve_aggregate_transtype(
    mut aggfuncid: Oid,
    mut aggtranstype: Oid,
    mut inputTypes: *mut Oid,
    mut numArguments: libc::c_int,
) -> Oid {
    if IsPolymorphicType(aggtranstype) != 0 {
        let mut declaredArgTypes: *mut Oid = 0 as *mut Oid;
        let mut agg_nargs: libc::c_int = 0;
        get_func_signature(aggfuncid, &mut declaredArgTypes, &mut agg_nargs);
        aggtranstype = enforce_generic_type_consistency(
            inputTypes,
            declaredArgTypes,
            agg_nargs,
            aggtranstype,
            0 as libc::c_int as bool_0,
        );
        pfree(declaredArgTypes as *mut libc::c_void);
    }
    return aggtranstype;
}
#[no_mangle]
pub unsafe extern "C" fn build_aggregate_transfn_expr(
    mut agg_input_types: *mut Oid,
    mut agg_num_inputs: libc::c_int,
    mut agg_num_direct_inputs: libc::c_int,
    mut agg_variadic: bool_0,
    mut agg_state_type: Oid,
    mut agg_input_collation: Oid,
    mut transfn_oid: Oid,
    mut invtransfn_oid: Oid,
    mut transfnexpr: *mut *mut Expr,
    mut invtransfnexpr: *mut *mut Expr,
) {
    let mut args: *mut List = 0 as *mut List;
    let mut fexpr: *mut FuncExpr = 0 as *mut FuncExpr;
    let mut i: libc::c_int = 0;
    args = list_make1_impl(
        T_List,
        ListCell {
            ptr_value: make_agg_arg(agg_state_type, agg_input_collation)
                as *mut libc::c_void,
        },
    );
    i = agg_num_direct_inputs;
    while i < agg_num_inputs {
        args = lappend(
            args,
            make_agg_arg(*agg_input_types.offset(i as isize), agg_input_collation)
                as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    fexpr = makeFuncExpr(
        transfn_oid,
        agg_state_type,
        args,
        0 as libc::c_int as Oid,
        agg_input_collation,
        COERCE_EXPLICIT_CALL,
    );
    (*fexpr).funcvariadic = agg_variadic;
    *transfnexpr = fexpr as *mut Expr;
    if !invtransfnexpr.is_null() {
        if (invtransfn_oid != 0 as libc::c_int as Oid) as libc::c_int as bool_0 != 0 {
            fexpr = makeFuncExpr(
                invtransfn_oid,
                agg_state_type,
                args,
                0 as libc::c_int as Oid,
                agg_input_collation,
                COERCE_EXPLICIT_CALL,
            );
            (*fexpr).funcvariadic = agg_variadic;
            *invtransfnexpr = fexpr as *mut Expr;
        } else {
            *invtransfnexpr = 0 as *mut Expr;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn build_aggregate_combinefn_expr(
    mut agg_state_type: Oid,
    mut agg_input_collation: Oid,
    mut combinefn_oid: Oid,
    mut combinefnexpr: *mut *mut Expr,
) {
    let mut argp: *mut Node = 0 as *mut Node;
    let mut args: *mut List = 0 as *mut List;
    let mut fexpr: *mut FuncExpr = 0 as *mut FuncExpr;
    argp = make_agg_arg(agg_state_type, agg_input_collation);
    args = list_make2_impl(
        T_List,
        ListCell {
            ptr_value: argp as *mut libc::c_void,
        },
        ListCell {
            ptr_value: argp as *mut libc::c_void,
        },
    );
    fexpr = makeFuncExpr(
        combinefn_oid,
        agg_state_type,
        args,
        0 as libc::c_int as Oid,
        agg_input_collation,
        COERCE_EXPLICIT_CALL,
    );
    *combinefnexpr = fexpr as *mut Expr;
}
#[no_mangle]
pub unsafe extern "C" fn build_aggregate_serialfn_expr(
    mut serialfn_oid: Oid,
    mut serialfnexpr: *mut *mut Expr,
) {
    let mut args: *mut List = 0 as *mut List;
    let mut fexpr: *mut FuncExpr = 0 as *mut FuncExpr;
    *serialfnexpr = fexpr as *mut Expr;
}
#[no_mangle]
pub unsafe extern "C" fn build_aggregate_deserialfn_expr(
    mut deserialfn_oid: Oid,
    mut deserialfnexpr: *mut *mut Expr,
) {
    let mut args: *mut List = 0 as *mut List;
    let mut fexpr: *mut FuncExpr = 0 as *mut FuncExpr;
    *deserialfnexpr = fexpr as *mut Expr;
}
#[no_mangle]
pub unsafe extern "C" fn build_aggregate_finalfn_expr(
    mut agg_input_types: *mut Oid,
    mut num_finalfn_inputs: libc::c_int,
    mut agg_state_type: Oid,
    mut agg_result_type: Oid,
    mut agg_input_collation: Oid,
    mut finalfn_oid: Oid,
    mut finalfnexpr: *mut *mut Expr,
) {
    let mut args: *mut List = 0 as *mut List;
    let mut i: libc::c_int = 0;
    args = list_make1_impl(
        T_List,
        ListCell {
            ptr_value: make_agg_arg(agg_state_type, agg_input_collation)
                as *mut libc::c_void,
        },
    );
    i = 0 as libc::c_int;
    while i < num_finalfn_inputs - 1 as libc::c_int {
        args = lappend(
            args,
            make_agg_arg(*agg_input_types.offset(i as isize), agg_input_collation)
                as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    *finalfnexpr = makeFuncExpr(
        finalfn_oid,
        agg_result_type,
        args,
        0 as libc::c_int as Oid,
        agg_input_collation,
        COERCE_EXPLICIT_CALL,
    ) as *mut Expr;
}
unsafe extern "C" fn make_agg_arg(mut argtype: Oid, mut argcollation: Oid) -> *mut Node {
    let mut argp: *mut Param = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_Param;
        _result
    }) as *mut Param;
    (*argp).paramkind = PARAM_EXEC;
    (*argp).paramid = -(1 as libc::c_int);
    (*argp).paramtype = argtype;
    (*argp).paramtypmod = -(1 as libc::c_int);
    (*argp).paramcollid = argcollation;
    (*argp).location = -(1 as libc::c_int);
    return argp as *mut Node;
}
