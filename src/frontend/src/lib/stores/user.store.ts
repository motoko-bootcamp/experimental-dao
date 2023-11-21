import { backendCanisterId, host } from '$lib/constants';
import type { ActorSubclass, Identity } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';
import type { Principal } from '@dfinity/principal';
import { writable } from 'svelte/store';
import { createActor } from '../../../../declarations/backend';
import type { _SERVICE } from '../../../../declarations/backend/backend.did';

let authClient: AuthClient | null = null;

export type Session = {
	backend: ActorSubclass<_SERVICE> | null;
	identity: Identity | null | undefined;
};

export const dS = writable<Session>({
	backend: null,
	identity: null
});

function update_dS_State() {
	const identity = authClient?.getIdentity();
	dS.update((value) => ({
		...value,
		identity: identity,
		backend: createActor(backendCanisterId, { agentOptions: { host, identity } })
	}));
}

export async function syncAuth() {
	authClient = await AuthClient.create({
		idleOptions: {
			idleTimeout: 1000 * 60 * 60 * 24 * 30,
			disableDefaultIdleCallback: true
		}
	});
	if (await authClient.isAuthenticated()) {
		update_dS_State();
	}
}

function popupCenter({ width, height }: { width: number; height: number }) {
	const top = window.innerHeight / 2 - height / 2 + window.screenY;
	const left = window.innerWidth / 2 - width / 2 + window.screenX;
	return `top=${top},left=${left},width=${width},height=${height}`;
}

export async function login() {
	await authClient?.login({
		identityProvider:
			process.env.DFX_NETWORK === 'ic'
				? 'https://identity.ic0.app/#authorize'
				: `http://127.0.0.1:8000/?canisterId=${process.env.INTERNET_IDENTITY_CANISTER_ID}`,
		maxTimeToLive: BigInt(30 * 24 * 60 * 60 * 1000 * 1000 * 1000 * 1000),
		onSuccess: async () => {
			update_dS_State();
			// goto('/oversyn');
		},
		windowOpenerFeatures: popupCenter({ width: 600, height: 600 })
	});
}

export async function logout() {
	await authClient?.logout();
	dS.update((value) => ({ ...value, identity: null }));
}
