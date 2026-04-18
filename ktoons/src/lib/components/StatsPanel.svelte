<script lang="ts">
	import type { FullCharacter } from '$lib/types';

	let { character }: { character: FullCharacter } = $props();

	const stats = $derived(character.statistics);
	const specs = $derived(character.specializations);
</script>

<div class="stats-panel">
	<h2>Stats & Specs</h2>

	{#if specs}
		<div class="section">
			<h3>Specializations</h3>
			{#if specs.active_specialization}
				<p class="active-spec">Active: <strong>{specs.active_specialization.name}</strong></p>
			{/if}
		</div>
	{/if}

	{#if stats}
		<div class="section">
			<h3>Statistics</h3>
			<div class="stats-grid">
				{#if stats.health}
					<div class="stat-item">
						<span class="stat-label">Health</span>
						<span class="stat-value">{stats.health.toLocaleString()}</span>
					</div>
				{/if}
				{#if stats.power}
					<div class="stat-item">
						<span class="stat-label">{stats.power_type?.name ?? 'Power'}</span>
						<span class="stat-value">{stats.power.toLocaleString()}</span>
					</div>
				{/if}
				{#if stats.strength}
					<div class="stat-item">
						<span class="stat-label">Strength</span>
						<span class="stat-value">{stats.strength.effective}</span>
					</div>
				{/if}
				{#if stats.agility}
					<div class="stat-item">
						<span class="stat-label">Agility</span>
						<span class="stat-value">{stats.agility.effective}</span>
					</div>
				{/if}
				{#if stats.intellect}
					<div class="stat-item">
						<span class="stat-label">Intellect</span>
						<span class="stat-value">{stats.intellect.effective}</span>
					</div>
				{/if}
				{#if stats.stamina}
					<div class="stat-item">
						<span class="stat-label">Stamina</span>
						<span class="stat-value">{stats.stamina.effective}</span>
					</div>
				{/if}
			</div>
		</div>
	{:else}
		<p class="no-data">No statistics data available</p>
	{/if}
</div>

<style>
	.stats-panel {
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

	h3 {
		margin: 0 0 0.5rem;
		font-size: 1rem;
		color: var(--color-heading, #e0e0e0);
	}

	.section {
		margin-bottom: 1.5rem;
	}

	.section:last-child {
		margin-bottom: 0;
	}

	.active-spec {
		margin: 0;
		color: var(--color-text, #ccc);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
		gap: 0.5rem;
	}

	.stat-item {
		display: flex;
		justify-content: space-between;
		padding: 0.375rem 0.625rem;
		background: var(--color-item-bg, #252525);
		border-radius: 4px;
	}

	.stat-label {
		color: var(--color-muted, #888);
		font-size: 0.875rem;
	}

	.stat-value {
		color: var(--color-text, #e0e0e0);
		font-weight: 500;
	}

	.no-data {
		color: var(--color-muted, #888);
		margin: 0;
	}
</style>
