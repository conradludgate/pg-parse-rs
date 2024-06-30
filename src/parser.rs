#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// #![feature(extern_types, linkage)]
// extern "C" {
//     pub type SelectLimit;
//     pub type ImportQual;
//     pub type PrivTarget;
//     fn abort() -> !;
//     fn strlen(_: *const libc::c_char) -> libc::c_ulong;
//     static mut _DefaultRuneLocale: _RuneLocale;
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool_0;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errcode(sqlerrcode: libc::c_int) -> libc::c_int;
//     fn errmsg(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn errhint(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn palloc(size: Size) -> *mut libc::c_void;
//     fn repalloc(pointer: *mut libc::c_void, size: Size) -> *mut libc::c_void;
//     fn pg_unicode_to_server(c: pg_wchar, s: *mut libc::c_uchar);
//     fn base_yyparse(yyscanner: core_yyscan_t) -> libc::c_int;
//     fn parser_init(yyext: *mut base_yy_extra_type);
//     fn scanner_yyerror(message: *const libc::c_char, yyscanner: core_yyscan_t) -> !;
//     fn cancel_scanner_errposition_callback(scbstate: *mut ScannerCallbackState);
//     fn setup_scanner_errposition_callback(
//         scbstate: *mut ScannerCallbackState,
//         yyscanner: core_yyscan_t,
//         location: libc::c_int,
//     );
//     fn scanner_errposition(
//         location: libc::c_int,
//         yyscanner: core_yyscan_t,
//     ) -> libc::c_int;
//     fn core_yylex(
//         lvalp: *mut core_YYSTYPE,
//         llocp: *mut libc::c_int,
//         yyscanner: core_yyscan_t,
//     ) -> libc::c_int;
//     fn scanner_finish(yyscanner: core_yyscan_t);
//     fn scanner_init(
//         str: *const libc::c_char,
//         yyext: *mut core_yy_extra_type,
//         keywordlist: *const ScanKeywordList,
//         keyword_tokens: *const uint16,
//     ) -> core_yyscan_t;
//     static ScanKeywordTokens: [uint16; 0];
//     static ScanKeywords: ScanKeywordList;
//     fn truncate_identifier(ident: *mut libc::c_char, len: libc::c_int, warn: bool_0);
//     fn scanner_isspace(ch: libc::c_char) -> bool_0;
// }
use super::*;
pub type Oid = libc::c_uint;
pub type __uint32_t = libc::c_uint;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type size_t = __darwin_size_t;
pub type bool_0 = libc::c_uchar;
pub type int32 = libc::c_int;
pub type uint16 = libc::c_ushort;
pub type Size = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [libc::c_char; 8],
    pub __encoding: [libc::c_char; 32],
    pub __sgetrune: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            __darwin_size_t,
            *mut *const libc::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option::<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut libc::c_char,
            __darwin_size_t,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: libc::c_int,
    pub __ncharclasses: libc::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ErrorContextCallback {
    pub previous: *mut ErrorContextCallback,
    pub callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
pub type pg_wchar = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub type_0: NodeTag,
}
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
    pub inh: bool_0,
    pub relpersistence: libc::c_char,
    pub alias: *mut Alias,
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
    pub skipData: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JoinExpr {
    pub type_0: NodeTag,
    pub jointype: JoinType,
    pub isNatural: bool_0,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PartitionBoundSpec {
    pub type_0: NodeTag,
    pub strategy: libc::c_char,
    pub is_default: bool_0,
    pub modulus: libc::c_int,
    pub remainder: libc::c_int,
    pub listdatums: *mut List,
    pub lowerdatums: *mut List,
    pub upperdatums: *mut List,
    pub location: libc::c_int,
}
pub type OverridingKind = libc::c_uint;
pub const OVERRIDING_SYSTEM_VALUE: OverridingKind = 2;
pub const OVERRIDING_USER_VALUE: OverridingKind = 1;
pub const OVERRIDING_NOT_SET: OverridingKind = 0;
pub type SortByDir = libc::c_uint;
pub const SORTBY_USING: SortByDir = 3;
pub const SORTBY_DESC: SortByDir = 2;
pub const SORTBY_ASC: SortByDir = 1;
pub const SORTBY_DEFAULT: SortByDir = 0;
pub type SortByNulls = libc::c_uint;
pub const SORTBY_NULLS_LAST: SortByNulls = 2;
pub const SORTBY_NULLS_FIRST: SortByNulls = 1;
pub const SORTBY_NULLS_DEFAULT: SortByNulls = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TypeName {
    pub type_0: NodeTag,
    pub names: *mut List,
    pub typeOid: Oid,
    pub setof: bool_0,
    pub pct_type: bool_0,
    pub typmods: *mut List,
    pub typemod: int32,
    pub arrayBounds: *mut List,
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
    pub is_slice: bool_0,
    pub lidx: *mut Node,
    pub uidx: *mut Node,
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
pub struct PartitionElem {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub expr: *mut Node,
    pub collation: *mut List,
    pub opclass: *mut List,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WithClause {
    pub type_0: NodeTag,
    pub ctes: *mut List,
    pub recursive: bool_0,
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
pub type DropBehavior = libc::c_uint;
pub const DROP_CASCADE: DropBehavior = 1;
pub const DROP_RESTRICT: DropBehavior = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectWithArgs {
    pub type_0: NodeTag,
    pub objname: *mut List,
    pub objargs: *mut List,
    pub args_unspecified: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AccessPriv {
    pub type_0: NodeTag,
    pub priv_name: *mut libc::c_char,
    pub cols: *mut List,
}
pub type VariableSetKind = libc::c_uint;
pub const VAR_RESET_ALL: VariableSetKind = 5;
pub const VAR_RESET: VariableSetKind = 4;
pub const VAR_SET_MULTI: VariableSetKind = 3;
pub const VAR_SET_CURRENT: VariableSetKind = 2;
pub const VAR_SET_DEFAULT: VariableSetKind = 1;
pub const VAR_SET_VALUE: VariableSetKind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VariableSetStmt {
    pub type_0: NodeTag,
    pub kind: VariableSetKind,
    pub name: *mut libc::c_char,
    pub args: *mut List,
    pub is_local: bool_0,
}
pub type FunctionParameterMode = libc::c_uint;
pub const FUNC_PARAM_TABLE: FunctionParameterMode = 116;
pub const FUNC_PARAM_VARIADIC: FunctionParameterMode = 118;
pub const FUNC_PARAM_INOUT: FunctionParameterMode = 98;
pub const FUNC_PARAM_OUT: FunctionParameterMode = 111;
pub const FUNC_PARAM_IN: FunctionParameterMode = 105;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionParameter {
    pub type_0: NodeTag,
    pub name: *mut libc::c_char,
    pub argType: *mut TypeName,
    pub mode: FunctionParameterMode,
    pub defexpr: *mut Node,
}
pub type ScanKeywordHashFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScanKeywordList {
    pub kw_string: *const libc::c_char,
    pub kw_offsets: *const uint16,
    pub hash: ScanKeywordHashFunc,
    pub num_keywords: libc::c_int,
    pub max_kw_len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union core_YYSTYPE {
    pub ival: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub keyword: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct core_yy_extra_type {
    pub scanbuf: *mut libc::c_char,
    pub scanbuflen: Size,
    pub keywordlist: *const ScanKeywordList,
    pub keyword_tokens: *const uint16,
    pub backslash_quote: libc::c_int,
    pub escape_string_warning: bool_0,
    pub standard_conforming_strings: bool_0,
    pub literalbuf: *mut libc::c_char,
    pub literallen: libc::c_int,
    pub literalalloc: libc::c_int,
    pub state_before_str_stop: libc::c_int,
    pub xcdepth: libc::c_int,
    pub dolqstart: *mut libc::c_char,
    pub save_yylloc: libc::c_int,
    pub utf16_first_part: int32,
    pub warn_on_first_escape: bool_0,
    pub saw_non_ascii: bool_0,
}
pub type core_yyscan_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScannerCallbackState {
    pub yyscanner: core_yyscan_t,
    pub location: libc::c_int,
    pub errcallback: ErrorContextCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub core_yystype: core_YYSTYPE,
    pub ival: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub keyword: *const libc::c_char,
    pub chr: libc::c_char,
    pub boolean: bool_0,
    pub jtype: JoinType,
    pub dbehavior: DropBehavior,
    pub oncommit: OnCommitAction,
    pub list: *mut List,
    pub node: *mut Node,
    pub value: *mut Value,
    pub objtype: ObjectType,
    pub typnam: *mut TypeName,
    pub fun_param: *mut FunctionParameter,
    pub fun_param_mode: FunctionParameterMode,
    pub objwithargs: *mut ObjectWithArgs,
    pub defelt: *mut DefElem,
    pub sortby: *mut SortBy,
    pub windef: *mut WindowDef,
    pub jexpr: *mut JoinExpr,
    pub ielem: *mut IndexElem,
    pub alias: *mut Alias,
    pub range: *mut RangeVar,
    pub into: *mut IntoClause,
    pub with: *mut WithClause,
    pub infer: *mut InferClause,
    pub onconflict: *mut OnConflictClause,
    pub aind: *mut A_Indices,
    pub target: *mut ResTarget,
    pub privtarget: *mut PrivTarget,
    pub accesspriv: *mut AccessPriv,
    pub importqual: *mut ImportQual,
    pub istmt: *mut InsertStmt,
    pub vsetstmt: *mut VariableSetStmt,
    pub partelem: *mut PartitionElem,
    pub partspec: *mut PartitionSpec,
    pub partboundspec: *mut PartitionBoundSpec,
    pub rolespec: *mut RoleSpec,
    pub selectlimit: *mut SelectLimit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base_yy_extra_type {
    pub core_yy_extra: core_yy_extra_type,
    pub have_lookahead: bool_0,
    pub lookahead_token: libc::c_int,
    pub lookahead_yylval: core_YYSTYPE,
    pub lookahead_yylloc: libc::c_int,
    pub lookahead_end: *mut libc::c_char,
    pub lookahead_hold_char: libc::c_char,
    pub parsetree: *mut List,
}
pub type RawParseMode = libc::c_uint;
pub const RAW_PARSE_PLPGSQL_ASSIGN3: RawParseMode = 5;
pub const RAW_PARSE_PLPGSQL_ASSIGN2: RawParseMode = 4;
pub const RAW_PARSE_PLPGSQL_ASSIGN1: RawParseMode = 3;
pub const RAW_PARSE_PLPGSQL_EXPR: RawParseMode = 2;
pub const RAW_PARSE_TYPE_NAME: RawParseMode = 1;
pub const RAW_PARSE_DEFAULT: RawParseMode = 0;
#[inline]
unsafe extern "C" fn __isctype(
    mut _c: __darwin_ct_rune_t,
    mut _f: libc::c_ulong,
) -> __darwin_ct_rune_t {
    return if _c < 0 as libc::c_int || _c >= (1 as libc::c_int) << 8 as libc::c_int {
        0 as libc::c_int
    } else {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0)
            as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isxdigit(mut _c: libc::c_int) -> libc::c_int {
    return __isctype(_c, 0x10000 as libc::c_long as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn is_valid_unicode_codepoint(mut c: pg_wchar) -> bool_0 {
    return (c > 0 as libc::c_int as pg_wchar && c <= 0x10ffff as libc::c_int as pg_wchar)
        as libc::c_int as bool_0;
}
#[inline]
unsafe extern "C" fn is_utf16_surrogate_first(mut c: pg_wchar) -> bool_0 {
    return (c >= 0xd800 as libc::c_int as pg_wchar
        && c <= 0xdbff as libc::c_int as pg_wchar) as libc::c_int as bool_0;
}
#[inline]
unsafe extern "C" fn is_utf16_surrogate_second(mut c: pg_wchar) -> bool_0 {
    return (c >= 0xdc00 as libc::c_int as pg_wchar
        && c <= 0xdfff as libc::c_int as pg_wchar) as libc::c_int as bool_0;
}
#[inline]
unsafe extern "C" fn surrogate_pair_to_codepoint(
    mut first: pg_wchar,
    mut second: pg_wchar,
) -> pg_wchar {
    return ((first & 0x3ff as libc::c_int as pg_wchar) << 10 as libc::c_int)
        .wrapping_add(0x10000 as libc::c_int as pg_wchar)
        .wrapping_add(second & 0x3ff as libc::c_int as pg_wchar);
}
#[no_mangle]
pub unsafe extern "C" fn raw_parser(
    mut str: *const libc::c_char,
    mut mode: RawParseMode,
) -> *mut List {
    let mut yyscanner: core_yyscan_t = 0 as *mut libc::c_void;
    let mut yyextra: base_yy_extra_type = base_yy_extra_type {
        core_yy_extra: core_yy_extra_type {
            scanbuf: 0 as *mut libc::c_char,
            scanbuflen: 0,
            keywordlist: 0 as *const ScanKeywordList,
            keyword_tokens: 0 as *const uint16,
            backslash_quote: 0,
            escape_string_warning: 0,
            standard_conforming_strings: 0,
            literalbuf: 0 as *mut libc::c_char,
            literallen: 0,
            literalalloc: 0,
            state_before_str_stop: 0,
            xcdepth: 0,
            dolqstart: 0 as *mut libc::c_char,
            save_yylloc: 0,
            utf16_first_part: 0,
            warn_on_first_escape: 0,
            saw_non_ascii: 0,
        },
        have_lookahead: 0,
        lookahead_token: 0,
        lookahead_yylval: core_YYSTYPE { ival: 0 },
        lookahead_yylloc: 0,
        lookahead_end: 0 as *mut libc::c_char,
        lookahead_hold_char: 0,
        parsetree: 0 as *mut List,
    };
    let mut yyresult: libc::c_int = 0;
    yyscanner = scanner_init(
        str,
        &mut yyextra.core_yy_extra,
        &ScanKeywords,
        ScanKeywordTokens.as_ptr(),
    );
    if mode as libc::c_uint == RAW_PARSE_DEFAULT as libc::c_int as libc::c_uint {
        yyextra.have_lookahead = 0 as libc::c_int as bool_0;
    } else {
        static mut mode_token: [libc::c_int; 6] = [
            0 as libc::c_int,
            730 as libc::c_int,
            731 as libc::c_int,
            732 as libc::c_int,
            733 as libc::c_int,
            734 as libc::c_int,
        ];
        yyextra.have_lookahead = 1 as libc::c_int as bool_0;
        yyextra.lookahead_token = mode_token[mode as usize];
        yyextra.lookahead_yylloc = 0 as libc::c_int;
        yyextra.lookahead_end = 0 as *mut libc::c_char;
    }
    parser_init(&mut yyextra);
    yyresult = base_yyparse(yyscanner);
    scanner_finish(yyscanner);
    if yyresult != 0 {
        return 0 as *mut libc::c_void as *mut List;
    }
    return yyextra.parsetree;
}
#[no_mangle]
pub unsafe extern "C" fn base_yylex(
    mut lvalp: *mut YYSTYPE,
    mut llocp: *mut libc::c_int,
    mut yyscanner: core_yyscan_t,
) -> libc::c_int {
    let mut yyextra: *mut base_yy_extra_type = *(yyscanner
        as *mut *mut base_yy_extra_type);
    let mut cur_token: libc::c_int = 0;
    let mut next_token: libc::c_int = 0;
    let mut cur_token_length: libc::c_int = 0;
    let mut cur_yylloc: libc::c_int = 0;
    if (*yyextra).have_lookahead != 0 {
        cur_token = (*yyextra).lookahead_token;
        (*lvalp).core_yystype = (*yyextra).lookahead_yylval;
        *llocp = (*yyextra).lookahead_yylloc;
        if !((*yyextra).lookahead_end).is_null() {
            *(*yyextra).lookahead_end = (*yyextra).lookahead_hold_char;
        }
        (*yyextra).have_lookahead = 0 as libc::c_int as bool_0;
    } else {
        cur_token = core_yylex(&mut (*lvalp).core_yystype, llocp, yyscanner);
    }
    match cur_token {
        520 => {
            cur_token_length = 3 as libc::c_int;
        }
        527 => {
            cur_token_length = 5 as libc::c_int;
        }
        706 => {
            cur_token_length = 4 as libc::c_int;
        }
        259 | 262 => {
            cur_token_length = strlen(
                ((*yyextra).core_yy_extra.scanbuf).offset(*llocp as isize),
            ) as libc::c_int;
        }
        _ => return cur_token,
    }
    (*yyextra)
        .lookahead_end = ((*yyextra).core_yy_extra.scanbuf)
        .offset(*llocp as isize)
        .offset(cur_token_length as isize);
    cur_yylloc = *llocp;
    next_token = core_yylex(&mut (*yyextra).lookahead_yylval, llocp, yyscanner);
    (*yyextra).lookahead_token = next_token;
    (*yyextra).lookahead_yylloc = *llocp;
    *llocp = cur_yylloc;
    (*yyextra).lookahead_hold_char = *(*yyextra).lookahead_end;
    *(*yyextra).lookahead_end = '\0' as i32 as libc::c_char;
    (*yyextra).have_lookahead = 1 as libc::c_int as bool_0;
    match cur_token {
        520 => {
            match next_token {
                304 | 447 | 484 | 442 | 627 => {
                    cur_token = 727 as libc::c_int;
                }
                _ => {}
            }
        }
        527 => {
            match next_token {
                415 | 477 => {
                    cur_token = 728 as libc::c_int;
                }
                _ => {}
            }
        }
        706 => {
            match next_token {
                661 | 542 => {
                    cur_token = 729 as libc::c_int;
                }
                _ => {}
            }
        }
        259 | 262 => {
            if next_token == 675 as libc::c_int {
                let mut escstr: *const libc::c_char = 0 as *const libc::c_char;
                cur_yylloc = *llocp;
                *(*yyextra).lookahead_end = (*yyextra).lookahead_hold_char;
                next_token = core_yylex(
                    &mut (*yyextra).lookahead_yylval,
                    llocp,
                    yyscanner,
                );
                if next_token != 261 as libc::c_int {
                    scanner_yyerror(
                        b"UESCAPE must be followed by a simple string literal\0"
                            as *const u8 as *const libc::c_char,
                        yyscanner,
                    );
                }
                escstr = (*yyextra).lookahead_yylval.str_0;
                if strlen(escstr) != 1 as libc::c_int as libc::c_ulong
                    || check_uescapechar(
                        *escstr.offset(0 as libc::c_int as isize) as libc::c_uchar,
                    ) == 0
                {
                    scanner_yyerror(
                        b"invalid Unicode escape character\0" as *const u8
                            as *const libc::c_char,
                        yyscanner,
                    );
                }
                *llocp = cur_yylloc;
                (*lvalp)
                    .core_yystype
                    .str_0 = str_udeescape(
                    (*lvalp).core_yystype.str_0,
                    *escstr.offset(0 as libc::c_int as isize),
                    *llocp,
                    yyscanner,
                );
                (*yyextra).have_lookahead = 0 as libc::c_int as bool_0;
            } else {
                (*lvalp)
                    .core_yystype
                    .str_0 = str_udeescape(
                    (*lvalp).core_yystype.str_0,
                    '\\' as i32 as libc::c_char,
                    *llocp,
                    yyscanner,
                );
            }
            if cur_token == 259 as libc::c_int {
                truncate_identifier(
                    (*lvalp).core_yystype.str_0,
                    strlen((*lvalp).core_yystype.str_0) as libc::c_int,
                    1 as libc::c_int as bool_0,
                );
                cur_token = 258 as libc::c_int;
            } else if cur_token == 262 as libc::c_int {
                cur_token = 261 as libc::c_int;
            }
        }
        _ => {}
    }
    return cur_token;
}
unsafe extern "C" fn hexval(mut c: libc::c_uchar) -> libc::c_uint {
    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
        return (c as libc::c_int - '0' as i32) as libc::c_uint;
    }
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'f' as i32 {
        return (c as libc::c_int - 'a' as i32 + 0xa as libc::c_int) as libc::c_uint;
    }
    if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'F' as i32 {
        return (c as libc::c_int - 'A' as i32 + 0xa as libc::c_int) as libc::c_uint;
    }
    let elevel_: libc::c_int = 21 as libc::c_int;
    let mut __error: libc::c_int = 0;
    if errstart(elevel_, 0 as *const libc::c_char) != 0 {
        errmsg_internal(
            b"invalid hexadecimal digit\0" as *const u8 as *const libc::c_char,
        );
        errfinish(
            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parser.c\0"
                as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            0 as *const libc::c_char,
        );
    }
    if elevel_ >= 21 as libc::c_int {
        abort();
    }
    return 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn check_unicode_value(mut c: pg_wchar) {
    if is_valid_unicode_codepoint(c) == 0 {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errcode(
                ('4' as i32 - '0' as i32 & 0x3f as libc::c_int)
                    + (('2' as i32 - '0' as i32 & 0x3f as libc::c_int)
                        << 6 as libc::c_int)
                    + (('6' as i32 - '0' as i32 & 0x3f as libc::c_int)
                        << 12 as libc::c_int)
                    + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int)
                        << 18 as libc::c_int)
                    + (('1' as i32 - '0' as i32 & 0x3f as libc::c_int)
                        << 24 as libc::c_int),
            );
            errmsg(
                b"invalid Unicode escape value\0" as *const u8 as *const libc::c_char,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parser.c\0"
                    as *const u8 as *const libc::c_char,
                321 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
}
unsafe extern "C" fn check_uescapechar(mut escape: libc::c_uchar) -> bool_0 {
    if isxdigit(escape as libc::c_int) != 0 || escape as libc::c_int == '+' as i32
        || escape as libc::c_int == '\'' as i32 || escape as libc::c_int == '"' as i32
        || scanner_isspace(escape as libc::c_char) as libc::c_int != 0
    {
        return 0 as libc::c_int as bool_0
    } else {
        return 1 as libc::c_int as bool_0
    };
}
unsafe extern "C" fn str_udeescape(
    mut str: *const libc::c_char,
    mut escape: libc::c_char,
    mut position: libc::c_int,
    mut yyscanner: core_yyscan_t,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut in_0: *const libc::c_char = 0 as *const libc::c_char;
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_len: size_t = 0;
    let mut pair_first: pg_wchar = 0 as libc::c_int as pg_wchar;
    let mut scbstate: ScannerCallbackState = ScannerCallbackState {
        yyscanner: 0 as *mut libc::c_void,
        location: 0,
        errcallback: ErrorContextCallback {
            previous: 0 as *mut ErrorContextCallback,
            callback: None,
            arg: 0 as *mut libc::c_void,
        },
    };
    new_len = (strlen(str))
        .wrapping_add(16 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    new = palloc(new_len) as *mut libc::c_char;
    in_0 = str;
    out = new;
    loop {
        if !(*in_0 != 0) {
            current_block = 9627623479216730126;
            break;
        }
        let mut out_dist: size_t = out.offset_from(new) as libc::c_long as size_t;
        if out_dist
            > new_len.wrapping_sub((16 as libc::c_int + 1 as libc::c_int) as size_t)
        {
            new_len = new_len * 2 as libc::c_int as size_t;
            new = repalloc(new as *mut libc::c_void, new_len) as *mut libc::c_char;
            out = new.offset(out_dist as isize);
        }
        if *in_0.offset(0 as libc::c_int as isize) as libc::c_int
            == escape as libc::c_int
        {
            setup_scanner_errposition_callback(
                &mut scbstate,
                yyscanner,
                (in_0.offset_from(str) as libc::c_long + position as libc::c_long
                    + 3 as libc::c_int as libc::c_long) as libc::c_int,
            );
            if *in_0.offset(1 as libc::c_int as isize) as libc::c_int
                == escape as libc::c_int
            {
                if pair_first != 0 {
                    current_block = 2947293050515664763;
                    break;
                }
                let fresh0 = out;
                out = out.offset(1);
                *fresh0 = escape;
                in_0 = in_0.offset(2 as libc::c_int as isize);
            } else if isxdigit(
                *in_0.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int,
            ) != 0
                && isxdigit(
                    *in_0.offset(2 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                ) != 0
                && isxdigit(
                    *in_0.offset(3 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                ) != 0
                && isxdigit(
                    *in_0.offset(4 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                ) != 0
            {
                let mut unicode: pg_wchar = 0;
                unicode = (hexval(
                    *in_0.offset(1 as libc::c_int as isize) as libc::c_uchar,
                ) << 12 as libc::c_int)
                    .wrapping_add(
                        hexval(*in_0.offset(2 as libc::c_int as isize) as libc::c_uchar)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        hexval(*in_0.offset(3 as libc::c_int as isize) as libc::c_uchar)
                            << 4 as libc::c_int,
                    )
                    .wrapping_add(
                        hexval(*in_0.offset(4 as libc::c_int as isize) as libc::c_uchar),
                    );
                check_unicode_value(unicode);
                if pair_first != 0 {
                    if !(is_utf16_surrogate_second(unicode) != 0) {
                        current_block = 2947293050515664763;
                        break;
                    }
                    unicode = surrogate_pair_to_codepoint(pair_first, unicode);
                    pair_first = 0 as libc::c_int as pg_wchar;
                } else if is_utf16_surrogate_second(unicode) != 0 {
                    current_block = 2947293050515664763;
                    break;
                }
                if is_utf16_surrogate_first(unicode) != 0 {
                    pair_first = unicode;
                } else {
                    pg_unicode_to_server(unicode, out as *mut libc::c_uchar);
                    out = out.offset(strlen(out) as isize);
                }
                in_0 = in_0.offset(5 as libc::c_int as isize);
            } else if *in_0.offset(1 as libc::c_int as isize) as libc::c_int
                == '+' as i32
                && isxdigit(
                    *in_0.offset(2 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                ) != 0
                && isxdigit(
                    *in_0.offset(3 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                ) != 0
                && isxdigit(
                    *in_0.offset(4 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                ) != 0
                && isxdigit(
                    *in_0.offset(5 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                ) != 0
                && isxdigit(
                    *in_0.offset(6 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                ) != 0
                && isxdigit(
                    *in_0.offset(7 as libc::c_int as isize) as libc::c_uchar
                        as libc::c_int,
                ) != 0
            {
                let mut unicode_0: pg_wchar = 0;
                unicode_0 = (hexval(
                    *in_0.offset(2 as libc::c_int as isize) as libc::c_uchar,
                ) << 20 as libc::c_int)
                    .wrapping_add(
                        hexval(*in_0.offset(3 as libc::c_int as isize) as libc::c_uchar)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        hexval(*in_0.offset(4 as libc::c_int as isize) as libc::c_uchar)
                            << 12 as libc::c_int,
                    )
                    .wrapping_add(
                        hexval(*in_0.offset(5 as libc::c_int as isize) as libc::c_uchar)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        hexval(*in_0.offset(6 as libc::c_int as isize) as libc::c_uchar)
                            << 4 as libc::c_int,
                    )
                    .wrapping_add(
                        hexval(*in_0.offset(7 as libc::c_int as isize) as libc::c_uchar),
                    );
                check_unicode_value(unicode_0);
                if pair_first != 0 {
                    if !(is_utf16_surrogate_second(unicode_0) != 0) {
                        current_block = 2947293050515664763;
                        break;
                    }
                    unicode_0 = surrogate_pair_to_codepoint(pair_first, unicode_0);
                    pair_first = 0 as libc::c_int as pg_wchar;
                } else if is_utf16_surrogate_second(unicode_0) != 0 {
                    current_block = 2947293050515664763;
                    break;
                }
                if is_utf16_surrogate_first(unicode_0) != 0 {
                    pair_first = unicode_0;
                } else {
                    pg_unicode_to_server(unicode_0, out as *mut libc::c_uchar);
                    out = out.offset(strlen(out) as isize);
                }
                in_0 = in_0.offset(8 as libc::c_int as isize);
            } else {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                    errcode(
                        ('4' as i32 - '0' as i32 & 0x3f as libc::c_int)
                            + (('2' as i32 - '0' as i32 & 0x3f as libc::c_int)
                                << 6 as libc::c_int)
                            + (('6' as i32 - '0' as i32 & 0x3f as libc::c_int)
                                << 12 as libc::c_int)
                            + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int)
                                << 18 as libc::c_int)
                            + (('1' as i32 - '0' as i32 & 0x3f as libc::c_int)
                                << 24 as libc::c_int),
                    );
                    errmsg(
                        b"invalid Unicode escape\0" as *const u8 as *const libc::c_char,
                    );
                    errhint(
                        b"Unicode escapes must be \\XXXX or \\+XXXXXX.\0" as *const u8
                            as *const libc::c_char,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parser.c\0"
                            as *const u8 as *const libc::c_char,
                        469 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            cancel_scanner_errposition_callback(&mut scbstate);
        } else {
            if pair_first != 0 {
                current_block = 2947293050515664763;
                break;
            }
            let fresh1 = in_0;
            in_0 = in_0.offset(1);
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 = *fresh1;
        }
    }
    match current_block {
        9627623479216730126 => {
            if !(pair_first != 0) {
                *out = '\0' as i32 as libc::c_char;
                return new;
            }
        }
        _ => {}
    }
    let elevel__0: libc::c_int = 21 as libc::c_int;
    let mut __error_0: libc::c_int = 0;
    if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
        errcode(
            ('4' as i32 - '0' as i32 & 0x3f as libc::c_int)
                + (('2' as i32 - '0' as i32 & 0x3f as libc::c_int) << 6 as libc::c_int)
                + (('6' as i32 - '0' as i32 & 0x3f as libc::c_int) << 12 as libc::c_int)
                + (('0' as i32 - '0' as i32 & 0x3f as libc::c_int) << 18 as libc::c_int)
                + (('1' as i32 - '0' as i32 & 0x3f as libc::c_int) << 24 as libc::c_int),
        );
        errmsg(b"invalid Unicode surrogate pair\0" as *const u8 as *const libc::c_char);
        scanner_errposition(
            (in_0.offset_from(str) as libc::c_long + position as libc::c_long
                + 3 as libc::c_int as libc::c_long) as libc::c_int,
            yyscanner,
        );
        errfinish(
            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parser.c\0"
                as *const u8 as *const libc::c_char,
            499 as libc::c_int,
            0 as *const libc::c_char,
        );
    }
    if elevel__0 >= 21 as libc::c_int {
        abort();
    }
    return 0 as *mut libc::c_char;
}
