<script>
	import AuthGuard from './components/AuthGuard.svelte';
	import NetworkPreference from './components/NetworkPreference.svelte';
	import BalanceDisplay from './components/BalanceDisplay.svelte';
	import ConversionHistory from './components/ConversionHistory.svelte';
	import WalletHeader from './components/WalletHeader.svelte';
</script>

<AuthGuard>
	<main class="bg-gray-50 min-h-screen">
		<WalletHeader />
		<div class="container mx-auto px-4 py-8">
			<div class="grid grid-cols-1 gap-6">
				<NetworkPreference />
				<BalanceDisplay />
				<ConversionHistory />
			</div>
		</div>
	</main>
</AuthGuard>
