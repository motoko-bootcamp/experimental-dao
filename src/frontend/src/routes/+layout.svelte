<script lang="ts">
    // import Spinner from '$lib/components/Spinner.svelte';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { dS } from '$lib/stores/user.store';
    import { syncAuth } from '$lib/stores/user.store';
    // import { i18n } from '$lib/stores/i18n.store';
	// import { loadConfigFromFile } from 'vite';

    let isLoading = true;

    onMount(async () => {
        // const userLang = localStorage.getItem('userLang') || 'en';
        // await i18n.switchLang(userLang);
        await syncAuth();
        if (!$dS.identity && location.pathname !== '/login') {
            goto('/login');
        }
        isLoading = false;
    });
</script>

{#if isLoading}
    is loading ...
    <!-- <Spinner /> -->
{:else}
    <slot />
{/if}


<style lang="scss" global>
	// @import '../lib/styles/global.scss';
</style>
