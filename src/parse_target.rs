#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
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
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool_0;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn copyObjectImpl(obj: *const libc::c_void) -> *mut libc::c_void;
//     fn bms_add_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
//     fn bms_is_member(x: libc::c_int, a: *const Bitmapset) -> bool_0;
//     fn lappend_int(list: *mut List, datum: libc::c_int) -> *mut List;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
//     fn CreateTemplateTupleDesc(natts: libc::c_int) -> TupleDesc;
//     fn TupleDescInitEntry(
//         desc: TupleDesc,
//         attributeNumber: AttrNumber,
//         attributeName: *const libc::c_char,
//         oidtypeid: Oid,
//         typmod: int32,
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
//         containerTypMod: int32,
//         indirection: *mut List,
//         isAssignment: bool_0,
//     ) -> *mut SubscriptingRef;
//     fn transformContainerType(containerType: *mut Oid, containerTypmod: *mut int32);
//     fn get_database_name(dbid: Oid) -> *mut libc::c_char;
//     fn get_expr_result_tupdesc(expr: *mut Node, noError: bool_0) -> TupleDesc;
//     static mut MyDatabaseId: Oid;
//     fn makeVar(
//         varno: Index,
//         varattno: AttrNumber,
//         vartype: Oid,
//         vartypmod: int32,
//         varcollid: Oid,
//         varlevelsup: Index,
//     ) -> *mut Var;
//     fn makeTargetEntry(
//         expr: *mut Expr,
//         resno: AttrNumber,
//         resname: *mut libc::c_char,
//         resjunk: bool_0,
//     ) -> *mut TargetEntry;
//     fn makeNullConst(consttype: Oid, consttypmod: int32, constcollid: Oid) -> *mut Const;
//     fn makeRangeVar(
//         schemaname: *mut libc::c_char,
//         relname: *mut libc::c_char,
//         location: libc::c_int,
//     ) -> *mut RangeVar;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprTypmod(expr: *const Node) -> int32;
//     fn exprCollation(expr: *const Node) -> Oid;
//     fn coerce_to_target_type(
//         pstate: *mut ParseState,
//         expr: *mut Node,
//         exprtype: Oid,
//         targettype: Oid,
//         targettypmod: int32,
//         ccontext: CoercionContext,
//         cformat: CoercionForm,
//         location: libc::c_int,
//     ) -> *mut Node;
//     fn coerce_to_domain(
//         arg: *mut Node,
//         baseTypeId: Oid,
//         baseTypeMod: int32,
//         typeId: Oid,
//         ccontext: CoercionContext,
//         cformat: CoercionForm,
//         location: libc::c_int,
//         hideInputCoercion: bool_0,
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
//         include_dropped: bool_0,
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
//         sysColOK: bool_0,
//     ) -> libc::c_int;
//     fn attnumTypeId(rd: Relation, attid: libc::c_int) -> Oid;
//     fn typeidTypeRelid(type_id: Oid) -> Oid;
//     fn get_tle_by_resno(tlist: *mut List, resno: AttrNumber) -> *mut TargetEntry;
//     fn get_attnum(relid: Oid, attname: *const libc::c_char) -> AttrNumber;
//     fn get_atttypetypmodcoll(
//         relid: Oid,
//         attnum: AttrNumber,
//         typid: *mut Oid,
//         typmod: *mut int32,
//         collid: *mut Oid,
//     );
//     fn get_typcollation(typid: Oid) -> Oid;
//     fn getBaseTypeAndTypmod(typid: Oid, typmod: *mut int32) -> Oid;
// }
use super::*;
pub type Oid = libc::c_uint;
pub type __darwin_size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type bool_0 = libc::c_uchar;
pub type int16 = libc::c_short;
pub type int32 = libc::c_int;
pub type uint8 = libc::c_uchar;
pub type uint16 = libc::c_ushort;
pub type uint32 = libc::c_uint;
pub type bits8 = uint8;
pub type uint64 = libc::c_ulong;
pub type Size = size_t;
pub type Index = libc::c_uint;
pub type float4 = libc::c_float;
pub type regproc = Oid;
pub type RegProcedure = regproc;
pub type TransactionId = uint32;
pub type SubTransactionId = uint32;
pub type CommandId = uint32;
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
    pub vl_len_: int32,
    pub ndim: libc::c_int,
    pub dataoffset: int32,
    pub elemtype: Oid,
    pub dim1: libc::c_int,
    pub lbound1: libc::c_int,
    pub values: [int16; 0],
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
    pub isReset: bool_0,
    pub allowInCritSection: bool_0,
    pub mem_allocated: Size,
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
pub type MemoryContextCallbackFunction = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
pub type MemoryContext = *mut MemoryContextData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryContextMethods {
    pub alloc: Option::<unsafe extern "C" fn(MemoryContext, Size) -> *mut libc::c_void>,
    pub free_p: Option::<unsafe extern "C" fn(MemoryContext, *mut libc::c_void) -> ()>,
    pub realloc: Option::<
        unsafe extern "C" fn(MemoryContext, *mut libc::c_void, Size) -> *mut libc::c_void,
    >,
    pub reset: Option::<unsafe extern "C" fn(MemoryContext) -> ()>,
    pub delete_context: Option::<unsafe extern "C" fn(MemoryContext) -> ()>,
    pub get_chunk_space: Option::<
        unsafe extern "C" fn(MemoryContext, *mut libc::c_void) -> Size,
    >,
    pub is_empty: Option::<unsafe extern "C" fn(MemoryContext) -> bool_0>,
    pub stats: Option::<
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
    pub nblocks: Size,
    pub freechunks: Size,
    pub totalspace: Size,
    pub freespace: Size,
}
pub type MemoryStatsPrintFunc = Option::<
    unsafe extern "C" fn(MemoryContext, *mut libc::c_void, *const libc::c_char) -> (),
