import { Actor, HttpAgent } from '@dfinity/agent';
import { AuthClient } from '@dfinity/auth-client';

export const II_URL = import.meta.env.VITE_II_URL;
export const DFX_NETWORK = import.meta.env.VITE_DFX_NETWORK;

export async function getActor(canisterId, idlFactory) {
	const authClient = await AuthClient.create();
	const identity = authClient.getIdentity();

	const agent = new HttpAgent({
		identity,
		host: DFX_NETWORK === 'ic' ? 'https://ic0.app' : 'http://127.0.0.1:8001'
	});

	// Only fetch root key in development
	if (DFX_NETWORK !== 'ic') {
		await agent.fetchRootKey();
	}

	return Actor.createActor(idlFactory, {
		agent,
		canisterId
	});
}

export const canisterIds = {
	internetIdentity: 'bnz7o-iuaaa-aaaaa-qaaaa-cai',
	walletBackend: 'bd3sg-teaaa-aaaaa-qaaba-cai'
};
