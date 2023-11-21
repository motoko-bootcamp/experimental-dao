<script lang="ts">
	import { dS, login, logout } from '$lib/stores/user.store';
	// import { i18n } from '$lib/stores/i18n.store';

	const handleLoginII = async () => {
		login();
	};

	const handleLogout = async () => {
		logout();
	};

</script>

<section>
	<br />
	<br />

	{#if $dS.identity}
	<p>yes</p>
	logged_in
		<button on:click={handleLogout}>Logout</button>
	{:else}
		<button on:click={handleLoginII}>Login with internet identity</button>
	{/if}

	<br />
	<br />
</section>