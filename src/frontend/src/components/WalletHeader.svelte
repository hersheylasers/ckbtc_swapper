<script>
	import { authStore } from '../lib/stores/auth';

	$: ({ isAuthenticated, principal, isLoading, error } = $authStore);

	async function copyPrincipal() {
		if (!principal) return;

		try {
			await navigator.clipboard.writeText(principal.toString());
		} catch (error) {
			console.error('Failed to copy principal:', error);
		}
	}

	async function handleLogout() {
		await authStore.logout();
	}

	function formatPrincipal(principal) {
		const text = principal.toString();
		return `${text.slice(0, 10)}...${text.slice(-8)}`;
	}
</script>

<header class="rounded-lg bg-white p-6 shadow">
	<div class="flex items-center justify-between">
		<h1 class="text-2xl font-bold">BTC-ckBTC Wallet</h1>

		{#if error}
			<div class="text-red-600 text-sm">
				{error}
				<button
					class="text-blue-600 hover:text-blue-800 ml-2"
					on:click={() => authStore.clearError()}
				>
					Dismiss
				</button>
			</div>
		{/if}

		{#if isAuthenticated && principal}
			<div class="flex items-center space-x-4">
				<button
					on:click={copyPrincipal}
					disabled={isLoading}
					class="text-gray-500 hover:text-gray-700 flex items-center space-x-1 text-sm disabled:opacity-50"
				>
					<span class="font-mono">{formatPrincipal(principal)}</span>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-4 w-4"
						viewBox="0 0 20 20"
						fill="currentColor"
					>
						<path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z" />
						<path
							d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z"
						/>
					</svg>
				</button>

				<button
					on:click={handleLogout}
					disabled={isLoading}
					class="text-red-600 hover:text-red-800 text-sm disabled:opacity-50"
				>
					Logout
				</button>
			</div>
		{/if}
	</div>
</header>
