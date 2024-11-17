<script>
	import { onMount } from 'svelte';
	import { walletStore } from '../lib/stores/wallet';

	$: ({ conversionHistory, isLoading, error } = $walletStore);

	function formatDate(timestamp) {
		return new Date(Number(timestamp)).toLocaleString();
	}

	function formatAmount(record) {
		const amount = Number(record.amount) / 100_000_000;
		const fromNetwork = Object.keys(record.fromNetwork)[0];
		const toNetwork = Object.keys(record.toNetwork)[0];
		return `${amount.toFixed(8)} ${fromNetwork} → ${toNetwork}`;
	}

	function getStatusColor(status) {
		if ('Pending' in status) return 'text-yellow-600 bg-yellow-100';
		if ('Complete' in status) return 'text-green-600 bg-green-100';
		if ('Failed' in status) return 'text-red-600 bg-red-100';
		return 'text-gray-600 bg-gray-100';
	}

	function getStatusText(status) {
		if ('Pending' in status) return 'Pending';
		if ('Complete' in status) return 'Complete';
		if ('Failed' in status) return `Failed: ${status.Failed}`;
		return 'Unknown';
	}

	onMount(() => {
		walletStore.refreshHistory();
	});
</script>

<div class="rounded-lg bg-white p-6 shadow">
	<h2 class="mb-4 text-xl font-semibold">Conversion History</h2>

	{#if isLoading}
		<div class="py-4 text-center">
			<span class="text-gray-500">Loading history...</span>
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
	{:else if conversionHistory.length === 0}
		<div class="py-4 text-center">
			<span class="text-gray-500">No conversions yet</span>
		</div>
	{:else}
		<div class="space-y-4">
			{#each conversionHistory as conversion}
				<div class="rounded-lg border p-4">
					<div class="flex items-start justify-between">
						<div>
							<p class="text-gray-500 text-sm">{formatDate(conversion.timestamp)}</p>
							<p class="mt-1">{formatAmount(conversion)}</p>
						</div>
						<span class={`rounded-full px-2 py-1 text-sm ${getStatusColor(conversion.status)}`}>
							{getStatusText(conversion.status)}
						</span>
					</div>
				</div>
			{/each}
		</div>
	{/if}

	<button
		class="text-blue-600 hover:text-blue-800 mt-4 text-sm"
		on:click={() => walletStore.refreshHistory()}
		disabled={isLoading}
	>
		Refresh History
	</button>
</div>
