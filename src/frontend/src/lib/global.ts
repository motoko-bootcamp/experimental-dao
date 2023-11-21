import type { ActorSubclass } from '@dfinity/agent';
import type { AuthClient } from '@dfinity/auth-client';
import type { Principal } from '@dfinity/principal';
import type { _SERVICE } from '../../../declarations/backend/backend.did';
import type { Identity } from '@dfinity/agent';

export type All = {
	backend: ActorSubclass<_SERVICE> | null;
	isAuth: boolean | null;
	principal: Principal | null;
	// authClient: AuthClient | null;
	delegIdentity: Identity | null;
};

export let all: All = {
	backend: null,
	isAuth: null,
	principal: null,
	// authClient: null
	delegIdentity: null
};
