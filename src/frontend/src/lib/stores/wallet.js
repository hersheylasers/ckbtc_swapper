import { writable } from 'svelte/store';
import { canisterIds, getActor } from '../api';
import { idlFactory } from '../wallet.did';
import { authStore } from './auth';

const initialState = {
	preferredNetwork: { Bitcoin: null },
	balances: {
		btc: BigInt(0),
		ckbtc: BigInt(0)
	},
	conversionHistory: [],
	isLoading: false,
	error: null
};

function createWalletStore() {
	const { subscribe, set, update } = writable({ ...initialState });

	let actor = null;

	async function getWalletActor() {
		try {
			if (!actor) {
				actor = await getActor(canisterIds.walletBackend, idlFactory);
			}
			return actor;
		} catch (error) {
			console.error('Failed to create wallet actor:', error);
			throw error;
		}
	}

	const walletStore = {
		subscribe,
		reset: () => {
			actor = null;
			set({ ...initialState });
		},
		refreshBalances: async () => {
			update((state) => ({ ...state, isLoading: true, error: null }));
			try {
				const actor = await getWalletActor();
				console.log('Fetching balances...');
				const result = await actor.get_balances();
				console.log('Balances result:', result);

				if ('Ok' in result) {
					const [btc, ckbtc] = result.Ok;
					update((state) => ({
						...state,
						balances: {
							btc: BigInt(btc.toString()),
							ckbtc: BigInt(ckbtc.toString())
						},
						isLoading: false
					}));
				} else if ('Err' in result) {
					throw new Error(result.Err);
				}
			} catch (error) {
				console.error('Failed to refresh balances:', error);
				update((state) => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},
		refreshHistory: async () => {
			update((state) => ({ ...state, isLoading: true, error: null }));
			try {
				const actor = await getWalletActor();
				console.log('Fetching conversion history...');
				const history = await actor.get_conversion_history();
				console.log('History result:', history);

				update((state) => ({
					...state,
					conversionHistory: history || [],
					isLoading: false
				}));
			} catch (error) {
				console.error('Failed to refresh history:', error);
				update((state) => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},
		setPreferredNetwork: async (network) => {
			update((state) => ({ ...state, isLoading: true, error: null }));
			try {
				const actor = await getWalletActor();
				console.log('Setting preferred network:', network);
				await actor.set_preferred_network(network);

				update((state) => ({
					...state,
					preferredNetwork: network,
					isLoading: false
				}));
			} catch (error) {
				console.error('Failed to set preferred network:', error);
				update((state) => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},
		clearError: () => update((state) => ({ ...state, error: null }))
	};

	return walletStore;
}

export const walletStore = createWalletStore();

// Initialize wallet when authenticated
authStore.subscribe(async (authState) => {
	if (authState.isAuthenticated) {
		console.log('Auth state changed, initializing wallet...');
		await walletStore.refreshBalances();
		await walletStore.refreshHistory();
	} else {
		console.log('User logged out, resetting wallet...');
		walletStore.reset();
	}
});
