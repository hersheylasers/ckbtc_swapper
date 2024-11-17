<script>
	import { onMount } from 'svelte';
	import { walletStore } from '../lib/stores/wallet';

	$: ({ preferredNetwork, isLoading, error } = $walletStore);

	async function handleNetworkChange(event) {
		const select = event.target;
		const network = select.value === 'Bitcoin' ? { Bitcoin: null } : { CkBTC: null };

		try {
			await walletStore.setPreferredNetwork(network);
		} catch (err) {
			console.error('Failed to update preference:', err);
		}
	}

	onMount(() => {
		walletStore.init();
	});
</script>

<div class="rounded-lg bg-white p-6 shadow">
	<h2 class="mb-4 text-xl font-semibold">Network Preferences</h2>

	<div class="space-y-4">
		<div>
			<label for="network-select" class="text-gray-700 block text-sm font-medium">
				Preferred Network
			</label>
			<select
				id="network-select"
				value={Object.keys(preferredNetwork)[0]}
				on:change={handleNetworkChange}
				class="border-gray-300 focus:border-blue-500 focus:ring-blue-500 mt-1 block w-full rounded-md shadow-sm"
				disabled={isLoading}
			>
				<option value="Bitcoin">Bitcoin Network</option>
				<option value="CkBTC">Internet Computer (ckBTC)</option>
			</select>
		</div>

		{#if error}
			<div class="text-red-600 mt-2 text-sm">
				{error}
				<button
					class="text-blue-600 hover:text-blue-800 ml-2"
					on:click={() => walletStore.clearError()}
				>
					Dismiss
				</button>
			</div>
		{/if}

		{#if isLoading}
			<div class="text-gray-500 text-sm">Updating preference...</div>
		{/if}
	</div>
</div>
