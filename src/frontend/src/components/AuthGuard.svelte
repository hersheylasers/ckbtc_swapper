<script>
	import { onMount } from 'svelte';
	import { authStore } from '../lib/stores/auth';

	let isLoading = true;

	$: ({ isAuthenticated, error } = $authStore);

	onMount(async () => {
		try {
			console.log('Initializing auth...');
			const isAuthed = await authStore.init();
			console.log('Auth initialized, isAuthed:', isAuthed);
			isLoading = false;
		} catch (err) {
			console.error('Auth initialization error:', err);
			isLoading = false;
		}
	});

	async function handleLogin() {
		console.log('Login clicked');
		await authStore.login();
	}
</script>

{#if isLoading}
	<div class="flex min-h-screen items-center justify-center">
		<div class="text-center">
			<div class="border-blue-600 mx-auto h-12 w-12 animate-spin rounded-full border-b-2"></div>
			<p class="text-gray-600 mt-4">Loading...</p>
		</div>
	</div>
{:else if !isAuthenticated}
	<div class="flex min-h-screen items-center justify-center">
		<div class="text-center">
			<h1 class="mb-4 text-2xl font-bold">Welcome to BTC-ckBTC Wallet</h1>
			{#if error}
				<div class="text-red-600 mb-4">
					{error}
					<button
						class="text-blue-600 hover:text-blue-800 ml-2"
						on:click={() => authStore.clearError()}
					>
						Dismiss
					</button>
				</div>
			{/if}
			<button
				class="bg-blue-600 hover:bg-blue-700 rounded-lg px-6 py-2 text-white"
				on:click={handleLogin}
			>
				Login with Internet Identity
			</button>
		</div>
	</div>
{:else}
	<slot />
{/if}
