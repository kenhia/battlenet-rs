<script lang="ts">
	import { onMount } from 'svelte';
	import { getRealms, lookupCharacter, login, getCharacter, refreshCharacter } from '$lib/commands';
	import type { RealmEntry, FullCharacter, CharacterListEntry, AccountCharacterEntry } from '$lib/types';
	import LaunchScreen from '$lib/components/LaunchScreen.svelte';
	import CharacterHeader from '$lib/components/CharacterHeader.svelte';
	import EquipmentPanel from '$lib/components/EquipmentPanel.svelte';
	import StatsPanel from '$lib/components/StatsPanel.svelte';
	import SpecializationsPanel from '$lib/components/SpecializationsPanel.svelte';
	import CharacterNav from '$lib/components/CharacterNav.svelte';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
	import ErrorDisplay from '$lib/components/ErrorDisplay.svelte';

	type View = 'launch' | 'loading' | 'character' | 'error';

	let view: View = $state('loading');
	let realms: RealmEntry[] = $state([]);
	let characters: { entry: CharacterListEntry; data: FullCharacter }[] = $state([]);
	let activeIndex = $state(-1);
	let errorMessage = $state('');
	let lastLookup: { realm: string; name: string } | null = $state(null);
	let loggedIn = $state(false);
	let accountCharacters: AccountCharacterEntry[] = $state([]);
	let refreshing = $state(false);

	const activeCharacter = $derived(
		activeIndex >= 0 && activeIndex < characters.length ? characters[activeIndex].data : null
	);

	onMount(async () => {
		try {
			realms = await getRealms();
			view = 'launch';
		} catch (e) {
			errorMessage = `Failed to load realms: ${e}`;
			view = 'error';
		}
	});

	async function handleLookup(realmSlug: string, characterName: string) {
		lastLookup = { realm: realmSlug, name: characterName };
		view = 'loading';
		try {
			const fetchFn = loggedIn ? getCharacter : lookupCharacter;
			const fc: FullCharacter = await fetchFn(realmSlug, characterName);
			const realmEntry = realms.find((r) => r.slug === realmSlug);

			const entry: CharacterListEntry = {
				name: fc.profile?.name ?? characterName,
				realmName: realmEntry?.name ?? realmSlug,
				realmSlug,
				level: fc.profile?.level ?? 0,
				className: fc.profile?.character_class?.name ?? '',
				raceName: fc.profile?.race?.name ?? '',
				faction: fc.profile?.faction?.name ?? ''
			};

			characters = [...characters, { entry, data: fc }];
			activeIndex = characters.length - 1;
			view = 'character';
		} catch (e) {
			errorMessage = `${e}`;
			view = 'error';
		}
	}

	async function handleLogin() {
		view = 'loading';
		try {
			accountCharacters = await login();
			loggedIn = true;

			// Populate nav with account characters
			characters = accountCharacters.map((ac) => ({
				entry: {
					name: ac.name,
					realmName: ac.realm_name,
					realmSlug: ac.realm_slug,
					level: ac.level,
					className: ac.class_name,
					raceName: '',
					faction: ac.faction
				},
				data: null as unknown as FullCharacter // placeholder — fetched on click
			}));
			activeIndex = -1;
			view = 'launch';
		} catch (e) {
			errorMessage = `Login failed: ${e}`;
			view = 'error';
		}
	}

	async function handleSelectCharacter(index: number) {
		const char = characters[index];
		if (!char.data) {
			// Need to fetch character data
			view = 'loading';
			try {
				const fc = await getCharacter(char.entry.realmSlug, char.entry.name);
				characters[index] = { ...char, data: fc };
				activeIndex = index;
				view = 'character';
			} catch (e) {
				errorMessage = `${e}`;
				view = 'error';
			}
		} else {
			activeIndex = index;
			view = 'character';
		}
	}

	function handleNewLookup() {
		view = 'launch';
	}

	function handleRetry() {
		if (lastLookup) {
			handleLookup(lastLookup.realm, lastLookup.name);
		} else {
			view = 'launch';
		}
	}

	async function handleRefresh() {
		if (activeIndex < 0 || !activeCharacter) return;
		const char = characters[activeIndex];
		refreshing = true;
		try {
			const fc = await refreshCharacter(char.entry.realmSlug, char.entry.name);
			characters[activeIndex] = { ...char, data: fc };
		} catch (e) {
			errorMessage = `Refresh failed: ${e}`;
		}
		refreshing = false;
	}
