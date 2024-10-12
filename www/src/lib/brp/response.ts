// deno-fmt-ignore-file
/* eslint-disable @typescript-eslint/no-explicit-any */

interface JsonRpcResponse<TResult> {
    jsonrpc: "2.0";
    id?: number | string | null;
    result?: TResult;
    error?: JsonRpcError;
}

interface JsonRpcError {
    code: number;
    message: string;
    data?: any;
}

interface GetResult {
    components: Record<string, any>;
    errors?: Record<string, JsonRpcError>;
}

interface QueryResult {
    entity: number;
    components: Record<string, any>;
    has?: Record<string, boolean>;
}

interface SpawnResult {
    entity: number;
}

interface ListResult {
    components: string[];
}

interface GetWatchResult {
    components: Record<string, any>;
    removed: string[];
    errors?: Record<string, JsonRpcError>;
}

interface ListWatchResult {
    added: string[];
    removed: string[];
}

export type GetResponse = JsonRpcResponse<GetResult>;
export type QueryResponse = JsonRpcResponse<QueryResult[]>;
export type SpawnResponse = JsonRpcResponse<SpawnResult>;
export type DestroyResponse = JsonRpcResponse<null>;
export type RemoveResponse = JsonRpcResponse<null>;
export type InsertResponse = JsonRpcResponse<null>;
export type ReparentResponse = JsonRpcResponse<null>;
export type ListResponse = JsonRpcResponse<ListResult>;
export type GetWatchResponse = JsonRpcResponse<GetWatchResult>;
export type ListWatchResponse = JsonRpcResponse<ListWatchResult>;
