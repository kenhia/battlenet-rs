<script lang="ts">
	import type { FullCharacter } from '$lib/types';

	let { character }: { character: FullCharacter } = $props();

	const profile = $derived(character.profile);

	function getPortraitUrl(): string | null {
		if (!character.media?.assets) return null;
		const avatar = character.media.assets.find((a) => a.key === 'avatar');
		return avatar?.value ?? null;
	}

	const portraitUrl = $derived(getPortraitUrl());
</script>

<div class="character-header">
	{#if portraitUrl}
		<img class="portrait" src={portraitUrl} alt="{character.character_name} portrait" />
	{/if}

	<div class="info">
		<h1 class="name">{character.character_name}</h1>

		{#if profile}
			<p class="details">
				Level {profile.level}
				{profile.race?.name ?? ''}
				{profile.character_class?.name ?? ''}
			</p>

			{#if profile.active_title}
				<p class="title">&lt;{profile.active_title.name}&gt;</p>
			{/if}

			{#if profile.guild}
				<p class="guild">&lt;{profile.guild.name}&gt; — {profile.guild.realm.name}</p>
			{/if}

			<div class="stats-row">
				{#if profile.faction}
					<span class="stat faction-{profile.faction.type_?.toLowerCase()}">{profile.faction.name}</span>
				{/if}
				{#if profile.equipped_item_level}
					<span class="stat">iLvl {profile.equipped_item_level}</span>
				{/if}
				{#if profile.achievement_points}
					<span class="stat">{profile.achievement_points} AP</span>
				{/if}
			</div>
		{:else}
			<p class="no-data">No profile data available</p>
		{/if}
	</div>
</div>

<style>
	.character-header {
		display: flex;
		gap: 1.5rem;
		padding: 1.5rem;
		background: var(--color-surface, #1e1e1e);
		border-radius: 8px;
		border: 1px solid var(--color-border, #444);
	}

	.portrait {
		width: 84px;
		height: 84px;
		border-radius: 4px;
		border: 2px solid var(--color-border, #444);
		object-fit: cover;
	}

	.info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.name {
		font-size: 1.5rem;
		margin: 0;
		color: var(--color-heading, #e0e0e0);
	}

	.details {
		margin: 0;
		color: var(--color-text, #ccc);
	}

	.title {
		margin: 0;
		color: var(--color-muted, #888);
		font-style: italic;
	}

	.guild {
		margin: 0;
		color: var(--color-muted, #888);
	}

	.stats-row {
		display: flex;
		gap: 1rem;
		margin-top: 0.5rem;
	}

	.stat {
		font-size: 0.875rem;
		padding: 0.125rem 0.5rem;
		border-radius: 4px;
		background: var(--color-badge-bg, #333);
		color: var(--color-text, #ccc);
	}

	.faction-alliance {
		background: #1a3a5c;
		color: #7ab3ff;
	}

	.faction-horde {
		background: #5c1a1a;
		color: #ff7a7a;
	}

	.no-data {
		color: var(--color-muted, #888);
		margin: 0;
	}
</style>
