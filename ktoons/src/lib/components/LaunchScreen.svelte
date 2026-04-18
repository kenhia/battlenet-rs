<script lang="ts">
	import type { RealmEntry } from '$lib/types';

	let {
		realms,
		onLookup,
		onLogin
	}: {
		realms: RealmEntry[];
		onLookup: (realmSlug: string, characterName: string) => void;
		onLogin: () => void;
	} = $props();

	let selectedRealm = $state('');
	let characterName = $state('');

	function handleLookup() {
		if (selectedRealm && characterName.trim()) {
			onLookup(selectedRealm, characterName.trim());
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			handleLookup();
		}
	}
</script>

<div class="launch-screen">
	<h1>ktoons</h1>
	<p class="subtitle">WoW Character Viewer</p>

	<div class="lookup-form">
		<div class="form-group">
			<label for="realm-select">Realm</label>
			<select id="realm-select" bind:value={selectedRealm}>
				<option value="" disabled>Select a realm…</option>
				{#each realms as realm}
					<option value={realm.slug}>{realm.name}</option>
				{/each}
			</select>
		</div>

		<div class="form-group">
			<label for="character-name">Character Name</label>
			<input
				id="character-name"
				type="text"
				bind:value={characterName}
				placeholder="Enter character name"
				onkeydown={handleKeydown}
			/>
		</div>

		<button
			class="btn btn-primary"
			onclick={handleLookup}
			disabled={!selectedRealm || !characterName.trim()}
		>
			Lookup
		</button>
	</div>

	<div class="divider">
		<span>or</span>
	</div>

	<button class="btn btn-secondary" onclick={onLogin}>
		Login with Battle.net
	</button>
</div>

<style>
	.launch-screen {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
		padding: 2rem;
	}

	h1 {
		font-size: 2.5rem;
		margin-bottom: 0.25rem;
		color: var(--color-heading, #e0e0e0);
	}

	.subtitle {
		color: var(--color-muted, #888);
		margin-bottom: 2rem;
	}

	.lookup-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		width: 100%;
		max-width: 400px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	label {
		font-size: 0.875rem;
		color: var(--color-muted, #888);
	}

	select,
	input {
		padding: 0.5rem 0.75rem;
		border: 1px solid var(--color-border, #444);
		border-radius: 4px;
		background: var(--color-input-bg, #2a2a2a);
		color: var(--color-text, #e0e0e0);
		font-size: 1rem;
	}

	.btn {
		padding: 0.625rem 1.25rem;
		border: none;
		border-radius: 4px;
		font-size: 1rem;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-primary {
		background: var(--color-primary, #1a73e8);
		color: #fff;
	}

	.btn-primary:hover:not(:disabled) {
		background: var(--color-primary-hover, #1557b0);
	}

	.btn-secondary {
		background: var(--color-secondary, #333);
		color: var(--color-text, #e0e0e0);
		border: 1px solid var(--color-border, #444);
	}

	.btn-secondary:hover {
		background: var(--color-secondary-hover, #444);
	}

	.divider {
		display: flex;
		align-items: center;
		width: 100%;
		max-width: 400px;
		margin: 1.5rem 0;
		gap: 1rem;
	}

	.divider::before,
	.divider::after {
		content: '';
		flex: 1;
		border-bottom: 1px solid var(--color-border, #444);
	}

	.divider span {
		color: var(--color-muted, #888);
		font-size: 0.875rem;
	}
</style>
