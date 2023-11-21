import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Member {
	xp: bigint;
	principal: Principal;
	username: string;
	level: bigint;
}
export type MemberError = { PrincipalAlreadyRegistered: null } | { UsernameExists: null };
export type Result = { Ok: [] | [Member] } | { Err: MemberError };
export interface _SERVICE {
	create_member: ActorMethod<[string, bigint, bigint], Result>;
	get_all_members: ActorMethod<[], Array<Member>>;
}