>;
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
pub type Datum = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NullableDatum {
    pub value: Datum,
    pub isnull: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockIdData {
    pub bi_hi: uint16,
    pub bi_lo: uint16,
}
pub type OffsetNumber = uint16;
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
    pub t_infomask2: uint16,
    pub t_infomask: uint16,
    pub t_hoff: uint8,
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
    pub datum_len_: int32,
    pub datum_typmod: int32,
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
    pub t_len: uint32,
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
    pub inh: bool_0,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Const {
    pub xpr: Expr,
    pub consttype: Oid,
    pub consttypmod: int32,
    pub constcollid: Oid,
    pub constlen: libc::c_int,
    pub constvalue: Datum,
    pub constisnull: bool_0,
    pub constbyval: bool_0,
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
pub struct SubscriptingRef {
    pub xpr: Expr,
    pub refcontainertype: Oid,
    pub refelemtype: Oid,
    pub refrestype: Oid,
    pub reftypmod: int32,
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
    pub resulttypmod: int32,
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
    pub typeMod: int32,
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
    pub typmod: int32,
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
    pub typmod: int32,
    pub location: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SetToDefault {
    pub xpr: Expr,
    pub typeId: Oid,
    pub typeMod: int32,
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
    pub agg_within_group: bool_0,
    pub agg_star: bool_0,
    pub agg_distinct: bool_0,
    pub func_variadic: bool_0,
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
    pub is_slice: bool_0,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_attribute {
    pub attrelid: Oid,
    pub attname: NameData,
    pub atttypid: Oid,
    pub attstattarget: int32,
    pub attlen: int16,
    pub attnum: int16,
    pub attndims: int32,
    pub attcacheoff: int32,
    pub atttypmod: int32,
    pub attbyval: bool_0,
    pub attstorage: libc::c_char,
    pub attalign: libc::c_char,
    pub attnotnull: bool_0,
    pub atthasdef: bool_0,
    pub atthasmissing: bool_0,
    pub attidentity: libc::c_char,
    pub attgenerated: libc::c_char,
    pub attisdropped: bool_0,
    pub attislocal: bool_0,
    pub attinhcount: int32,
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
    pub ccvalid: bool_0,
    pub ccnoinherit: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleConstr {
    pub defval: *mut AttrDefault,
    pub check: *mut ConstrCheck,
    pub missing: *mut AttrMissing,
    pub num_defval: uint16,
    pub num_check: uint16,
    pub has_not_null: bool_0,
    pub has_generated_stored: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TupleDescData {
    pub natts: libc::c_int,
    pub tdtypeid: Oid,
    pub tdtypmod: int32,
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
    pub rd_islocaltemp: bool_0,
    pub rd_isnailed: bool_0,
    pub rd_isvalid: bool_0,
    pub rd_indexvalid: bool_0,
    pub rd_statvalid: bool_0,
    pub rd_version_checked: bool_0,
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
    pub rd_fkeyvalid: bool_0,
    pub rd_partkey: PartitionKey,
    pub rd_partkeycxt: MemoryContext,
    pub rd_partdesc: PartitionDesc,
    pub rd_pdcxt: MemoryContext,
    pub rd_partcheck: *mut List,
    pub rd_partcheckvalid: bool_0,
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
    pub rd_indoption: *mut int16,
    pub rd_indexprs: *mut List,
    pub rd_indpred: *mut List,
    pub rd_exclops: *mut Oid,
    pub rd_exclprocs: *mut Oid,
    pub rd_exclstrats: *mut uint16,
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
    pub fn_strict: bool_0,
    pub fn_retset: bool_0,
    pub fn_stats: libc::c_uchar,
    pub fn_extra: *mut libc::c_void,
    pub fn_mcxt: MemoryContext,
    pub fn_expr: fmNodePtr,
}
pub type fmNodePtr = *mut Node;
pub type PGFunction = Option::<unsafe extern "C" fn(FunctionCallInfo) -> Datum>;
pub type FunctionCallInfo = *mut FunctionCallInfoBaseData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FunctionCallInfoBaseData {
    pub flinfo: *mut FmgrInfo,
    pub context: fmNodePtr,
    pub resultinfo: fmNodePtr,
    pub fncollation: Oid,
    pub isnull: bool_0,
    pub nargs: libc::c_short,
    pub args: [NullableDatum; 0],
}
pub type Form_pg_index = *mut FormData_pg_index;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormData_pg_index {
    pub indexrelid: Oid,
    pub indrelid: Oid,
    pub indnatts: int16,
    pub indnkeyatts: int16,
    pub indisunique: bool_0,
    pub indisprimary: bool_0,
    pub indisexclusion: bool_0,
    pub indimmediate: bool_0,
    pub indisclustered: bool_0,
    pub indisvalid: bool_0,
    pub indcheckxmin: bool_0,
    pub indisready: bool_0,
    pub indislive: bool_0,
    pub indisreplident: bool_0,
    pub indkey: int2vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PublicationActions {
    pub pubinsert: bool_0,
    pub pubupdate: bool_0,
    pub pubdelete: bool_0,
    pub pubtruncate: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TriggerDesc {
    pub triggers: *mut Trigger,
    pub numtriggers: libc::c_int,
    pub trig_insert_before_row: bool_0,
    pub trig_insert_after_row: bool_0,
    pub trig_insert_instead_row: bool_0,
    pub trig_insert_before_statement: bool_0,
    pub trig_insert_after_statement: bool_0,
    pub trig_update_before_row: bool_0,
    pub trig_update_after_row: bool_0,
    pub trig_update_instead_row: bool_0,
    pub trig_update_before_statement: bool_0,
    pub trig_update_after_statement: bool_0,
    pub trig_delete_before_row: bool_0,
    pub trig_delete_after_row: bool_0,
    pub trig_delete_instead_row: bool_0,
    pub trig_delete_before_statement: bool_0,
    pub trig_delete_after_statement: bool_0,
    pub trig_truncate_before_statement: bool_0,
    pub trig_truncate_after_statement: bool_0,
    pub trig_insert_new_table: bool_0,
    pub trig_update_old_table: bool_0,
    pub trig_update_new_table: bool_0,
    pub trig_delete_old_table: bool_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Trigger {
    pub tgoid: Oid,
    pub tgname: *mut libc::c_char,
    pub tgfoid: Oid,
    pub tgtype: int16,
    pub tgenabled: libc::c_char,
    pub tgisinternal: bool_0,
    pub tgisclone: bool_0,
    pub tgconstrrelid: Oid,
    pub tgconstrindid: Oid,
    pub tgconstraint: Oid,
    pub tgdeferrable: bool_0,
    pub tginitdeferred: bool_0,
    pub tgnargs: int16,
    pub tgnattr: int16,
    pub tgattr: *mut int16,
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
    pub isInstead: bool_0,
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
    pub relpages: int32,
    pub reltuples: float4,
    pub relallvisible: int32,
    pub reltoastrelid: Oid,
    pub relhasindex: bool_0,
    pub relisshared: bool_0,
    pub relpersistence: libc::c_char,
    pub relkind: libc::c_char,
    pub relnatts: int16,
    pub relchecks: int16,
    pub relhasrules: bool_0,
    pub relhastriggers: bool_0,
    pub relhassubclass: bool_0,
    pub relrowsecurity: bool_0,
    pub relforcerowsecurity: bool_0,
    pub relispopulated: bool_0,
    pub relreplident: libc::c_char,
    pub relispartition: bool_0,
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
    if c
        < &mut *((*l).elements).offset((*l).length as isize) as *mut ListCell
            as *const ListCell
    {
        return c as *mut ListCell
    } else {
        return 0 as *mut ListCell
    };
}
#[inline]
unsafe extern "C" fn list_cell_number(
    mut l: *const List,
    mut c: *const ListCell,
) -> libc::c_int {
    return c.offset_from((*l).elements) as libc::c_long as libc::c_int;
}
#[inline]
unsafe extern "C" fn list_nth(
    mut list: *const List,
    mut n: libc::c_int,
) -> *mut libc::c_void {
    return (*list_nth_cell(list, n)).ptr_value;
}
#[inline]
unsafe extern "C" fn list_last_cell(mut list: *const List) -> *mut ListCell {
    return &mut *((*list).elements).offset(((*list).length - 1 as libc::c_int) as isize)
        as *mut ListCell;
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
    mut resjunk: bool_0,
) -> *mut TargetEntry {
    if expr.is_null() {
        if exprKind as libc::c_uint
            == EXPR_KIND_UPDATE_SOURCE as libc::c_int as libc::c_uint
            && (*(node as *const Node)).type_0 as libc::c_uint
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
    let mut expand_star: bool_0 = 0;
    let mut o_target: *mut ListCell = 0 as *mut ListCell;
    expand_star = (exprKind as libc::c_uint
        != EXPR_KIND_UPDATE_SOURCE as libc::c_int as libc::c_uint) as libc::c_int
        as bool_0;
    let mut current_block_4: u64;
    let mut o_target__state: ForEachState = {
        let mut init = ForEachState {
            l: targetlist,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(o_target__state.l).is_null()
        && o_target__state.i < (*o_target__state.l).length
    {
        o_target = &mut *((*o_target__state.l).elements)
            .offset(o_target__state.i as isize) as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        o_target = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut res: *mut ResTarget = (*o_target).ptr_value as *mut ResTarget;
        if expand_star != 0 {
            if (*((*res).val as *const Node)).type_0 as libc::c_uint
                == T_ColumnRef as libc::c_int as libc::c_uint
            {
                let mut cref: *mut ColumnRef = (*res).val as *mut ColumnRef;
                if (*((*list_last_cell((*cref).fields)).ptr_value as *const Node)).type_0
                    as libc::c_uint == T_A_Star as libc::c_int as libc::c_uint
                {
                    p_target = list_concat(
                        p_target,
                        ExpandColumnRefStar(pstate, cref, 1 as libc::c_int as bool_0),
                    );
                    current_block_4 = 12517898123489920830;
                } else {
                    current_block_4 = 12800627514080957624;
                }
            } else if (*((*res).val as *const Node)).type_0 as libc::c_uint
                == T_A_Indirection as libc::c_int as libc::c_uint
            {
                let mut ind: *mut A_Indirection = (*res).val as *mut A_Indirection;
                if (*((*list_last_cell((*ind).indirection)).ptr_value as *const Node))
                    .type_0 as libc::c_uint == T_A_Star as libc::c_int as libc::c_uint
                {
                    p_target = list_concat(
                        p_target,
                        ExpandIndirectionStar(
                            pstate,
                            ind,
                            1 as libc::c_int as bool_0,
                            exprKind,
                        ),
                    );
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
                        0 as libc::c_int as bool_0,
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
    mut allowDefault: bool_0,
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
        lc = &mut *((*lc__state.l).elements).offset(lc__state.i as isize)
            as *mut ListCell;
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        lc = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut e: *mut Node = (*lc).ptr_value as *mut Node;
        if (*(e as *const Node)).type_0 as libc::c_uint
            == T_ColumnRef as libc::c_int as libc::c_uint
        {
            let mut cref: *mut ColumnRef = e as *mut ColumnRef;
            if (*((*list_last_cell((*cref).fields)).ptr_value as *const Node)).type_0
                as libc::c_uint == T_A_Star as libc::c_int as libc::c_uint
            {
                result = list_concat(
                    result,
                    ExpandColumnRefStar(pstate, cref, 0 as libc::c_int as bool_0),
                );
                current_block_5 = 16668937799742929182;
            } else {
                current_block_5 = 13109137661213826276;
            }
        } else if (*(e as *const Node)).type_0 as libc::c_uint
            == T_A_Indirection as libc::c_int as libc::c_uint
        {
            let mut ind: *mut A_Indirection = e as *mut A_Indirection;
            if (*((*list_last_cell((*ind).indirection)).ptr_value as *const Node)).type_0
                as libc::c_uint == T_A_Star as libc::c_int as libc::c_uint
            {
                result = list_concat(
                    result,
                    ExpandIndirectionStar(
                        pstate,
                        ind,
                        0 as libc::c_int as bool_0,
                        exprKind,
                    ),
                );
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
                    && (*(e as *const Node)).type_0 as libc::c_uint
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
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        l = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
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
        || !((*(var as *const Node)).type_0 as libc::c_uint
            == T_Var as libc::c_int as libc::c_uint)
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
                let mut ste: *mut TargetEntry = get_tle_by_resno(
                    (*(*rte).subquery).targetList,
                    attnum,
                );
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
                let mut cte: *mut CommonTableExpr = GetCTEForRTE(
                    pstate,
                    rte,
                    netlevelsup,
                );
                let mut ste_0: *mut TargetEntry = 0 as *mut TargetEntry;
                let mut tl: *mut List = (if (*((*cte).ctequery as *mut Query))
                    .commandType as libc::c_uint
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
                if !(extra_cols != 0 && attnum as libc::c_int > list_length(tl)
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
    let mut attrtypmod: int32 = 0;
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
        && (*(expr as *const Node)).type_0 as libc::c_uint
            == T_SetToDefault as libc::c_int as libc::c_uint
    {
        let mut def: *mut SetToDefault = expr as *mut SetToDefault;
        (*def).typeId = attrtype;
        (*def).typeMod = attrtypmod;
        (*def).collation = attrcollation;
        if !indirection.is_null() {
            if (*((*list_nth_cell(indirection, 0 as libc::c_int)).ptr_value
                as *const Node))
                .type_0 as libc::c_uint == T_A_Indices as libc::c_int as libc::c_uint
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
            0 as libc::c_int as bool_0,
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
    (*tle)
        .expr = transformAssignedExpr(
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
    mut targetIsSubscripting: bool_0,
    mut targetTypeId: Oid,
    mut targetTypMod: int32,
    mut targetCollation: Oid,
    mut indirection: *mut List,
    mut indirection_cell: *mut ListCell,
    mut rhs: *mut Node,
    mut ccontext: CoercionContext,
    mut location: libc::c_int,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut subscripts: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut isSlice: bool_0 = 0 as libc::c_int as bool_0;
    let mut i: *mut ListCell = 0 as *mut ListCell;
    if !indirection_cell.is_null() && basenode.is_null() {
        let mut ctest: *mut CaseTestExpr = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_CaseTestExpr;
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
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        i = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut n: *mut Node = (*i).ptr_value as *mut Node;
        if (*(n as *const Node)).type_0 as libc::c_uint
            == T_A_Indices as libc::c_int as libc::c_uint
        {
            subscripts = lappend(subscripts, n as *mut libc::c_void);
            if (*(n as *mut A_Indices)).is_slice != 0 {
                isSlice = 1 as libc::c_int as bool_0;
            }
        } else if (*(n as *const Node)).type_0 as libc::c_uint
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
            let mut baseTypeMod: int32 = 0;
            let mut typrelid: Oid = 0;
            let mut attnum: AttrNumber = 0;
            let mut fieldTypeId: Oid = 0;
            let mut fieldTypMod: int32 = 0;
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
                0 as libc::c_int as bool_0,
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
                (*_result).type_0 = T_FieldStore;
                _result
            }) as *mut FieldStore;
            (*fstore).arg = basenode as *mut Expr;
            (*fstore)
                .newvals = list_make1_impl(
                T_List,
                ListCell {
                    ptr_value: rhs as *mut libc::c_void,
                },
            );
            (*fstore)
                .fieldnums = list_make1_impl(
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
                    0 as libc::c_int as bool_0,
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
    mut targetTypMod: int32,
    mut targetCollation: Oid,
    mut subscripts: *mut List,
    mut isSlice: bool_0,
    mut indirection: *mut List,
    mut next_indirection: *mut ListCell,
    mut rhs: *mut Node,
    mut ccontext: CoercionContext,
    mut location: libc::c_int,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut sbsref: *mut SubscriptingRef = 0 as *mut SubscriptingRef;
    let mut containerType: Oid = 0;
    let mut containerTypMod: int32 = 0;
    let mut typeNeeded: Oid = 0;
    let mut typmodNeeded: int32 = 0;
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
        1 as libc::c_int as bool_0,
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
        1 as libc::c_int as bool_0,
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
        let mut numcol: libc::c_int = (*(*(*pstate).p_target_relation).rd_rel).relnatts
            as libc::c_int;
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
                    (*_result).type_0 = T_ResTarget;
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
            tl = &mut *((*tl__state.l).elements).offset(tl__state.i as isize)
                as *mut ListCell;
            1 as libc::c_int as bool_0 as libc::c_int
        } else {
            tl = 0 as *mut ListCell;
            0 as libc::c_int as bool_0 as libc::c_int
        } != 0
        {
            let mut col_0: *mut ResTarget = (*tl).ptr_value as *mut ResTarget;
            let mut name: *mut libc::c_char = (*col_0).name;
            let mut attrno: libc::c_int = 0;
            attrno = attnameAttNum(
                (*pstate).p_target_relation,
                name,
                0 as libc::c_int as bool_0,
            );
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
                if bms_is_member(attrno, wholecols) != 0 {
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
    mut make_target_entry: bool_0,
) -> *mut List {
    let mut fields: *mut List = (*cref).fields;
    let mut numnames: libc::c_int = list_length(fields);
    if numnames == 1 as libc::c_int {
        return ExpandAllTables(pstate, (*cref).location)
    } else {
        let mut nspname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut relname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
        let mut levels_up: libc::c_int = 0;
        let mut crserr: C2RustUnnamed_1 = CRSERR_NO_RTE;
        if ((*pstate).p_pre_columnref_hook).is_some() {
            let mut node: *mut Node = 0 as *mut Node;
            node = ((*pstate).p_pre_columnref_hook)
                .expect("non-null function pointer")(pstate, cref);
            if !node.is_null() {
                return ExpandRowReference(pstate, node, make_target_entry);
            }
        }
        match numnames {
            2 => {
                relname = (*((*list_nth_cell(fields, 0 as libc::c_int)).ptr_value
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
            3 => {
                nspname = (*((*list_nth_cell(fields, 0 as libc::c_int)).ptr_value
                    as *mut Value))
                    .val
                    .str_0;
                relname = (*((*list_nth_cell(fields, 1 as libc::c_int)).ptr_value
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
            4 => {
                let mut catname: *mut libc::c_char = (*((*list_nth_cell(
                    fields,
                    0 as libc::c_int,
                ))
                    .ptr_value as *mut Value))
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
            node_0 = ((*pstate).p_post_columnref_hook)
                .expect(
                    "non-null function pointer",
                )(
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
                    errorMissingRTE(
                        pstate,
                        makeRangeVar(nspname, relname, (*cref).location),
                    );
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
    let mut found_table: bool_0 = 0 as libc::c_int as bool_0;
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
        1 as libc::c_int as bool_0 as libc::c_int
    } else {
        l = 0 as *mut ListCell;
        0 as libc::c_int as bool_0 as libc::c_int
    } != 0
    {
        let mut nsitem: *mut ParseNamespaceItem = (*l).ptr_value
            as *mut ParseNamespaceItem;
        if !((*nsitem).p_cols_visible == 0) {
            found_table = 1 as libc::c_int as bool_0;
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
    mut make_target_entry: bool_0,
    mut exprKind: ParseExprKind,
) -> *mut List {
    let mut expr: *mut Node = 0 as *mut Node;
    ind = copyObjectImpl(ind as *const libc::c_void) as *mut A_Indirection;
    (*ind)
        .indirection = list_truncate(
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
    mut make_target_entry: bool_0,
) -> *mut List {
    if make_target_entry != 0 {
        return expandNSItemAttrs(pstate, nsitem, sublevels_up, location)
    } else {
        let mut rte: *mut RangeTblEntry = (*nsitem).p_rte;
        let mut vars: *mut List = 0 as *mut List;
        let mut l: *mut ListCell = 0 as *mut ListCell;
        vars = expandNSItemVars(nsitem, sublevels_up, location, 0 as *mut *mut List);
        if (*rte).rtekind as libc::c_uint == RTE_RELATION as libc::c_int as libc::c_uint
        {
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
            l = &mut *((*l__state.l).elements).offset(l__state.i as isize)
                as *mut ListCell;
            1 as libc::c_int as bool_0 as libc::c_int
        } else {
            l = 0 as *mut ListCell;
            0 as libc::c_int as bool_0 as libc::c_int
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
    match (*(node as *const Node)).type_0 as libc::c_uint {
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
                l = &mut *((*l__state.l).elements).offset(l__state.i as isize)
                    as *mut ListCell;
                1 as libc::c_int as bool_0 as libc::c_int
            } else {
                l = 0 as *mut ListCell;
                0 as libc::c_int as bool_0 as libc::c_int
            } != 0
            {
                let mut i: *mut Node = (*l).ptr_value as *mut Node;
                if (*(i as *const Node)).type_0 as libc::c_uint
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
                l_0 = &mut *((*l__state_0.l).elements).offset(l__state_0.i as isize)
                    as *mut ListCell;
                1 as libc::c_int as bool_0 as libc::c_int
            } else {
                l_0 = 0 as *mut ListCell;
                0 as libc::c_int as bool_0 as libc::c_int
            } != 0
            {
                let mut i_0: *mut Node = (*l_0).ptr_value as *mut Node;
                if (*(i_0 as *const Node)).type_0 as libc::c_uint
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
                *name = b"nullif\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                return 2 as libc::c_int;
            }
        }
        359 => {
            strength = FigureColnameInternal((*(node as *mut TypeCast)).arg, name);
            if strength <= 1 as libc::c_int {
                if !((*(node as *mut TypeCast)).typeName).is_null() {
                    *name = (*((*list_last_cell(
                        (*(*(node as *mut TypeCast)).typeName).names,
                    ))
                        .ptr_value as *mut Value))
                        .val
                        .str_0;
                    return 1 as libc::c_int;
                }
            }
        }
        360 => return FigureColnameInternal((*(node as *mut CollateClause)).arg, name),
        112 => {
            *name = b"grouping\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            return 2 as libc::c_int;
        }
        122 => {
            match (*(node as *mut SubLink)).subLinkType as libc::c_uint {
                0 => {
                    *name = b"exists\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                6 => {
                    *name = b"array\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                4 => {
                    let mut sublink: *mut SubLink = node as *mut SubLink;
                    let mut query: *mut Query = (*sublink).subselect as *mut Query;
                    if (*(query as *const Node)).type_0 as libc::c_uint
                        == T_Query as libc::c_int as libc::c_uint
                    {
                        let mut te: *mut TargetEntry = (*list_nth_cell(
                            (*query).targetList,
                            0 as libc::c_int,
                        ))
                            .ptr_value as *mut TargetEntry;
                        if !((*te).resname).is_null() {
                            *name = (*te).resname;
                            return 2 as libc::c_int;
                        }
                    }
                }
                5 | 1 | 2 | 3 | 7 | _ => {}
            }
        }
        132 => {
            strength = FigureColnameInternal(
                (*(node as *mut CaseExpr)).defresult as *mut Node,
                name,
            );
            if strength <= 1 as libc::c_int {
                *name = b"case\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
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
            *name = b"coalesce\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            return 2 as libc::c_int;
        }
        139 => {
            match (*(node as *mut MinMaxExpr)).op as libc::c_uint {
                0 => {
                    *name = b"greatest\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                1 => {
                    *name = b"least\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                _ => {}
            }
        }
        140 => {
            match (*(node as *mut SQLValueFunction)).op as libc::c_uint {
                0 => {
                    *name = b"current_date\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                1 | 2 => {
                    *name = b"current_time\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                3 | 4 => {
                    *name = b"current_timestamp\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                5 | 6 => {
                    *name = b"localtime\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                7 | 8 => {
                    *name = b"localtimestamp\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                9 => {
                    *name = b"current_role\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                10 => {
                    *name = b"current_user\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                11 => {
                    *name = b"user\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                12 => {
                    *name = b"session_user\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                13 => {
                    *name = b"current_catalog\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                14 => {
                    *name = b"current_schema\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                _ => {}
            }
        }
        141 => {
            match (*(node as *mut XmlExpr)).op as libc::c_uint {
                0 => {
                    *name = b"xmlconcat\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                1 => {
                    *name = b"xmlelement\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                2 => {
                    *name = b"xmlforest\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                3 => {
                    *name = b"xmlparse\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                4 => {
                    *name = b"xmlpi\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                5 => {
                    *name = b"xmlroot\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                6 => {
                    *name = b"xmlserialize\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    return 2 as libc::c_int;
                }
                7 | _ => {}
            }
        }
        387 => {
            *name = b"xmlserialize\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            return 2 as libc::c_int;
        }
        _ => {}
    }
    return strength;
}
