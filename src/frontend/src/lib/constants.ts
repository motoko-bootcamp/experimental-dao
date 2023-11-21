import type { Principal } from '@dfinity/principal';

export const backendCanisterId: string | Principal = import.meta.env.VITE_BACKEND_CANISTER_ID;

export const FrontendCanisterId: string | Principal = import.meta.env.VITE_FRONTEND_CANISTER_ID;

export const host = import.meta.env.VITE_HOST;
