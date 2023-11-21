import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
	hello: ActorMethod<[string], string>;
	world: ActorMethod<[string], string>;
}
