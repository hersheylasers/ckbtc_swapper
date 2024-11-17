<script>
	import { onMount } from 'svelte';
	import { walletStore } from '../lib/stores/wallet';

	let refreshInterval;

	$: ({ balances, isLoading, error } = $walletStore);

	onMount(() => {
		console.log('BalanceDisplay mounted');
		// Initial refresh
		walletStore.refreshBalances();

		// Refresh balances every 30 seconds
		refreshInterval = setInterval(() => {
			walletStore.refreshBalances();
		}, 30000);

		return () => {
			if (refreshInterval) {
				clearInterval(refreshInterval);
			}
		};
	});

	function formatBalance(balance) {
		if (!balance) return '0.00000000';
		try {
			return (Number(balance) / 100_000_000).toFixed(8);
		} catch (error) {
			console.error('Error formatting balance:', error);
			return '0.00000000';
		}
	}
</script>

<div class="rounded-lg bg-white p-6 shadow">
	<h2 class="mb-4 text-xl font-semibold">Balances</h2>

	{#if isLoading}
		<div class="py-4 text-center">
			<div class="border-blue-600 mx-auto h-8 w-8 animate-spin rounded-full border-b-2"></div>
			<span class="text-gray-500 mt-2 block">Loading balances...</span>
		</div>
	{:else if error}
		<div class="text-red-600 bg-red-50 rounded-lg p-4">
			{error}
			<button
				class="text-blue-600 hover:text-blue-800 ml-2"
				on:click={() => walletStore.clearError()}
			>
				Dismiss
			</button>
		</div>
	{:else}
		<div class="grid grid-cols-2 gap-4">
			<div class="bg-gray-50 rounded-lg p-4">
				<h3 class="text-gray-500 text-sm font-medium">Bitcoin</h3>
				<p class="mt-2 text-2xl font-semibold">{formatBalance(balances.btc)} BTC</p>
			</div>

			<div class="bg-gray-50 rounded-lg p-4">
				<h3 class="text-gray-500 text-sm font-medium">ckBTC</h3>
				<p class="mt-2 text-2xl font-semibold">{formatBalance(balances.ckbtc)} ckBTC</p>
			</div>
		</div>
	{/if}

	<button
		class="text-blue-600 hover:text-blue-800 mt-4 text-sm"
		on:click={() => walletStore.refreshBalances()}
		disabled={isLoading}
	>
		{#if isLoading}
			Refreshing...
		{:else}
			Refresh Balances
		{/if}
	</button>
</div>
