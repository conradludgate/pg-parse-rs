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
//     pub type PgStat_TableStatus;
//     pub type FdwRoutine;
//     pub type IndexAmRoutine;
//     pub type TableAmRoutine;
//     pub type PartitionDescData;
//     pub type PartitionKeyData;
//     pub type RowSecurityDesc;
//     pub type SMgrRelationData;
//     pub type QueryEnvironment;
//     fn abort() -> !;
//     fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
//     fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
//     fn palloc(size: usize) -> *mut libc::c_void;
//     fn palloc0(size: usize) -> *mut libc::c_void;
//     fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
//     fn copyObjectImpl(obj: *const libc::c_void) -> *mut libc::c_void;
//     fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
//     fn lappend_int(list: *mut List, datum: libc::c_int) -> *mut List;
//     fn lappend_oid(list: *mut List, datum: Oid) -> *mut List;
//     fn list_concat(list1: *mut List, list2: *const List) -> *mut List;
//     fn list_truncate(list: *mut List, new_size: libc::c_int) -> *mut List;
//     fn list_copy(list: *const List) -> *mut List;
//     fn list_copy_tail(list: *const List, nskip: libc::c_int) -> *mut List;
//     fn CreateTemplateTupleDesc(natts: libc::c_int) -> TupleDesc;
//     fn TupleDescCopyEntry(
//         dst: TupleDesc,
//         dstAttno: AttrNumber,
//         src: TupleDesc,
//         srcAttno: AttrNumber,
//     );
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
//     fn relation_close(relation: Relation, lockmode: LOCKMODE);
//     fn relation_open(relationId: Oid, lockmode: LOCKMODE) -> Relation;
//     fn bms_add_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
//     fn table_open(relationId: Oid, lockmode: LOCKMODE) -> Relation;
//     fn table_openrv_extended(
//         relation: *const RangeVar,
//         lockmode: LOCKMODE,
//         missing_ok: bool,
//     ) -> Relation;
//     fn table_close(relation: Relation, lockmode: LOCKMODE);
//     fn ENRMetadataGetTupDesc(enrmd: EphemeralNamedRelationMetadata) -> TupleDesc;
//     fn makeString(str: *mut libc::c_char) -> *mut Value;
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
//     fn RangeVarGetRelidExtended(
//         relation: *const RangeVar,
//         lockmode: LOCKMODE,
//         flags: u32,
//         callback: RangeVarGetRelidCallback,
//         callback_arg: *mut libc::c_void,
//     ) -> Oid;
//     fn LookupNamespaceNoError(nspname: *const libc::c_char) -> Oid;
//     fn get_func_result_name(functionId: Oid) -> *mut libc::c_char;
//     fn get_expr_result_tupdesc(expr: *mut Node, noError: bool) -> TupleDesc;
//     fn get_expr_result_type(
//         expr: *mut Node,
//         resultTypeId: *mut Oid,
//         resultTupleDesc: *mut TupleDesc,
//     ) -> TypeFuncClass;
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
//     fn makeAlias(aliasname: *const libc::c_char, colnames: *mut List) -> *mut Alias;
//     fn exprType(expr: *const Node) -> Oid;
//     fn exprTypmod(expr: *const Node) -> i32;
//     fn exprCollation(expr: *const Node) -> Oid;
//     fn expression_tree_walker(
//         node: *mut Node,
//         walker: Option::<unsafe extern "C" fn() -> bool>,
//         context: *mut libc::c_void,
//     ) -> bool;
//     fn query_tree_walker(
//         query: *mut Query,
//         walker: Option::<unsafe extern "C" fn() -> bool>,
//         context: *mut libc::c_void,
//         flags: libc::c_int,
//     ) -> bool;
//     fn name_matches_visible_ENR(
//         pstate: *mut ParseState,
//         refname: *const libc::c_char,
//     ) -> bool;
//     fn get_visible_ENR(
//         pstate: *mut ParseState,
//         refname: *const libc::c_char,
//     ) -> EphemeralNamedRelationMetadata;
//     fn typenameTypeIdAndMod(
//         pstate: *mut ParseState,
//         typeName: *const TypeName,
//         typeid_p: *mut Oid,
//         typmod_p: *mut i32,
//     );
//     fn GetColumnDefCollation(
//         pstate: *mut ParseState,
//         coldef: *mut ColumnDef,
//         typeOid: Oid,
//     ) -> Oid;
//     fn namestrcmp(name: Name, str: *const libc::c_char) -> libc::c_int;
//     fn get_attname(
//         relid: Oid,
//         attnum: AttrNumber,
//         missing_ok: bool,
//     ) -> *mut libc::c_char;
//     fn get_relname_relid(relname: *const libc::c_char, relnamespace: Oid) -> Oid;
//     fn SearchSysCache2(cacheId: libc::c_int, key1: Datum, key2: Datum) -> HeapTuple;
//     fn ReleaseSysCache(tuple: HeapTuple);
//     fn SearchSysCacheExists(
//         cacheId: libc::c_int,
//         key1: Datum,
//         key2: Datum,
//         key3: Datum,
//         key4: Datum,
//     ) -> bool;
//     fn varstr_levenshtein_less_equal(
//         source: *const libc::c_char,
//         slen: libc::c_int,
//         target: *const libc::c_char,
//         tlen: libc::c_int,
//         ins_c: libc::c_int,
//         del_c: libc::c_int,
//         sub_c: libc::c_int,
//         max_d: libc::c_int,
//         trusted: bool,
//     ) -> libc::c_int;
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
pub type Name = *mut NameData;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ErrorContextCallback {
    pub previous: *mut ErrorContextCallback,
    pub callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
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
pub type Form_pg_attribute = *mut FormData_pg_attribute;
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
pub type LOCKMODE = libc::c_int;

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
pub type PartitionDesc = *mut PartitionDescData;
pub type PartitionKey = *mut PartitionKeyData;
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

pub type CoerceParamHook =
    Option<unsafe extern "C" fn(*mut ParseState, *mut Param, Oid, i32, libc::c_int) -> *mut Node>;
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
pub type EphemeralNameRelationType = libc::c_uint;
pub const ENR_NAMED_TUPLESTORE: EphemeralNameRelationType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EphemeralNamedRelationMetadataData {
    pub name: *mut libc::c_char,
    pub reliddesc: Oid,
    pub tupdesc: TupleDesc,
    pub enrtype: EphemeralNameRelationType,
    pub enrtuples: libc::c_double,
}
pub type EphemeralNamedRelationMetadata = *mut EphemeralNamedRelationMetadataData;
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
pub struct CollateClause {
    pub type_0: NodeTag,
    pub arg: *mut Node,
    pub collname: *mut List,
    pub location: libc::c_int,
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
pub struct LockingClause {
    pub type_0: NodeTag,
    pub lockedRels: *mut List,
    pub strength: LockClauseStrength,
    pub waitPolicy: LockWaitPolicy,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RangeTblFunction {
    pub type_0: NodeTag,
    pub funcexpr: *mut Node,
    pub funccolcount: libc::c_int,
    pub funccolnames: *mut List,
    pub funccoltypes: *mut List,
    pub funccoltypmods: *mut List,
    pub funccolcollations: *mut List,
    pub funcparams: *mut Bitmapset,
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
pub struct ParseCallbackState {
    pub pstate: *mut ParseState,
    pub location: libc::c_int,
    pub errcallback: ErrorContextCallback,
}
pub type RVROption = libc::c_uint;
pub const RVR_SKIP_LOCKED: RVROption = 4;
pub const RVR_NOWAIT: RVROption = 2;
pub const RVR_MISSING_OK: RVROption = 1;
pub type RangeVarGetRelidCallback =
    Option<unsafe extern "C" fn(*const RangeVar, Oid, Oid, *mut libc::c_void) -> ()>;
pub type TypeFuncClass = libc::c_uint;
pub const TYPEFUNC_OTHER: TypeFuncClass = 4;
pub const TYPEFUNC_RECORD: TypeFuncClass = 3;
pub const TYPEFUNC_COMPOSITE_DOMAIN: TypeFuncClass = 2;
pub const TYPEFUNC_COMPOSITE: TypeFuncClass = 1;
pub const TYPEFUNC_SCALAR: TypeFuncClass = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuzzyAttrMatchState {
    pub distance: libc::c_int,
    pub rfirst: *mut RangeTblEntry,
    pub first: AttrNumber,
    pub rsecond: *mut RangeTblEntry,
    pub second: AttrNumber,
}
pub const ATTNUM: SysCacheIdentifier = 7;
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
pub const ATTNAME: SysCacheIdentifier = 6;
pub const AMPROCNUM: SysCacheIdentifier = 5;
pub const AMOPSTRATEGY: SysCacheIdentifier = 4;
pub const AMOPOPID: SysCacheIdentifier = 3;
pub const AMOID: SysCacheIdentifier = 2;
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
unsafe extern "C" fn list_last_cell(mut list: *const List) -> *mut ListCell {
    return &mut *((*list).elements).offset(((*list).length - 1 as libc::c_int) as isize)
        as *mut ListCell;
}
#[inline]
unsafe extern "C" fn list_nth(mut list: *const List, mut n: libc::c_int) -> *mut libc::c_void {
    return (*list_nth_cell(list, n)).ptr_value;
}
#[inline]
unsafe extern "C" fn list_nth_oid(mut list: *const List, mut n: libc::c_int) -> Oid {
    return (*list_nth_cell(list, n)).oid_value;
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
pub unsafe extern "C" fn refnameNamespaceItem(
    mut pstate: *mut ParseState,
    mut schemaname: *const libc::c_char,
    mut refname: *const libc::c_char,
    mut location: libc::c_int,
    mut sublevels_up: *mut libc::c_int,
) -> *mut ParseNamespaceItem {
    let mut relId: Oid = 0 as libc::c_int as Oid;
    if !sublevels_up.is_null() {
        *sublevels_up = 0 as libc::c_int;
    }
    if !schemaname.is_null() {
        let mut namespaceId: Oid = 0;
        namespaceId = LookupNamespaceNoError(schemaname);
        if (namespaceId != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
            return 0 as *mut ParseNamespaceItem;
        }
        relId = get_relname_relid(refname, namespaceId);
        if (relId != 0 as libc::c_int as Oid) as libc::c_int as bool == 0 {
            return 0 as *mut ParseNamespaceItem;
        }
    }
    while !pstate.is_null() {
        let mut result: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
        if (relId != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
            result = scanNameSpaceForRelid(pstate, relId, location);
        } else {
            result = scanNameSpaceForRefname(pstate, refname, location);
        }
        if !result.is_null() {
            return result;
        }
        if sublevels_up.is_null() {
            break;
        }
        *sublevels_up += 1;
        *sublevels_up;
        pstate = (*pstate).parentParseState;
    }
    return 0 as *mut ParseNamespaceItem;
}
unsafe extern "C" fn scanNameSpaceForRefname(
    mut pstate: *mut ParseState,
    mut refname: *const libc::c_char,
    mut location: libc::c_int,
) -> *mut ParseNamespaceItem {
    let mut result: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
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
        let mut rte: *mut RangeTblEntry = (*nsitem).p_rte;
        if !((*nsitem).p_rel_visible == 0) {
            if !((*nsitem).p_lateral_only as libc::c_int != 0 && (*pstate).p_lateral_active == 0) {
                if strcmp((*(*rte).eref).aliasname, refname) == 0 as libc::c_int {
                    if !result.is_null() {
                        let elevel_: libc::c_int = 21 as libc::c_int;
                        let mut __error: libc::c_int = 0;
                        if elevel_ >= 21 as libc::c_int {
                            abort();
                        }
                    }
                    check_lateral_ref_ok(pstate, nsitem, location);
                    result = nsitem;
                }
            }
        }
        l__state.i += 1;
        l__state.i;
    }
    return result;
}
unsafe extern "C" fn scanNameSpaceForRelid(
    mut pstate: *mut ParseState,
    mut relid: Oid,
    mut location: libc::c_int,
) -> *mut ParseNamespaceItem {
    let mut result: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
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
        let mut rte: *mut RangeTblEntry = (*nsitem).p_rte;
        if !((*nsitem).p_rel_visible == 0) {
            if !((*nsitem).p_lateral_only as libc::c_int != 0 && (*pstate).p_lateral_active == 0) {
                if (*rte).rtekind as libc::c_uint == RTE_RELATION as libc::c_int as libc::c_uint
                    && (*rte).relid == relid
                    && ((*rte).alias).is_null()
                {
                    if !result.is_null() {
                        let elevel_: libc::c_int = 21 as libc::c_int;
                        let mut __error: libc::c_int = 0;
                        if elevel_ >= 21 as libc::c_int {
                            abort();
                        }
                    }
                    check_lateral_ref_ok(pstate, nsitem, location);
                    result = nsitem;
                }
            }
        }
        l__state.i += 1;
        l__state.i;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn scanNameSpaceForCTE(
    mut pstate: *mut ParseState,
    mut refname: *const libc::c_char,
    mut ctelevelsup: *mut Index,
) -> *mut CommonTableExpr {
    let mut levelsup: Index = 0;
    levelsup = 0 as libc::c_int as Index;
    while !pstate.is_null() {
        let mut lc: *mut ListCell = 0 as *mut ListCell;
        let mut lc__state: ForEachState = {
            let mut init = ForEachState {
                l: (*pstate).p_ctenamespace,
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
            let mut cte: *mut CommonTableExpr = (*lc).ptr_value as *mut CommonTableExpr;
            if strcmp((*cte).ctename, refname) == 0 as libc::c_int {
                *ctelevelsup = levelsup;
                return cte;
            }
            lc__state.i += 1;
            lc__state.i;
        }
        pstate = (*pstate).parentParseState;
        levelsup = levelsup.wrapping_add(1);
        levelsup;
    }
    return 0 as *mut CommonTableExpr;
}
unsafe extern "C" fn isFutureCTE(
    mut pstate: *mut ParseState,
    mut refname: *const libc::c_char,
) -> bool {
    while !pstate.is_null() {
        let mut lc: *mut ListCell = 0 as *mut ListCell;
        let mut lc__state: ForEachState = {
            let mut init = ForEachState {
                l: (*pstate).p_future_ctes,
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
            let mut cte: *mut CommonTableExpr = (*lc).ptr_value as *mut CommonTableExpr;
            if strcmp((*cte).ctename, refname) == 0 as libc::c_int {
                return true;
            }
            lc__state.i += 1;
            lc__state.i;
        }
        pstate = (*pstate).parentParseState;
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn scanNameSpaceForENR(
    mut pstate: *mut ParseState,
    mut refname: *const libc::c_char,
) -> bool {
    return name_matches_visible_ENR(pstate, refname);
}
unsafe extern "C" fn searchRangeTableForRel(
    mut pstate: *mut ParseState,
    mut relation: *mut RangeVar,
) -> *mut RangeTblEntry {
    let mut refname: *const libc::c_char = (*relation).relname;
    let mut relId: Oid = 0 as libc::c_int as Oid;
    let mut cte: *mut CommonTableExpr = 0 as *mut CommonTableExpr;
    let mut isenr: bool = false;
    let mut ctelevelsup: Index = 0 as libc::c_int as Index;
    let mut levelsup: Index = 0;
    if ((*relation).schemaname).is_null() {
        cte = scanNameSpaceForCTE(pstate, refname, &mut ctelevelsup);
        if cte.is_null() {
            isenr = scanNameSpaceForENR(pstate, refname);
        }
    }
    if cte.is_null() && isenr == 0 {
        relId = RangeVarGetRelidExtended(
            relation,
            0 as libc::c_int,
            (if true as libc::c_int != 0 {
                RVR_MISSING_OK as libc::c_int
            } else {
                0 as libc::c_int
            }) as u32,
            None,
            0 as *mut libc::c_void,
        );
    }
    levelsup = 0 as libc::c_int as Index;
    while !pstate.is_null() {
        let mut l: *mut ListCell = 0 as *mut ListCell;
        let mut l__state: ForEachState = {
            let mut init = ForEachState {
                l: (*pstate).p_rtable,
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
            let mut rte: *mut RangeTblEntry = (*l).ptr_value as *mut RangeTblEntry;
            if (*rte).rtekind as libc::c_uint == RTE_RELATION as libc::c_int as libc::c_uint
                && (relId != 0 as libc::c_int as Oid) as libc::c_int as bool as libc::c_int != 0
                && (*rte).relid == relId
            {
                return rte;
            }
            if (*rte).rtekind as libc::c_uint == RTE_CTE as libc::c_int as libc::c_uint
                && !cte.is_null()
                && ((*rte).ctelevelsup).wrapping_add(levelsup) == ctelevelsup
                && strcmp((*rte).ctename, refname) == 0 as libc::c_int
            {
                return rte;
            }
            if (*rte).rtekind as libc::c_uint == RTE_NAMEDTUPLESTORE as libc::c_int as libc::c_uint
                && isenr as libc::c_int != 0
                && strcmp((*rte).enrname, refname) == 0 as libc::c_int
            {
                return rte;
            }
            if strcmp((*(*rte).eref).aliasname, refname) == 0 as libc::c_int {
                return rte;
            }
            l__state.i += 1;
            l__state.i;
        }
        pstate = (*pstate).parentParseState;
        levelsup = levelsup.wrapping_add(1);
        levelsup;
    }
    return 0 as *mut RangeTblEntry;
}
#[no_mangle]
pub unsafe extern "C" fn checkNameSpaceConflicts(
    mut pstate: *mut ParseState,
    mut namespace1: *mut List,
    mut namespace2: *mut List,
) {
    let mut l1: *mut ListCell = 0 as *mut ListCell;
    let mut l1__state: ForEachState = {
        let mut init = ForEachState {
            l: namespace1,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(l1__state.l).is_null() && l1__state.i < (*l1__state.l).length {
        l1 = &mut *((*l1__state.l).elements).offset(l1__state.i as isize) as *mut ListCell;
        true as libc::c_int
    } else {
        l1 = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut nsitem1: *mut ParseNamespaceItem = (*l1).ptr_value as *mut ParseNamespaceItem;
        let mut rte1: *mut RangeTblEntry = (*nsitem1).p_rte;
        let mut aliasname1: *const libc::c_char = (*(*rte1).eref).aliasname;
        let mut l2: *mut ListCell = 0 as *mut ListCell;
        if !((*nsitem1).p_rel_visible == 0) {
            let mut l2__state: ForEachState = {
                let mut init = ForEachState {
                    l: namespace2,
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
                let mut nsitem2: *mut ParseNamespaceItem =
                    (*l2).ptr_value as *mut ParseNamespaceItem;
                let mut rte2: *mut RangeTblEntry = (*nsitem2).p_rte;
                if !((*nsitem2).p_rel_visible == 0) {
                    if !(strcmp((*(*rte2).eref).aliasname, aliasname1) != 0 as libc::c_int) {
                        if !((*rte1).rtekind as libc::c_uint
                            == RTE_RELATION as libc::c_int as libc::c_uint
                            && ((*rte1).alias).is_null()
                            && (*rte2).rtekind as libc::c_uint
                                == RTE_RELATION as libc::c_int as libc::c_uint
                            && ((*rte2).alias).is_null()
                            && (*rte1).relid != (*rte2).relid)
                        {
                            let elevel_: libc::c_int = 21 as libc::c_int;
                            let mut __error: libc::c_int = 0;
                            if elevel_ >= 21 as libc::c_int {
                                abort();
                            }
                        }
                    }
                }
                l2__state.i += 1;
                l2__state.i;
            }
        }
        l1__state.i += 1;
        l1__state.i;
    }
}
unsafe extern "C" fn check_lateral_ref_ok(
    mut pstate: *mut ParseState,
    mut nsitem: *mut ParseNamespaceItem,
    mut location: libc::c_int,
) {
    if (*nsitem).p_lateral_only as libc::c_int != 0 && (*nsitem).p_lateral_ok == 0 {
        let mut rte: *mut RangeTblEntry = (*nsitem).p_rte;
        let mut refname: *mut libc::c_char = (*(*rte).eref).aliasname;
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetNSItemByRangeTablePosn(
    mut pstate: *mut ParseState,
    mut varno: libc::c_int,
    mut sublevels_up: libc::c_int,
) -> *mut ParseNamespaceItem {
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    loop {
        let fresh0 = sublevels_up;
        sublevels_up = sublevels_up - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        pstate = (*pstate).parentParseState;
    }
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*pstate).p_namespace,
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
        if (*nsitem).p_rtindex == varno {
            return nsitem;
        }
        lc__state.i += 1;
        lc__state.i;
    }
    let elevel_: libc::c_int = 21 as libc::c_int;
    let mut __error: libc::c_int = 0;
    if errstart(elevel_, 0 as *const libc::c_char) != 0 {
        errmsg_internal(b"nsitem not found (internal error)\0" as *const u8 as *const libc::c_char);
        errfinish(
            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                as *const u8 as *const libc::c_char,
            507 as libc::c_int,
            0 as *const libc::c_char,
        );
    }
    if elevel_ >= 21 as libc::c_int {
        abort();
    }
    return 0 as *mut ParseNamespaceItem;
}
#[no_mangle]
pub unsafe extern "C" fn GetRTEByRangeTablePosn(
    mut pstate: *mut ParseState,
    mut varno: libc::c_int,
    mut sublevels_up: libc::c_int,
) -> *mut RangeTblEntry {
    loop {
        let fresh1 = sublevels_up;
        sublevels_up = sublevels_up - 1;
        if !(fresh1 > 0 as libc::c_int) {
            break;
        }
        pstate = (*pstate).parentParseState;
    }
    return list_nth((*pstate).p_rtable, varno - 1 as libc::c_int) as *mut RangeTblEntry;
}
#[no_mangle]
pub unsafe extern "C" fn GetCTEForRTE(
    mut pstate: *mut ParseState,
    mut rte: *mut RangeTblEntry,
    mut rtelevelsup: libc::c_int,
) -> *mut CommonTableExpr {
    let mut levelsup: Index = 0;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    levelsup = ((*rte).ctelevelsup).wrapping_add(rtelevelsup as Index);
    loop {
        let fresh2 = levelsup;
        levelsup = levelsup.wrapping_sub(1);
        if !(fresh2 > 0 as libc::c_int as Index) {
            break;
        }
        pstate = (*pstate).parentParseState;
        if pstate.is_null() {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"bad levelsup for CTE \"%s\"\0" as *const u8 as *const libc::c_char,
                    (*rte).ctename,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                        as *const u8 as *const libc::c_char,
                    547 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
    }
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*pstate).p_ctenamespace,
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
        let mut cte: *mut CommonTableExpr = (*lc).ptr_value as *mut CommonTableExpr;
        if strcmp((*cte).ctename, (*rte).ctename) == 0 as libc::c_int {
            return cte;
        }
        lc__state.i += 1;
        lc__state.i;
    }
    let elevel__0: libc::c_int = 21 as libc::c_int;
    let mut __error_0: libc::c_int = 0;
    if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
        errmsg_internal(
            b"could not find CTE \"%s\"\0" as *const u8 as *const libc::c_char,
            (*rte).ctename,
        );
        errfinish(
            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                as *const u8 as *const libc::c_char,
            557 as libc::c_int,
            0 as *const libc::c_char,
        );
    }
    if elevel__0 >= 21 as libc::c_int {
        abort();
    }
    return 0 as *mut CommonTableExpr;
}
unsafe extern "C" fn updateFuzzyAttrMatchState(
    mut fuzzy_rte_penalty: libc::c_int,
    mut fuzzystate: *mut FuzzyAttrMatchState,
    mut rte: *mut RangeTblEntry,
    mut actual: *const libc::c_char,
    mut match_0: *const libc::c_char,
    mut attnum: libc::c_int,
) {
    let mut columndistance: libc::c_int = 0;
    let mut matchlen: libc::c_int = 0;
    if fuzzy_rte_penalty > (*fuzzystate).distance {
        return;
    }
    if *actual.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return;
    }
    matchlen = strlen(match_0) as libc::c_int;
    columndistance = varstr_levenshtein_less_equal(
        actual,
        strlen(actual) as libc::c_int,
        match_0,
        matchlen,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        (*fuzzystate).distance + 1 as libc::c_int - fuzzy_rte_penalty,
        true,
    );
    if columndistance > matchlen / 2 as libc::c_int {
        return;
    }
    columndistance += fuzzy_rte_penalty;
    if columndistance < (*fuzzystate).distance {
        (*fuzzystate).distance = columndistance;
        (*fuzzystate).rfirst = rte;
        (*fuzzystate).first = attnum as AttrNumber;
        (*fuzzystate).rsecond = 0 as *mut RangeTblEntry;
        (*fuzzystate).second = 0 as libc::c_int as AttrNumber;
    } else if columndistance == (*fuzzystate).distance {
        if ((*fuzzystate).second as libc::c_int != 0 as libc::c_int) as libc::c_int as bool != 0 {
            (*fuzzystate).rfirst = 0 as *mut RangeTblEntry;
            (*fuzzystate).first = 0 as libc::c_int as AttrNumber;
            (*fuzzystate).rsecond = 0 as *mut RangeTblEntry;
            (*fuzzystate).second = 0 as libc::c_int as AttrNumber;
            (*fuzzystate).distance = columndistance - 1 as libc::c_int;
        } else if ((*fuzzystate).first as libc::c_int != 0 as libc::c_int) as libc::c_int as bool
            != 0
        {
            (*fuzzystate).rsecond = rte;
            (*fuzzystate).second = attnum as AttrNumber;
        } else if (*fuzzystate).distance <= 3 as libc::c_int {
            (*fuzzystate).rfirst = rte;
            (*fuzzystate).first = attnum as AttrNumber;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn scanNSItemForColumn(
    mut pstate: *mut ParseState,
    mut nsitem: *mut ParseNamespaceItem,
    mut sublevels_up: libc::c_int,
    mut colname: *const libc::c_char,
    mut location: libc::c_int,
) -> *mut Node {
    let mut rte: *mut RangeTblEntry = (*nsitem).p_rte;
    let mut attnum: libc::c_int = 0;
    let mut var: *mut Var = 0 as *mut Var;
    attnum = scanRTEForColumn(
        pstate,
        rte,
        colname,
        location,
        0 as libc::c_int,
        0 as *mut FuzzyAttrMatchState,
    );
    if attnum == 0 as libc::c_int {
        return 0 as *mut Node;
    }
    if (*pstate).p_expr_kind as libc::c_uint
        == EXPR_KIND_CHECK_CONSTRAINT as libc::c_int as libc::c_uint
        && attnum < 0 as libc::c_int
        && attnum != -(6 as libc::c_int)
    {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    if (*pstate).p_expr_kind as libc::c_uint
        == EXPR_KIND_GENERATED_COLUMN as libc::c_int as libc::c_uint
        && attnum < 0 as libc::c_int
        && attnum != -(6 as libc::c_int)
    {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    if attnum > 0 as libc::c_int {
        let mut nscol: *mut ParseNamespaceColumn = &mut *((*nsitem).p_nscolumns)
            .offset((attnum - 1 as libc::c_int) as isize)
            as *mut ParseNamespaceColumn;
        if (*nscol).p_varno == 0 as libc::c_int as Index {
            let elevel__1: libc::c_int = 21 as libc::c_int;
            let mut __error_1: libc::c_int = 0;
            if elevel__1 >= 21 as libc::c_int {
                abort();
            }
        }
        var = makeVar(
            (*nscol).p_varno,
            (*nscol).p_varattno,
            (*nscol).p_vartype,
            (*nscol).p_vartypmod,
            (*nscol).p_varcollid,
            sublevels_up as Index,
        );
        (*var).varnosyn = (*nscol).p_varnosyn;
        (*var).varattnosyn = (*nscol).p_varattnosyn;
    } else {
        let mut sysatt: *const FormData_pg_attribute = 0 as *const FormData_pg_attribute;
        sysatt = SystemAttributeDefinition(attnum as AttrNumber);
        var = makeVar(
            (*nsitem).p_rtindex as Index,
            attnum as AttrNumber,
            (*sysatt).atttypid,
            (*sysatt).atttypmod,
            (*sysatt).attcollation,
            sublevels_up as Index,
        );
    }
    (*var).location = location;
    markVarForSelectPriv(pstate, var);
    return var as *mut Node;
}
#[no_mangle]
pub unsafe extern "C" fn colNameToVar(
    mut pstate: *mut ParseState,
    mut colname: *const libc::c_char,
    mut localonly: bool,
    mut location: libc::c_int,
) -> *mut Node {
    let mut result: *mut Node = 0 as *mut Node;
    let mut sublevels_up: libc::c_int = 0 as libc::c_int;
    let mut orig_pstate: *mut ParseState = pstate;
    while !pstate.is_null() {
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
            let mut newresult: *mut Node = 0 as *mut Node;
            if !((*nsitem).p_cols_visible == 0) {
                if !((*nsitem).p_lateral_only as libc::c_int != 0
                    && (*pstate).p_lateral_active == 0)
                {
                    newresult =
                        scanNSItemForColumn(orig_pstate, nsitem, sublevels_up, colname, location);
                    if !newresult.is_null() {
                        if !result.is_null() {
                            let elevel_: libc::c_int = 21 as libc::c_int;
                            let mut __error: libc::c_int = 0;
                            if elevel_ >= 21 as libc::c_int {
                                abort();
                            }
                        }
                        check_lateral_ref_ok(pstate, nsitem, location);
                        result = newresult;
                    }
                }
            }
            l__state.i += 1;
            l__state.i;
        }
        if !result.is_null() || localonly as libc::c_int != 0 {
            break;
        }
        pstate = (*pstate).parentParseState;
        sublevels_up += 1;
        sublevels_up;
    }
    return result;
}
unsafe extern "C" fn searchRangeTableForCol(
    mut pstate: *mut ParseState,
    mut alias: *const libc::c_char,
    mut colname: *const libc::c_char,
    mut location: libc::c_int,
) -> *mut FuzzyAttrMatchState {
    let mut orig_pstate: *mut ParseState = pstate;
    let mut fuzzystate: *mut FuzzyAttrMatchState =
        palloc(::core::mem::size_of::<FuzzyAttrMatchState>() as libc::c_ulong)
            as *mut FuzzyAttrMatchState;
    (*fuzzystate).distance = 3 as libc::c_int + 1 as libc::c_int;
    (*fuzzystate).rfirst = 0 as *mut RangeTblEntry;
    (*fuzzystate).rsecond = 0 as *mut RangeTblEntry;
    (*fuzzystate).first = 0 as libc::c_int as AttrNumber;
    (*fuzzystate).second = 0 as libc::c_int as AttrNumber;
    while !pstate.is_null() {
        let mut l: *mut ListCell = 0 as *mut ListCell;
        let mut l__state: ForEachState = {
            let mut init = ForEachState {
                l: (*pstate).p_rtable,
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
            let mut rte: *mut RangeTblEntry = (*l).ptr_value as *mut RangeTblEntry;
            let mut fuzzy_rte_penalty: libc::c_int = 0 as libc::c_int;
            if !((*rte).rtekind as libc::c_uint == RTE_JOIN as libc::c_int as libc::c_uint) {
                if !alias.is_null() {
                    fuzzy_rte_penalty = varstr_levenshtein_less_equal(
                        alias,
                        strlen(alias) as libc::c_int,
                        (*(*rte).eref).aliasname,
                        strlen((*(*rte).eref).aliasname) as libc::c_int,
                        1 as libc::c_int,
                        1 as libc::c_int,
                        1 as libc::c_int,
                        3 as libc::c_int + 1 as libc::c_int,
                        true,
                    );
                }
                if scanRTEForColumn(
                    orig_pstate,
                    rte,
                    colname,
                    location,
                    fuzzy_rte_penalty,
                    fuzzystate,
                ) != 0
                    && fuzzy_rte_penalty == 0 as libc::c_int
                {
                    (*fuzzystate).rfirst = rte;
                    (*fuzzystate).first = 0 as libc::c_int as AttrNumber;
                    (*fuzzystate).rsecond = 0 as *mut RangeTblEntry;
                    (*fuzzystate).second = 0 as libc::c_int as AttrNumber;
                    return fuzzystate;
                }
            }
            l__state.i += 1;
            l__state.i;
        }
        pstate = (*pstate).parentParseState;
    }
    return fuzzystate;
}
unsafe extern "C" fn markRTEForSelectPriv(
    mut pstate: *mut ParseState,
    mut rtindex: libc::c_int,
    mut col: AttrNumber,
) {
    let mut rte: *mut RangeTblEntry =
        list_nth((*pstate).p_rtable, rtindex - 1 as libc::c_int) as *mut RangeTblEntry;
    if (*rte).rtekind as libc::c_uint == RTE_RELATION as libc::c_int as libc::c_uint {
        (*rte).requiredPerms |= ((1 as libc::c_int) << 1 as libc::c_int) as AclMode;
        (*rte).selectedCols = bms_add_member(
            (*rte).selectedCols,
            col as libc::c_int - -(7 as libc::c_int),
        );
    } else if (*rte).rtekind as libc::c_uint == RTE_JOIN as libc::c_int as libc::c_uint {
        if col as libc::c_int == 0 as libc::c_int {
            let mut j: *mut JoinExpr = 0 as *mut JoinExpr;
            if rtindex > 0 as libc::c_int && rtindex <= list_length((*pstate).p_joinexprs) {
                j = list_nth((*pstate).p_joinexprs, rtindex - 1 as libc::c_int) as *mut JoinExpr;
            } else {
                j = 0 as *mut JoinExpr;
            }
            if j.is_null() {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"could not find JoinExpr for whole-row reference\0" as *const u8
                            as *const libc::c_char,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                            as *const u8 as *const libc::c_char,
                        1026 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            if (*((*j).larg as *const Node)).type_0 as libc::c_uint
                == T_RangeTblRef as libc::c_int as libc::c_uint
            {
                let mut varno: libc::c_int = (*((*j).larg as *mut RangeTblRef)).rtindex;
                markRTEForSelectPriv(pstate, varno, 0 as libc::c_int as AttrNumber);
            } else if (*((*j).larg as *const Node)).type_0 as libc::c_uint
                == T_JoinExpr as libc::c_int as libc::c_uint
            {
                let mut varno_0: libc::c_int = (*((*j).larg as *mut JoinExpr)).rtindex;
                markRTEForSelectPriv(pstate, varno_0, 0 as libc::c_int as AttrNumber);
            } else {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"unrecognized node type: %d\0" as *const u8 as *const libc::c_char,
                        (*((*j).larg as *const Node)).type_0 as libc::c_int,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                            as *const u8 as *const libc::c_char,
                        1043 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            if (*((*j).rarg as *const Node)).type_0 as libc::c_uint
                == T_RangeTblRef as libc::c_int as libc::c_uint
            {
                let mut varno_1: libc::c_int = (*((*j).rarg as *mut RangeTblRef)).rtindex;
                markRTEForSelectPriv(pstate, varno_1, 0 as libc::c_int as AttrNumber);
            } else if (*((*j).rarg as *const Node)).type_0 as libc::c_uint
                == T_JoinExpr as libc::c_int as libc::c_uint
            {
                let mut varno_2: libc::c_int = (*((*j).rarg as *mut JoinExpr)).rtindex;
                markRTEForSelectPriv(pstate, varno_2, 0 as libc::c_int as AttrNumber);
            } else {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if errstart(elevel__1, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"unrecognized node type: %d\0" as *const u8 as *const libc::c_char,
                        (*((*j).rarg as *const Node)).type_0 as libc::c_int,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                            as *const u8 as *const libc::c_char,
                        1058 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn markVarForSelectPriv(mut pstate: *mut ParseState, mut var: *mut Var) {
    let mut lv: Index = 0;
    lv = 0 as libc::c_int as Index;
    while lv < (*var).varlevelsup {
        pstate = (*pstate).parentParseState;
        lv = lv.wrapping_add(1);
        lv;
    }
    markRTEForSelectPriv(pstate, (*var).varno as libc::c_int, (*var).varattno);
}
unsafe extern "C" fn buildRelationAliases(
    mut tupdesc: TupleDesc,
    mut alias: *mut Alias,
    mut eref: *mut Alias,
) {
    let mut maxattrs: libc::c_int = (*tupdesc).natts;
    let mut aliaslist: *mut List = 0 as *mut List;
    let mut aliaslc: *mut ListCell = 0 as *mut ListCell;
    let mut numaliases: libc::c_int = 0;
    let mut varattno: libc::c_int = 0;
    let mut numdropped: libc::c_int = 0 as libc::c_int;
    if !alias.is_null() {
        aliaslist = (*alias).colnames;
        aliaslc = list_head(aliaslist);
        numaliases = list_length(aliaslist);
        (*alias).colnames = 0 as *mut libc::c_void as *mut List;
    } else {
        aliaslist = 0 as *mut libc::c_void as *mut List;
        aliaslc = 0 as *mut ListCell;
        numaliases = 0 as libc::c_int;
    }
    varattno = 0 as libc::c_int;
    while varattno < maxattrs {
        let mut attr: Form_pg_attribute =
            &mut *((*tupdesc).attrs).as_mut_ptr().offset(varattno as isize)
                as *mut FormData_pg_attribute;
        let mut attrname: *mut Value = 0 as *mut Value;
        if (*attr).attisdropped != 0 {
            attrname = makeString(pstrdup(b"\0" as *const u8 as *const libc::c_char));
            if !aliaslc.is_null() {
                (*alias).colnames = lappend((*alias).colnames, attrname as *mut libc::c_void);
            }
            numdropped += 1;
            numdropped;
        } else if !aliaslc.is_null() {
            attrname = (*aliaslc).ptr_value as *mut Value;
            aliaslc = lnext(aliaslist, aliaslc);
            (*alias).colnames = lappend((*alias).colnames, attrname as *mut libc::c_void);
        } else {
            attrname = makeString(pstrdup(((*attr).attname.data).as_mut_ptr()));
        }
        (*eref).colnames = lappend((*eref).colnames, attrname as *mut libc::c_void);
        varattno += 1;
        varattno;
    }
    if !aliaslc.is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
}
unsafe extern "C" fn chooseScalarFunctionAlias(
    mut funcexpr: *mut Node,
    mut funcname: *mut libc::c_char,
    mut alias: *mut Alias,
    mut nfuncs: libc::c_int,
) -> *mut libc::c_char {
    let mut pname: *mut libc::c_char = 0 as *mut libc::c_char;
    if !funcexpr.is_null()
        && (*(funcexpr as *const Node)).type_0 as libc::c_uint
            == T_FuncExpr as libc::c_int as libc::c_uint
    {
        pname = get_func_result_name((*(funcexpr as *mut FuncExpr)).funcid);
        if !pname.is_null() {
            return pname;
        }
    }
    if nfuncs == 1 as libc::c_int && !alias.is_null() {
        return (*alias).aliasname;
    }
    return funcname;
}
unsafe extern "C" fn buildNSItemFromTupleDesc(
    mut rte: *mut RangeTblEntry,
    mut rtindex: Index,
    mut tupdesc: TupleDesc,
) -> *mut ParseNamespaceItem {
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    let mut nscolumns: *mut ParseNamespaceColumn = 0 as *mut ParseNamespaceColumn;
    let mut maxattrs: libc::c_int = (*tupdesc).natts;
    let mut varattno: libc::c_int = 0;
    nscolumns = palloc0(
        (maxattrs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<ParseNamespaceColumn>() as libc::c_ulong),
    ) as *mut ParseNamespaceColumn;
    varattno = 0 as libc::c_int;
    while varattno < maxattrs {
        let mut attr: Form_pg_attribute =
            &mut *((*tupdesc).attrs).as_mut_ptr().offset(varattno as isize)
                as *mut FormData_pg_attribute;
        if !((*attr).attisdropped != 0) {
            (*nscolumns.offset(varattno as isize)).p_varno = rtindex;
            (*nscolumns.offset(varattno as isize)).p_varattno =
                (varattno + 1 as libc::c_int) as AttrNumber;
            (*nscolumns.offset(varattno as isize)).p_vartype = (*attr).atttypid;
            (*nscolumns.offset(varattno as isize)).p_vartypmod = (*attr).atttypmod;
            (*nscolumns.offset(varattno as isize)).p_varcollid = (*attr).attcollation;
            (*nscolumns.offset(varattno as isize)).p_varnosyn = rtindex;
            (*nscolumns.offset(varattno as isize)).p_varattnosyn =
                (varattno + 1 as libc::c_int) as AttrNumber;
        }
        varattno += 1;
        varattno;
    }
    nsitem = palloc(::core::mem::size_of::<ParseNamespaceItem>() as libc::c_ulong)
        as *mut ParseNamespaceItem;
    (*nsitem).p_rte = rte;
    (*nsitem).p_rtindex = rtindex as libc::c_int;
    (*nsitem).p_nscolumns = nscolumns;
    (*nsitem).p_rel_visible = true;
    (*nsitem).p_cols_visible = true;
    (*nsitem).p_lateral_only = false;
    (*nsitem).p_lateral_ok = true;
    return nsitem;
}
unsafe extern "C" fn buildNSItemFromLists(
    mut rte: *mut RangeTblEntry,
    mut rtindex: Index,
    mut coltypes: *mut List,
    mut coltypmods: *mut List,
    mut colcollations: *mut List,
) -> *mut ParseNamespaceItem {
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    let mut nscolumns: *mut ParseNamespaceColumn = 0 as *mut ParseNamespaceColumn;
    let mut maxattrs: libc::c_int = list_length(coltypes);
    let mut varattno: libc::c_int = 0;
    let mut lct: *mut ListCell = 0 as *mut ListCell;
    let mut lcm: *mut ListCell = 0 as *mut ListCell;
    let mut lcc: *mut ListCell = 0 as *mut ListCell;
    nscolumns = palloc0(
        (maxattrs as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<ParseNamespaceColumn>() as libc::c_ulong),
    ) as *mut ParseNamespaceColumn;
    varattno = 0 as libc::c_int;
    let mut lct__state: ForThreeState = {
        let mut init = ForThreeState {
            l1: coltypes,
            l2: coltypmods,
            l3: colcollations,
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
        if !(!lct.is_null() && !lcm.is_null() && !lcc.is_null()) {
            break;
        }
        (*nscolumns.offset(varattno as isize)).p_varno = rtindex;
        (*nscolumns.offset(varattno as isize)).p_varattno =
            (varattno + 1 as libc::c_int) as AttrNumber;
        (*nscolumns.offset(varattno as isize)).p_vartype = (*lct).oid_value;
        (*nscolumns.offset(varattno as isize)).p_vartypmod = (*lcm).int_value;
        (*nscolumns.offset(varattno as isize)).p_varcollid = (*lcc).oid_value;
        (*nscolumns.offset(varattno as isize)).p_varnosyn = rtindex;
        (*nscolumns.offset(varattno as isize)).p_varattnosyn =
            (varattno + 1 as libc::c_int) as AttrNumber;
        varattno += 1;
        varattno;
        lct__state.i += 1;
        lct__state.i;
    }
    nsitem = palloc(::core::mem::size_of::<ParseNamespaceItem>() as libc::c_ulong)
        as *mut ParseNamespaceItem;
    (*nsitem).p_rte = rte;
    (*nsitem).p_rtindex = rtindex as libc::c_int;
    (*nsitem).p_nscolumns = nscolumns;
    (*nsitem).p_rel_visible = true;
    (*nsitem).p_cols_visible = true;
    (*nsitem).p_lateral_only = false;
    (*nsitem).p_lateral_ok = true;
    return nsitem;
}
#[no_mangle]
pub unsafe extern "C" fn parserOpenTable(
    mut pstate: *mut ParseState,
    mut relation: *const RangeVar,
    mut lockmode: libc::c_int,
) -> Relation {
    let mut rel: Relation = 0 as *mut RelationData;
    let mut pcbstate: ParseCallbackState = ParseCallbackState {
        pstate: 0 as *mut ParseState,
        location: 0,
        errcallback: ErrorContextCallback {
            previous: 0 as *mut ErrorContextCallback,
            callback: None,
            arg: 0 as *mut libc::c_void,
        },
    };
    setup_parser_errposition_callback(&mut pcbstate, pstate, (*relation).location);
    rel = table_openrv_extended(relation, lockmode, true);
    if rel.is_null() {
        if !((*relation).schemaname).is_null() {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        } else if isFutureCTE(pstate, (*relation).relname) != 0 {
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
    cancel_parser_errposition_callback(&mut pcbstate);
    return rel;
}
#[no_mangle]
pub unsafe extern "C" fn addRangeTableEntry(
    mut pstate: *mut ParseState,
    mut relation: *mut RangeVar,
    mut alias: *mut Alias,
    mut inh: bool,
    mut inFromCl: bool,
) -> *mut ParseNamespaceItem {
    let mut rte: *mut RangeTblEntry = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_RangeTblEntry;
        _result
    }) as *mut RangeTblEntry;
    let mut refname: *mut libc::c_char = if !alias.is_null() {
        (*alias).aliasname
    } else {
        (*relation).relname
    };
    let mut lockmode: LOCKMODE = 0;
    let mut rel: Relation = 0 as *mut RelationData;
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    (*rte).rtekind = RTE_RELATION;
    (*rte).alias = alias;
    lockmode = if isLockedRefname(pstate, refname) as libc::c_int != 0 {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    rel = parserOpenTable(pstate, relation, lockmode);
    (*rte).relid = (*rel).rd_id;
    (*rte).relkind = (*(*rel).rd_rel).relkind;
    (*rte).rellockmode = lockmode;
    (*rte).eref = makeAlias(refname, 0 as *mut libc::c_void as *mut List);
    buildRelationAliases((*rel).rd_att, alias, (*rte).eref);
    (*rte).lateral = false;
    (*rte).inh = inh;
    (*rte).inFromCl = inFromCl;
    (*rte).requiredPerms = ((1 as libc::c_int) << 1 as libc::c_int) as AclMode;
    (*rte).checkAsUser = 0 as libc::c_int as Oid;
    (*rte).selectedCols = 0 as *mut Bitmapset;
    (*rte).insertedCols = 0 as *mut Bitmapset;
    (*rte).updatedCols = 0 as *mut Bitmapset;
    (*rte).extraUpdatedCols = 0 as *mut Bitmapset;
    (*pstate).p_rtable = lappend((*pstate).p_rtable, rte as *mut libc::c_void);
    nsitem = buildNSItemFromTupleDesc(rte, list_length((*pstate).p_rtable) as Index, (*rel).rd_att);
    table_close(rel, 0 as libc::c_int);
    return nsitem;
}
#[no_mangle]
pub unsafe extern "C" fn addRangeTableEntryForRelation(
    mut pstate: *mut ParseState,
    mut rel: Relation,
    mut lockmode: libc::c_int,
    mut alias: *mut Alias,
    mut inh: bool,
    mut inFromCl: bool,
) -> *mut ParseNamespaceItem {
    let mut rte: *mut RangeTblEntry = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_RangeTblEntry;
        _result
    }) as *mut RangeTblEntry;
    let mut refname: *mut libc::c_char = if !alias.is_null() {
        (*alias).aliasname
    } else {
        ((*(*rel).rd_rel).relname.data).as_mut_ptr()
    };
    (*rte).rtekind = RTE_RELATION;
    (*rte).alias = alias;
    (*rte).relid = (*rel).rd_id;
    (*rte).relkind = (*(*rel).rd_rel).relkind;
    (*rte).rellockmode = lockmode;
    (*rte).eref = makeAlias(refname, 0 as *mut libc::c_void as *mut List);
    buildRelationAliases((*rel).rd_att, alias, (*rte).eref);
    (*rte).lateral = false;
    (*rte).inh = inh;
    (*rte).inFromCl = inFromCl;
    (*rte).requiredPerms = ((1 as libc::c_int) << 1 as libc::c_int) as AclMode;
    (*rte).checkAsUser = 0 as libc::c_int as Oid;
    (*rte).selectedCols = 0 as *mut Bitmapset;
    (*rte).insertedCols = 0 as *mut Bitmapset;
    (*rte).updatedCols = 0 as *mut Bitmapset;
    (*rte).extraUpdatedCols = 0 as *mut Bitmapset;
    (*pstate).p_rtable = lappend((*pstate).p_rtable, rte as *mut libc::c_void);
    return buildNSItemFromTupleDesc(rte, list_length((*pstate).p_rtable) as Index, (*rel).rd_att);
}
#[no_mangle]
pub unsafe extern "C" fn addRangeTableEntryForSubquery(
    mut pstate: *mut ParseState,
    mut subquery: *mut Query,
    mut alias: *mut Alias,
    mut lateral: bool,
    mut inFromCl: bool,
) -> *mut ParseNamespaceItem {
    let mut rte: *mut RangeTblEntry = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_RangeTblEntry;
        _result
    }) as *mut RangeTblEntry;
    let mut refname: *mut libc::c_char = (*alias).aliasname;
    let mut eref: *mut Alias = 0 as *mut Alias;
    let mut numaliases: libc::c_int = 0;
    let mut coltypes: *mut List = 0 as *mut List;
    let mut coltypmods: *mut List = 0 as *mut List;
    let mut colcollations: *mut List = 0 as *mut List;
    let mut varattno: libc::c_int = 0;
    let mut tlistitem: *mut ListCell = 0 as *mut ListCell;
    (*rte).rtekind = RTE_SUBQUERY;
    (*rte).subquery = subquery;
    (*rte).alias = alias;
    eref = copyObjectImpl(alias as *const libc::c_void) as *mut Alias;
    numaliases = list_length((*eref).colnames);
    colcollations = 0 as *mut libc::c_void as *mut List;
    coltypmods = colcollations;
    coltypes = coltypmods;
    varattno = 0 as libc::c_int;
    let mut tlistitem__state: ForEachState = {
        let mut init = ForEachState {
            l: (*subquery).targetList,
            i: 0 as libc::c_int,
        };
        init
    };
    while if !(tlistitem__state.l).is_null() && tlistitem__state.i < (*tlistitem__state.l).length {
        tlistitem = &mut *((*tlistitem__state.l).elements).offset(tlistitem__state.i as isize)
            as *mut ListCell;
        true as libc::c_int
    } else {
        tlistitem = 0 as *mut ListCell;
        false as libc::c_int
    } != 0
    {
        let mut te: *mut TargetEntry = (*tlistitem).ptr_value as *mut TargetEntry;
        if !((*te).resjunk != 0) {
            varattno += 1;
            varattno;
            if varattno > numaliases {
                let mut attrname: *mut libc::c_char = 0 as *mut libc::c_char;
                attrname = pstrdup((*te).resname);
                (*eref).colnames =
                    lappend((*eref).colnames, makeString(attrname) as *mut libc::c_void);
            }
            coltypes = lappend_oid(coltypes, exprType((*te).expr as *mut Node));
            coltypmods = lappend_int(coltypmods, exprTypmod((*te).expr as *mut Node));
            colcollations = lappend_oid(colcollations, exprCollation((*te).expr as *mut Node));
        }
        tlistitem__state.i += 1;
        tlistitem__state.i;
    }
    if varattno < numaliases {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    (*rte).eref = eref;
    (*rte).lateral = lateral;
    (*rte).inh = false;
    (*rte).inFromCl = inFromCl;
    (*rte).requiredPerms = 0 as libc::c_int as AclMode;
    (*rte).checkAsUser = 0 as libc::c_int as Oid;
    (*rte).selectedCols = 0 as *mut Bitmapset;
    (*rte).insertedCols = 0 as *mut Bitmapset;
    (*rte).updatedCols = 0 as *mut Bitmapset;
    (*rte).extraUpdatedCols = 0 as *mut Bitmapset;
    (*pstate).p_rtable = lappend((*pstate).p_rtable, rte as *mut libc::c_void);
    return buildNSItemFromLists(
        rte,
        list_length((*pstate).p_rtable) as Index,
        coltypes,
        coltypmods,
        colcollations,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addRangeTableEntryForTableFunc(
    mut pstate: *mut ParseState,
    mut tf: *mut TableFunc,
    mut alias: *mut Alias,
    mut lateral: bool,
    mut inFromCl: bool,
) -> *mut ParseNamespaceItem {
    let mut rte: *mut RangeTblEntry = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_RangeTblEntry;
        _result
    }) as *mut RangeTblEntry;
    let mut refname: *mut libc::c_char = if !alias.is_null() {
        (*alias).aliasname
    } else {
        pstrdup(b"xmltable\0" as *const u8 as *const libc::c_char)
    };
    let mut eref: *mut Alias = 0 as *mut Alias;
    let mut numaliases: libc::c_int = 0;
    (*rte).rtekind = RTE_TABLEFUNC;
    (*rte).relid = 0 as libc::c_int as Oid;
    (*rte).subquery = 0 as *mut Query;
    (*rte).tablefunc = tf;
    (*rte).coltypes = (*tf).coltypes;
    (*rte).coltypmods = (*tf).coltypmods;
    (*rte).colcollations = (*tf).colcollations;
    (*rte).alias = alias;
    eref = (if !alias.is_null() {
        copyObjectImpl(alias as *const libc::c_void)
    } else {
        makeAlias(refname, 0 as *mut libc::c_void as *mut List) as *mut libc::c_void
    }) as *mut Alias;
    numaliases = list_length((*eref).colnames);
    if numaliases < list_length((*tf).colnames) {
        (*eref).colnames =
            list_concat((*eref).colnames, list_copy_tail((*tf).colnames, numaliases));
    }
    (*rte).eref = eref;
    (*rte).lateral = lateral;
    (*rte).inh = false;
    (*rte).inFromCl = inFromCl;
    (*rte).requiredPerms = 0 as libc::c_int as AclMode;
    (*rte).checkAsUser = 0 as libc::c_int as Oid;
    (*rte).selectedCols = 0 as *mut Bitmapset;
    (*rte).insertedCols = 0 as *mut Bitmapset;
    (*rte).updatedCols = 0 as *mut Bitmapset;
    (*rte).extraUpdatedCols = 0 as *mut Bitmapset;
    (*pstate).p_rtable = lappend((*pstate).p_rtable, rte as *mut libc::c_void);
    return buildNSItemFromLists(
        rte,
        list_length((*pstate).p_rtable) as Index,
        (*rte).coltypes,
        (*rte).coltypmods,
        (*rte).colcollations,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addRangeTableEntryForValues(
    mut pstate: *mut ParseState,
    mut exprs: *mut List,
    mut coltypes: *mut List,
    mut coltypmods: *mut List,
    mut colcollations: *mut List,
    mut alias: *mut Alias,
    mut lateral: bool,
    mut inFromCl: bool,
) -> *mut ParseNamespaceItem {
    let mut rte: *mut RangeTblEntry = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_RangeTblEntry;
        _result
    }) as *mut RangeTblEntry;
    let mut refname: *mut libc::c_char = if !alias.is_null() {
        (*alias).aliasname
    } else {
        pstrdup(b"*VALUES*\0" as *const u8 as *const libc::c_char)
    };
    let mut eref: *mut Alias = 0 as *mut Alias;
    let mut numaliases: libc::c_int = 0;
    let mut numcolumns: libc::c_int = 0;
    (*rte).rtekind = RTE_VALUES;
    (*rte).relid = 0 as libc::c_int as Oid;
    (*rte).subquery = 0 as *mut Query;
    (*rte).values_lists = exprs;
    (*rte).coltypes = coltypes;
    (*rte).coltypmods = coltypmods;
    (*rte).colcollations = colcollations;
    (*rte).alias = alias;
    eref = (if !alias.is_null() {
        copyObjectImpl(alias as *const libc::c_void)
    } else {
        makeAlias(refname, 0 as *mut libc::c_void as *mut List) as *mut libc::c_void
    }) as *mut Alias;
    numcolumns = list_length((*list_nth_cell(exprs, 0 as libc::c_int)).ptr_value as *mut List);
    numaliases = list_length((*eref).colnames);
    while numaliases < numcolumns {
        let mut attrname: [libc::c_char; 64] = [0; 64];
        numaliases += 1;
        numaliases;
        pg_snprintf(
            attrname.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"column%d\0" as *const u8 as *const libc::c_char,
            numaliases,
        );
        (*eref).colnames = lappend(
            (*eref).colnames,
            makeString(pstrdup(attrname.as_mut_ptr())) as *mut libc::c_void,
        );
    }
    if numcolumns < numaliases {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    (*rte).eref = eref;
    (*rte).lateral = lateral;
    (*rte).inh = false;
    (*rte).inFromCl = inFromCl;
    (*rte).requiredPerms = 0 as libc::c_int as AclMode;
    (*rte).checkAsUser = 0 as libc::c_int as Oid;
    (*rte).selectedCols = 0 as *mut Bitmapset;
    (*rte).insertedCols = 0 as *mut Bitmapset;
    (*rte).updatedCols = 0 as *mut Bitmapset;
    (*rte).extraUpdatedCols = 0 as *mut Bitmapset;
    (*pstate).p_rtable = lappend((*pstate).p_rtable, rte as *mut libc::c_void);
    return buildNSItemFromLists(
        rte,
        list_length((*pstate).p_rtable) as Index,
        (*rte).coltypes,
        (*rte).coltypmods,
        (*rte).colcollations,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addRangeTableEntryForJoin(
    mut pstate: *mut ParseState,
    mut colnames: *mut List,
    mut nscolumns: *mut ParseNamespaceColumn,
    mut jointype: JoinType,
    mut nummergedcols: libc::c_int,
    mut aliasvars: *mut List,
    mut leftcols: *mut List,
    mut rightcols: *mut List,
    mut alias: *mut Alias,
    mut inFromCl: bool,
) -> *mut ParseNamespaceItem {
    let mut rte: *mut RangeTblEntry = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_RangeTblEntry;
        _result
    }) as *mut RangeTblEntry;
    let mut eref: *mut Alias = 0 as *mut Alias;
    let mut numaliases: libc::c_int = 0;
    let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    if list_length(aliasvars) > 32767 as libc::c_int {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    (*rte).rtekind = RTE_JOIN;
    (*rte).relid = 0 as libc::c_int as Oid;
    (*rte).subquery = 0 as *mut Query;
    (*rte).jointype = jointype;
    (*rte).joinmergedcols = nummergedcols;
    (*rte).joinaliasvars = aliasvars;
    (*rte).joinleftcols = leftcols;
    (*rte).joinrightcols = rightcols;
    (*rte).alias = alias;
    eref = (if !alias.is_null() {
        copyObjectImpl(alias as *const libc::c_void)
    } else {
        makeAlias(
            b"unnamed_join\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void as *mut List,
        ) as *mut libc::c_void
    }) as *mut Alias;
    numaliases = list_length((*eref).colnames);
    if numaliases < list_length(colnames) {
        (*eref).colnames = list_concat((*eref).colnames, list_copy_tail(colnames, numaliases));
    }
    (*rte).eref = eref;
    (*rte).lateral = false;
    (*rte).inh = false;
    (*rte).inFromCl = inFromCl;
    (*rte).requiredPerms = 0 as libc::c_int as AclMode;
    (*rte).checkAsUser = 0 as libc::c_int as Oid;
    (*rte).selectedCols = 0 as *mut Bitmapset;
    (*rte).insertedCols = 0 as *mut Bitmapset;
    (*rte).updatedCols = 0 as *mut Bitmapset;
    (*rte).extraUpdatedCols = 0 as *mut Bitmapset;
    (*pstate).p_rtable = lappend((*pstate).p_rtable, rte as *mut libc::c_void);
    nsitem = palloc(::core::mem::size_of::<ParseNamespaceItem>() as libc::c_ulong)
        as *mut ParseNamespaceItem;
    (*nsitem).p_rte = rte;
    (*nsitem).p_rtindex = list_length((*pstate).p_rtable);
    (*nsitem).p_nscolumns = nscolumns;
    (*nsitem).p_rel_visible = true;
    (*nsitem).p_cols_visible = true;
    (*nsitem).p_lateral_only = false;
    (*nsitem).p_lateral_ok = true;
    return nsitem;
}
#[no_mangle]
pub unsafe extern "C" fn addRangeTableEntryForCTE(
    mut pstate: *mut ParseState,
    mut cte: *mut CommonTableExpr,
    mut levelsup: Index,
    mut rv: *mut RangeVar,
    mut inFromCl: bool,
) -> *mut ParseNamespaceItem {
    let mut rte: *mut RangeTblEntry = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_RangeTblEntry;
        _result
    }) as *mut RangeTblEntry;
    let mut alias: *mut Alias = (*rv).alias;
    let mut refname: *mut libc::c_char = if !alias.is_null() {
        (*alias).aliasname
    } else {
        (*cte).ctename
    };
    let mut eref: *mut Alias = 0 as *mut Alias;
    let mut numaliases: libc::c_int = 0;
    let mut varattno: libc::c_int = 0;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    let mut n_dontexpand_columns: libc::c_int = 0 as libc::c_int;
    let mut psi: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
    (*rte).rtekind = RTE_CTE;
    (*rte).ctename = (*cte).ctename;
    (*rte).ctelevelsup = levelsup;
    (*rte).self_reference = !((*((*cte).ctequery as *const Node)).type_0 as libc::c_uint
        == T_Query as libc::c_int as libc::c_uint) as libc::c_int
        as bool;
    if (*rte).self_reference == 0 {
        (*cte).cterefcount += 1;
        (*cte).cterefcount;
    }
    if (*((*cte).ctequery as *const Node)).type_0 as libc::c_uint
        == T_Query as libc::c_int as libc::c_uint
    {
        let mut ctequery: *mut Query = (*cte).ctequery as *mut Query;
        if (*ctequery).commandType as libc::c_uint != CMD_SELECT as libc::c_int as libc::c_uint
            && ((*ctequery).returningList).is_null()
        {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
    }
    (*rte).coltypes = list_copy((*cte).ctecoltypes);
    (*rte).coltypmods = list_copy((*cte).ctecoltypmods);
    (*rte).colcollations = list_copy((*cte).ctecolcollations);
    (*rte).alias = alias;
    if !alias.is_null() {
        eref = copyObjectImpl(alias as *const libc::c_void) as *mut Alias;
    } else {
        eref = makeAlias(refname, 0 as *mut libc::c_void as *mut List);
    }
    numaliases = list_length((*eref).colnames);
    varattno = 0 as libc::c_int;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*cte).ctecolnames,
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
        varattno += 1;
        varattno;
        if varattno > numaliases {
            (*eref).colnames = lappend((*eref).colnames, (*lc).ptr_value);
        }
        lc__state.i += 1;
        lc__state.i;
    }
    if varattno < numaliases {
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    }
    (*rte).eref = eref;
    if !((*cte).search_clause).is_null() {
        (*(*rte).eref).colnames = lappend(
            (*(*rte).eref).colnames,
            makeString((*(*cte).search_clause).search_seq_column) as *mut libc::c_void,
        );
        (*rte).coltypmods = lappend_int((*rte).coltypmods, -(1 as libc::c_int));
        (*rte).colcollations = lappend_oid((*rte).colcollations, 0 as libc::c_int as Oid);
        n_dontexpand_columns += 1 as libc::c_int;
    }
    if !((*cte).cycle_clause).is_null() {
        (*(*rte).eref).colnames = lappend(
            (*(*rte).eref).colnames,
            makeString((*(*cte).cycle_clause).cycle_mark_column) as *mut libc::c_void,
        );
        (*rte).coltypes = lappend_oid((*rte).coltypes, (*(*cte).cycle_clause).cycle_mark_type);
        (*rte).coltypmods =
            lappend_int((*rte).coltypmods, (*(*cte).cycle_clause).cycle_mark_typmod);
        (*rte).colcollations = lappend_oid(
            (*rte).colcollations,
            (*(*cte).cycle_clause).cycle_mark_collation,
        );
        (*(*rte).eref).colnames = lappend(
            (*(*rte).eref).colnames,
            makeString((*(*cte).cycle_clause).cycle_path_column) as *mut libc::c_void,
        );
        (*rte).coltypmods = lappend_int((*rte).coltypmods, -(1 as libc::c_int));
        (*rte).colcollations = lappend_oid((*rte).colcollations, 0 as libc::c_int as Oid);
        n_dontexpand_columns += 2 as libc::c_int;
    }
    (*rte).lateral = false;
    (*rte).inh = false;
    (*rte).inFromCl = inFromCl;
    (*rte).requiredPerms = 0 as libc::c_int as AclMode;
    (*rte).checkAsUser = 0 as libc::c_int as Oid;
    (*rte).selectedCols = 0 as *mut Bitmapset;
    (*rte).insertedCols = 0 as *mut Bitmapset;
    (*rte).updatedCols = 0 as *mut Bitmapset;
    (*rte).extraUpdatedCols = 0 as *mut Bitmapset;
    (*pstate).p_rtable = lappend((*pstate).p_rtable, rte as *mut libc::c_void);
    psi = buildNSItemFromLists(
        rte,
        list_length((*pstate).p_rtable) as Index,
        (*rte).coltypes,
        (*rte).coltypmods,
        (*rte).colcollations,
    );
    if (*rte).ctelevelsup > 0 as libc::c_int as Index {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < n_dontexpand_columns {
            (*((*psi).p_nscolumns).offset(
                (list_length((*(*(*psi).p_rte).eref).colnames) - 1 as libc::c_int - i) as isize,
            ))
            .p_dontexpand = true;
            i += 1;
            i;
        }
    }
    return psi;
}
#[no_mangle]
pub unsafe extern "C" fn addRangeTableEntryForENR(
    mut pstate: *mut ParseState,
    mut rv: *mut RangeVar,
    mut inFromCl: bool,
) -> *mut ParseNamespaceItem {
    let mut rte: *mut RangeTblEntry = ({
        let mut _result: *mut Node = 0 as *mut Node;
        (*_result).type_0 = T_RangeTblEntry;
        _result
    }) as *mut RangeTblEntry;
    let mut alias: *mut Alias = (*rv).alias;
    let mut refname: *mut libc::c_char = if !alias.is_null() {
        (*alias).aliasname
    } else {
        (*rv).relname
    };
    let mut enrmd: EphemeralNamedRelationMetadata = 0 as *mut EphemeralNamedRelationMetadataData;
    let mut tupdesc: TupleDesc = 0 as *mut TupleDescData;
    let mut attno: libc::c_int = 0;
    enrmd = get_visible_ENR(pstate, (*rv).relname);
    match (*enrmd).enrtype as libc::c_uint {
        0 => {
            (*rte).rtekind = RTE_NAMEDTUPLESTORE;
        }
        _ => {
            let elevel_: libc::c_int = 21 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unexpected enrtype: %d\0" as *const u8 as *const libc::c_char,
                    (*enrmd).enrtype as libc::c_uint,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                        as *const u8 as *const libc::c_char,
                    2400 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
            return 0 as *mut ParseNamespaceItem;
        }
    }
    (*rte).relid = (*enrmd).reliddesc;
    tupdesc = ENRMetadataGetTupDesc(enrmd);
    (*rte).eref = makeAlias(refname, 0 as *mut libc::c_void as *mut List);
    buildRelationAliases(tupdesc, alias, (*rte).eref);
    (*rte).enrname = (*enrmd).name;
    (*rte).enrtuples = (*enrmd).enrtuples;
    (*rte).coltypes = 0 as *mut libc::c_void as *mut List;
    (*rte).coltypmods = 0 as *mut libc::c_void as *mut List;
    (*rte).colcollations = 0 as *mut libc::c_void as *mut List;
    attno = 1 as libc::c_int;
    while attno <= (*tupdesc).natts {
        let mut att: Form_pg_attribute = &mut *((*tupdesc).attrs)
            .as_mut_ptr()
            .offset((attno - 1 as libc::c_int) as isize)
            as *mut FormData_pg_attribute;
        if (*att).attisdropped != 0 {
            (*rte).coltypes = lappend_oid((*rte).coltypes, 0 as libc::c_int as Oid);
            (*rte).coltypmods = lappend_int((*rte).coltypmods, 0 as libc::c_int);
            (*rte).colcollations = lappend_oid((*rte).colcollations, 0 as libc::c_int as Oid);
        } else {
            if (*att).atttypid == 0 as libc::c_int as Oid {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"atttypid is invalid for non-dropped column in \"%s\"\0" as *const u8
                            as *const libc::c_char,
                        (*rv).relname,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                            as *const u8 as *const libc::c_char,
                        2440 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            (*rte).coltypes = lappend_oid((*rte).coltypes, (*att).atttypid);
            (*rte).coltypmods = lappend_int((*rte).coltypmods, (*att).atttypmod);
            (*rte).colcollations = lappend_oid((*rte).colcollations, (*att).attcollation);
        }
        attno += 1;
        attno;
    }
    (*rte).lateral = false;
    (*rte).inh = false;
    (*rte).inFromCl = inFromCl;
    (*rte).requiredPerms = 0 as libc::c_int as AclMode;
    (*rte).checkAsUser = 0 as libc::c_int as Oid;
    (*rte).selectedCols = 0 as *mut Bitmapset;
    (*pstate).p_rtable = lappend((*pstate).p_rtable, rte as *mut libc::c_void);
    return buildNSItemFromTupleDesc(rte, list_length((*pstate).p_rtable) as Index, tupdesc);
}
#[no_mangle]
pub unsafe extern "C" fn isLockedRefname(
    mut pstate: *mut ParseState,
    mut refname: *const libc::c_char,
) -> bool {
    let mut l: *mut ListCell = 0 as *mut ListCell;
    if (*pstate).p_locked_from_parent != 0 {
        return true;
    }
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: (*pstate).p_locking_clause,
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
        let mut lc: *mut LockingClause = (*l).ptr_value as *mut LockingClause;
        if ((*lc).lockedRels).is_null() {
            return true;
        } else {
            let mut l2: *mut ListCell = 0 as *mut ListCell;
            let mut l2__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*lc).lockedRels,
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
                let mut thisrel: *mut RangeVar = (*l2).ptr_value as *mut RangeVar;
                if strcmp(refname, (*thisrel).relname) == 0 as libc::c_int {
                    return true;
                }
                l2__state.i += 1;
                l2__state.i;
            }
        }
        l__state.i += 1;
        l__state.i;
    }
    return false;
}
#[no_mangle]
pub unsafe extern "C" fn addNSItemToQuery(
    mut pstate: *mut ParseState,
    mut nsitem: *mut ParseNamespaceItem,
    mut addToJoinList: bool,
    mut addToRelNameSpace: bool,
    mut addToVarNameSpace: bool,
) {
    if addToJoinList != 0 {
        let mut rtr: *mut RangeTblRef = ({
            let mut _result: *mut Node = 0 as *mut Node;
            (*_result).type_0 = T_RangeTblRef;
            _result
        }) as *mut RangeTblRef;
        (*rtr).rtindex = (*nsitem).p_rtindex;
        (*pstate).p_joinlist = lappend((*pstate).p_joinlist, rtr as *mut libc::c_void);
    }
    if addToRelNameSpace as libc::c_int != 0 || addToVarNameSpace as libc::c_int != 0 {
        (*nsitem).p_rel_visible = addToRelNameSpace;
        (*nsitem).p_cols_visible = addToVarNameSpace;
        (*nsitem).p_lateral_only = false;
        (*nsitem).p_lateral_ok = true;
        (*pstate).p_namespace = lappend((*pstate).p_namespace, nsitem as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn expandRTE(
    mut rte: *mut RangeTblEntry,
    mut rtindex: libc::c_int,
    mut sublevels_up: libc::c_int,
    mut location: libc::c_int,
    mut include_dropped: bool,
    mut colnames: *mut *mut List,
    mut colvars: *mut *mut List,
) {
    let mut varattno: libc::c_int = 0;
    if !colnames.is_null() {
        *colnames = 0 as *mut libc::c_void as *mut List;
    }
    if !colvars.is_null() {
        *colvars = 0 as *mut libc::c_void as *mut List;
    }
    match (*rte).rtekind as libc::c_uint {
        0 => {
            expandRelation(
                (*rte).relid,
                (*rte).eref,
                rtindex,
                sublevels_up,
                location,
                include_dropped,
                colnames,
                colvars,
            );
        }
        1 => {
            let mut aliasp_item: *mut ListCell = list_head((*(*rte).eref).colnames);
            let mut tlistitem: *mut ListCell = 0 as *mut ListCell;
            varattno = 0 as libc::c_int;
            let mut tlistitem__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*(*rte).subquery).targetList,
                    i: 0 as libc::c_int,
                };
                init
            };
            while if !(tlistitem__state.l).is_null()
                && tlistitem__state.i < (*tlistitem__state.l).length
            {
                tlistitem = &mut *((*tlistitem__state.l).elements)
                    .offset(tlistitem__state.i as isize)
                    as *mut ListCell;
                true as libc::c_int
            } else {
                tlistitem = 0 as *mut ListCell;
                false as libc::c_int
            } != 0
            {
                let mut te: *mut TargetEntry = (*tlistitem).ptr_value as *mut TargetEntry;
                if !((*te).resjunk != 0) {
                    varattno += 1;
                    varattno;
                    if aliasp_item.is_null() {
                        break;
                    }
                    if !colnames.is_null() {
                        let mut label: *mut libc::c_char =
                            (*((*aliasp_item).ptr_value as *mut Value)).val.str_0;
                        *colnames =
                            lappend(*colnames, makeString(pstrdup(label)) as *mut libc::c_void);
                    }
                    if !colvars.is_null() {
                        let mut varnode: *mut Var = 0 as *mut Var;
                        varnode = makeVar(
                            rtindex as Index,
                            varattno as AttrNumber,
                            exprType((*te).expr as *mut Node),
                            exprTypmod((*te).expr as *mut Node),
                            exprCollation((*te).expr as *mut Node),
                            sublevels_up as Index,
                        );
                        (*varnode).location = location;
                        *colvars = lappend(*colvars, varnode as *mut libc::c_void);
                    }
                    aliasp_item = lnext((*(*rte).eref).colnames, aliasp_item);
                }
                tlistitem__state.i += 1;
                tlistitem__state.i;
            }
        }
        3 => {
            let mut atts_done: libc::c_int = 0 as libc::c_int;
            let mut lc: *mut ListCell = 0 as *mut ListCell;
            let mut lc__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*rte).functions,
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
                let mut rtfunc: *mut RangeTblFunction = (*lc).ptr_value as *mut RangeTblFunction;
                let mut functypclass: TypeFuncClass = TYPEFUNC_SCALAR;
                let mut funcrettype: Oid = 0;
                let mut tupdesc: TupleDesc = 0 as *mut TupleDescData;
                functypclass =
                    get_expr_result_type((*rtfunc).funcexpr, &mut funcrettype, &mut tupdesc);
                if functypclass as libc::c_uint == TYPEFUNC_COMPOSITE as libc::c_int as libc::c_uint
                    || functypclass as libc::c_uint
                        == TYPEFUNC_COMPOSITE_DOMAIN as libc::c_int as libc::c_uint
                {
                    expandTupleDesc(
                        tupdesc,
                        (*rte).eref,
                        (*rtfunc).funccolcount,
                        atts_done,
                        rtindex,
                        sublevels_up,
                        location,
                        include_dropped,
                        colnames,
                        colvars,
                    );
                } else if functypclass as libc::c_uint
                    == TYPEFUNC_SCALAR as libc::c_int as libc::c_uint
                {
                    if !colnames.is_null() {
                        *colnames =
                            lappend(*colnames, list_nth((*(*rte).eref).colnames, atts_done));
                    }
                    if !colvars.is_null() {
                        let mut varnode_0: *mut Var = 0 as *mut Var;
                        varnode_0 = makeVar(
                            rtindex as Index,
                            (atts_done + 1 as libc::c_int) as AttrNumber,
                            funcrettype,
                            exprTypmod((*rtfunc).funcexpr),
                            exprCollation((*rtfunc).funcexpr),
                            sublevels_up as Index,
                        );
                        (*varnode_0).location = location;
                        *colvars = lappend(*colvars, varnode_0 as *mut libc::c_void);
                    }
                } else if functypclass as libc::c_uint
                    == TYPEFUNC_RECORD as libc::c_int as libc::c_uint
                {
                    if !colnames.is_null() {
                        let mut namelist: *mut List = 0 as *mut List;
                        namelist = list_copy_tail((*(*rte).eref).colnames, atts_done);
                        namelist = list_truncate(namelist, (*rtfunc).funccolcount);
                        *colnames = list_concat(*colnames, namelist);
                    }
                    if !colvars.is_null() {
                        let mut l1: *mut ListCell = 0 as *mut ListCell;
                        let mut l2: *mut ListCell = 0 as *mut ListCell;
                        let mut l3: *mut ListCell = 0 as *mut ListCell;
                        let mut attnum: libc::c_int = atts_done;
                        let mut l1__state: ForThreeState = {
                            let mut init = ForThreeState {
                                l1: (*rtfunc).funccoltypes,
                                l2: (*rtfunc).funccoltypmods,
                                l3: (*rtfunc).funccolcollations,
                                i: 0 as libc::c_int,
                            };
                            init
                        };
                        loop {
                            l1 = (if !(l1__state.l1).is_null()
                                && l1__state.i < (*l1__state.l1).length
                            {
                                &mut *((*l1__state.l1).elements).offset(l1__state.i as isize)
                                    as *mut ListCell
                            } else {
                                0 as *mut ListCell
                            });
                            l2 = (if !(l1__state.l2).is_null()
                                && l1__state.i < (*l1__state.l2).length
                            {
                                &mut *((*l1__state.l2).elements).offset(l1__state.i as isize)
                                    as *mut ListCell
                            } else {
                                0 as *mut ListCell
                            });
                            l3 = (if !(l1__state.l3).is_null()
                                && l1__state.i < (*l1__state.l3).length
                            {
                                &mut *((*l1__state.l3).elements).offset(l1__state.i as isize)
                                    as *mut ListCell
                            } else {
                                0 as *mut ListCell
                            });
                            if !(!l1.is_null() && !l2.is_null() && !l3.is_null()) {
                                break;
                            }
                            let mut attrtype: Oid = (*l1).oid_value;
                            let mut attrtypmod: i32 = (*l2).int_value;
                            let mut attrcollation: Oid = (*l3).oid_value;
                            let mut varnode_1: *mut Var = 0 as *mut Var;
                            attnum += 1;
                            attnum;
                            varnode_1 = makeVar(
                                rtindex as Index,
                                attnum as AttrNumber,
                                attrtype,
                                attrtypmod,
                                attrcollation,
                                sublevels_up as Index,
                            );
                            (*varnode_1).location = location;
                            *colvars = lappend(*colvars, varnode_1 as *mut libc::c_void);
                            l1__state.i += 1;
                            l1__state.i;
                        }
                    }
                } else {
                    let elevel_: libc::c_int = 21 as libc::c_int;
                    let mut __error: libc::c_int = 0;
                    if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                        errmsg_internal(
                            b"function in FROM has unsupported return type\0" as *const u8
                                as *const libc::c_char,
                        );
                        errfinish(
                            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                                as *const u8 as *const libc::c_char,
                            2735 as libc::c_int,
                            0 as *const libc::c_char,
                        );
                    }
                    if elevel_ >= 21 as libc::c_int {
                        abort();
                    }
                }
                atts_done += (*rtfunc).funccolcount;
                lc__state.i += 1;
                lc__state.i;
            }
            if (*rte).funcordinality != 0 {
                if !colnames.is_null() {
                    *colnames = lappend(
                        *colnames,
                        (*list_last_cell((*(*rte).eref).colnames)).ptr_value,
                    );
                }
                if !colvars.is_null() {
                    let mut varnode_2: *mut Var = 0 as *mut Var;
                    *colvars = lappend(*colvars, varnode_2 as *mut libc::c_void);
                }
            }
        }
        2 => {
            let mut colname: *mut ListCell = 0 as *mut ListCell;
            let mut aliasvar: *mut ListCell = 0 as *mut ListCell;
            varattno = 0 as libc::c_int;
            let mut colname__state: ForBothState = {
                let mut init = ForBothState {
                    l1: (*(*rte).eref).colnames,
                    l2: (*rte).joinaliasvars,
                    i: 0 as libc::c_int,
                };
                init
            };
            loop {
                colname = (if !(colname__state.l1).is_null()
                    && colname__state.i < (*colname__state.l1).length
                {
                    &mut *((*colname__state.l1).elements).offset(colname__state.i as isize)
                        as *mut ListCell
                } else {
                    0 as *mut ListCell
                });
                aliasvar = (if !(colname__state.l2).is_null()
                    && colname__state.i < (*colname__state.l2).length
                {
                    &mut *((*colname__state.l2).elements).offset(colname__state.i as isize)
                        as *mut ListCell
                } else {
                    0 as *mut ListCell
                });
                if !(!colname.is_null() && !aliasvar.is_null()) {
                    break;
                }
                let mut avar: *mut Node = (*aliasvar).ptr_value as *mut Node;
                varattno += 1;
                varattno;
                if avar.is_null() {
                    if include_dropped != 0 {
                        if !colnames.is_null() {
                            *colnames = lappend(
                                *colnames,
                                makeString(pstrdup(b"\0" as *const u8 as *const libc::c_char))
                                    as *mut libc::c_void,
                            );
                        }
                        !colvars.is_null();
                    }
                } else {
                    if !colnames.is_null() {
                        let mut label_0: *mut libc::c_char =
                            (*((*colname).ptr_value as *mut Value)).val.str_0;
                        *colnames =
                            lappend(*colnames, makeString(pstrdup(label_0)) as *mut libc::c_void);
                    }
                    if !colvars.is_null() {
                        let mut varnode_3: *mut Var = 0 as *mut Var;
                        if (*(avar as *const Node)).type_0 as libc::c_uint
                            == T_Var as libc::c_int as libc::c_uint
                        {
                            varnode_3 =
                                copyObjectImpl(avar as *mut Var as *const libc::c_void) as *mut Var;
                            (*varnode_3).varlevelsup = sublevels_up as Index;
                        } else {
                            varnode_3 = makeVar(
                                rtindex as Index,
                                varattno as AttrNumber,
                                exprType(avar),
                                exprTypmod(avar),
                                exprCollation(avar),
                                sublevels_up as Index,
                            );
                        }
                        (*varnode_3).location = location;
                        *colvars = lappend(*colvars, varnode_3 as *mut libc::c_void);
                    }
                }
                colname__state.i += 1;
                colname__state.i;
            }
        }
        4 | 5 | 6 | 7 => {
            let mut aliasp_item_0: *mut ListCell = list_head((*(*rte).eref).colnames);
            let mut lct: *mut ListCell = 0 as *mut ListCell;
            let mut lcm: *mut ListCell = 0 as *mut ListCell;
            let mut lcc: *mut ListCell = 0 as *mut ListCell;
            varattno = 0 as libc::c_int;
            let mut lct__state: ForThreeState = {
                let mut init = ForThreeState {
                    l1: (*rte).coltypes,
                    l2: (*rte).coltypmods,
                    l3: (*rte).colcollations,
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
                if !(!lct.is_null() && !lcm.is_null() && !lcc.is_null()) {
                    break;
                }
                let mut coltype: Oid = (*lct).oid_value;
                let mut coltypmod: i32 = (*lcm).int_value;
                let mut colcoll: Oid = (*lcc).oid_value;
                varattno += 1;
                varattno;
                if !colnames.is_null() {
                    if (coltype != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
                        let mut label_1: *mut libc::c_char =
                            (*((*aliasp_item_0).ptr_value as *mut Value)).val.str_0;
                        *colnames =
                            lappend(*colnames, makeString(pstrdup(label_1)) as *mut libc::c_void);
                    } else if include_dropped != 0 {
                        *colnames = lappend(
                            *colnames,
                            makeString(pstrdup(b"\0" as *const u8 as *const libc::c_char))
                                as *mut libc::c_void,
                        );
                    }
                    aliasp_item_0 = lnext((*(*rte).eref).colnames, aliasp_item_0);
                }
                if !colvars.is_null() {
                    if (coltype != 0 as libc::c_int as Oid) as libc::c_int as bool != 0 {
                        let mut varnode_4: *mut Var = 0 as *mut Var;
                        varnode_4 = makeVar(
                            rtindex as Index,
                            varattno as AttrNumber,
                            coltype,
                            coltypmod,
                            colcoll,
                            sublevels_up as Index,
                        );
                        (*varnode_4).location = location;
                        *colvars = lappend(*colvars, varnode_4 as *mut libc::c_void);
                    } else {
                        include_dropped != 0;
                    }
                }
                lct__state.i += 1;
                lct__state.i;
            }
        }
        8 => {}
        _ => {
            let elevel__0: libc::c_int = 21 as libc::c_int;
            let mut __error_0: libc::c_int = 0;
            if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unrecognized RTE kind: %d\0" as *const u8 as *const libc::c_char,
                    (*rte).rtekind as libc::c_int,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                        as *const u8 as *const libc::c_char,
                    2915 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel__0 >= 21 as libc::c_int {
                abort();
            }
        }
    };
}
unsafe extern "C" fn expandRelation(
    mut relid: Oid,
    mut eref: *mut Alias,
    mut rtindex: libc::c_int,
    mut sublevels_up: libc::c_int,
    mut location: libc::c_int,
    mut include_dropped: bool,
    mut colnames: *mut *mut List,
    mut colvars: *mut *mut List,
) {
    let mut rel: Relation = 0 as *mut RelationData;
    rel = relation_open(relid, 1 as libc::c_int);
    expandTupleDesc(
        (*rel).rd_att,
        eref,
        (*(*rel).rd_att).natts,
        0 as libc::c_int,
        rtindex,
        sublevels_up,
        location,
        include_dropped,
        colnames,
        colvars,
    );
    relation_close(rel, 1 as libc::c_int);
}
unsafe extern "C" fn expandTupleDesc(
    mut tupdesc: TupleDesc,
    mut eref: *mut Alias,
    mut count: libc::c_int,
    mut offset: libc::c_int,
    mut rtindex: libc::c_int,
    mut sublevels_up: libc::c_int,
    mut location: libc::c_int,
    mut include_dropped: bool,
    mut colnames: *mut *mut List,
    mut colvars: *mut *mut List,
) {
    let mut aliascell: *mut ListCell = 0 as *mut ListCell;
    let mut varattno: libc::c_int = 0;
    aliascell = if offset < list_length((*eref).colnames) {
        list_nth_cell((*eref).colnames, offset)
    } else {
        0 as *mut ListCell
    };
    varattno = 0 as libc::c_int;
    while varattno < count {
        let mut attr: Form_pg_attribute =
            &mut *((*tupdesc).attrs).as_mut_ptr().offset(varattno as isize)
                as *mut FormData_pg_attribute;
        if (*attr).attisdropped != 0 {
            if include_dropped != 0 {
                if !colnames.is_null() {
                    *colnames = lappend(
                        *colnames,
                        makeString(pstrdup(b"\0" as *const u8 as *const libc::c_char))
                            as *mut libc::c_void,
                    );
                }
                !colvars.is_null();
            }
            if !aliascell.is_null() {
                aliascell = lnext((*eref).colnames, aliascell);
            }
        } else {
            if !colnames.is_null() {
                let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
                if !aliascell.is_null() {
                    label = (*((*aliascell).ptr_value as *mut Value)).val.str_0;
                    aliascell = lnext((*eref).colnames, aliascell);
                } else {
                    label = ((*attr).attname.data).as_mut_ptr();
                }
                *colnames = lappend(*colnames, makeString(pstrdup(label)) as *mut libc::c_void);
            }
            if !colvars.is_null() {
                let mut varnode: *mut Var = 0 as *mut Var;
                varnode = makeVar(
                    rtindex as Index,
                    (varattno + offset + 1 as libc::c_int) as AttrNumber,
                    (*attr).atttypid,
                    (*attr).atttypmod,
                    (*attr).attcollation,
                    sublevels_up as Index,
                );
                (*varnode).location = location;
                *colvars = lappend(*colvars, varnode as *mut libc::c_void);
            }
        }
        varattno += 1;
        varattno;
    }
}
#[no_mangle]
pub unsafe extern "C" fn expandNSItemVars(
    mut nsitem: *mut ParseNamespaceItem,
    mut sublevels_up: libc::c_int,
    mut location: libc::c_int,
    mut colnames: *mut *mut List,
) -> *mut List {
    let mut result: *mut List = 0 as *mut libc::c_void as *mut List;
    let mut colindex: libc::c_int = 0;
    let mut lc: *mut ListCell = 0 as *mut ListCell;
    if !colnames.is_null() {
        *colnames = 0 as *mut libc::c_void as *mut List;
    }
    colindex = 0 as libc::c_int;
    let mut lc__state: ForEachState = {
        let mut init = ForEachState {
            l: (*(*(*nsitem).p_rte).eref).colnames,
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
        let mut colnameval: *mut Value = (*lc).ptr_value as *mut Value;
        let mut colname: *const libc::c_char = (*colnameval).val.str_0;
        let mut nscol: *mut ParseNamespaceColumn =
            ((*nsitem).p_nscolumns).offset(colindex as isize);
        if !((*nscol).p_dontexpand != 0) {
            if *colname.offset(0 as libc::c_int as isize) != 0 {
                let mut var: *mut Var = 0 as *mut Var;
                var = makeVar(
                    (*nscol).p_varno,
                    (*nscol).p_varattno,
                    (*nscol).p_vartype,
                    (*nscol).p_vartypmod,
                    (*nscol).p_varcollid,
                    sublevels_up as Index,
                );
                (*var).varnosyn = (*nscol).p_varnosyn;
                (*var).varattnosyn = (*nscol).p_varattnosyn;
                (*var).location = location;
                result = lappend(result, var as *mut libc::c_void);
                if !colnames.is_null() {
                    *colnames = lappend(*colnames, colnameval as *mut libc::c_void);
                }
            }
        }
        colindex += 1;
        colindex;
        lc__state.i += 1;
        lc__state.i;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn expandNSItemAttrs(
    mut pstate: *mut ParseState,
    mut nsitem: *mut ParseNamespaceItem,
    mut sublevels_up: libc::c_int,
    mut location: libc::c_int,
) -> *mut List {
    let mut rte: *mut RangeTblEntry = (*nsitem).p_rte;
    let mut names: *mut List = 0 as *mut List;
    let mut vars: *mut List = 0 as *mut List;
    let mut name: *mut ListCell = 0 as *mut ListCell;
    let mut var: *mut ListCell = 0 as *mut ListCell;
    let mut te_list: *mut List = 0 as *mut libc::c_void as *mut List;
    vars = expandNSItemVars(nsitem, sublevels_up, location, &mut names);
    if (*rte).rtekind as libc::c_uint == RTE_RELATION as libc::c_int as libc::c_uint {
        (*rte).requiredPerms |= ((1 as libc::c_int) << 1 as libc::c_int) as AclMode;
    }
    let mut name__state: ForBothState = {
        let mut init = ForBothState {
            l1: names,
            l2: vars,
            i: 0 as libc::c_int,
        };
        init
    };
    loop {
        name = (if !(name__state.l1).is_null() && name__state.i < (*name__state.l1).length {
            &mut *((*name__state.l1).elements).offset(name__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        var = (if !(name__state.l2).is_null() && name__state.i < (*name__state.l2).length {
            &mut *((*name__state.l2).elements).offset(name__state.i as isize) as *mut ListCell
        } else {
            0 as *mut ListCell
        });
        if !(!name.is_null() && !var.is_null()) {
            break;
        }
        let mut label: *mut libc::c_char = (*((*name).ptr_value as *mut Value)).val.str_0;
        let mut varnode: *mut Var = (*var).ptr_value as *mut Var;
        let mut te: *mut TargetEntry = 0 as *mut TargetEntry;
        let fresh3 = (*pstate).p_next_resno;
        (*pstate).p_next_resno = (*pstate).p_next_resno + 1;
        te = makeTargetEntry(varnode as *mut Expr, fresh3 as AttrNumber, label, false);
        te_list = lappend(te_list, te as *mut libc::c_void);
        markVarForSelectPriv(pstate, varnode);
        name__state.i += 1;
        name__state.i;
    }
    return te_list;
}
#[no_mangle]
pub unsafe extern "C" fn get_rte_attribute_name(
    mut rte: *mut RangeTblEntry,
    mut attnum: AttrNumber,
) -> *mut libc::c_char {
    if attnum as libc::c_int == 0 as libc::c_int {
        return b"*\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if !((*rte).alias).is_null()
        && attnum as libc::c_int > 0 as libc::c_int
        && attnum as libc::c_int <= list_length((*(*rte).alias).colnames)
    {
        return (*(list_nth(
            (*(*rte).alias).colnames,
            attnum as libc::c_int - 1 as libc::c_int,
        ) as *mut Value))
            .val
            .str_0;
    }
    if (*rte).rtekind as libc::c_uint == RTE_RELATION as libc::c_int as libc::c_uint {
        return get_attname((*rte).relid, attnum, false);
    }
    if attnum as libc::c_int > 0 as libc::c_int
        && attnum as libc::c_int <= list_length((*(*rte).eref).colnames)
    {
        return (*(list_nth(
            (*(*rte).eref).colnames,
            attnum as libc::c_int - 1 as libc::c_int,
        ) as *mut Value))
            .val
            .str_0;
    }
    let elevel_: libc::c_int = 21 as libc::c_int;
    let mut __error: libc::c_int = 0;
    if errstart(elevel_, 0 as *const libc::c_char) != 0 {
        errmsg_internal(
            b"invalid attnum %d for rangetable entry %s\0" as *const u8 as *const libc::c_char,
            attnum as libc::c_int,
            (*(*rte).eref).aliasname,
        );
        errfinish(
            b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                as *const u8 as *const libc::c_char,
            3173 as libc::c_int,
            0 as *const libc::c_char,
        );
    }
    if elevel_ >= 21 as libc::c_int {
        abort();
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_rte_attribute_is_dropped(
    mut rte: *mut RangeTblEntry,
    mut attnum: AttrNumber,
) -> bool {
    let mut result: bool = false;
    match (*rte).rtekind as libc::c_uint {
        0 => {
            let mut tp: HeapTuple = 0 as *mut HeapTupleData;
            let mut att_tup: Form_pg_attribute = 0 as *mut FormData_pg_attribute;
            tp = SearchSysCache2(
                ATTNUM as libc::c_int,
                (*rte).relid as Datum,
                attnum as Datum,
            );
            if (tp as *const libc::c_void).is_null() {
                let elevel_: libc::c_int = 21 as libc::c_int;
                let mut __error: libc::c_int = 0;
                if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"cache lookup failed for attribute %d of relation %u\0" as *const u8
                            as *const libc::c_char,
                        attnum as libc::c_int,
                        (*rte).relid,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                            as *const u8 as *const libc::c_char,
                        3201 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel_ >= 21 as libc::c_int {
                    abort();
                }
            }
            att_tup = ((*tp).t_data as *mut libc::c_char)
                .offset((*(*tp).t_data).t_hoff as libc::c_int as isize)
                as Form_pg_attribute;
            result = (*att_tup).attisdropped;
            ReleaseSysCache(tp);
        }
        1 | 4 | 5 | 6 => {
            result = false;
        }
        7 => {
            if attnum as libc::c_int <= 0 as libc::c_int
                || attnum as libc::c_int > list_length((*rte).coltypes)
            {
                let elevel__0: libc::c_int = 21 as libc::c_int;
                let mut __error_0: libc::c_int = 0;
                if errstart(elevel__0, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"invalid varattno %d\0" as *const u8 as *const libc::c_char,
                        attnum as libc::c_int,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                            as *const u8 as *const libc::c_char,
                        3223 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel__0 >= 21 as libc::c_int {
                    abort();
                }
            }
            result = ((list_nth_oid((*rte).coltypes, attnum as libc::c_int - 1 as libc::c_int)
                != 0 as libc::c_int as Oid) as libc::c_int as bool
                == 0) as libc::c_int as bool;
        }
        2 => {
            let mut aliasvar: *mut Var = 0 as *mut Var;
            if attnum as libc::c_int <= 0 as libc::c_int
                || attnum as libc::c_int > list_length((*rte).joinaliasvars)
            {
                let elevel__1: libc::c_int = 21 as libc::c_int;
                let mut __error_1: libc::c_int = 0;
                if errstart(elevel__1, 0 as *const libc::c_char) != 0 {
                    errmsg_internal(
                        b"invalid varattno %d\0" as *const u8 as *const libc::c_char,
                        attnum as libc::c_int,
                    );
                    errfinish(
                        b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                            as *const u8 as *const libc::c_char,
                        3240 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                if elevel__1 >= 21 as libc::c_int {
                    abort();
                }
            }
            aliasvar = list_nth(
                (*rte).joinaliasvars,
                attnum as libc::c_int - 1 as libc::c_int,
            ) as *mut Var;
            result = (aliasvar == 0 as *mut libc::c_void as *mut Var) as libc::c_int as bool;
        }
        3 => {
            let mut lc: *mut ListCell = 0 as *mut ListCell;
            let mut atts_done: libc::c_int = 0 as libc::c_int;
            let mut lc__state: ForEachState = {
                let mut init = ForEachState {
                    l: (*rte).functions,
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
                let mut rtfunc: *mut RangeTblFunction = (*lc).ptr_value as *mut RangeTblFunction;
                if attnum as libc::c_int > atts_done
                    && attnum as libc::c_int <= atts_done + (*rtfunc).funccolcount
                {
                    let mut tupdesc: TupleDesc = 0 as *mut TupleDescData;
                    tupdesc = get_expr_result_tupdesc((*rtfunc).funcexpr, true);
                    if !tupdesc.is_null() {
                        let mut att_tup_0: Form_pg_attribute = 0 as *mut FormData_pg_attribute;
                        att_tup_0 = &mut *((*tupdesc).attrs)
                            .as_mut_ptr()
                            .offset((attnum as libc::c_int - atts_done - 1 as libc::c_int) as isize)
                            as *mut FormData_pg_attribute;
                        return (*att_tup_0).attisdropped;
                    }
                    return false;
                }
                atts_done += (*rtfunc).funccolcount;
                lc__state.i += 1;
                lc__state.i;
            }
            if (*rte).funcordinality as libc::c_int != 0
                && attnum as libc::c_int == atts_done + 1 as libc::c_int
            {
                return false;
            }
            let elevel__2: libc::c_int = 21 as libc::c_int;
            let mut __error_2: libc::c_int = 0;
            if elevel__2 >= 21 as libc::c_int {
                abort();
            }
            result = false;
        }
        8 => {
            let elevel__3: libc::c_int = 21 as libc::c_int;
            let mut __error_3: libc::c_int = 0;
            if elevel__3 >= 21 as libc::c_int {
                abort();
            }
            result = false;
        }
        _ => {
            let elevel__4: libc::c_int = 21 as libc::c_int;
            let mut __error_4: libc::c_int = 0;
            if errstart(elevel__4, 0 as *const libc::c_char) != 0 {
                errmsg_internal(
                    b"unrecognized RTE kind: %d\0" as *const u8 as *const libc::c_char,
                    (*rte).rtekind as libc::c_int,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                        as *const u8 as *const libc::c_char,
                    3310 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel__4 >= 21 as libc::c_int {
                abort();
            }
            result = false;
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn get_tle_by_resno(
    mut tlist: *mut List,
    mut resno: AttrNumber,
) -> *mut TargetEntry {
    let mut l: *mut ListCell = 0 as *mut ListCell;
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
        let mut tle: *mut TargetEntry = (*l).ptr_value as *mut TargetEntry;
        if (*tle).resno as libc::c_int == resno as libc::c_int {
            return tle;
        }
        l__state.i += 1;
        l__state.i;
    }
    return 0 as *mut TargetEntry;
}
#[no_mangle]
pub unsafe extern "C" fn get_parse_rowmark(
    mut qry: *mut Query,
    mut rtindex: Index,
) -> *mut RowMarkClause {
    let mut l: *mut ListCell = 0 as *mut ListCell;
    let mut l__state: ForEachState = {
        let mut init = ForEachState {
            l: (*qry).rowMarks,
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
        let mut rc: *mut RowMarkClause = (*l).ptr_value as *mut RowMarkClause;
        if (*rc).rti == rtindex {
            return rc;
        }
        l__state.i += 1;
        l__state.i;
    }
    return 0 as *mut RowMarkClause;
}
#[no_mangle]
pub unsafe extern "C" fn attnameAttNum(
    mut rd: Relation,
    mut attname: *const libc::c_char,
    mut sysColOK: bool,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*(*rd).rd_rel).relnatts as libc::c_int {
        let mut att: Form_pg_attribute =
            &mut *((*(*rd).rd_att).attrs).as_mut_ptr().offset(i as isize)
                as *mut FormData_pg_attribute;
        if namestrcmp(&mut (*att).attname, attname) == 0 as libc::c_int && (*att).attisdropped == 0
        {
            return i + 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    if sysColOK != 0 {
        i = specialAttNum(attname);
        if i != 0 as libc::c_int {
            return i;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn specialAttNum(mut attname: *const libc::c_char) -> libc::c_int {
    let mut sysatt: *const FormData_pg_attribute = 0 as *const FormData_pg_attribute;
    sysatt = SystemAttributeByName(attname);
    if !sysatt.is_null() {
        return (*sysatt).attnum as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn attnumAttName(
    mut rd: Relation,
    mut attid: libc::c_int,
) -> *const NameData {
    if attid <= 0 as libc::c_int {
        let mut sysatt: *const FormData_pg_attribute = 0 as *const FormData_pg_attribute;
        sysatt = SystemAttributeDefinition(attid as AttrNumber);
        return &(*sysatt).attname;
    }
    if attid > (*(*rd).rd_att).natts {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"invalid attribute number %d\0" as *const u8 as *const libc::c_char,
                attid,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                    as *const u8 as *const libc::c_char,
                3430 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return &mut (*((*(*rd).rd_att).attrs)
        .as_mut_ptr()
        .offset((attid - 1 as libc::c_int) as isize))
    .attname;
}
#[no_mangle]
pub unsafe extern "C" fn attnumTypeId(mut rd: Relation, mut attid: libc::c_int) -> Oid {
    if attid <= 0 as libc::c_int {
        let mut sysatt: *const FormData_pg_attribute = 0 as *const FormData_pg_attribute;
        sysatt = SystemAttributeDefinition(attid as AttrNumber);
        return (*sysatt).atttypid;
    }
    if attid > (*(*rd).rd_att).natts {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"invalid attribute number %d\0" as *const u8 as *const libc::c_char,
                attid,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                    as *const u8 as *const libc::c_char,
                3452 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return (*((*(*rd).rd_att).attrs)
        .as_mut_ptr()
        .offset((attid - 1 as libc::c_int) as isize))
    .atttypid;
}
#[no_mangle]
pub unsafe extern "C" fn attnumCollationId(mut rd: Relation, mut attid: libc::c_int) -> Oid {
    if attid <= 0 as libc::c_int {
        return 0 as libc::c_int as Oid;
    }
    if attid > (*(*rd).rd_att).natts {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if errstart(elevel_, 0 as *const libc::c_char) != 0 {
            errmsg_internal(
                b"invalid attribute number %d\0" as *const u8 as *const libc::c_char,
                attid,
            );
            errfinish(
                b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/parse_relation.c\0"
                    as *const u8 as *const libc::c_char,
                3470 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    }
    return (*((*(*rd).rd_att).attrs)
        .as_mut_ptr()
        .offset((attid - 1 as libc::c_int) as isize))
    .attcollation;
}
#[no_mangle]
pub unsafe extern "C" fn errorMissingRTE(
    mut pstate: *mut ParseState,
    mut relation: *mut RangeVar,
) -> ! {
    let mut rte: *mut RangeTblEntry = 0 as *mut RangeTblEntry;
    let mut badAlias: *const libc::c_char = 0 as *const libc::c_char;
    rte = searchRangeTableForRel(pstate, relation);
    if !rte.is_null()
        && !((*rte).alias).is_null()
        && strcmp((*(*rte).eref).aliasname, (*relation).relname) != 0 as libc::c_int
    {
        let mut nsitem: *mut ParseNamespaceItem = 0 as *mut ParseNamespaceItem;
        let mut sublevels_up: libc::c_int = 0;
        nsitem = refnameNamespaceItem(
            pstate,
            0 as *const libc::c_char,
            (*(*rte).eref).aliasname,
            (*relation).location,
            &mut sublevels_up,
        );
        if !nsitem.is_null() && (*nsitem).p_rte == rte {
            badAlias = (*(*rte).eref).aliasname;
        }
    }
    if !rte.is_null() {
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
pub unsafe extern "C" fn errorMissingColumn(
    mut pstate: *mut ParseState,
    mut relname: *const libc::c_char,
    mut colname: *const libc::c_char,
    mut location: libc::c_int,
) -> ! {
    let mut state: *mut FuzzyAttrMatchState = 0 as *mut FuzzyAttrMatchState;
    let mut closestfirst: *mut libc::c_char = 0 as *mut libc::c_char;
    state = searchRangeTableForCol(pstate, relname, colname, location);
    if !((*state).rfirst).is_null()
        && ((*state).first as libc::c_int != 0 as libc::c_int) as libc::c_int as bool as libc::c_int
            != 0
    {
        closestfirst = (*(list_nth(
            (*(*(*state).rfirst).eref).colnames,
            (*state).first as libc::c_int - 1 as libc::c_int,
        ) as *mut Value))
            .val
            .str_0;
    }
    if ((*state).rsecond).is_null() {
        let elevel_: libc::c_int = 21 as libc::c_int;
        let mut __error: libc::c_int = 0;
        if elevel_ >= 21 as libc::c_int {
            abort();
        }
    } else {
        let mut closestsecond: *mut libc::c_char = 0 as *mut libc::c_char;
        closestsecond = (*(list_nth(
            (*(*(*state).rsecond).eref).colnames,
            (*state).second as libc::c_int - 1 as libc::c_int,
        ) as *mut Value))
            .val
            .str_0;
        let elevel__0: libc::c_int = 21 as libc::c_int;
        let mut __error_0: libc::c_int = 0;
        if elevel__0 >= 21 as libc::c_int {
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn isQueryUsingTempRelation(mut query: *mut Query) -> bool {
    return isQueryUsingTempRelation_walker(query as *mut Node, 0 as *mut libc::c_void);
}