</script>

<div class="app-layout">
	{#if characters.length > 0}
		<CharacterNav
			characters={characters.map((c) => c.entry)}
			{activeIndex}
			onSelect={handleSelectCharacter}
			onNewLookup={handleNewLookup}
			groupByRealm={loggedIn}
		/>
	{/if}

	<main class="main-panel">
		{#if view === 'loading'}
			<LoadingSpinner message="Loading…" />
		{:else if view === 'launch'}
			<LaunchScreen {realms} onLookup={handleLookup} onLogin={handleLogin} />
		{:else if view === 'character' && activeCharacter}
			<div class="character-view">
				<div class="character-toolbar">
					<span class="fetched-at">
						{#if activeCharacter.fetched_at}
							Fetched: {new Date(activeCharacter.fetched_at).toLocaleString()}
						{/if}
					</span>
					<button class="btn-refresh" onclick={handleRefresh} disabled={refreshing}>
						{refreshing ? 'Refreshing…' : 'Refresh'}
					</button>
				</div>
				<CharacterHeader character={activeCharacter} />
				<div class="panels">
					<EquipmentPanel character={activeCharacter} />
					<div class="right-panels">
						<StatsPanel character={activeCharacter} />
						<SpecializationsPanel character={activeCharacter} />
					</div>
				</div>
				{#if activeCharacter.errors.length > 0}
					<div class="endpoint-errors">
						<h3>Endpoint Errors</h3>
						{#each activeCharacter.errors as err}
							<p class="endpoint-error">{err.endpoint}: {err.message}</p>
						{/each}
					</div>
				{/if}
			</div>
		{:else if view === 'error'}
			<div class="error-view">
				<ErrorDisplay message={errorMessage} onRetry={handleRetry} />
				<button class="btn-back" onclick={handleNewLookup}>← Back to Search</button>
			</div>
		{/if}
	</main>
</div>

<style>
	:global(body) {
		margin: 0;
		font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
		background: #1a1a1a;
		color: #e0e0e0;
	}

	.app-layout {
		display: flex;
		min-height: 100vh;
	}

	.main-panel {
		flex: 1;
		overflow-y: auto;
		height: 100vh;
	}

	.character-view {
		max-width: 900px;
		margin: 0 auto;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.character-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.fetched-at {
		font-size: 0.75rem;
		color: var(--color-muted, #888);
	}

	.btn-refresh {
		background: var(--color-secondary, #333);
		border: 1px solid var(--color-border, #444);
		color: var(--color-text, #ccc);
		padding: 0.375rem 0.75rem;
		border-radius: 4px;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.btn-refresh:hover:not(:disabled) {
		background: var(--color-secondary-hover, #444);
	}

	.btn-refresh:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.panels {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.right-panels {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	@media (max-width: 700px) {
		.panels {
			grid-template-columns: 1fr;
		}
	}

	.endpoint-errors {
		padding: 1rem;
		background: #2a1a1a;
		border: 1px solid #5c2a2a;
		border-radius: 8px;
	}

	.endpoint-errors h3 {
		margin: 0 0 0.5rem;
		color: #ff7a7a;
	}

	.endpoint-error {
		margin: 0.25rem 0;
		font-size: 0.875rem;
		color: #cc8888;
	}

	.error-view {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
		gap: 1rem;
	}

	.btn-back {
		background: none;
		border: 1px solid #444;
		color: #ccc;
		padding: 0.375rem 0.75rem;
		border-radius: 4px;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.btn-back:hover {
		background: #333;
	}
</style>

