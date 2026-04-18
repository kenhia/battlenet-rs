<script lang="ts">
	import type { CharacterListEntry } from '$lib/types';

	let {
		characters,
		activeIndex,
		onSelect,
		onNewLookup,
		groupByRealm = false
	}: {
		characters: CharacterListEntry[];
		activeIndex: number;
		onSelect: (index: number) => void;
		onNewLookup: () => void;
		groupByRealm?: boolean;
	} = $props();

	interface RealmGroup {
		realm: string;
		entries: { index: number; entry: CharacterListEntry }[];
	}

	const groups: RealmGroup[] = $derived.by(() => {
		if (!groupByRealm) return [];
		const map = new Map<string, { index: number; entry: CharacterListEntry }[]>();
		characters.forEach((entry, index) => {
			const list = map.get(entry.realmName) ?? [];
			list.push({ index, entry });
			map.set(entry.realmName, list);
		});
		return Array.from(map.entries()).map(([realm, entries]) => ({ realm, entries }));
	});
</script>

<aside class="character-nav">
	<div class="nav-header">
		<h2>Characters</h2>
		<button class="btn-new" onclick={onNewLookup} title="New lookup">+</button>
	</div>

	{#if characters.length === 0}
		<p class="empty">No characters looked up yet</p>
	{:else if groupByRealm && groups.length > 0}
		{#each groups as group}
			<div class="realm-group">
				<h3 class="realm-header">{group.realm}</h3>
				<ul class="character-list">
					{#each group.entries as { index, entry }}
						<li>
							<button
								class="character-entry"
								class:active={index === activeIndex}
								onclick={() => onSelect(index)}
							>
								<span class="char-name">{entry.name}</span>
								<span class="char-meta">{entry.level} {entry.className}</span>
							</button>
						</li>
					{/each}
				</ul>
			</div>
		{/each}
	{:else}
		<ul class="character-list">
			{#each characters as entry, i}
				<li>
					<button
						class="character-entry"
						class:active={i === activeIndex}
						onclick={() => onSelect(i)}
					>
						<span class="char-name">{entry.name}</span>
						<span class="char-realm">{entry.realmName}</span>
					</button>
				</li>
			{/each}
		</ul>
	{/if}
</aside>

<style>
	.character-nav {
		width: 220px;
		min-width: 220px;
		background: var(--color-surface, #1e1e1e);
		border-right: 1px solid var(--color-border, #444);
		display: flex;
		flex-direction: column;
		height: 100vh;
		overflow-y: auto;
	}

	.nav-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		border-bottom: 1px solid var(--color-border, #444);
	}

	h2 {
		margin: 0;
		font-size: 1rem;
		color: var(--color-heading, #e0e0e0);
	}

	.btn-new {
		background: none;
		border: 1px solid var(--color-border, #444);
		color: var(--color-text, #ccc);
		width: 28px;
		height: 28px;
		border-radius: 4px;
		cursor: pointer;
		font-size: 1.125rem;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.btn-new:hover {
		background: var(--color-secondary-hover, #333);
	}

	.character-list {
		list-style: none;
		margin: 0;
		padding: 0.5rem 0;
	}

	.character-entry {
		display: flex;
		flex-direction: column;
		width: 100%;
		padding: 0.5rem 1rem;
		background: none;
		border: none;
		cursor: pointer;
		text-align: left;
		color: var(--color-text, #ccc);
		border-left: 3px solid transparent;
	}

	.character-entry:hover {
		background: var(--color-item-bg, #252525);
	}

	.character-entry.active {
		background: var(--color-item-bg, #252525);
		border-left-color: var(--color-primary, #1a73e8);
	}

	.char-name {
		font-weight: 500;
	}

	.char-realm {
		font-size: 0.75rem;
		color: var(--color-muted, #888);
	}

	.empty {
		padding: 1rem;
		color: var(--color-muted, #888);
		font-size: 0.875rem;
	}

	.realm-group {
		border-bottom: 1px solid var(--color-border, #333);
	}

	.realm-header {
		margin: 0;
		padding: 0.5rem 1rem 0.25rem;
		font-size: 0.75rem;
		text-transform: uppercase;
		color: var(--color-muted, #888);
		letter-spacing: 0.05em;
	}

	.char-meta {
		font-size: 0.75rem;
		color: var(--color-muted, #888);
	}
</style>
