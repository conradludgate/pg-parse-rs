#![feature(extern_types, linkage)]

pub type Oid = u32;

extern "C" {
    pub type MemoryContextData;
    pub type AttrMissing;
    pub type dsa_area;
    pub type TIDBitmap;
    pub type PgStat_TableStatus;
    pub type FdwRoutine;
    pub type GlobalVisState;
    pub type PartitionBoundInfoData;
    pub type HTAB;
    pub type PartitionDirectoryData;
    pub type QueryEnvironment;
    pub type JitInstrumentation;
    pub type JitContext;
    pub type CopyMultiInsertBuffer;
    pub type SharedJitInstrumentation;
    pub type ExprEvalStep;
    pub type BufferAccessStrategyData;
    pub type ValidateIndexState;
    pub type VacuumParams;
    pub type BulkInsertStateData;
    pub type PartitionDescData;
    pub type PartitionKeyData;
    pub type RowSecurityDesc;
    pub type SMgrRelationData;
    fn abort() -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
    fn errfinish(filename: *const libc::c_char, lineno: libc::c_int, funcname: *const libc::c_char);
    fn errmsg_internal(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn palloc(size: usize) -> *mut libc::c_void;
    fn palloc0(size: usize) -> *mut libc::c_void;
    fn pfree(pointer: *mut libc::c_void);
    fn pstrdup(in_0: *const libc::c_char) -> *mut libc::c_char;
    fn list_make1_impl(t: NodeTag, datum1: ListCell) -> *mut List;
    fn list_make2_impl(t: NodeTag, datum1: ListCell, datum2: ListCell) -> *mut List;
    fn lappend(list: *mut List, datum: *mut libc::c_void) -> *mut List;
    fn lappend_int(list: *mut List, datum: libc::c_int) -> *mut List;
    fn lappend_oid(list: *mut List, datum: Oid) -> *mut List;
    fn list_concat(list1: *mut List, list2: *const List) -> *mut List;
    fn list_truncate(list: *mut List, new_size: libc::c_int) -> *mut List;
    fn list_member_int(list: *const List, datum: libc::c_int) -> bool;
    fn copyObjectImpl(obj: *const libc::c_void) -> *mut libc::c_void;
    fn equal(a: *const libc::c_void, b: *const libc::c_void) -> bool;
    fn bms_is_member(x: libc::c_int, a: *const Bitmapset) -> bool;
    fn bms_add_member(a: *mut Bitmapset, x: libc::c_int) -> *mut Bitmapset;
    fn bms_add_members(a: *mut Bitmapset, b: *const Bitmapset) -> *mut Bitmapset;
    fn makeString(str: *mut libc::c_char) -> *mut Value;
    fn table_close(relation: Relation, lockmode: LOCKMODE);
    fn GetTsmRoutine(tsmhandler: Oid) -> *mut TsmRoutine;
    fn IsCatalogRelation(relation: Relation) -> bool;
    fn setup_parser_errposition_callback(
        pcbstate: *mut ParseCallbackState,
        pstate: *mut ParseState,
        location: libc::c_int,
    );
    fn cancel_parser_errposition_callback(pcbstate: *mut ParseCallbackState);
    fn get_relation_constraint_attnos(
        relid: Oid,
        conname: *const libc::c_char,
        missing_ok: bool,
        constraintOid: *mut Oid,
    ) -> *mut Bitmapset;
    fn check_stack_depth();
    fn makeSimpleA_Expr(
        kind: A_Expr_Kind,
        name: *mut libc::c_char,
        lexpr: *mut Node,
        rexpr: *mut Node,
        location: libc::c_int,
    ) -> *mut A_Expr;
    fn makeVar(
        varno: Index,
        varattno: AttrNumber,
        vartype: Oid,
        vartypmod: i32,
        varcollid: Oid,
        varlevelsup: Index,
    ) -> *mut Var;
    fn makeBoolExpr(boolop: BoolExprType, args: *mut List, location: libc::c_int) -> *mut Expr;
    fn makeRelabelType(
        arg: *mut Expr,
        rtype: Oid,
        rtypmod: i32,
        rcollid: Oid,
        rformat: CoercionForm,
    ) -> *mut RelabelType;
    fn makeFuncCall(
        name: *mut List,
        args: *mut List,
        funcformat: CoercionForm,
        location: libc::c_int,
    ) -> *mut FuncCall;
    fn makeGroupingSet(
        kind: GroupingSetKind,
        content: *mut List,
        location: libc::c_int,
    ) -> *mut GroupingSet;
    fn exprType(expr: *const Node) -> Oid;
    fn exprTypmod(expr: *const Node) -> i32;
    fn strip_implicit_coercions(node: *mut Node) -> *mut Node;
    fn exprCollation(expr: *const Node) -> Oid;
    fn exprLocation(expr: *const Node) -> libc::c_int;
    fn get_sortgroupref_tle(sortref: Index, targetList: *mut List) -> *mut TargetEntry;
    fn get_sortgroupclause_tle(
        sgClause: *mut SortGroupClause,
        targetList: *mut List,
    ) -> *mut TargetEntry;
    fn get_sortgroupclause_expr(sgClause: *mut SortGroupClause, targetList: *mut List)
        -> *mut Node;
    fn contain_vars_of_level(node: *mut Node, levelsup: libc::c_int) -> bool;
    fn parse_sub_analyze(
        parseTree: *mut Node,
        parentParseState: *mut ParseState,
        parentCTE: *mut CommonTableExpr,
        locked_from_parent: bool,
        resolve_unknowns: bool,
    ) -> *mut Query;
    fn can_coerce_type(
        nargs: libc::c_int,
        input_typeids: *const Oid,
        target_typeids: *const Oid,
        ccontext: CoercionContext,
    ) -> bool;
    fn coerce_type(
        pstate: *mut ParseState,
        node: *mut Node,
        inputTypeId: Oid,
        targetTypeId: Oid,
        targetTypeMod: i32,
        ccontext: CoercionContext,
        cformat: CoercionForm,
        location: libc::c_int,
    ) -> *mut Node;
    fn coerce_to_boolean(
        pstate: *mut ParseState,
        node: *mut Node,
        constructName: *const libc::c_char,
    ) -> *mut Node;
    fn coerce_to_specific_type(
        pstate: *mut ParseState,
        node: *mut Node,
        targetTypeId: Oid,
        constructName: *const libc::c_char,
    ) -> *mut Node;
    fn coerce_to_specific_type_typmod(
        pstate: *mut ParseState,
        node: *mut Node,
        targetTypeId: Oid,
        targetTypmod: i32,
        constructName: *const libc::c_char,
    ) -> *mut Node;
    fn select_common_type(
        pstate: *mut ParseState,
        exprs: *mut List,
        context: *const libc::c_char,
        which_expr: *mut *mut Node,
    ) -> Oid;
    fn select_common_typmod(pstate: *mut ParseState, exprs: *mut List, common_type: Oid) -> i32;
    fn assign_list_collations(pstate: *mut ParseState, exprs: *mut List);
    fn assign_expr_collations(pstate: *mut ParseState, expr: *mut Node);
    fn transformExpr(
        pstate: *mut ParseState,
        expr: *mut Node,
        exprKind: ParseExprKind,
    ) -> *mut Node;
    fn LookupFuncName(
        funcname: *mut List,
        nargs: libc::c_int,
        argtypes: *const Oid,
        missing_ok: bool,
    ) -> Oid;
    fn get_sort_group_operators(
        argtype: Oid,
        needLT: bool,
        needEQ: bool,
        needGT: bool,
        ltOpr: *mut Oid,
        eqOpr: *mut Oid,
        gtOpr: *mut Oid,
        isHashable: *mut bool,
    );
    fn compatible_oper_opid(op: *mut List, arg1: Oid, arg2: Oid, noError: bool) -> Oid;
    fn scanNameSpaceForCTE(
        pstate: *mut ParseState,
        refname: *const libc::c_char,
        ctelevelsup: *mut Index,
    ) -> *mut CommonTableExpr;
    fn scanNameSpaceForENR(pstate: *mut ParseState, refname: *const libc::c_char) -> bool;
    fn checkNameSpaceConflicts(
        pstate: *mut ParseState,
        namespace1: *mut List,
        namespace2: *mut List,
    );
    fn colNameToVar(
        pstate: *mut ParseState,
        colname: *const libc::c_char,
        localonly: bool,
        location: libc::c_int,
    ) -> *mut Node;
    fn markVarForSelectPriv(pstate: *mut ParseState, var: *mut Var);
    fn parserOpenTable(
        pstate: *mut ParseState,
        relation: *const RangeVar,
        lockmode: libc::c_int,
    ) -> Relation;
    fn addRangeTableEntry(
        pstate: *mut ParseState,
        relation: *mut RangeVar,
        alias: *mut Alias,
        inh: bool,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForRelation(
        pstate: *mut ParseState,
        rel: Relation,
        lockmode: libc::c_int,
        alias: *mut Alias,
        inh: bool,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForSubquery(
        pstate: *mut ParseState,
        subquery: *mut Query,
        alias: *mut Alias,
        lateral: bool,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForFunction(
        pstate: *mut ParseState,
        funcnames: *mut List,
        funcexprs: *mut List,
        coldeflists: *mut List,
        rangefunc: *mut RangeFunction,
        lateral: bool,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForTableFunc(
        pstate: *mut ParseState,
        tf: *mut TableFunc,
        alias: *mut Alias,
        lateral: bool,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForJoin(
        pstate: *mut ParseState,
        colnames: *mut List,
        nscolumns: *mut ParseNamespaceColumn,
        jointype: JoinType,
        nummergedcols: libc::c_int,
        aliasvars: *mut List,
        leftcols: *mut List,
        rightcols: *mut List,
        alias: *mut Alias,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForCTE(
        pstate: *mut ParseState,
        cte: *mut CommonTableExpr,
        levelsup: Index,
        rv: *mut RangeVar,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn addRangeTableEntryForENR(
        pstate: *mut ParseState,
        rv: *mut RangeVar,
        inFromCl: bool,
    ) -> *mut ParseNamespaceItem;
    fn isLockedRefname(pstate: *mut ParseState, refname: *const libc::c_char) -> bool;
    fn addNSItemToQuery(
        pstate: *mut ParseState,
        nsitem: *mut ParseNamespaceItem,
        addToJoinList: bool,
        addToRelNameSpace: bool,
        addToVarNameSpace: bool,
    );
    fn transformTargetEntry(
        pstate: *mut ParseState,
        node: *mut Node,
        expr: *mut Node,
        exprKind: ParseExprKind,
        colname: *mut libc::c_char,
        resjunk: bool,
    ) -> *mut TargetEntry;
    fn FigureColname(node: *mut Node) -> *mut libc::c_char;
    fn typenameTypeIdAndMod(
        pstate: *mut ParseState,
        typeName: *const TypeName,
        typeid_p: *mut Oid,
        typmod_p: *mut i32,
    );
    fn LookupCollation(pstate: *mut ParseState, collnames: *mut List, location: libc::c_int)
        -> Oid;
    fn SystemFuncName(name: *mut libc::c_char) -> *mut List;
    fn contain_aggs_of_level(node: *mut Node, levelsup: libc::c_int) -> bool;
    fn contain_windowfuncs(node: *mut Node) -> bool;
    fn ReleaseCatCacheList(list: *mut CatCList);
    fn get_ordering_op_properties(
        opno: Oid,
        opfamily: *mut Oid,
        opcintype: *mut Oid,
        strategy: *mut i16,
    ) -> bool;
    fn get_equality_op_for_ordering_op(opno: Oid, reverse: *mut bool) -> Oid;
    fn op_hashjoinable(opno: Oid, inputtype: Oid) -> bool;
    fn get_commutator(opno: Oid) -> Oid;
    fn get_typcollation(typid: Oid) -> Oid;
    fn SearchSysCacheList(
        cacheId: libc::c_int,
        nkeys: libc::c_int,
        key1: Datum,
        key2: Datum,
        key3: Datum,
    ) -> *mut catclist;

    fn errcode(sqlerrcode: i32) -> i32;
}

pub type Datum = usize;
pub type Relation = *mut RelationData;

#[repr(C)]
#[allow(non_camel_case_types)]
enum NodeTag {
    T_Invalid = 0,

    /*
     * TAGS FOR EXECUTOR NODES (execnodes.h)
     */
    T_IndexInfo,
    T_ExprContext,
    T_ProjectionInfo,
    T_JunkFilter,
    T_OnConflictSetState,
    T_ResultRelInfo,
    T_EState,
    T_TupleTableSlot,

    /*
     * TAGS FOR PLAN NODES (plannodes.h)
     */
    T_Plan,
    T_Result,
    T_ProjectSet,
    T_ModifyTable,
    T_Append,
    T_MergeAppend,
    T_RecursiveUnion,
    T_BitmapAnd,
    T_BitmapOr,
    T_Scan,
    T_SeqScan,
    T_SampleScan,
    T_IndexScan,
    T_IndexOnlyScan,
    T_BitmapIndexScan,
    T_BitmapHeapScan,
    T_TidScan,
    T_TidRangeScan,
    T_SubqueryScan,
    T_FunctionScan,
    T_ValuesScan,
    T_TableFuncScan,
    T_CteScan,
    T_NamedTuplestoreScan,
    T_WorkTableScan,
    T_ForeignScan,
    T_CustomScan,
    T_Join,
    T_NestLoop,
    T_MergeJoin,
    T_HashJoin,
    T_Material,
    T_Sort,
    T_IncrementalSort,
    T_Group,
    T_Agg,
    T_WindowAgg,
    T_Unique,
    T_Gather,
    T_GatherMerge,
    T_Hash,
    T_SetOp,
    T_LockRows,
    T_Limit,
    /* these aren't subclasses of Plan: */
    T_NestLoopParam,
    T_PlanRowMark,
    T_PartitionPruneInfo,
    T_PartitionedRelPruneInfo,
    T_PartitionPruneStepOp,
    T_PartitionPruneStepCombine,
    T_PlanInvalItem,

    /*
     * TAGS FOR PLAN STATE NODES (execnodes.h)
     *
     * These should correspond one-to-one with Plan node types.
     */
    T_PlanState,
    T_ResultState,
    T_ProjectSetState,
    T_ModifyTableState,
    T_AppendState,
    T_MergeAppendState,
    T_RecursiveUnionState,
    T_BitmapAndState,
    T_BitmapOrState,
    T_ScanState,
    T_SeqScanState,
    T_SampleScanState,
    T_IndexScanState,
    T_IndexOnlyScanState,
    T_BitmapIndexScanState,
    T_BitmapHeapScanState,
    T_TidScanState,
    T_TidRangeScanState,
    T_SubqueryScanState,
    T_FunctionScanState,
    T_TableFuncScanState,
    T_ValuesScanState,
    T_CteScanState,
    T_NamedTuplestoreScanState,
    T_WorkTableScanState,
    T_ForeignScanState,
    T_CustomScanState,
    T_JoinState,
    T_NestLoopState,
    T_MergeJoinState,
    T_HashJoinState,
    T_MaterialState,
    T_SortState,
    T_IncrementalSortState,
    T_GroupState,
    T_AggState,
    T_WindowAggState,
    T_UniqueState,
    T_GatherState,
    T_GatherMergeState,
    T_HashState,
    T_SetOpState,
    T_LockRowsState,
    T_LimitState,

    /*
     * TAGS FOR PRIMITIVE NODES (primnodes.h)
     */
    T_Alias,
    T_RangeVar,
    T_TableFunc,
    T_Expr,
    T_Var,
    T_Const,
    T_Param,
    T_Aggref,
    T_GroupingFunc,
    T_WindowFunc,
    T_SubscriptingRef,
    T_FuncExpr,
    T_NamedArgExpr,
    T_OpExpr,
    T_DistinctExpr,
    T_NullIfExpr,
    T_ScalarArrayOpExpr,
    T_BoolExpr,
    T_SubLink,
    T_SubPlan,
    T_AlternativeSubPlan,
    T_FieldSelect,
    T_FieldStore,
    T_RelabelType,
    T_CoerceViaIO,
    T_ArrayCoerceExpr,
    T_ConvertRowtypeExpr,
    T_CollateExpr,
    T_CaseExpr,
    T_CaseWhen,
    T_CaseTestExpr,
    T_ArrayExpr,
    T_RowExpr,
    T_RowCompareExpr,
    T_CoalesceExpr,
    T_MinMaxExpr,
    T_SQLValueFunction,
    T_XmlExpr,
    T_NullTest,
    T_BooleanTest,
    T_CoerceToDomain,
    T_CoerceToDomainValue,
    T_SetToDefault,
    T_CurrentOfExpr,
    T_NextValueExpr,
    T_InferenceElem,
    T_TargetEntry,
    T_RangeTblRef,
    T_JoinExpr,
    T_FromExpr,
    T_OnConflictExpr,
    T_IntoClause,

    /*
     * TAGS FOR EXPRESSION STATE NODES (execnodes.h)
     *
     * ExprState represents the evaluation state for a whole expression tree.
     * Most Expr-based plan nodes do not have a corresponding expression state
     * node, they're fully handled within execExpr* - but sometimes the state
     * needs to be shared with other parts of the executor, as for example
     * with SubPlanState, which nodeSubplan.c has to modify.
     */
    T_ExprState,
    T_WindowFuncExprState,
    T_SetExprState,
    T_SubPlanState,
    T_DomainConstraintState,

    /*
     * TAGS FOR PLANNER NODES (pathnodes.h)
     */
    T_PlannerInfo,
    T_PlannerGlobal,
    T_RelOptInfo,
    T_IndexOptInfo,
    T_ForeignKeyOptInfo,
    T_ParamPathInfo,
    T_Path,
    T_IndexPath,
    T_BitmapHeapPath,
    T_BitmapAndPath,
    T_BitmapOrPath,
    T_TidPath,
    T_TidRangePath,
    T_SubqueryScanPath,
    T_ForeignPath,
    T_CustomPath,
    T_NestPath,
    T_MergePath,
    T_HashPath,
    T_AppendPath,
    T_MergeAppendPath,
    T_GroupResultPath,
    T_MaterialPath,
    T_UniquePath,
    T_GatherPath,
    T_GatherMergePath,
    T_ProjectionPath,
    T_ProjectSetPath,
    T_SortPath,
    T_IncrementalSortPath,
    T_GroupPath,
    T_UpperUniquePath,
    T_AggPath,
    T_GroupingSetsPath,
    T_MinMaxAggPath,
    T_WindowAggPath,
    T_SetOpPath,
    T_RecursiveUnionPath,
    T_LockRowsPath,
    T_ModifyTablePath,
    T_LimitPath,
    /* these aren't subclasses of Path: */
    T_EquivalenceClass,
    T_EquivalenceMember,
    T_PathKey,
    T_PathTarget,
    T_RestrictInfo,
    T_IndexClause,
    T_PlaceHolderVar,
    T_SpecialJoinInfo,
    T_AppendRelInfo,
    T_PlaceHolderInfo,
    T_MinMaxAggInfo,
    T_PlannerParamItem,
    T_RollupData,
    T_GroupingSetData,
    T_StatisticExtInfo,

    /*
     * TAGS FOR MEMORY NODES (memnodes.h)
     */
    T_MemoryContext,
    T_AllocSetContext,
    T_SlabContext,
    T_GenerationContext,

    /*
     * TAGS FOR VALUE NODES (value.h)
     */
    T_Value,
    T_Integer,
    T_Float,
    T_String,
    T_BitString,
    T_Null,

    /*
     * TAGS FOR LIST NODES (pg_list.h)
     */
    T_List,
    T_IntList,
    T_OidList,

    /*
     * TAGS FOR EXTENSIBLE NODES (extensible.h)
     */
    T_ExtensibleNode,

    /*
     * TAGS FOR STATEMENT NODES (mostly in parsenodes.h)
     */
    T_RawStmt,
    T_Query,
    T_PlannedStmt,
    T_InsertStmt,
    T_DeleteStmt,
    T_UpdateStmt,
    T_SelectStmt,
    T_PLAssignStmt,
    T_AlterTableStmt,
    T_AlterTableCmd,
    T_AlterDomainStmt,
    T_SetOperationStmt,
    T_GrantStmt,
    T_GrantRoleStmt,
    T_AlterDefaultPrivilegesStmt,
    T_ClosePortalStmt,
    T_ClusterStmt,
    T_CopyStmt,
    T_CreateStmt,
    T_DefineStmt,
    T_DropStmt,
    T_TruncateStmt,
    T_CommentStmt,
    T_FetchStmt,
    T_IndexStmt,
    T_CreateFunctionStmt,
    T_AlterFunctionStmt,
    T_DoStmt,
    T_RenameStmt,
    T_RuleStmt,
    T_NotifyStmt,
    T_ListenStmt,
    T_UnlistenStmt,
    T_TransactionStmt,
    T_ViewStmt,
    T_LoadStmt,
    T_CreateDomainStmt,
    T_CreatedbStmt,
    T_DropdbStmt,
    T_VacuumStmt,
    T_ExplainStmt,
    T_CreateTableAsStmt,
    T_CreateSeqStmt,
    T_AlterSeqStmt,
    T_VariableSetStmt,
    T_VariableShowStmt,
    T_DiscardStmt,
    T_CreateTrigStmt,
    T_CreatePLangStmt,
    T_CreateRoleStmt,
    T_AlterRoleStmt,
    T_DropRoleStmt,
    T_LockStmt,
    T_ConstraintsSetStmt,
    T_ReindexStmt,
    T_CheckPointStmt,
    T_CreateSchemaStmt,
    T_AlterDatabaseStmt,
    T_AlterDatabaseSetStmt,
    T_AlterRoleSetStmt,
    T_CreateConversionStmt,
    T_CreateCastStmt,
    T_CreateOpClassStmt,
    T_CreateOpFamilyStmt,
    T_AlterOpFamilyStmt,
    T_PrepareStmt,
    T_ExecuteStmt,
    T_DeallocateStmt,
    T_DeclareCursorStmt,
    T_CreateTableSpaceStmt,
    T_DropTableSpaceStmt,
    T_AlterObjectDependsStmt,
    T_AlterObjectSchemaStmt,
    T_AlterOwnerStmt,
    T_AlterOperatorStmt,
    T_AlterTypeStmt,
    T_DropOwnedStmt,
    T_ReassignOwnedStmt,
    T_CompositeTypeStmt,
    T_CreateEnumStmt,
    T_CreateRangeStmt,
    T_AlterEnumStmt,
    T_AlterTSDictionaryStmt,
    T_AlterTSConfigurationStmt,
    T_CreateFdwStmt,
    T_AlterFdwStmt,
    T_CreateForeignServerStmt,
    T_AlterForeignServerStmt,
    T_CreateUserMappingStmt,
    T_AlterUserMappingStmt,
    T_DropUserMappingStmt,
    T_AlterTableSpaceOptionsStmt,
    T_AlterTableMoveAllStmt,
    T_SecLabelStmt,
    T_CreateForeignTableStmt,
    T_ImportForeignSchemaStmt,
    T_CreateExtensionStmt,
    T_AlterExtensionStmt,
    T_AlterExtensionContentsStmt,
    T_CreateEventTrigStmt,
    T_AlterEventTrigStmt,
    T_RefreshMatViewStmt,
    T_ReplicaIdentityStmt,
    T_AlterSystemStmt,
    T_CreatePolicyStmt,
    T_AlterPolicyStmt,
    T_CreateTransformStmt,
    T_CreateAmStmt,
    T_CreatePublicationStmt,
    T_AlterPublicationStmt,
    T_CreateSubscriptionStmt,
    T_AlterSubscriptionStmt,
    T_DropSubscriptionStmt,
    T_CreateStatsStmt,
    T_AlterCollationStmt,
    T_CallStmt,
    T_AlterStatsStmt,

    /*
     * TAGS FOR PARSE TREE NODES (parsenodes.h)
     */
    T_A_Expr,
    T_ColumnRef,
    T_ParamRef,
    T_A_Const,
    T_FuncCall,
    T_A_Star,
    T_A_Indices,
    T_A_Indirection,
    T_A_ArrayExpr,
    T_ResTarget,
    T_MultiAssignRef,
    T_TypeCast,
    T_CollateClause,
    T_SortBy,
    T_WindowDef,
    T_RangeSubselect,
    T_RangeFunction,
    T_RangeTableSample,
    T_RangeTableFunc,
    T_RangeTableFuncCol,
    T_TypeName,
    T_ColumnDef,
    T_IndexElem,
    T_Constraint,
    T_DefElem,
    T_RangeTblEntry,
    T_RangeTblFunction,
    T_TableSampleClause,
    T_WithCheckOption,
    T_SortGroupClause,
    T_GroupingSet,
    T_WindowClause,
    T_ObjectWithArgs,
    T_AccessPriv,
    T_CreateOpClassItem,
    T_TableLikeClause,
    T_FunctionParameter,
    T_LockingClause,
    T_RowMarkClause,
    T_XmlSerialize,
    T_WithClause,
    T_InferClause,
    T_OnConflictClause,
    T_CTESearchClause,
    T_CTECycleClause,
    T_CommonTableExpr,
    T_RoleSpec,
    T_TriggerTransition,
    T_PartitionElem,
    T_PartitionSpec,
    T_PartitionBoundSpec,
    T_PartitionRangeDatum,
    T_PartitionCmd,
    T_VacuumRelation,

    /*
     * TAGS FOR REPLICATION GRAMMAR PARSE NODES (replnodes.h)
     */
    T_IdentifySystemCmd,
    T_BaseBackupCmd,
    T_CreateReplicationSlotCmd,
    T_DropReplicationSlotCmd,
    T_StartReplicationCmd,
    T_TimeLineHistoryCmd,
    T_SQLCmd,

    /*
     * TAGS FOR RANDOM OTHER STUFF
     *
     * These are objects that aren't part of parse/plan/execute node tree
     * structures, but we give them NodeTags anyway for identification
     * purposes (usually because they are involved in APIs where we want to
     * pass multiple object types through the same pointer).
     */
    T_TriggerData,                  /* in commands/trigger.h */
    T_EventTriggerData,             /* in commands/event_trigger.h */
    T_ReturnSetInfo,                /* in nodes/execnodes.h */
    T_WindowObjectData,             /* private in nodeWindowAgg.c */
    T_TIDBitmap,                    /* in nodes/tidbitmap.h */
    T_InlineCodeBlock,              /* in nodes/parsenodes.h */
    T_FdwRoutine,                   /* in foreign/fdwapi.h */
    T_IndexAmRoutine,               /* in access/amapi.h */
    T_TableAmRoutine,               /* in access/tableam.h */
    T_TsmRoutine,                   /* in access/tsmapi.h */
    T_ForeignKeyCacheInfo,          /* in utils/rel.h */
    T_CallContext,                  /* in nodes/parsenodes.h */
    T_SupportRequestSimplify,       /* in nodes/supportnodes.h */
    T_SupportRequestSelectivity,    /* in nodes/supportnodes.h */
    T_SupportRequestCost,           /* in nodes/supportnodes.h */
    T_SupportRequestRows,           /* in nodes/supportnodes.h */
    T_SupportRequestIndexCondition, /* in nodes/supportnodes.h */
}
#[repr(C)]
pub struct Node {
    tag: NodeTag,
}

mod analyze;
mod gram;
mod parse_agg;
mod parse_clause;
mod parse_coerce;
mod parse_collate;
mod parse_cte;
mod parse_enr;
mod parse_expr;
mod parse_func;
mod parse_node;
mod parse_oper;
mod parse_param;
mod parse_relation;
mod parse_target;
mod parse_type;
mod parse_utilcmd;
mod parser;
mod scansup;

use analyze::*;
use gram::*;
use parse_agg::*;
use parse_clause::*;
use parse_coerce::*;
use parse_collate::*;
use parse_cte::*;
use parse_enr::*;
use parse_expr::*;
use parse_func::*;
use parse_node::*;
use parse_oper::*;
use parse_param::*;
use parse_relation::*;
use parse_target::*;
use parse_type::*;
use parse_utilcmd::*;
use parser::*;
use scansup::*;
