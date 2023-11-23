import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Uint8Array | number[],
  'headers' : Array<[string, string]>,
  'streaming_strategy' : [] | [StreamingStrategy],
  'status_code' : number,
}
export interface Member {
  'xp' : bigint,
  'principal' : Principal,
  'username' : string,
  'level' : bigint,
  'roles' : bigint,
}
export type MemberError = { 'PrincipalAlreadyRegistered' : null } |
  { 'UsernameExists' : null };
export type Result = { 'Ok' : [] | [Member] } |
  { 'Err' : MemberError };
export type StreamingStrategy = {
    'Callback' : { 'token' : {}, 'callback' : [Principal, string] }
  };
export interface _SERVICE {
  'add_to_log' : ActorMethod<[string], undefined>,
  'create_member' : ActorMethod<[string], Result>,
  'get_all_members' : ActorMethod<[], Array<Member>>,
  'get_logs' : ActorMethod<[], Array<string>>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
}
