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
export interface IssueData {
  'title' : string,
  'body' : string,
  'state' : string,
  'number' : bigint,
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
  'create_member' : ActorMethod<[string], Result>,
  'get_all_issues' : ActorMethod<[], Array<IssueData>>,
  'get_all_members' : ActorMethod<[], Array<Member>>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
}
