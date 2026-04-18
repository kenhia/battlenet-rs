<script lang="ts">
	import type { FullCharacter, EquippedItem } from '$lib/types';

	let { character }: { character: FullCharacter } = $props();

	const items: EquippedItem[] = $derived(character.equipment?.equipped_items ?? []);
</script>

<div class="equipment-panel">
	<h2>Equipment</h2>
	{#if items.length > 0}
		<div class="equipment-list">
			{#each items as item}
				<div class="equipment-item quality-{item.quality?.type_?.toLowerCase() ?? 'common'}">
					<span class="slot">{item.slot.name}</span>
					<span class="item-name">{item.name}</span>
					{#if item.level}
						<span class="item-level">{item.level.value}</span>
					{/if}
				</div>
			{/each}
		</div>
	{:else}
		<p class="no-data">No equipment data available</p>
	{/if}
</div>

<style>
	.equipment-panel {
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

	.equipment-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.equipment-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem 0.75rem;
		border-radius: 4px;
		background: var(--color-item-bg, #252525);
	}

	.slot {
		min-width: 80px;
		font-size: 0.75rem;
		color: var(--color-muted, #888);
		text-transform: uppercase;
	}

	.item-name {
		flex: 1;
	}

	.item-level {
		font-size: 0.875rem;
		color: var(--color-muted, #888);
	}

	.quality-poor { color: #9d9d9d; }
	.quality-common { color: #ffffff; }
	.quality-uncommon { color: #1eff00; }
	.quality-rare { color: #0070dd; }
	.quality-epic { color: #a335ee; }
	.quality-legendary { color: #ff8000; }
	.quality-artifact { color: #e6cc80; }
	.quality-heirloom { color: #00ccff; }

	.no-data {
		color: var(--color-muted, #888);
		margin: 0;
	}
</style>
