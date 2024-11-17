import { AuthClient } from '@dfinity/auth-client';
import { writable } from 'svelte/store';
import { II_URL } from '../api';

function createAuthStore() {
	const { subscribe, set, update } = writable({
		isAuthenticated: false,
		principal: null,
		client: null,
		isLoading: false,
		error: null
	});

	let store;
	subscribe((value) => {
		store = value;
	});

	return {
		subscribe,
		init: async () => {
			try {
				const client = await AuthClient.create();
				const isAuthenticated = await client.isAuthenticated();

				update((state) => ({
					...state,
					client,
					isAuthenticated,
					principal: isAuthenticated ? client.getIdentity().getPrincipal() : null
				}));

				return isAuthenticated;
			} catch (error) {
				console.error('Auth init error:', error);
				update((state) => ({ ...state, error: error.message }));
				return false;
			}
		},
		login: async () => {
			update((state) => ({ ...state, isLoading: true }));
			try {
				if (!store.client) {
					const client = await AuthClient.create();
					update((state) => ({ ...state, client }));
				}

				await store.client.login({
					identityProvider: II_URL,
					maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000), // 7 days in nanoseconds
					onSuccess: () => {
						const principal = store.client.getIdentity().getPrincipal();
						console.log('Login successful. Principal:', principal.toString());
						update((state) => ({
							...state,
							isAuthenticated: true,
							principal,
							isLoading: false
						}));
					},
					onError: (error) => {
						console.error('Login error:', error);
						update((state) => ({
							...state,
							error: error.message,
							isLoading: false
						}));
					}
				});
			} catch (error) {
				console.error('Login error:', error);
				update((state) => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},
		logout: async () => {
			update((state) => ({ ...state, isLoading: true }));
			try {
				if (!store.client) {
					throw new Error('Auth client not initialized');
				}

				await store.client.logout();
				update((state) => ({
					...state,
					isAuthenticated: false,
					principal: null,
					isLoading: false
				}));
			} catch (error) {
				console.error('Logout error:', error);
				update((state) => ({
					...state,
					error: error.message,
					isLoading: false
				}));
			}
		},
		clearError: () => update((state) => ({ ...state, error: null }))
	};
}

export const authStore = createAuthStore();
