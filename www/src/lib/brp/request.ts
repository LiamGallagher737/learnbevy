// deno-fmt-ignore-file
/* eslint-disable @typescript-eslint/no-explicit-any */

interface JsonRpcRequest<TParams> {
    jsonrpc?: "2.0";
    id?: number | string | null;
    method: string;
    params: TParams;
}

interface GetParams {
    entity: number;
    components: string[];
    strict?: boolean;
}

interface QueryParams {
    data?: {
        components?: string[];
        option?: string[];
        has?: string[];
    };
    filter?: {
        with?: string[];
        without?: string[];
    };
}

interface SpawnParams {
    components: Record<string, any>;
}

interface DestroyParams {
    entity: number;
}

interface RemoveParams {
    entity: number;
    components: string[];
}

interface InsertParams {
    entity: number;
    components: Record<string, any>;
}

interface ReparentParams {
    entities: number[];
    parent?: number;
}

interface ListParams {
    entity?: number;
}

interface GetWatchParams {
    entity: number;
    components: string[];
    strict?: boolean;
}

interface ListWatchParams {
    entity: number;
}

export type GetRequest = JsonRpcRequest<GetParams> & { method: "bevy/get" };
export type QueryRequest = JsonRpcRequest<QueryParams> & { method: "bevy/query" };
export type SpawnRequest = JsonRpcRequest<SpawnParams> & { method: "bevy/spawn" };
export type DestroyRequest = JsonRpcRequest<DestroyParams> & { method: "bevy/destroy" };
export type RemoveRequest = JsonRpcRequest<RemoveParams> & { method: "bevy/remove" };
export type InsertRequest = JsonRpcRequest<InsertParams> & { method: "bevy/insert" };
export type ReparentRequest = JsonRpcRequest<ReparentParams> & { method: "bevy/reparent" };
export type ListRequest = JsonRpcRequest<ListParams> & { method: "bevy/list" };
export type GetWatchRequest = JsonRpcRequest<GetWatchParams> & { method: "bevy/get+watch" };
export type ListWatchRequest = JsonRpcRequest<ListWatchParams> & { method: "bevy/list+watch" };

export type BrpRequest =
    | GetRequest
    | QueryRequest
    | SpawnRequest
    | DestroyRequest
    | RemoveRequest
    | InsertRequest
    | ReparentRequest
    | ListRequest
    | GetWatchRequest
    | ListWatchRequest;
