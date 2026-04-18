import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import CharacterHeader from '../../src/lib/components/CharacterHeader.svelte';
import type { FullCharacter } from '../../src/lib/types';

function mockFullCharacter(): FullCharacter {
	return {
		realm_slug: 'trollbane',
		character_name: 'Belarsa',
		has_profile_data: true,
		fetched_at: '2026-04-17T00:00:00Z',
		errors: [],
		profile: {
			name: 'Belarsa',
			level: 80,
			race: { name: 'Void Elf', id: 29 },
			character_class: { name: 'Mage', id: 8 },
			faction: { type_: 'ALLIANCE', name: 'Alliance' },
			guild: { name: 'Test Guild', realm: { name: 'Trollbane', slug: 'trollbane' } },
			realm: { name: 'Trollbane', slug: 'trollbane', id: 1 },
			active_title: { name: 'the Patient' },
			achievement_points: 12345,
			equipped_item_level: 620,
			average_item_level: 615
		},
		equipment: null,
		statistics: null,
		specializations: null,
		media: {
			assets: [
				{ key: 'avatar', value: 'https://render.worldofwarcraft.com/avatar.jpg' },
				{ key: 'main-raw', value: 'https://render.worldofwarcraft.com/main.jpg' }
			]
		}
	};
}

describe('CharacterHeader', () => {
	it('renders character name and level', () => {
		const character = mockFullCharacter();
		render(CharacterHeader, { props: { character } });
		expect(screen.getByText('Belarsa')).toBeTruthy();
		expect(screen.getByText(/80/)).toBeTruthy();
	});

	it('renders race and class', () => {
		const character = mockFullCharacter();
		render(CharacterHeader, { props: { character } });
		expect(screen.getByText(/Void Elf/)).toBeTruthy();
		expect(screen.getByText(/Mage/)).toBeTruthy();
	});

	it('renders faction', () => {
		const character = mockFullCharacter();
		render(CharacterHeader, { props: { character } });
		expect(screen.getByText(/Alliance/)).toBeTruthy();
	});

	it('renders guild name', () => {
		const character = mockFullCharacter();
		render(CharacterHeader, { props: { character } });
		expect(screen.getByText(/Test Guild/)).toBeTruthy();
	});

	it('renders item level', () => {
		const character = mockFullCharacter();
		render(CharacterHeader, { props: { character } });
		expect(screen.getByText(/620/)).toBeTruthy();
	});

	it('renders achievement points', () => {
		const character = mockFullCharacter();
		render(CharacterHeader, { props: { character } });
		expect(screen.getByText(/12345/)).toBeTruthy();
	});

	it('renders portrait image when media is available', () => {
		const character = mockFullCharacter();
		render(CharacterHeader, { props: { character } });
		const img = document.querySelector('img');
		expect(img).toBeTruthy();
		expect(img?.src).toContain('avatar.jpg');
	});
});
