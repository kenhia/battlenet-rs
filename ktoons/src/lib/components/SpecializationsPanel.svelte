<script lang="ts">
	import type { FullCharacter } from '$lib/types';

	let { character }: { character: FullCharacter } = $props();

	const specs = $derived(character.specializations);
</script>

<div class="specs-panel">
	<h2>Specializations</h2>
	{#if specs}
		{#if specs.specializations && specs.specializations.length > 0}
			<div class="spec-list">
				{#each specs.specializations as entry}
					<div
						class="spec-item"
						class:active={specs.active_specialization &&
							entry.specialization.id === specs.active_specialization.id}
					>
						<span class="spec-name">{entry.specialization.name}</span>
						{#if specs.active_specialization && entry.specialization.id === specs.active_specialization.id}
							<span class="active-badge">Active</span>
						{/if}
					</div>
				{/each}
			</div>
		{:else if specs.active_specialization}
			<p>Active: <strong>{specs.active_specialization.name}</strong></p>
		{/if}
	{:else}
		<p class="no-data">No specialization data available</p>
	{/if}
</div>

<style>
	.specs-panel {
		padding: 1.5rem;
		background: var(--color-surface, #1e1e1e);
		border-radius: 8px;
		border: 1px solid var(--color-border, #444);
	}

	h2 {
		margin: 0 0 1rem;
		font-size: 1.25rem;
		color: var(--color-heading, #e0e0e0);
	}

	.spec-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.spec-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.5rem 0.75rem;
		border-radius: 4px;
		background: var(--color-item-bg, #252525);
		border: 1px solid transparent;
	}

	.spec-item.active {
		border-color: var(--color-primary, #1a73e8);
		background: #1a2a3e;
	}

	.spec-name {
		color: var(--color-text, #e0e0e0);
	}

	.active-badge {
		font-size: 0.75rem;
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
		background: var(--color-primary, #1a73e8);
		color: #fff;
	}

	.no-data {
		color: var(--color-muted, #888);
		margin: 0;
	}
</style>
